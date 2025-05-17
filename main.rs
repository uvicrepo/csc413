#![no_std]
use wokwi::prelude::*;

const RESPONSE: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF]; // fake UID

#[entry]
fn main() -> ! {
    let sck = pin!("SCK");
    let mosi = pin!("MOSI");
    let miso = pin!("MISO");
    let cs = pin!("CS");

    let mut bit_buf = 0u8;
    let mut bit_count = 0;
    let mut response_ptr = 0;

    loop {
        sck.wait_for_high(); // rising edge of clock

        if cs.is_low() {
            bit_buf <<= 1;
            if mosi.is_high() {
                bit_buf |= 1;
            }
            bit_count += 1;

            if bit_count == 8 {
                // received a byte from master
                if bit_buf == 0x26 {
                    // if REQA command, start sending UID
                    response_ptr = 0;
                }

                // send response byte
                let out_byte = RESPONSE[response_ptr % RESPONSE.len()];
                miso.set_level((out_byte & 0x80) != 0); // send MSB first
                response_ptr += 1;
                bit_count = 0;
                bit_buf = 0;
            }
        }

        sck.wait_for_low(); // falling edge (optional, just pacing)
    }
}
