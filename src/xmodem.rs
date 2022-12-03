use hal::prelude::*;
use hal::serial::{FullConfig, Serial};
use nb::block;

pub fn xmodem_recv(
    uart: Serial<hal::stm32::USART1, FullConfig>,
    cb: Fn([u8; 128])
) {
    
}
