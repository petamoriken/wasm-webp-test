use std::{mem, alloc, slice};
use image::{ImageDecoder as _, webp::WebpDecoder};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub unsafe extern fn alloc(size: usize) -> *mut u8 {
    let layout = alloc::Layout::from_size_align(size, mem::align_of::<usize>()).unwrap();
    alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern fn dealloc(ptr: *mut u8, size: usize) {
    if size == 0 { return }
    let layout = alloc::Layout::from_size_align_unchecked(size, mem::align_of::<usize>());
    alloc::dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe extern fn decode_webp(input_ptr: *const u8, input_size: usize, output_size: *mut usize) -> *const u8 {
    let input = slice::from_raw_parts(input_ptr, input_size);
    let decoder = WebpDecoder::new(input).unwrap();
    let output = decoder.read_image().unwrap().into_boxed_slice();
    *output_size = output.len();
    Box::into_raw(output) as *const u8
}

#[no_mangle]
pub unsafe extern fn dealloc_buffer(ptr: *mut u8, size: usize) {
    let s = slice::from_raw_parts_mut(ptr, size);
    drop(Box::from_raw(s));
}
