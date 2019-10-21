#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

     gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs10().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    gpioe.bsrr.write(|w| w.bs12().set_bit());
    gpioe.bsrr.write(|w| w.bs13().set_bit());
    gpioe.bsrr.write(|w| w.bs14().set_bit());
    gpioe.bsrr.write(|w| w.bs15().set_bit());
    gpioe.bsrr.write(|w| w.bs8().set_bit());
    loop {}
}

