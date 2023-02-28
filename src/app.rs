use core::fmt::Write;
use klaptik::*;
use pid_loop::PID;

pub enum Button {
    ButtonA,
    ButtonB,
}

pub struct AppState {
    frame: usize,
    set_temp: u16,
    cur_temp: u16,
    heater_power: u16,
    heater_duty: u16,
    heater_max_duty: u16,
}

pub struct App {
    reg: PID<f32, 4>,
    state: AppState,
}

impl App {
    pub fn new(heater_max_duty: u16) -> Self {
        Self {
            reg: PID::new(0.02, 0.05, 0.0, 0.0, 0.0),
            state: AppState {
                frame: 0,
                heater_max_duty,
                heater_duty: 0,
                heater_power: 0,
                cur_temp: 0,
                set_temp: 24_000,
            },
        }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn animate(&mut self) {
        self.state.frame += 1;
    }

    pub fn button_click(&mut self, btn: Button) {
        match btn {
            Button::ButtonA => {
                self.state.set_temp = self
                    .state
                    .set_temp
                    .saturating_sub(100)
                    .clamp(15_000, 40_000)
            }
            Button::ButtonB => {
                self.state.set_temp = self
                    .state
                    .set_temp
                    .saturating_add(100)
                    .clamp(15_000, 40_000)
            }
        }
    }

    pub fn get_heater_duty(&mut self, temp: u16) -> u16 {
        let temp = (5296 - temp) / 12 * 100;
        self.state.cur_temp = temp;

        let correction = self.reg.next(self.state.set_temp, temp);
        let duty = self.state.heater_duty as i32;

        self.state.heater_duty = duty
            .saturating_add(correction as _)
            .clamp(0, self.state.heater_max_duty as i32) as _;

        self.state.heater_power =
            (self.state.heater_duty as f32 * 100.0 / self.state.heater_max_duty as f32) as u16;
        defmt::info!(
            "{} ({})\t{}%",
            temp,
            self.state.set_temp,
            self.state.heater_power
        );
        self.state.heater_duty.min(self.state.heater_max_duty)
    }
}

pub enum Asset {
    Background = 0,
    FontSmall = 1,
    FontLarge = 2,
}

impl From<Asset> for SpriteId {
    fn from(asset: Asset) -> Self {
        asset as _
    }
}

pub const SPRITES: [(FlashSprite, Glyphs); 3] = [
    (
        FlashSprite::new(
            Asset::Background as _,
            2,
            Size::new(128, 64),
            include_bytes!("assets/background.bin"),
        ),
        Glyphs::Sequential(2),
    ),
    (
        FlashSprite::new(
            Asset::FontSmall as _,
            11,
            Size::new(10, 16),
            include_bytes!("assets/font_small.bin"),
        ),
        Glyphs::Alphabet(b" 0123456789"),
    ),
    (
        FlashSprite::new(
            Asset::FontLarge as _,
            11,
            Size::new(36, 32),
            include_bytes!("assets/font_large.bin"),
        ),
        Glyphs::Alphabet(b" 0123456789"),
    ),
];

widget_group! {
    UI<&AppState>,
    {
      bg: GlyphIcon, Asset::Background, 1, Point::zero();
      heater_power: Label<2>, Asset::FontSmall, "00", Point::new(18, 0), Size::new(10, 16);
      cur_temp: Label<2>, Asset::FontSmall, "00", Point::new(76, 0), Size::new(10, 16);
      cur_temp_frac: Label<1>, Asset::FontSmall, "0", Point::new(100, 0), Size::new(10, 16);
      set_temp: Label<2>, Asset::FontLarge, "00", Point::new(2, 24), Size::new(36, 32);
      set_temp_frac: Label<1>, Asset::FontLarge, "0", Point::new(86, 24), Size::new(36, 32);
    },
    |widget: &mut UI, state: &AppState| {
        write!(widget.heater_power, "{:0>2}", state.heater_power.clamp(0, 99)).ok();

        write!(widget.cur_temp, "{: >2}", state.cur_temp / 1000).ok();
        write!(widget.cur_temp_frac, "{}", (state.cur_temp / 100) % 10).ok();

        write!(widget.set_temp, "{: >2}", state.set_temp / 1000).ok();
        write!(widget.set_temp_frac, "{}", (state.set_temp / 100) % 10).ok();
    }
}
