use crate::*;
use hal::timer::delay::Delay;
use hal::timer::pwm::PwmPin;
use klaptik::drivers::st7567::*;
use klaptik::Point;

pub type DisplayDriver = ST7567<SpiDev, LcdReset, LcdCS, LcdDC>;
pub type Backlight = PwmPin<TIM14, Channel1>;

pub struct DisplayController {
    backlight_pwm: Backlight,
    canvas: DisplayDriver,
}

impl DisplayController {
    pub fn new(
        spi: SpiDev,
        lcd_reset: LcdReset,
        lcd_cs: LcdCS,
        lcd_dc: LcdDC,
        backlight_pwm: Backlight,
        delay: &mut Delay<TIM3>,
    ) -> Self {
        let mut canvas = ST7567::new(spi, lcd_cs, lcd_dc, lcd_reset);
        canvas.set_offset(Point::new(4, 0));
        canvas.reset(delay);
        canvas
            .link()
            .command(|tx| tx.write(&[Command::SegmentDirectionRev as _]))
            .ok();
        Self {
            backlight_pwm,
            canvas,
        }
    }

    pub fn power_on(&mut self) {
        self.backlight_pwm
            .set_duty(self.backlight_pwm.get_max_duty() / 2);
        self.canvas.on();
    }
}

impl Canvas for DisplayController {
    fn draw(&mut self, bounds: Rectangle, bitmap: &[u8]) {
        self.canvas.draw(bounds, bitmap);
    }
}
