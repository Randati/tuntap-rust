use libc::types::os::common::bsd44::in6_addr;
use libc::{c_int, c_short};

include!(concat!(env!("OUT_DIR"), "/constants.rs"));


#[repr(C)]
pub struct in6_ifreq {
	pub ifr6_addr: in6_addr,
	pub ifr6_prefixlen: u32,
	pub ifr6_ifindex: c_int
}

#[repr(C)]
pub struct ioctl_flags_data {
	pub ifr_name: [u8; IFNAMSIZ],
	pub ifr_flags: c_short
}

#[repr(C)]
pub struct ioctl_ifindex_data {
	pub ifr_name: [u8; IFNAMSIZ],
	pub ifr_ifindex: c_int
}
