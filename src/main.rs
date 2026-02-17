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

use crate::mem::alloc::init_heap;

extern crate alloc;

mod mem;
mod ramfb;
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

const FBHEIGHT: u32 = 800;
const FBWIDTH: u32 = 800;

static mut BUFFER: [u32; (FBHEIGHT * FBWIDTH) as usize] = [0; (FBHEIGHT * FBWIDTH) as usize];

const fn rgba_to_fb_color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24)
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    init_heap();
    unsafe {
        let buffer_pointer = core::ptr::addr_of_mut!(BUFFER) as *mut u8;
        ramfb::setup_ramfb(buffer_pointer, FBWIDTH, FBHEIGHT);
        let mut flip = true;
        for x in 0..(FBWIDTH * FBHEIGHT) {
            if flip {
                buffer_pointer.add((x * 4) as usize).write_volatile(0xFF);
            } else {
                buffer_pointer.add((x * 4) as usize).write_volatile(0x00);
            }
            flip = !flip;
        }
    }
    println!("hey there");
    println!("this is an example project");
    println!("its just enough to get working on whatever you wanna do");
    println!("have fun <3");
    println!("");
    loop {}
}
