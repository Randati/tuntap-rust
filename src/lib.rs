#![feature(globs)]

pub use tuntap::TunTap;
pub use tuntap::TunTapType::{Tun, Tap};

mod tuntap;
mod c_interop;
