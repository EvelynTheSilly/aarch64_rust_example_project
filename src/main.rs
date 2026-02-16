#![no_std]
#![no_main]
#![feature(macro_metavar_expr_concat)]
#![allow(unused_unsafe)]
#![allow(
    clippy::doc_markdown,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc
)]

use core::panic::PanicInfo;
use qemu_exit::QEMUExit;

#[panic_handler]
#[allow(unreachable_code)] // rustc complains code isnt reachable when it very much is when qemu isnt enabled
fn panic(_: &PanicInfo) -> ! {
    qemu_exit::AArch64::new().exit(1);
    loop {} // failsafe
}

core::arch::global_asm!(
    "
    .global _ENTRY
    _ENTRY:
        b .
    "
);
