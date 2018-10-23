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
    start: usize,
    end: usize,
    max_size: usize,
    external_frag: usize,
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        let max_size = Allocator::bin_size(end - start);
        Allocator {
            bins: [LinkedList::new(); MAX_BINS],
            start,
            end,
            max_size,
            external_frag: 0,
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
        assert!(layout.align().is_power_of_two());
        let size = cmp::max(layout.align(), layout.size());
        let num = Allocator::bin_num(size);
        let bin_size = Allocator::bin_size(size);

        if size > self.max_size {
            return Err(AllocErr::Exhausted { request: layout });
        }
        for ref mut bin in &mut self.bins[num..] {
            if !bin.is_empty() {
                let node = bin.pop().expect("Pop free node");
                self.start += bin_size;
                self.external_frag += bin_size - size;
                return Ok(node as *mut u8);
            }
        }
        let cur = align_up(self.start, layout.align());
        if cur > self.end {
            return Err(AllocErr::Exhausted { request: layout });
        }
        self.start = cur + bin_size;
        self.external_frag += bin_size - size;
        Ok(self.start as *mut u8)
    }

    // fn get_best_fit(&self, size: usize) -> Option<*mut usize> {
    //     if self.bin[Allocator::bin_num(size)].head.is_null() {
    //         return None;
    //     }
    //     while
    // }

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
        self.start -= bin_size;
    }
}
//
// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Allocator:")?;
        writeln!(f, "    allocated: {}", self.start)?;
        writeln!(f, "    end: {}", self.end)?;
        writeln!(f, "    external fragmentation: {}", self.external_frag)?;
        writeln!(f, "    max bin size: {}", self.max_size)?;
        writeln!(f, "    Bins:")?;
        let mut size = MIN_SIZE;
        for bin in self.bins.iter() {
            size.next_power_of_two();
            writeln!(f, "        size: {} bin: {:#?}", size, bin)?;
        }
        Ok(())
    }
}
