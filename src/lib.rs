#![feature(collections, core, hash, io, libc, path, std_misc)]

extern crate libc;

pub use tuntap::TunTap;
pub use tuntap::TunTapType::{Tun, Tap};

mod tuntap;
mod c_interop;
