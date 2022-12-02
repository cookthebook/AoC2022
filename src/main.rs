#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;
extern crate rtt_target;
extern crate nb;

use core::fmt::Write;
use cortex_m_rt::entry;

use hal::{prelude::*, serial::Serial};
use hal::serial::FullConfig;
use hal::stm32;

mod util;
mod day01;
mod day02;

use util::{readln, strtoul};

fn ask_chal(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];

    writeln!(uart, "Please select a day").ok();

    let rlen = readln(uart, &mut buf);
    let selection = strtoul(&buf, 10);

    match selection {
    1 => {
        write!(uart, "You selected day 1").ok();
        if buf[1] == b'*' {
            write!(uart, "*\r\n").ok();
            day01::solve_star(uart);
        } else {
            write!(uart, "\r\n").ok();
            day01::solve(uart);
        }
    },

    2 => {
        write!(uart, "You selected day 2").ok();
        if buf[1] == b'*' {
            write!(uart, "*\r\n").ok();
            day02::solve_star(uart);
        } else {
            write!(uart, "\r\n").ok();
            day02::solve(uart);
        }
    },

    _ => {
        write!(uart, "Invalid selection: ").ok();
        for i in 0..rlen {
            uart.write(buf[i]).ok();
        }
        write!(uart, "\r\n").ok();
    }
    }
}

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut usart = dp.USART1.usart(
        gpioc.pc4, gpioc.pc5,
        FullConfig::default().baudrate(115200.bps()),
        &mut rcc
    ).unwrap();

    loop {
        ask_chal(&mut usart);
    }
}
