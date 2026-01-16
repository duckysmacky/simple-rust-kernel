use x86_64::VirtAddr;
use x86_64::structures::{
    tss::TaskStateSegment,
    gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector},
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static::lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };

    static ref GDT: GDTData = {
        let mut table = GlobalDescriptorTable::new();

        let code_selector = table.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = table.add_entry(Descriptor::tss_segment(&TSS));
        let selectors = Selectors {
            code_selector,
            tss_selector,
        };

        GDTData {
            table,
            selectors,
        }
    };
}

struct GDTData {
    table: GlobalDescriptorTable,
    selectors: Selectors,
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init_gdt() {
    use x86_64::instructions::{
        tables,
        segmentation::{CS, Segment}
    };

    GDT.table.load();

    unsafe {
        CS::set_reg(GDT.selectors.code_selector);
        tables::load_tss(GDT.selectors.tss_selector);
    }
}