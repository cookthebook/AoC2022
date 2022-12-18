use core::fmt::Write;
use xmodem;
use util;

use crate::util::{strtoul, print_buf_str, buf_eq, memcpy};

#[derive(Copy, Clone)]
struct Folder {
    name: [u8;16],
    parent: usize,
    children: [usize;16],
    nchildren: usize,
    size: u128,
    recursed: bool
}

impl Folder {
    pub fn print(&mut self, p: &mut util::CorePerphs, idx: usize) {
        write!(p.uart, "Folder {} ", idx).ok();
        util::print_buf_str(&mut p.uart, &self.name);
        write!(p.uart, "\r\n  Parent: {}\r\n", self.parent).ok();
        write!(p.uart, "  n-Children: {}\r\n", self.nchildren).ok();
        write!(p.uart, "  Size: {}\r\n", self.size).ok();
        write!(p.uart, "  Recursed: {}\r\n", self.recursed).ok();
    }
}

fn fsize(folders: &mut [Folder], idx: usize) {
    if folders[idx].recursed {
        return;
    }

    for i in 0..folders[idx].nchildren {
        fsize(folders, folders[idx].children[i]);
        folders[idx].size += folders[folders[idx].children[i]].size;
    }

    folders[idx].recursed = true;
}

pub fn solve(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    let mut folders = [
        Folder {
            name: [0u8;16],
            parent: usize::MAX,
            children: [usize::MAX;16],
            nchildren: 0,
            size: 0,
            recursed: false
        }; 256
    ];
    let mut nfolders: usize = 0;
    let mut cur: usize = usize::MAX;

    write!(p.uart, "=== Day 07, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */

        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
            b'\n' => {
                line[linep] = 0;

                if linep == 0 {
                    continue;
                }

                /* first line must be going to root dir */
                if cur == usize::MAX {
                    if !buf_eq(&line[0..linep], b"$ cd /") {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "First line must be \"$ cd /\"\r\n").ok();
                        return;
                    }

                    cur = 0;
                    folders[cur].name[0] = b'/';
                    nfolders = 1;
                }
                /* new directory listing */
                else if buf_eq(&line[0..4], b"dir ") {
                    if nfolders == folders.len() {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Out of folder memory!\r\n").ok();
                        return;
                    }

                    if folders[cur].nchildren == folders[cur].children.len() {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Out of child folder slots\r\n").ok();
                        folders[cur].print(p, cur);
                    }

                    if (linep - 4) > 16 {
                        linep = 4 + 16;
                    }
                    folders[cur].children[folders[cur].nchildren] = nfolders;
                    memcpy(&mut folders[nfolders].name, &line[4..linep], linep-4);
                    nfolders += 1;
                    folders[cur].nchildren += 1;
                }
                else if line[0] >= b'1' && line[0] <= b'9' {
                    let sz = strtoul(&line, 10);
                    folders[cur].size += sz as u128;
                }
                else if buf_eq(&line[0..7], b"$ cd ..") {
                    cur = folders[cur].parent;

                    if cur == usize::MAX {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "cd .. at root\r\n").ok();
                        return;
                    }
                }
                else if buf_eq(&line[0..5], b"$ cd ") {
                    let mut idx: usize = usize::MAX;

                    for i in 0..folders[cur].nchildren {
                        if buf_eq(&line[5..linep], &folders[folders[cur].children[i]].name[0..(linep-5)]) {
                            idx = i;
                            break;
                        }
                    }

                    if idx == usize::MAX {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Unkown folder ").ok();
                        print_buf_str(&mut p.uart, &line[5..linep]);
                        write!(p.uart, "\r\n").ok();
                        return;
                    }

                    let tmp = cur;
                    cur = folders[cur].children[idx];
                    folders[cur].parent = tmp;
                }

                linep = 0;
            },

            xmodem::XMODEM_PAD => {},

            _ => {
                line[linep] = buf[i];
                linep += 1;

                if linep == (xmodem::BLOCK_SZ - 1) {
                    xm.xmodem_cancel(p);
                    write!(p.uart, "Line too large\r\n").ok();
                    return;
                }
            }
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    write!(p.uart, "Folder collection done\r\n").ok();
    fsize(&mut folders, 0);
    write!(p.uart, "Folder size calc done\r\n").ok();

    let mut ret: u128 = 0;
    for i in 0..nfolders {
        if folders[i].size <= 100000 {
            ret += folders[i].size;
        }
    }

    write!(p.uart, "Sum of big sizes: {}\r\n", ret).ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut line = [0u8; xmodem::BLOCK_SZ];
    let mut linep = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    let mut folders = [
        Folder {
            name: [0u8;16],
            parent: usize::MAX,
            children: [usize::MAX;16],
            nchildren: 0,
            size: 0,
            recursed: false
        }; 256
    ];
    let mut nfolders: usize = 0;
    let mut cur: usize = usize::MAX;

    write!(p.uart, "=== Day 07, star ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */

        for i in 0..xmodem::BLOCK_SZ {
            match buf[i] {
            b'\n' => {
                line[linep] = 0;

                if linep == 0 {
                    continue;
                }

                /* first line must be going to root dir */
                if cur == usize::MAX {
                    if !buf_eq(&line[0..linep], b"$ cd /") {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "First line must be \"$ cd /\"\r\n").ok();
                        return;
                    }

                    cur = 0;
                    folders[cur].name[0] = b'/';
                    nfolders = 1;
                }
                /* new directory listing */
                else if buf_eq(&line[0..4], b"dir ") {
                    if nfolders == folders.len() {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Out of folder memory!\r\n").ok();
                        return;
                    }

                    if folders[cur].nchildren == folders[cur].children.len() {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Out of child folder slots\r\n").ok();
                        folders[cur].print(p, cur);
                    }

                    if (linep - 4) > 16 {
                        linep = 4 + 16;
                    }
                    folders[cur].children[folders[cur].nchildren] = nfolders;
                    memcpy(&mut folders[nfolders].name, &line[4..linep], linep-4);
                    nfolders += 1;
                    folders[cur].nchildren += 1;
                }
                else if line[0] >= b'1' && line[0] <= b'9' {
                    let sz = strtoul(&line, 10);
                    folders[cur].size += sz as u128;
                }
                else if buf_eq(&line[0..7], b"$ cd ..") {
                    cur = folders[cur].parent;

                    if cur == usize::MAX {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "cd .. at root\r\n").ok();
                        return;
                    }
                }
                else if buf_eq(&line[0..5], b"$ cd ") {
                    let mut idx: usize = usize::MAX;

                    for i in 0..folders[cur].nchildren {
                        if buf_eq(&line[5..linep], &folders[folders[cur].children[i]].name[0..(linep-5)]) {
                            idx = i;
                            break;
                        }
                    }

                    if idx == usize::MAX {
                        xm.xmodem_cancel(p);
                        write!(p.uart, "Unkown folder ").ok();
                        print_buf_str(&mut p.uart, &line[5..linep]);
                        write!(p.uart, "\r\n").ok();
                        return;
                    }

                    let tmp = cur;
                    cur = folders[cur].children[idx];
                    folders[cur].parent = tmp;
                }

                linep = 0;
            },

            xmodem::XMODEM_PAD => {},

            _ => {
                line[linep] = buf[i];
                linep += 1;

                if linep == (xmodem::BLOCK_SZ - 1) {
                    xm.xmodem_cancel(p);
                    write!(p.uart, "Line too large\r\n").ok();
                    return;
                }
            }
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    write!(p.uart, "Folder collection done\r\n").ok();
    fsize(&mut folders, 0);
    write!(p.uart, "Folder size calc done\r\n").ok();

    let mut ret: usize = 0;
    for i in 1..nfolders {
        if ((folders[0].size - folders[i].size) <= 40000000) &&
           (folders[i].size < folders[ret].size) {
            ret = i;
        }
    }

    write!(p.uart, "Smallest folder you can delete:\r\n").ok();
    folders[ret].print(p, ret);
}
