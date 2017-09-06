#![feature(custom_attribute)]
#![feature(alloc, allocator_api)]
#![feature(global_allocator)]
#![feature(compiler_builtins_lib)]

#![no_std]

extern crate spin;
extern crate alloc;
extern crate linked_list_allocator;

#[macro_use]
extern crate lazy_static;

extern crate compiler_builtins;

use spin::Mutex;
use linked_list_allocator::Heap;
use alloc::allocator::{Alloc, Layout, AllocErr};


pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

lazy_static! {
    static ref HEAP: Mutex<Heap> = Mutex::new(unsafe {
        Heap::new(HEAP_START, HEAP_SIZE)
    });
}

pub struct Allocator;

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        HEAP.lock().allocate_first_fit(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        HEAP.lock().deallocate(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL_ALLOC: Allocator = Allocator;

