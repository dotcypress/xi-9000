#![no_std]
#![no_main]

use defmt_rtt as _;

extern crate stm32g0xx_hal as hal;

#[cfg(feature = "probe")]
extern crate panic_probe;

#[cfg(not(feature = "probe"))]
extern crate panic_halt;

mod app;
mod display;
mod pins;

use app::*;
use display::*;
use hal::analog::adc;
use hal::{exti::*, gpio::*, prelude::*, rcc, spi, stm32, stm32::*, timer::*, watchdog};
use klaptik::*;
use pins::*;

#[rtic::app(device = stm32, peripherals = true, dispatchers = [USART1, USART2])]
mod xi {
    use super::*;

    #[shared]
    struct Shared {
        app: App,
    }

    #[local]
    struct Local {
        adc: adc::Adc,
        display: SpriteDisplay<DisplayController, { SPRITES.len() }>,
        exti: EXTI,
        heater: pwm::PwmPin<TIM1, Channel1>,
        reg_timer: Timer<stm32::TIM16>,
        temp_sense: GpioB1,
        ui: UI,
        ui_timer: Timer<stm32::TIM17>,
        watchdog: watchdog::IndependedWatchdog,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = ctx.device.RCC.freeze(rcc::Config::pll());
        let mut exti = ctx.device.EXTI;

        let pins = Pins::new(
            ctx.device.GPIOA,
            ctx.device.GPIOB,
            ctx.device.GPIOC,
            &mut rcc,
        );
        pins.gpio_a1.listen(SignalEdge::Falling, &mut exti);
        pins.gpio_a2.listen(SignalEdge::Falling, &mut exti);

        let mut adc = ctx.device.ADC.constrain(&mut rcc);
        adc.set_sample_time(adc::SampleTime::T_160);
        adc.set_precision(adc::Precision::B_12);
        adc.set_oversampling_ratio(adc::OversamplingRatio::X_16);
        adc.set_oversampling_shift(20);
        adc.oversampling_enable(true);

        let backlight_pwm = ctx.device.TIM14.pwm(16.kHz(), &mut rcc);
        let mut lcd_backlight = backlight_pwm.bind_pin(pins.lcd_backlight);
        lcd_backlight.enable();
        lcd_backlight.set_duty(0);

        let heater_pwm = ctx.device.TIM1.pwm(8.kHz(), &mut rcc);
        let mut heater = heater_pwm.bind_pin(pins.gpio_b2);
        heater.set_duty(0);
        heater.enable();

        let spi = ctx.device.SPI1.spi(
            (pins.spi_clk, pins.spi_miso, pins.spi_mosi),
            spi::MODE_0,
            16.MHz(),
            &mut rcc,
        );

        let mut delay = ctx.device.TIM3.delay(&mut rcc);
        let display = DisplayController::new(
            spi,
            pins.lcd_reset,
            pins.lcd_cs,
            pins.lcd_dc,
            lcd_backlight,
            &mut delay,
        );

        let temp_sense = pins.gpio_b1;
        adc.calibrate();

        let mut reg_timer = ctx.device.TIM16.timer(&mut rcc);
        reg_timer.start(200.millis());
        reg_timer.listen();

        let mut ui_timer = ctx.device.TIM17.timer(&mut rcc);
        ui_timer.start(350.millis());
        ui_timer.listen();

        let mut display = SpriteDisplay::new(display, SPRITES);
        display.canvas().power_on();

        let app = App::new(heater.get_max_duty());
        let ui = UI::new();

        let mut watchdog = ctx.device.IWDG.constrain();
        watchdog.start(300.millis());

        (
            Shared { app },
            Local {
                adc,
                display,
                exti,
                heater,
                temp_sense,
                ui,
                ui_timer,
                reg_timer,
                watchdog,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM16, local = [adc, heater, reg_timer, temp_sense, watchdog], shared = [app])]
    fn reg_timer_tick(ctx: reg_timer_tick::Context) {
        let mut app = ctx.shared.app;
        let temp: u16 = ctx.local.adc.read(ctx.local.temp_sense).unwrap_or(0);
        let duty = app.lock(|app| app.get_heater_duty(temp));
        ctx.local.heater.set_duty(duty);
        ctx.local.reg_timer.clear_irq();
        ctx.local.watchdog.feed();
    }

    #[task(binds = TIM17, local = [ui, ui_timer, display], shared = [app])]
    fn ui_timer_tick(ctx: ui_timer_tick::Context) {
        let mut app = ctx.shared.app;
        app.lock(|app| {
            app.animate();
            ctx.local.ui.update(app.state());
        });
        ctx.local.ui.render(ctx.local.display);
    }

    #[task(binds = EXTI0_1, local = [exti], shared = [app])]
    fn button_click(ctx: button_click::Context) {
        let mut app = ctx.shared.app;
        if ctx.local.exti.is_pending(Event::GPIO0, SignalEdge::Falling) {
            ctx.local.exti.unpend(Event::GPIO0);
            app.lock(|app| app.button_click(Button::ButtonB));
        }
        if ctx.local.exti.is_pending(Event::GPIO1, SignalEdge::Falling) {
            ctx.local.exti.unpend(Event::GPIO1);
            app.lock(|app| app.button_click(Button::ButtonA));
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            if cfg!(feature = "probe") {
                rtic::export::nop();
            } else {
                rtic::export::wfi();
            }
        }
    }
}
