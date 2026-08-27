//! IRQ trampolines matching the stock Valve bootloader.
//!
//! LPC11U37 has no usable VTOR. Stock behavior:
//! - most vectors: always forward to the app table at `0x2000` (no GPREG check)
//! - PendSV / CT32B1 / USART / USB_IRQ: multiplex on `PMU.GPREG1`
//!   (`0` = programming mode, nonzero = app)
//! - HardFault: forward to the app (not a local infinite loop)

use crate::programming_mode;

#[no_mangle]
pub unsafe extern "C" fn ProgPendSV() {
    programming_mode::PendSV();
}

#[no_mangle]
pub unsafe extern "C" fn ProgCT32B1() {
    programming_mode::CT32B1();
}

#[no_mangle]
pub unsafe extern "C" fn ProgUSART() {
    programming_mode::USART();
}

#[no_mangle]
pub unsafe extern "C" fn ProgUSB_IRQ() {
    programming_mode::USB_IRQ();
}

/// Must live in `.HardFault.*` so it sits next to cortex-m-rt's
/// `HardFaultTrampoline` (that trampoline uses a ±2 KiB Thumb `b`).
#[link_section = ".HardFault.forward"]
#[no_mangle]
pub unsafe extern "C" fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    let hdlr = core::ptr::read_volatile(0x200c as *const usize);
    let hdlr: extern "C" fn() = core::mem::transmute(hdlr);
    hdlr();
    loop {
        cortex_m::asm::wfi();
    }
}

core::arch::global_asm!(
    r#"
    .syntax unified
    .thumb

    // Stock-style: always forward via IPSR into the app vector table.
    // Device IRQs / NMI / SVCall / SysTick weakly alias this symbol.
    .thumb_func
    .global DefaultHandler
    DefaultHandler:
        mrs r0, ipsr
        lsls r0, r0, #2
        ldr r1, =0x2000
        ldr r0, [r1, r0]
        bx r0

    .macro mux name, vec, prog
    .thumb_func
    .global \name
    \name:
        ldr r0, =0x40038008
        ldr r0, [r0]
        cmp r0, #0
        bne 1f
        ldr r0, =\prog
        bx r0
    1:
        ldr r0, =\vec
        ldr r0, [r0]
        bx r0
    .endm

    mux PendSV,  0x2038, {prog_pendsv}
    mux CT32B1,  0x208C, {prog_ct32b1}
    mux USART,   0x2094, {prog_usart}
    mux USB_IRQ, 0x2098, {prog_usb}
    "#,
    prog_pendsv = sym ProgPendSV,
    prog_ct32b1 = sym ProgCT32B1,
    prog_usart = sym ProgUSART,
    prog_usb = sym ProgUSB_IRQ,
);
