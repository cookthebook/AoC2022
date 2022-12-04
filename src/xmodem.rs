use util;
use hal::prelude::*;
use hal::serial::{FullConfig, Serial};
use core::fmt::Write;
use nb::block;

pub const BLOCK_SZ: usize = 128;
pub const XMODEM_PAD: u8 = 0x1a;

const XMODEM_SOH: u8 = 0x01;
const XMODEM_EOT: u8 = 0x04;
const XMODEM_ACK: u8 = 0x06;
const XMODEM_NAK: u8 = 0x15;

pub struct Xmodem {
    p: util::CorePerphs,
    packets_read: u32,
    pkt_num: u8,
    error_cnt: u32
}

impl Xmodem {
    pub fn xmodem_begin(&mut self, buf: &mut [u8; BLOCK_SZ]) -> u8 {
        self.pkt_num = 1; /* XModem packet indeces start at 1 */
        self.error_cnt = 0;
        self.packets_read = 0;

        /* Send periodic start byte to begin transfer */
        for _ in 0..15 {
            self.p.uart.write(XMODEM_NAK).ok();

            if self.read_soh() == XMODEM_SOH {
                return self.xmodem_getpkt(buf);
            }
        }

        write!(self.p.uart, "Failed to begin XModem transfer\r\n").ok();
        return XMODEM_EOT;
    }

    pub fn xmodem_getpkt(&mut self, buf: &mut [u8; BLOCK_SZ]) -> u8 {
        loop {
            if self.packets_read != 0 {
                /* for all blocks after the first, we send an ack when we get
                 * back in here */
                self.p.uart.write(XMODEM_ACK).ok();

                let byte = self.read_soh();

                match byte {
                    XMODEM_SOH => {
                        /* OK */
                    }

                    XMODEM_EOT => {
                        self.p.uart.write(XMODEM_ACK).ok();
                        write!(self.p.uart, "XModem transfer done\r\n");
                        return XMODEM_EOT
                    }

                    _ => {
                        self.error_cnt += 1;
                        self.p.uart.write(XMODEM_NAK).ok();
                        continue;
                    }
                }
            }

            let (pkt, pktc, sz, csum) = self.read_packet(buf);

            if pkt != (!pktc) ||
                sz != BLOCK_SZ ||
                csum != self.csum(*buf) {
                self.error_cnt += 1;
                self.p.uart.write(XMODEM_NAK).ok();
                continue;
            }

            break;
        }

        return XMODEM_ACK;
    }

    fn read_soh(&mut self) -> u8 {
        self.p.watch.reset();
        self.p.watch.resume();

        let start = self.p.watch.now();

        loop {
            match self.p.uart.read() {
                Err(nb::Error::Other(_err)) => {
                    return 0;
                }

                Err(nb::Error::WouldBlock) => {
                    if self.p.watch.now().0 > (start.0 + (3*1000*1000)) {
                        return 0;
                    }
                }

                Ok(val) => {
                    return val;
                }
            }
        }
    }

    fn read_packet(&mut self, buf: &mut [u8; BLOCK_SZ]) -> (u8, u8, usize, u8) {
        let pkt;
        let pktc;
        let mut sz: usize = 0;
        let csum;
        let mut i = 0;

        while i < BLOCK_SZ+3 {
            match self.p.uart.read() {
                Err(nb::Error::Other(_err)) => {
                    break;
                },

                Err(nb::Error::WouldBlock) => {
                    continue;
                },

                Ok(val) => {
                    if i == 0 {
                        pkt = val;
                    }
                    else if i == 1 {
                        pktc = val;
                    }
                    else if i == (BLOCK_SZ + 2) {
                        csum = val;
                    }
                    else {
                        buf[sz] = val;
                        sz += 1;
                    }
                }
            }
        }

        return (pkt, pktc, sz, csum);
    }

    fn csum(&mut self, buf: [u8; BLOCK_SZ]) -> u8 {
        let mut sum: u8 = 0;

        for i in 0..BLOCK_SZ {
            sum += buf[i];
        }

        return (sum & 0x00ff) as u8;
    }
}
