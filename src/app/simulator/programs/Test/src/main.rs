#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*, serial::{Config, Serial}};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let tx_pin = gpioa.pa2.into_alternate();
    //let rx_pin = gpioa.pa3.into_alternate();

    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default()
                  .baudrate(115200.bps())
                  .wordlength_8()
                  .parity_none(),
        &clocks,
        )
      .unwrap();
    

    writeln!(tx, "UART Test Initialized\r").unwrap();

    loop {
        writeln!(tx, "USART OUTPUT ENABLED\r").unwrap();
        cortex_m::asm::delay(8_000_000);
    }
}


// use stm32f4xx_hal::{
//     pac,
//     prelude::*,
//     serial::{config::Config, Serial, },
// };
// use cortex_m_rt::entry;
// use core::panic::PanicInfo;
// use panic_halt as _;
// use fugit::{RateExtU32, ExtU32};
// use embedded_hal::serial::Write;

// #[entry]
// fn main() -> ! {
//     let dp = pac::Peripherals::take().unwrap();
//     let rcc = dp.RCC.constrain();
//     let clocks = rcc
//         .cfgr
//         .use_hse(8.MHz())
//         .sysclk(168.MHz())
//         .pclk1(24.MHz())
//         .i2s_clk(86.MHz())
//         .require_pll48clk()
//         .freeze();
//         // Test that the I2S clock is suitable for 48000kHz audio.

//     let gpioa = dp.GPIOA.split();
//     let tx = gpioa.pa9.into_alternate();
//     let rx = gpioa.pa10.into_alternate();

//     // Configure the UART
//     let serial = Serial::<_, _, u8>::new( 
//         dp.USART1,         
//         (tx, rx),
//         Config::default().baudrate(9600.bps()),
//         &clocks,
//     );
//     let (mut tx, _rx) = serial.split();
//     // Send a string through UART
//     loop {
//         write_str("Hello, world!").ok();
//         // Optionally, add a delay here to control the message frequency
//         cortex_m::asm::delay(8_000_000);
//     }
// }