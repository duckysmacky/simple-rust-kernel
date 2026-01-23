use alloc::alloc::{GlobalAlloc, Layout};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{FrameAllocator, Mapper, Size4KiB};
use x86_64::VirtAddr;
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("Dealloc should be never called")
    }
}

pub fn init_heap<M, A>(mapper: &mut M, allocator: &mut A) -> Result<(), MapToError<Size4KiB>>
where
    M: Mapper<Size4KiB>,
    A: FrameAllocator<Size4KiB>,
{
    use x86_64::structures::paging::{Page, PageTableFlags};

    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe {
            mapper.map_to(page, frame, flags, allocator)?.flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}