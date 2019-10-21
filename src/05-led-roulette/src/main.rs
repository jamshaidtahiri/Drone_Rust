// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux5::entry;

// #[entry]
// fn main() -> ! {
//     let _y;
//     let x = 42;
//     _y = x;

//     // infinite loop; just so we don't leave this stack frame
//     loop {}
// }


#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    }
}


