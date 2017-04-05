#![feature(lang_items, const_fn)]
#![feature(alloc, collections)]
#![feature(unique)]
#![feature(abi_x86_interrupt)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;
extern crate hole_list_allocator;
extern crate alloc;
extern crate collections;
extern crate bit_field;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;

mod memory;
mod interrupts;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_addr: usize) {
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe { multiboot2::load(multiboot_info_addr) };
    enable_nxe_bit();
    enable_write_protect_bit();

    let mut memory_controller = memory::init(boot_info);

    interrupts::init(&mut memory_controller);

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    x86_64::instructions::interrupts::int3();

    println!("It did not crash");

    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

