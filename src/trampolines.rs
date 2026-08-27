//! IRQ trampolines matching the stock Valve bootloader.
//!
//! LPC11U37 (Cortex-M0) has no usable VTOR — the CPU always vectors through
//! address 0. Device IRQs / PendSV weakly alias to `DefaultHandler`, which:
//! - in app mode (`GPREG1 != 0`): forwards via IPSR to the table at `0x2000`
//! - in programming mode (`GPREG1 == 0`): dispatches PendSV/CT32B1/USART/USB

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

core::arch::global_asm!(
    r#"
    .syntax unified
    .thumb

    // Exception numbers: PendSV=14, CT32B1=35, USART=37, USB_IRQ=38
    .thumb_func
    .global DefaultHandler
    DefaultHandler:
        ldr r1, =0x40038008
        ldr r1, [r1]
        cmp r1, #0
        bne 9f

        mrs r0, ipsr
        cmp r0, #14
        bne 1f
        ldr r0, ={prog_pendsv}
        bx r0
    1:
        cmp r0, #35
        bne 2f
        ldr r0, ={prog_ct32b1}
        bx r0
    2:
        cmp r0, #37
        bne 3f
        ldr r0, ={prog_usart}
        bx r0
    3:
        cmp r0, #38
        bne 4f
        ldr r0, ={prog_usb}
        bx r0
    4:
        bx lr

    9:
        mrs r0, ipsr
        lsls r0, r0, #2
        ldr r1, =0x2000
        ldr r0, [r1, r0]
        bx r0
    "#,
    prog_pendsv = sym ProgPendSV,
    prog_ct32b1 = sym ProgCT32B1,
    prog_usart = sym ProgUSART,
    prog_usb = sym ProgUSB_IRQ,
);
