#![feature(libc, clone_from_slice)]

extern crate libc;

pub use tuntap::TunTap;
pub use tuntap::TunTapType::{Tun, Tap};
pub use tuntap::IpType::{Ipv4, Ipv6};

mod tuntap;
mod c_interop;
