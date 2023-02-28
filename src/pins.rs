use crate::hal::gpio::{gpioa::*, gpiob::*, gpioc::*};
use hal::gpio::*;
use hal::gpio::{DefaultMode, OpenDrain, Output, PushPull};
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::spi;
use hal::stm32::*;

pub type SpiDev = spi::Spi<SPI1, (SpiClk, SpiMiso, SpiMosi)>;

// SWD
pub type SwdIo = PA13<DefaultMode>;
pub type SwdClk = PA14<DefaultMode>;

// Qwiic I2C
pub type I2cClk = PB8<Output<OpenDrain>>;
pub type I2cSda = PB9<Output<OpenDrain>>;

// SPI
pub type SpiClk = PA5<DefaultMode>;
pub type SpiMiso = PA6<DefaultMode>;
pub type SpiMosi = PA7<DefaultMode>;

// Display
pub type LcdDC = PA3<Output<PushPull>>;
pub type LcdCS = PA11<Output<PushPull>>;
pub type LcdReset = PA12<Output<PushPull>>;
pub type LcdBacklight = PA4<DefaultMode>;

// EEPROM
pub type EepromCS = PC15<Output<PushPull>>;

// GPIO
pub type GpioA1 = PA1<Input<Floating>>;
pub type GpioA2 = PA0<Input<Floating>>;
pub type GpioB1 = PA2<Analog>;
pub type GpioB2 = PA8<DefaultMode>;
pub type GpioB3 = PB3<DefaultMode>;

pub struct Pins {
    // SWD
    pub swd_io: SwdIo,
    pub swd_clk: SwdClk,

    // Qwiic I2C
    pub i2c_clk: I2cClk,
    pub i2c_sda: I2cSda,

    // SPI
    pub spi_clk: SpiClk,
    pub spi_miso: SpiMiso,
    pub spi_mosi: SpiMosi,

    // Display
    pub lcd_dc: LcdDC,
    pub lcd_backlight: LcdBacklight,
    pub lcd_cs: LcdCS,
    pub lcd_reset: LcdReset,

    // EEPROM
    pub eeprom_cs: EepromCS,

    // GPIO
    pub gpio_a1: GpioA1,
    pub gpio_a2: GpioA2,
    pub gpio_b1: GpioB1,
    pub gpio_b2: GpioB2,
    pub gpio_b3: GpioB3,
}

impl Pins {
    pub fn new(gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC, rcc: &mut Rcc) -> Self {
        let port_a = gpioa.split(rcc);
        let port_b = gpiob.split(rcc);
        let port_c = gpioc.split(rcc);

        Self {
            // SWD
            swd_io: port_a.pa13,
            swd_clk: port_a.pa14,

            // Qwiic I2C
            i2c_clk: port_b
                .pb8
                .set_speed(Speed::High)
                .into_open_drain_output_in_state(PinState::High),
            i2c_sda: port_b
                .pb9
                .set_speed(Speed::High)
                .into_open_drain_output_in_state(PinState::High),

            //SPI
            spi_clk: port_a.pa5.set_speed(Speed::VeryHigh),
            spi_miso: port_a.pa6.set_speed(Speed::VeryHigh),
            spi_mosi: port_a.pa7.set_speed(Speed::VeryHigh),

            // Display
            lcd_backlight: port_a.pa4,
            lcd_dc: port_a.pa3.into(),
            lcd_reset: port_a.pa12.into(),
            lcd_cs: port_a.pa11.into_push_pull_output_in_state(PinState::High),

            // EEPROM
            eeprom_cs: port_c.pc15.into_push_pull_output_in_state(PinState::High),

            // GPIO
            gpio_a1: port_a.pa1.into(),
            gpio_a2: port_a.pa0.into(),
            gpio_b1: port_a.pa2,
            gpio_b2: port_a.pa8,
            gpio_b3: port_b.pb3,
        }
    }
}
