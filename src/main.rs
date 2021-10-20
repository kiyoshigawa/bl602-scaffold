#![no_std]
#![no_main]

use bl602_hal as hal;
use core::fmt::Write;
use embedded_hal::delay::blocking::DelayMs;
use hal::{
    clock::{Strict, SysclkFreq, UART_PLL_FREQ},
    pac,
    prelude::*,
    serial::*,
};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    // Take control of the device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let mut gpio_pins = dp.GLB.split();

    // Set up all the clocks we need
    let clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(&mut gpio_pins.clk_cfg);

    // Set up uart output for debug printing. Since this microcontroller has a pin matrix,
    // we need to set up both the pins and the muxs
    let pin16 = gpio_pins.pin16.into_uart_sig0();
    let pin7 = gpio_pins.pin7.into_uart_sig7();
    let mux0 = gpio_pins.uart_mux0.into_uart0_tx();
    let mux7 = gpio_pins.uart_mux7.into_uart0_rx();

    // Configure our UART to 2MBaud, and use the pins we configured above
    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(2_000_000.Bd()),
        ((pin16, mux0), (pin7, mux7)),
        clocks,
    );

    // Create a blocking delay function based on the current cpu frequency
    let mut d = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

    writeln!(serial, "Debug Serial Initialized...\r\n").ok();
    let mut i = 0_u32;
    loop {
        i += 1;
        // Send a message every second while the device is running:
        writeln!(serial, "We've looped {} times.", i).ok();
        d.delay_ms(1000).ok();
    }
}
