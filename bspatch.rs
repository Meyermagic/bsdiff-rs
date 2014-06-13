
/*
struct bspatch_stream
{
	void* opaque;
	int (*read)(const struct bspatch_stream* stream, void* buffer, int length);
};

int bspatch(const uint8_t* old, int64_t oldsize, uint8_t* new, int64_t newsize, struct bspatch_stream* stream);
*/


#![crate_id = "bspatch"]
#![crate_type = "rlib"]
#![deny(missing_doc)]

//! Some crate documentation

extern crate libc;

use libc::{c_int, uint8_t, int64_t, c_void, size_t};
use std::vec::raw;
use std::mem;

struct bspatch_stream {
	opaque: *mut c_void,
	read: unsafe extern fn(stream: *bspatch_stream, buffer: *mut c_void, length: c_int) -> c_int
}

#[link(name = "bspatch", kind = "static")]
extern {
	fn bspatch(old: *uint8_t, oldsize: int64_t, new: *mut uint8_t, newsize: int64_t, stream: *mut bspatch_stream);
}

extern fn reader(stream: *bspatch_stream, buffer: *mut c_void, length: c_int) -> c_int {
	unsafe {
		let mut vec = raw::from_buf(buffer as *u8, length as uint);
		let stream: &bspatch_stream = mem::transmute(stream);
		let temp: &mut &mut Reader = mem::transmute(stream.opaque);
		match temp.read(vec.as_mut_slice()) {
			Ok(n) if n == length as uint => 0,
			_ => -1
		}
	}
}

/// Computes bspatch, writes into new (must be large enough, currently unsafe)
pub fn patch(old: &[u8], new: &mut [u8], mut input: &mut Reader) {
	unsafe {
		let temp: &mut &mut Reader = &mut input;
		let mut stream = bspatch_stream {
			opaque: mem::transmute(temp),
			read: reader
		};
		bspatch(old.as_ptr(), old.len() as int64_t, new.as_mut_ptr(), new.len() as int64_t, &mut stream as *mut bspatch_stream);
	}
}