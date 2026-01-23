use x86_64::structures::paging::{
    page_table::PageTableEntry,
    OffsetPageTable, PageTable, FrameAllocator, PhysFrame, Size4KiB
};
use x86_64::{PhysAddr, VirtAddr};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map.iter()
            .filter(|region| region.region_type == MemoryRegionType::Usable)
            .map(|region| region.range.start_addr()..region.range.end_addr())
            .flat_map(|region| region.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub unsafe fn get_memory_mapper(memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = get_level_4_table(memory_offset);
        OffsetPageTable::new(level_4_table, memory_offset)
    }
}

unsafe fn get_level_4_table(memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (table_frame, _) = Cr3::read();

    let physical_addr = table_frame.start_address();
    let virtual_addr = memory_offset + physical_addr.as_u64();
    let table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();

    unsafe { &mut *table_ptr }
}

pub fn traverse_table<F>(table: &PageTable, memory_offset: VirtAddr, callback: F)
where
    F: Fn(usize, &PageTableEntry, &PageTable)
{
    table.iter()
        .filter(|entry| !entry.is_unused())
        .enumerate()
        .for_each(|(i, entry)| {
            let physical_addr = match entry.frame() {
                Ok(frame) => frame.start_address(),
                Err(_) => entry.addr(),
            };

            let table_ptr = {
                let table_addr = memory_offset + physical_addr.as_u64();
                table_addr.as_mut_ptr()
            };

            let next_level_table: &PageTable = unsafe { &*table_ptr };

            callback(i, entry, next_level_table);
        });
}