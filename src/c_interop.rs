extern crate libc;

use std::slice::bytes::copy_memory;
use self::libc::types::os::common::bsd44::in6_addr;

// TODO There must be a better way
pub const TUNSETIFF: libc::c_int = 1074025674;
pub const SIOCSIFADDR: libc::c_int = 35094;
pub const SIOCGIFINDEX: libc::c_int = 35123;
pub const SIOCGIFFLAGS: libc::c_int = 35091;
pub const SIOCSIFFLAGS: libc::c_int = 35092;

pub const IFF_TUN: libc::c_short = 1;
pub const IFF_TAP: libc::c_short = 2;

pub const IFNAMSIZ: uint = 16;

pub const IFF_UP: libc::c_short = 1;
pub const IFF_RUNNING: libc::c_short = 64;


#[repr(C)]
pub struct in6_ifreq {
	pub ifr6_addr: in6_addr,
	pub ifr6_prefixlen: u32,
	pub ifr6_ifindex: libc::c_int
}

#[repr(C)]
pub struct ioctl_flags_data {
	pub ifr_name: [u8; IFNAMSIZ],
	pub ifr_flags: libc::c_short
}

#[repr(C)]
pub struct ioctl_ifindex_data {
	pub ifr_name: [u8; IFNAMSIZ],
	pub ifr_ifindex: libc::c_int
}


pub fn str_as_buffer(s: &str) -> [u8; IFNAMSIZ] {
	let bytes = s.as_bytes();
	let mut buffer = [0u8; IFNAMSIZ];
	copy_memory(&mut buffer, bytes);
	buffer
}
