#![crate_id = "bsdiff"]
#![crate_type = "rlib"]
#![deny(missing_doc)]

//! Some crate documentation

extern crate libc;

use libc::{c_int, uint8_t, int64_t, c_void, size_t};
use std::vec::raw;
use std::mem;

struct bsdiff_stream {
	opaque: *mut c_void,
	malloc: unsafe extern fn(size: size_t) -> *mut c_void,
	free: unsafe extern fn(ptr: *mut c_void),
	write: unsafe extern fn(stream: *mut bsdiff_stream, buffer: *c_void, size: c_int) -> c_int
}

#[link(name = "bsdiff", kind = "static")]
extern {
	fn bsdiff(old: *uint8_t, oldsize: int64_t, new: *uint8_t, newsize: int64_t, stream: *bsdiff_stream);
}

extern fn writer(stream: *mut bsdiff_stream, buffer: *c_void, size: c_int) -> c_int {
	unsafe {
		let vec = raw::from_buf(buffer as *u8, size as uint);
		let stream: &mut bsdiff_stream = mem::transmute(stream);
		let temp: &mut &mut Writer = mem::transmute(stream.opaque);
		match temp.write(vec.as_slice()) {
			Ok(_) => 0,
			Err(_) => -1
		}
	}
}

/// Computes bsdiff from old to new, writes to Writer 'out'.
pub fn diff(old: &[u8], new: &[u8], mut out: &mut Writer) {
	unsafe {
		let temp: &mut &mut Writer = &mut out;
		let stream = bsdiff_stream {
			opaque: mem::transmute(temp),
			malloc: libc::malloc,
			free: libc::free,
			write: writer
		};
		bsdiff(old.as_ptr(), old.len() as int64_t, new.as_ptr(), new.len() as int64_t, &stream as *bsdiff_stream);
	}
}