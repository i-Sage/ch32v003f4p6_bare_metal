#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Memory-mapped register addresses for CH32V003F4P6
// From ch32v00x.h:
// GPIOD_BASE = APB2PERIPH_BASE + 0x1400 = 0x40010000 + 0x1400 = 0x40011400
// RCC_BASE = AHBPERIPH_BASE + 0x1000 = 0x40020000 + 0x1000 = 0x40021000
const RCC_BASE: u32 = 0x4002_1000;
const RCC_APB2PCENR: *mut u32 = (RCC_BASE + 0x18) as *mut u32;

const GPIOD_BASE: u32 = 0x4001_1400;
const GPIOD_CFGLR: *mut u32 = (GPIOD_BASE + 0x00) as *mut u32;
const GPIOD_OUTDR: *mut u32 = (GPIOD_BASE + 0x0C) as *mut u32;

// This is where your Rust program actually begins after startup
// The startup.s file will call this after setting up the stack
// and initializing memory
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Enable clock for GPIOD peripheral
    // RCC_APB2Periph_GPIOD is bit 5 (0x00000020)
    unsafe {
        let rcc_val = RCC_APB2PCENR.read_volatile();
        RCC_APB2PCENR.write_volatile(rcc_val | (1 << 5));
    }

    // Configure PD6 as output push-pull, 50MHz
    // For pin 6, we need bits [27:24] of CFGLR:
    //   MODE6[1:0] = bits [25:24] = 11 (50MHz output)
    //   CNF6[1:0]  = bits [27:26] = 00 (push-pull)
    // So we set bits [27:24] = 0011 = 0x3
    unsafe {
        let mut config = GPIOD_CFGLR.read_volatile();
        config &= !(0xF << 24); // Clear all 4 bits for pin 6
        config |= 0x3 << 24;    // Set MODE6=11 (50MHz), CNF6=00 (push-pull)
        GPIOD_CFGLR.write_volatile(config);
    }

    // Simple blink loop to verify everything works
    // Using a much larger counter for a visible blink
    let mut counter: u32 = 0;
    loop {
        // Toggle PD6 - use a larger number for slower, more visible blinking
        unsafe {
            if counter & 0xFF_000 == 0 {
                // Set pin high
                GPIOD_OUTDR.write_volatile(GPIOD_OUTDR.read_volatile() | (1 << 6));
            } else {
                // Set pin low  
                GPIOD_OUTDR.write_volatile(GPIOD_OUTDR.read_volatile() & !(1 << 6));
            }
        }
        counter = counter.wrapping_add(1);
    }
}

// This is called if your program panics
// For now we just halt, but you could flash an LED in a pattern
// to signal different error codes
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}