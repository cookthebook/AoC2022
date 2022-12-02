use hal::prelude::*;
use hal::serial::{FullConfig, Serial};
use nb::block;

pub fn readln(uart: &mut Serial<hal::stm32::USART1, FullConfig>, buf: &mut [u8]) -> usize {
    let mut p = 0;

    while p < buf.len() {
        let byte = block!(uart.read()).unwrap();

        if byte == b'\r' {
            continue;
        } else if byte == b'\n' {
            break;
        }

        buf[p] = byte;
        p += 1;
    }

    return p;
}

pub fn strtoul(buf: &[u8], base: u8) -> u64 {
    let mut p = 0;
    let mut ret: u64 = 0;

    match base {
    10 => {
        while p < buf.len() {
            if !buf[p].is_ascii_digit() {
                break;
            }

            ret *= 10;
            ret += (buf[p] - b'0') as u64;
            p += 1;
        }
    },

    16 => {
        while p < buf.len() {
            let h: u8;

            if buf[p].is_ascii_digit() {
                h = buf[p] - b'0';
            } else if buf[p].is_ascii_digit() {
                if buf[p].is_ascii_uppercase() {
                    h = buf[p] - b'A' + 0x0a;
                } else {
                    h = buf[p] - b'a' + 0x0a;
                }
            } else {
                break;
            }

            ret *= 16;
            ret += h as u64;
            p += 1;
        }
    }

    _ => {}
    }

    return ret;
}