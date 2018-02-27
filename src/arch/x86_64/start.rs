use os_bootinfo::BootInfo;
use idt;
use interrupt;
use devices;
use memory;
use macros::println;
use paging;
use allocator;

/// Test of zero values in BSS.
static BSS_TEST_ZERO: usize = 0x0;
/// Test of non-zero values in data.
static DATA_TEST_NONZERO: usize = 0xFFFF_FFFF_FFFF_FFFF;


/// This function is extremely unsafe
/// Thus, it is marked unsafe
#[no_mangle]
pub unsafe fn _start(boot_info_ptr: *mut BootInfo) -> ! {
    let boot_info = &mut*boot_info_ptr;

    // .bss section should be zeroed
    {
        assert_eq!(BSS_TEST_ZERO, 0x0);
        assert_eq!(DATA_TEST_NONZERO, 0xFFFF_FFFF_FFFF_FFFF);
    }
    
    memory::init(boot_info);

    // Initialize paging
    let mut active_table = paging::init();
    
    // Initialize dynamic memory allocation
    allocator::init(&mut active_table);

    // Initialize the IDT
    idt::init();

    println!("IDT initialized");

    // Initialize essential devices
    devices::init();

    println!("Devices initialized");

    // Initialize non-essential devices
    devices::init_noncore();

    println!("OK");

    interrupt::enable_and_nop();

    println!("Interrupts enabled");

    ::kmain(1);
}