#![no_std]
#![feature(offset_of)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "run_unit_tests"]
#![no_main]
mod allocator;
pub mod graphics;
pub mod qemu;
pub mod result;
pub mod uefi;
pub mod x86;

#[cfg(test)]
pub mod test_runner;


#[cfg(test)]
#[macro_use]
pub fn efi_main() {
    run_unit_tests()
}
