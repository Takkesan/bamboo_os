extern crate alloc;
use crate::result::Result;
use crate::uefi::EfiMemoryDescriptor;
use crate::uefi::EfiMemoryType;
use crate::uefi::MemoryMapHolder;
use alloc::alloc::GlobalAlloc;
use alloc::alloc::Layout;
use alloc::boxed::Box;
use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::cmp::max;
use core::fmt;
use core::mem::size_of;
use core::ops::DerefMut;
use core::ptr::null_mut;

// checked_shl は、整数型の値を「指定したビット数だけ左シフト」
// 1usize は「値が1の usize 型（符号なし整数型）」

pub fn round_up_to_nearest_pow2(v: usize) -> Result<usize> {
    1usize
        .checked_shl(usize::BITS - v.wrapping_sub(1).leading_zeros())
        .ok_or("Out of range")
}

/// Vertical bar `|` represents the chunk that has a Header
/// before: |-- prev -------|---- self ---------------
/// align: |--------|-------|-------|-------|-------|
/// after: |---------------||-------|----------------

//Box は、ヒープ領域にデータを確保し、その所有権を持つスマートポインタ型です。
// Box<T> は T 型の値をヒープに格納し、スコープから外れると自動的にメモリを解放します。
// 主に大きなデータや再帰的なデータ構造（リストやツリーなど）で使われます。
struct Header {
    next_header: Option<Box<Header>>,
    size: usize,
    is_allocated: bool,
    _reserved: usize
}
const HEADER_SIZE: usize = size_of::<Header>();
