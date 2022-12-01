#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;
extern crate rtt_target;

use core::fmt::Write;
use cortex_m_rt::entry;

use rtt_target::{rtt_init, UpChannel, DownChannel};

mod util;
mod day01;

use util::{readln, strtoul};

fn ask_chal(input: &mut DownChannel, output: &mut UpChannel) {
    let mut buf = [0u8; 16];

    writeln!(output, "Please select a day").ok();

    let rlen = readln(input, &mut buf);
    let selection = strtoul(&buf, 10);

    match selection {
    1 => {
        writeln!(output, "You selected day 1").ok();

        if buf[1] == b'*' {
            day01::solve_star(input, output);
        } else {
            day01::solve(input, output);
        }
    }

    _ => {
        write!(output, "Invalid selection: ").ok();
        output.write(&buf[0..rlen]);
    }
    }
}

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: {
                size: 512
                mode: BlockIfFull
                name: "Device Output"
            }
        }
        down: {
            0: {
                size: 512
                mode: BlockIfFull
                name: "Device Input"
            }
        }
    };

    let mut output: UpChannel = channels.up.0;
    let mut input: DownChannel = channels.down.0;

    loop {
        ask_chal(&mut input, &mut output);
    }
}
