use paging::ActivePageTable;
use x86_64::VirtAddr;
use x86_64::structures::paging::{Page, PageTableFlags};

#[cfg(feature = "linked_alloc")]
pub use self::linked_list::Allocator;

mod linked_list;

unsafe fn map_heap(active_table: &mut ActivePageTable, offset: usize, size: usize) {
    let heap_start_page = Page::containing_address(VirtAddr::new(offset as u64));
    let heap_end_page = Page::containing_address(VirtAddr::new((offset + size - 1) as u64));
    let flags = PageTableFlags::PRESENT | PageTableFlags::GLOBAL | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, flags);
    }
}

pub unsafe fn init(active_table: &mut ActivePageTable) {
    let offset = ::KERNEL_HEAP_OFFSET;
    let size = ::KERNEL_HEAP_SIZE;

    // map heap pages
    map_heap(active_table, offset, size);

    // initialize global heap allocator
    Allocator::init(offset, size);
}