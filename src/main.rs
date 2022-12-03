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
mod day03;

use util::{readln, strtoul, print_buf};

fn print_help(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    write!(uart, "Input the day you wish to solve, followed by a \"*\" for the second half\r\n").ok();
}

fn ask_chal(uart: &mut Serial<hal::stm32::USART1, FullConfig>) {
    let mut buf = [0u8; 16];
    let rlen: usize;
    let selection: u64;

    write!(uart, "AoC 2022> ").ok();
    rlen = readln(uart, &mut buf);
    print_buf(uart, &buf, rlen);
    write!(uart, "\r\n").ok();

    selection = strtoul(&buf, 10);

    match selection {
    1 => {
        if buf[1] == b'*' {
            day01::solve_star(uart);
        } else {
            day01::solve(uart);
        }
    },

    2 => {
        if buf[1] == b'*' {
            day02::solve_star(uart);
        } else {
            day02::solve(uart);
        }
    },

    3 => {
        if buf[1] == b'*' {
            day03::solve_star(uart);
        } else {
            day03::solve(uart);
        }
    },

    _ => {
        write!(uart, "Invalid selection\r\n").ok();
        print_help(uart);
    }
    }
}

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();

    let periphs: util::CorePerphs;

    /* setup hardware timer */
    periphs.timer = dp.TIM17.timer(&mut rcc);

    /* setup UART interface */
    let gpioc = dp.GPIOC.split(&mut rcc);
    periphs.uart = dp.USART1.usart(
        gpioc.pc4, gpioc.pc5,
        FullConfig::default().baudrate(115200.bps()),
        &mut rcc
    ).unwrap();


    write!(periphs.uart, "\r\n~~~ Advent of Code 2022 ~~~\r\n").ok();
    write!(periphs.uart, "_________________________\r\n").ok();
    write!(periphs.uart, "< Time to save Christmas! >\r\n").ok();
    write!(periphs.uart, " -------------------------\r\n").ok();
    write!(periphs.uart, "        \\\r\n").ok();
    write!(periphs.uart, "         \\\r\n").ok();
    write!(periphs.uart, "            _~^~^~_\r\n").ok();
    write!(periphs.uart, "        \\) /  o o  \\ (/\r\n").ok();
    write!(periphs.uart, "          '_   -   _'\r\n").ok();
    write!(periphs.uart, "          / '-----' \\\r\n\r\n").ok();
    loop {
        ask_chal(&mut usart);
    }
}
