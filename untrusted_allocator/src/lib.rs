#![no_std]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_trts;

extern crate buddy_system_allocator;

use buddy_system_allocator::LockedHeap;
use sgx_trts::libc;
use std::alloc::Layout;
use std::prelude::v1::*;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap_allocator(size: usize) {
    let buf_ptr = unsafe { libc::ocall::malloc(size) };
    let buf_size = if buf_ptr.is_null() { 0 } else { size };
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(buf_ptr as *const u8 as usize, buf_size);
    }
    println!(
        "[untrusted_allocator] init_heap_allocator, want size: {}, real size: {}",
        size, buf_size
    );
}

/// An memory allocator for slices, backed by a fixed-size, untrusted buffer
pub struct UntrustedAllocator {
    /// The pointer to the untrusted buffer
    buf_ptr: *mut u8,
    /// The size of the untrusted buffer
    buf_size: usize,
    /// The next position to allocate new slice
    /// New slices must be allocated from [buf_ptr + buf_pos, buf_ptr + buf_size)
    buf_pos: AtomicUsize,

    buf_align: usize,
}

impl UntrustedAllocator {
    pub fn new(buf_size: usize, buf_align: usize) -> Result<Self, ()> {
        if buf_size == 0 {
            // Create a dummy object
            return Ok(Self {
                buf_ptr: std::ptr::null_mut(),
                buf_size: 0,
                buf_pos: AtomicUsize::new(0),
                buf_align,
            });
        }

        let layout = Layout::from_size_align(buf_size, buf_align).map_err(|_| ())?;
        let buf_ptr = HEAP_ALLOCATOR.lock().alloc(layout)?.as_ptr();
        let buf_pos = AtomicUsize::new(0);
        Ok(Self {
            buf_ptr,
            buf_size,
            buf_pos,
            buf_align,
        })
    }

    pub fn new_slice(&self, src_slice: &[u8]) -> Result<&[u8], ()> {
        let new_slice = self.new_slice_mut(src_slice.len())?;
        new_slice.copy_from_slice(src_slice);
        Ok(new_slice)
    }

    pub fn new_slice_mut(&self, new_slice_len: usize) -> Result<&mut [u8], ()> {
        let new_slice_ptr = {
            // Move self.buf_pos forward if enough space _atomically_.
            let old_pos = self
                .buf_pos
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |old_pos| {
                    let new_pos = old_pos + new_slice_len;
                    if new_pos <= self.buf_size {
                        Some(new_pos)
                    } else {
                        None
                    }
                })
                .map_err(|_| {
                    println!(
                        "No enough space in UntrustedAllocator, buf_size: {}",
                        self.buf_size
                    );
                    ()
                })?;
            unsafe { self.buf_ptr.add(old_pos) }
        };
        let new_slice = unsafe { std::slice::from_raw_parts_mut(new_slice_ptr, new_slice_len) };
        Ok(new_slice)
    }

    // need fix bug!
    pub fn new_align_slice_mut(&self, new_slice_len: usize, align: usize) -> Result<&mut [u8], ()> {
        let new_slice_ptr = {
            let mut pos = self
                .buf_pos
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |old_pos| {
                    let new_pos = align_up(old_pos, align) + new_slice_len;

                    if new_pos <= self.buf_size {
                        Some(new_pos)
                    } else {
                        None
                    }
                })
                .map_err(|_| {
                    println!(
                        "No enough space in UntrustedAllocator, buf_size: {}",
                        self.buf_size
                    );
                    ()
                })?;

            pos = align_up(pos, align);
            unsafe { self.buf_ptr.add(pos) }
        };
        debug_assert!(new_slice_ptr as usize % align == 0);
        let new_slice = unsafe { std::slice::from_raw_parts_mut(new_slice_ptr, new_slice_len) };
        Ok(new_slice)
    }
}

impl Drop for UntrustedAllocator {
    fn drop(&mut self) {
        // Do nothing for the dummy case
        if self.buf_size == 0 {
            return;
        }

        let layout = Layout::from_size_align(self.buf_size, self.buf_align).unwrap();
        HEAP_ALLOCATOR
            .lock()
            .dealloc(NonNull::new(self.buf_ptr).unwrap(), layout);
    }
}

pub fn align_up(addr: usize, align: usize) -> usize {
    debug_assert!(align != 0 && align.is_power_of_two());
    align_down(addr + (align - 1), align)
}

pub fn align_down(addr: usize, align: usize) -> usize {
    debug_assert!(align != 0 && align.is_power_of_two());
    addr & !(align - 1)
}
