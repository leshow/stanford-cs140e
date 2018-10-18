use alloc::heap::{AllocErr, Layout};
use std::{cmp, fmt, mem};

use allocator::linked_list::LinkedList;
use allocator::util::*;

const MIN_POW: u32 = 3;
const MIN_SIZE: usize = 1 << MIN_POW;
const USIZE_SIZE: usize = mem::size_of::<usize>();
const MAX_BINS: usize = 32;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    bins: [LinkedList; MAX_BINS],
    total_alloc: usize,
    end: usize,
    max_size: usize,
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        let max_size = Allocator::bin_size(end - start);
        Allocator {
            bins: [LinkedList::new(); MAX_BINS],
            total_alloc: start,
            end,
            max_size,
        }
    }
    /// Determines bin number based on its size
    fn bin_num(size: usize) -> usize {
        if size < MIN_SIZE {
            0
        } else {
            (Allocator::bin_size(size).trailing_zeros() - MIN_POW) as usize
        }
    }

    /// Determines size of bin based on the actual size
    fn bin_size(actual: usize) -> usize {
        if actual < MIN_SIZE {
            MIN_SIZE
        } else {
            actual.next_power_of_two()
        }
    }
    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let size = cmp::max(layout.align(), layout.size());
        let bin_size = Allocator::bin_size(size);
        let num = Allocator::bin_num(size);

        if size > self.max_size {
            return Err(AllocErr::Exhausted { request: layout });
        }
        for ref mut bin in &mut self.bins[num..] {
            if !bin.is_empty() {
                let node = bin.pop().expect("Pop free node");
                self.total_alloc += size;
                return Ok(node as *mut u8);
            }
        }
        let cur = align_up(self.total_alloc, layout.align());
        if cur > self.end {
            return Err(AllocErr::Exhausted { request: layout });
        }
        self.total_alloc = cur + bin_size;
        Ok(self.total_alloc as *mut u8)
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = cmp::max(layout.size(), layout.align());
        let num = Allocator::bin_num(size);
        let bin_size = Allocator::bin_size(size);
        unsafe {
            self.bins[num].push(ptr as *mut usize);
        }
        self.total_alloc -= bin_size;
    }
}
//
// FIXME: Implement `Debug` for `Allocator`.
