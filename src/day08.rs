use core::fmt::Write;
use xmodem;
use util;


fn vis_right(grid: &mut [u8], width: usize) {
    for r in 0..width {
        let mut tallest = 0;
        for c in (0..width).rev() {
            let h = grid[c*width + r] & 0x7f;
            if h > tallest {
                grid[c*width + r] |= 0x80;
                tallest = h;
            }
        }
    }
}

fn vis_left(grid: &mut [u8], width: usize) {
    for r in 0..width {
        let mut tallest = 0;
        for c in 0..width {
            let h = grid[c*width + r] & 0x7f;
            if h > tallest {
                grid[c*width + r] |= 0x80;
                tallest = h;
            }
        }
    }
}

fn vis_top(grid: &mut [u8], width: usize) {
    for c in 0..width {
        let mut tallest = 0;
        for r in 0..width {
            let h = grid[c*width + r] & 0x7f;
            if h > tallest {
                grid[c*width + r] |= 0x80;
                tallest = h;
            }
        }
    }
}

fn vis_bot(grid: &mut [u8], width: usize) {
    for c in 0..width {
        let mut tallest = 0;
        for r in (0..width).rev() {
            let h = grid[c*width + r] & 0x7f;
            if h > tallest {
                grid[c*width + r] |= 0x80;
                tallest = h;
            }
        }
    }
}

fn s_score(grid: &[u8], width: usize, row: usize, col: usize) -> u128 {
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bot = 0;
    let h = grid[row*width + col];

    let mut r = row;
    let mut c = col;
    /* go left */
    while c > 0 {
        c -= 1;
        left += 1;
        if grid[r*width + c] >= h {
            break;
        }
    }

    r = row;
    c = col;
    /* go right */
    while c < (width-1) {
        c += 1;
        right += 1;
        if grid[r*width + c] >= h {
            break
        }
    }

    r = row;
    c = col;
    /* go up */
    while r > 0 {
        r -= 1;
        top += 1;
        if grid[r*width + c] >= h {
            break;
        }
    }

    r = row;
    c = col;
    /* go down */
    while r < (width -1) {
        r += 1;
        bot += 1;
        if grid[r*width + c] >= h {
            break;
        }
    }

    return right*left*top*bot;
}

pub fn solve(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut grid = [0u8; 10000];
    let mut gridp = 0;
    let mut width = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    write!(p.uart, "=== Day 08, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */

        for i in 0..xmodem::BLOCK_SZ {
            if width == 0 && buf[i] == b'\n' {
                width = gridp;
            } else if buf[i] >= b'0' && buf[i] <= b'9' {
                if gridp >= grid.len() {
                    xm.xmodem_cancel(p);
                    write!(p.uart, "Out of grid memory\r\n").ok();
                    return;
                }

                /* shift it so the tallest tree has height 1, easily compared
                 * to a "null" height of 0 */
                grid[gridp] = buf[i] - b'0' + 1;
                gridp += 1;
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    vis_bot(&mut grid, width);
    vis_top(&mut grid, width);
    vis_right(&mut grid, width);
    vis_left(&mut grid, width);

    let mut result = 0;
    for i in 0..gridp {
        if (grid[i] & 0x80) != 0 {
            result += 1;
        }
    }

    write!(p.uart, "Trees visible: {}\r\n", result).ok();
}

pub fn solve_star(p: &mut util::CorePerphs) {
    let mut buf = [0u8; xmodem::BLOCK_SZ];
    let mut grid = [0u8; 10000];
    let mut gridp = 0;
    let mut width = 0;
    let mut xm = xmodem::Xmodem::new();
    let mut xm_ret;

    write!(p.uart, "=== Day 08, normal ===\r\n\r\n").ok();

    xm_ret = xm.xmodem_begin(p, &mut buf);
    while xm_ret == xmodem::XMODEM_ACK {
        /* parse this chunk of input */

        for i in 0..xmodem::BLOCK_SZ {
            if width == 0 && buf[i] == b'\n' {
                width = gridp;
            } else if buf[i] >= b'0' && buf[i] <= b'9' {
                if gridp >= grid.len() {
                    xm.xmodem_cancel(p);
                    write!(p.uart, "Out of grid memory\r\n").ok();
                    return;
                }

                /* shift it so the tallest tree has height 1, easily compared
                 * to a "null" height of 0 */
                grid[gridp] = buf[i] - b'0' + 1;
                gridp += 1;
            }
        }

        xm_ret = xm.xmodem_getpkt(p, &mut buf);
    }

    let mut best = 0;
    for i in 0..gridp {
        let score = s_score(&grid, width, i/width, i%width);
        if score > best {
            best = score;
        }
    }

    write!(p.uart, "Best vis score: {}\r\n", best).ok();
}
