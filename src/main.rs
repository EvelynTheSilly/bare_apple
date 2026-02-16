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

mod uart;
mod vectors;

#[panic_handler]
#[allow(unreachable_code)] // rustc complains code isnt reachable when it very much is when qemu isnt enabled
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info.message());
    qemu_exit::AArch64::new().exit(0);
    loop {} // failsafe
}

core::arch::global_asm!(
    "
    .global _ENTRY
    _ENTRY:
        bl load_stack
        
        bl enable_fpu
        
        bl setup_vtable
        
        mov x0, x20
        
        // enter rust
        bl main                        // go to rust entry point
        b .                            // hang forever
        
        load_stack:
            ldr x0, =el1_stack_top
            mov sp, x0
            ret
        
        enable_fpu:
            mrs x0, CPACR_EL1
            orr x0, x0, #(3 << 20)     // enable fp in el1 and el0
            msr CPACR_EL1, x0
            isb
            ret
        
        setup_vtable:
            ldr x0, =_vector_table     // load vtable into r0
            msr VBAR_EL1, x0
            isb                        // move r0 to base vector table register
            ret
    "
);

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    println!("hey there");
    println!("this is an example project");
    println!("its just enough to get working on whatever you wanna do");
    println!("have fun <3");
    println!("");
    panic!("main end reached");
}
