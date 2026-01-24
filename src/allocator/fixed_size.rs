use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr::NonNull};
use crate::allocator::Locked;
use super::linked_list::LinkedListAllocator;

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

struct RegionNode {
    next: Option<&'static mut RegionNode>,
}

pub struct FixedSizeBlockAllocator {
    block_heads: [Option<&'static mut RegionNode>; BLOCK_SIZES.len()],
    fallback_allocator: LinkedListAllocator,
}

impl FixedSizeBlockAllocator {
    /// Creates an empty FixedSizeBlockAllocator.
    pub const fn new() -> Self {
        const EMPTY_NODE: Option<&'static mut RegionNode> = None;

        FixedSizeBlockAllocator {
            block_heads: [EMPTY_NODE; BLOCK_SIZES.len()],
            fallback_allocator: LinkedListAllocator::new(),
        }
    }

    /// Initialize the allocator with the given heap bounds.
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.fallback_allocator.init(heap_start, heap_size);
        }
    }

    fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        match block_index(&layout) {
            Some(block_i) => {
                match self.block_heads[block_i].take() {
                    Some(node) => {
                        self.block_heads[block_i] = node.next.take();
                        let node_ptr = node as *mut RegionNode;

                        NonNull::new(node_ptr as *mut u8)
                    }
                    None => {
                        let block_size = BLOCK_SIZES[block_i];
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();

                        self.fallback_allocator.allocate(layout)
                    }
                }
            }
            None => self.fallback_allocator.allocate(layout),
        }
    }

    fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        match block_index(&layout) {
            Some(block_i) => {
                let new_node = RegionNode {
                    next: self.block_heads[block_i].take(),
                };

                // verify that block has size and alignment required for storing node
                assert!(mem::size_of::<RegionNode>() <= BLOCK_SIZES[block_i]);
                assert!(mem::align_of::<RegionNode>() <= BLOCK_SIZES[block_i]);

                let new_node_ptr = ptr.as_ptr() as *mut RegionNode;
                unsafe {
                    new_node_ptr.write(new_node);
                    self.block_heads[block_i] = Some(&mut *new_node_ptr);
                }
            }
            None => {
                unsafe {
                    self.fallback_allocator.deallocate(ptr, layout);
                }
            }
        }
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        use core::ptr;

        match self.lock().allocate(layout) {
            Some(ptr) => ptr.as_ptr(),
            None => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            self.lock().deallocate(NonNull::new_unchecked(ptr), layout);
        }
    }
}

/// Choose an appropriate block size for the given layout.
///
/// Returns an index into the `BLOCK_SIZES` array.
fn block_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());

    BLOCK_SIZES.iter()
        .position(|&s| s >= required_block_size)
}