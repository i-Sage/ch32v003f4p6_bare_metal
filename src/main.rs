#![no_std]
#![no_main]

use core::panic::PanicInfo;

const RCC_BASE: u32 = 0x4002_1000;
const RCC_APB2PCENR: *mut u32 = (RCC_BASE + 0x18) as *mut u32;

const GPIOA_BASE: u32 = 0x4001_0800;
const GPIOA_CFGLR: *mut u32 = (GPIOA_BASE + 0x00) as *mut u32;
const GPIOA_OUTDR: *mut u32 = (GPIOA_BASE + 0x0C) as *mut u32;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // To use the GPIOA peripheral, we need to first enable the clock for
    // the peripheral. For the GPIOA peripheral, the bit to enable the
    // clock is in in bit position 2 RCC_APB2PCENR_GPIOA = 2
    unsafe {
        // Enable clock for GPIOA peripheral
        let rcc_value = RCC_APB2PCENR.read_volatile();
        RCC_APB2PCENR.write_volatile(rcc_value | (1 << 2));
    }

    // Now we have setup the clock for the peripheral, we can now
    // configure the pin we want to use in the peripheral
    // Setting GPIOA PA1 for push pull output
    // CNF1  = [1:0] BITS [6:7] = 00
    // MODE1 = [1:0] BITS [4:5] = 11
    unsafe {
        let mut gpio_cfg = GPIOA_CFGLR.read_volatile();
        gpio_cfg &= !(0xF << 4); // Clear all 4 bits for pin 1
        gpio_cfg |= 0x3 << 4; // Set MODE bits for output 30MHz
        GPIOA_CFGLR.write_volatile(gpio_cfg);
    }

    let mut counter: u32 = 0;
    loop {
        unsafe {
            // We can toggle PA1 by setting or clearing the 1bit in GPIOA_OUTDR
            if counter & 0x80_000 == 0 {
                GPIOA_OUTDR.write_volatile(GPIOA_OUTDR.read_volatile() | (1 << 1));
            } else {
                GPIOA_OUTDR.write_volatile(GPIOA_OUTDR.read_volatile() & !(1 << 1));
            }
        }
        counter = counter.wrapping_add(1);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
