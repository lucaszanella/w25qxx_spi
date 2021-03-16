#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    const SPI_CHANNEL: i32 = 0;
    //2MHz
    const speed: i32 = 2000000;
    //unsafe{wiringPiSPISetup(SPI_CHANNEL, speed)};
    unsafe{pwmWrite(0, 0)};
    println!("Hello, world!");
}
