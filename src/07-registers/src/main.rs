// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux7::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     aux7::init();

//     unsafe {
//         // A magic address!
//         const GPIOE_BSRR: u32 = 0x48001018;

//         // Turn on the "North" LED (red)
//         *(GPIOE_BSRR as *mut u32) = 1 << 9;

//         // Turn on the "East" LED (green)
//         *(GPIOE_BSRR as *mut u32) = 1 << 11;

//         // Turn off the "North" LED
//         *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

//         // Turn off the "East" LED
//         *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
//     }

//     loop {}
// }


#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs10().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    gpioe.bsrr.write(|w| w.bs12().set_bit());
    gpioe.bsrr.write(|w| w.bs13().set_bit());
    gpioe.bsrr.write(|w| w.bs14().set_bit());
    gpioe.bsrr.write(|w| w.bs15().set_bit());
    gpioe.bsrr.write(|w| w.bs8().set_bit());
    // Turn on the East LED
    // gpioe.bsrr.write(|w| w.bs11().set_bit());

    // // Turn off the North LED
    // gpioe.bsrr.write(|w| w.br9().set_bit());

    // // Turn off the East LED
    // gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
