#![feature(offset_of)]
#![no_std]
#![no_main]

use bamboo_os::graphics::Bitmap;
use bamboo_os::graphics::draw_test_pattern;
use bamboo_os::graphics::fill_rect;
use bamboo_os::qemu::QemuExitCode;
use bamboo_os::qemu::exit_qemu;
use bamboo_os::uefi::EfiHandle;
use bamboo_os::uefi::EfiMemoryType;
use bamboo_os::uefi::EfiSystemTable;
use bamboo_os::uefi::MemoryMapHolder;
use bamboo_os::uefi::VramTextWriter;
use bamboo_os::uefi::exit_from_efi_boot_services;
use bamboo_os::uefi::init_vram;
use core::fmt::Write;
use core::mem::offset_of;
use core::panic::PanicInfo;
use core::writeln;

use bamboo_os::x86::hlt;

#[unsafe(no_mangle)]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable){
    let mut vram = init_vram(efi_system_table).expect("Failed to initialize vram");
    let vw = vram.width();
    let vh = vram.height();
    fill_rect(&mut vram, 0x000000, 0, 0, vw, vh).expect("Failed to fill rect");

    //図形
    draw_test_pattern(&mut vram);

    // メモリ情報の出力
    let mut w = VramTextWriter::new(&mut vram);
    for i in 0..10 {
        writeln!(w, "i={i}").unwrap()
    }
    let mut memory_map = MemoryMapHolder::new();
    let status = efi_system_table
        .boot_services()
        .get_memory_map(&mut memory_map);
    writeln!(w, "{status:?}").unwrap();
    let mut total_memory_pages = 0;
    for e in memory_map.iter() {
        if e.memory_type() != EfiMemoryType::CONVENTIONAL_MEMORY {
            continue;
        }
        total_memory_pages += e.number_of_pages();
        writeln!(w, "{e:?}").unwrap();
    }
    let total_memory_ssize_mib = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(
        w,
        "Total: {total_memory_pages} pages = {total_memory_ssize_mib} MiB"
    )
    .unwrap();

    exit_from_efi_boot_services(image_handle, efi_system_table, &mut memory_map);
    writeln!(w, "Hello, Non-UEFI world");

    loop {
        hlt()
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_qemu(QemuExitCode::Fail);
}
