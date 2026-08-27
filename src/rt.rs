use core::panic::PanicInfo;

use crate::led;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    led_panic(0x1000, 1000)
}

/// Busy-wait roughly `ms` milliseconds at ~48 MHz (no SysTick / HAL Delay).
fn spin_ms(ms: u32) {
    // Empirically tuned for ~48 MHz; panic path does not need exact timing.
    for _ in 0..ms {
        for _ in 0..12_000 {
            cortex_m::asm::nop();
        }
    }
}

pub fn led_panic(start_intensity: u16, delay_ms: u32) -> ! {
    led::initialize();

    let mut intensity = start_intensity;
    loop {
        led::set_intensity(intensity);

        if intensity == 0 {
            intensity = start_intensity;
        } else {
            intensity = 0;
        }
        spin_ms(delay_ms);
    }
}

#[allow(unused)]
pub fn led_blink_n_times(start_intensity: u16, n: u32) {
    led::initialize();

    let mut intensity = start_intensity;
    for _i in 0..n * 2 {
        led::set_intensity(intensity);

        if intensity == 0 {
            intensity = start_intensity;
        } else {
            intensity = 0;
        }
        spin_ms(1000);
    }
}
