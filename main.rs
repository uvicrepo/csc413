#![no_std]
use wokwi::prelude::*;

const RESPONSE: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF]; // Fake UID

#[entry]
fn main() -> ! {
    let sck = pin!("SCK");
    let mosi = pin!("MOSI");
    let miso = pin!("MISO");
    let cs = pin!("CS");

    let mut bit_buf: u8 = 0;
    let mut bit_count = 0;
    let mut response_ptr = 0;

    loop {
        sck.wait_for_high(); // Rising edge

        if cs.is_low() {
            // Shift in bit from MOSI
            bit_buf <<= 1;
            if mosi.is_high() {
                bit_buf |= 1;
            }
            bit_count += 1;

            if bit_count == 8 {
                // Byte received
                if bit_buf == 0x26 {
                    // If REQA command, reset response
                    response_ptr = 0;
                }

                // Send next byte from response buffer
                let out_byte = RESPONSE[response_ptr % RESPONSE.len()];
                miso.set_level((out_byte & 0x80) != 0); // MSB

                response_ptr += 1;
                bit_buf = 0;
                bit_count = 0;
            }
        }

        sck.wait_for_low(); // Optional pacing
    }
}

