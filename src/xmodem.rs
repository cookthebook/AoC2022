use util;
use hal::prelude::*;
use core::fmt::Write;

pub const BLOCK_SZ: usize = 128;
pub const XMODEM_PAD: u8 = 0x1a;

pub const XMODEM_SOH: u8 = 0x01;
pub const XMODEM_EOT: u8 = 0x04;
pub const XMODEM_ACK: u8 = 0x06;
pub const XMODEM_NAK: u8 = 0x15;
pub const XMODEM_CAN: u8 = 0x18;

pub struct Xmodem {
    packets_read: u32,
    pkt_num: u8,
    error_cnt: u32
}

impl Xmodem {
    pub fn new() -> Xmodem {
        return Xmodem {
            packets_read: 0,
            pkt_num: 1, /* XModem packet indeces start at 1 */
            error_cnt: 0,
        };
    }

    pub fn xmodem_begin(&mut self, p: &mut util::CorePerphs, buf: &mut [u8; BLOCK_SZ]) -> u8 {
        write!(p.uart, "Beginning XModem transfer...\r\n").ok();

        /* Send periodic start byte to begin transfer */
        for _ in 0..15 {
            p.uart.write(XMODEM_NAK).ok();
            p.uart.flush().ok();

            if self.read_soh(p) == XMODEM_SOH {
                return self.xmodem_getpkt(p, buf);
            }
        }

        write!(p.uart, "Failed to begin XModem transfer\r\n").ok();
        return XMODEM_EOT;
    }

    pub fn xmodem_getpkt(&mut self, p: &mut util::CorePerphs, buf: &mut [u8; BLOCK_SZ]) -> u8 {
        loop {
            if self.packets_read != 0 {
                /* for all blocks after the first, we send an ack when we get
                 * back in here */
                p.uart.write(XMODEM_ACK).ok();
                p.uart.flush().ok();

                let byte = self.read_soh(p);

                match byte {
                    XMODEM_SOH => {
                        /* OK */
                    },

                    XMODEM_EOT => {
                        p.uart.write(XMODEM_ACK).ok();
                        p.uart.flush().ok();
                        write!(p.uart, "XModem transfer done ({} errors)\r\n", self.error_cnt).ok();
                        return XMODEM_EOT;
                    },

                    _ => {
                        self.error_cnt += 1;
                        p.uart.write(XMODEM_NAK).ok();
                        p.uart.flush().ok();
                        continue;
                    }
                }
            }

            let (pkt, pktc, sz, csum) = self.read_packet(p, buf);

            if pkt != (!pktc) ||
                sz != BLOCK_SZ ||
                csum != self.csum(*buf) {
                self.error_cnt += 1;
                p.uart.write(XMODEM_NAK).ok();
                p.uart.flush().ok();
                continue;
            }

            self.packets_read += 1;
            self.pkt_num += 1; /* expect rollover */

            break
        }

        return XMODEM_ACK;
    }

    pub fn xmodem_cancel(&mut self, p: &mut util::CorePerphs) {
        p.uart.write(XMODEM_CAN).ok();
        p.uart.flush().ok();
    }

    fn read_soh(&mut self, p: &mut util::CorePerphs) -> u8 {
        /* this loop has been manually timed */
        for _ in  0..50_000 {
            match p.uart.read() {
                Err(nb::Error::Other(_err)) => {
                    return 0;
                },

                Err(nb::Error::WouldBlock) => {
                    /* do nothing */
                },

                Ok(val) => {
                    return val;
                }
            }
        }

        return 0;
    }

    fn read_packet(&mut self, p: &mut util::CorePerphs, buf: &mut [u8; BLOCK_SZ]) -> (u8, u8, usize, u8) {
        let mut pkt = 0;
        let mut pktc = 0;
        let mut sz: usize = 0;
        let mut csum = 0;
        let mut i = 0;

        while i < (BLOCK_SZ+3) {
            match p.uart.read() {
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

                    i += 1;
                }
            }
        }

        return (pkt, pktc, sz, csum);
    }

    fn csum(&mut self, buf: [u8; BLOCK_SZ]) -> u8 {
        let mut sum: u16 = 0;

        for i in 0..BLOCK_SZ {
            sum += buf[i] as u16;
        }

        return (sum & 0x00ff) as u8;
    }
}
