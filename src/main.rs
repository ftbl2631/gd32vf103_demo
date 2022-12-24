#![no_std]
#![no_main]
#![allow(unused)]

//mod driver;
//mod hal;
//mod kernel;
//mod runtime;

extern crate panic_halt;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    //let var_temp:u32;
    let freq_hz = 1;
    let timer_clock = 8_000_000 as u32;
    let ticks = timer_clock / freq_hz;
    let psc = ((ticks - 1) / (1 << 16)) as u16;
    loop{
        // to be added later
    }
}