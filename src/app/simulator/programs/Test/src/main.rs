#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{block, pac, prelude::*, serial::{Config, Serial}};

use core::fmt::Write;
use fugit::MicrosDurationU32;

use core::ops::Range;


#[entry]
fn main() -> ! {
    // Getting device and cortex peripherals 
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Splitting GPIOA and GPIOD into separate pins
    let gpioa = dp.GPIOA.split();
    let gpiod = dp.GPIOD.split();

    // Configuring Rest and Clock Control, sets up deviced at high speeds
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Delay function being initialized
    let mut delay = cp.SYST.delay(&clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;

    // configure serial
    let serial = dp
        .USART2
        .serial(
            (tx_pin, rx_pin),
            Config::default().baudrate(9600.bps()).wordlength_8(),
            &clocks,
        )
        .unwrap();
        // if we want differnet size data payload, we need to add additional lines here
        
    // split the serial struct into tx and rx and announce the start
    let (mut tx, mut rx) = serial.split();
    writeln!(tx, "Getting Started in UART\r").unwrap();
    
    loop {
        let received: u8 = block!(rx.read()).unwrap();
        let receivedChar = received as char;
                
        match receivedChar {
            '1' => {
                writeln!(tx, "LED 5 ON").unwrap();
            }
            '\r' => {
                writeln!(tx, "").unwrap();
            }
            _=>{
                writeln!(tx, "Received: {receivedChar:02}").unwrap();
                // write to a file in the same directory
                
            }
        }

        writeln!(tx, "\r").unwrap();
        delay.delay(MicrosDurationU32::millis(100));
    }
}