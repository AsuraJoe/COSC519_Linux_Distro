#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosc519_OS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cosc519_OS::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use cosc519_OS::memory::active_level_4_table;
    use x86_64::{VirtAddr, structures::paging::Page};
    use cosc519_OS::memory;

    println!("Hello, This is {}!!", "COSC519 OS");
    cosc519_OS::init();
    // fn stack_overflow(){
    //     stack_overflow();
    // }

    // stack_overflow();
    // x86_64::instructions::interrupts::int3();
    // let ptr = 0xdeadbeaf as *mut u32;
    // unsafe { *ptr = 42; }
    use x86_64::registers::control::Cr3;
    use cosc519_OS::memory::translate_addr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    // translating addresses

    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    // let addresses =[
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x21008,
    //     // some stack page
    //     0x0100_00020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = unsafe {translate_addr(virt, phys_mem_offset)};
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // for (i, entry) in l4_table.iter().enumerate() {
    //     use x86_64::structures::paging::PageTable;
        
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         // convert physical addresses
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt). as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         //print non empty entries of level 3 table
    //         for(i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    //Writing to memory
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    println!("By {} & {}", "Trung Nguyen", "Ha Xuan Nguyen");
    cosc519_OS::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cosc519_OS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cosc519_OS::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}