use std::ffi::CString;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::io;
use std::mem;
use std::os::unix::prelude::AsRawFd;
use std::path::Path;
use libc::{c_int, c_char, c_void, AF_INET, AF_INET6, SOCK_DGRAM, socket, ioctl, close, in_addr, in6_addr, sockaddr_in, sa_family_t};
use c_interop::*;

const DEVICE_PATH: &'static str = "/dev/net/tun";

// TODO Make not a constant
const MTU_SIZE: usize = 1500;


extern {
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TunTapType {
	Tun,
	Tap
}


pub struct TunTap {
	pub file: File,
	sock: c_int,
	if_name: [u8; IFNAMSIZ],
	if_index: c_int
}

impl Drop for TunTap {
	fn drop(&mut self) {
		unsafe { close(self.sock) };
	}
}

impl fmt::Debug for TunTap {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Tun({:?})", self.get_name())
	}
}


impl TunTap {
	pub fn create(typ: TunTapType) -> TunTap {
		TunTap::create_named(typ, &CString::new("").unwrap())
	}

	pub fn create_named(typ: TunTapType, name: &CString) -> TunTap {
		let (file, if_name) = TunTap::create_if(typ, name);
		let (sock, if_index) = TunTap::create_socket(if_name);

		TunTap {
			file: file,
			sock: sock,
			if_name: if_name,
			if_index: if_index
		}
	}

	fn create_if(typ: TunTapType, name: &CString) -> (File, [u8; IFNAMSIZ]) {
		let name_slice = name.as_bytes_with_nul();
		if name_slice.len() > IFNAMSIZ {
			panic!("Interface name too long, max length is {}", IFNAMSIZ - 1);
		}

		let path = Path::new(DEVICE_PATH);
		let file = match OpenOptions::new().read(true).write(true).open(&path) {
			Err(why) => panic!("Couldn't open tun device '{}': {:?}", path.display(), why),
			Ok(file) => file,
		};

		let mut req = ioctl_flags_data {
			ifr_name: {
				let mut buffer = [0u8; IFNAMSIZ];
				buffer[..name_slice.len()].clone_from_slice(name_slice);
				buffer
			},
			ifr_flags: match typ {
				TunTapType::Tun => IFF_TUN | IFF_NO_PI,
				TunTapType::Tap => IFF_TAP | IFF_NO_PI
			}
		};

		let res = unsafe { ioctl(file.as_raw_fd(), TUNSETIFF, &mut req) };
		if res < 0 {
			panic!("{}", io::Error::last_os_error());
		}

		(file, req.ifr_name)
	}

	fn create_socket(if_name: [u8; IFNAMSIZ], sock_type: c_int) -> (c_int, c_int) {
		let sock = unsafe { socket(sock_type, SOCK_DGRAM, 0) };
		if sock < 0 {
			panic!("{}", io::Error::last_os_error());
		}

		let mut req = ioctl_ifindex_data {
			ifr_name: if_name,
			ifr_ifindex: -1
		};

		let res = unsafe { ioctl(sock, SIOCGIFINDEX, &mut req) };
		if res < 0 {
			let err = io::Error::last_os_error();
			unsafe { close(sock) };
			panic!("{}", err);
		}

		(sock, req.ifr_ifindex)
	}

	pub fn get_name(&self) -> CString {
    let mut it = self.if_name.iter();
		let nul_pos = match it.position(|x| *x == 0) {
			Some(p) => p,
			None => panic!("Device name should be null-terminated")
		};

	  CString::new(&self.if_name[..nul_pos]).unwrap()
	}

	pub fn up(&self) {
		let mut req = ioctl_flags_data {
			ifr_name: self.if_name,
			ifr_flags: 0
		};


		let res = unsafe { ioctl(self.sock, SIOCGIFFLAGS, &mut req) };
		if res < 0 {
			panic!("{}", io::Error::last_os_error());
		}

		if req.ifr_flags & IFF_UP & IFF_RUNNING != 0 {
			// Already up
			return;
		}

		req.ifr_flags |= IFF_UP | IFF_RUNNING;

		let res = unsafe { ioctl(self.sock, SIOCSIFFLAGS, &mut req) };
		if res < 0 {
			panic!("{}", io::Error::last_os_error());
		}
	}

    fn get_in_addr(&self, ip: CString) -> Result<in_addr, &'static str> {
        let mut addr_4 = in_addr{ s_addr: 0};
        let addr_4_ptr: *mut c_void = &mut addr_4 as *mut _ as *mut c_void;

        match unsafe { inet_pton(AF_INET, ip.as_ptr(), addr_4_ptr) } {
            1 => Ok(unsafe { *(addr_4_ptr as *const in_addr) } ),
            _ => Err("not a valid IPv4 address")
        }
    }

    fn get_in6_addr(&self, ip: CString) -> Result<in6_addr, &'static str> {
        let mut addr_6: in6_addr = unsafe { mem::uninitialized() };
        let addr_6_ptr: *mut c_void = &mut addr_6 as *mut _ as *mut c_void;

        match unsafe { inet_pton(AF_INET6, ip.as_ptr(), addr_6_ptr) } {
            1 => Ok(unsafe { *(addr_6_ptr as *const in6_addr) } ),
            _ => Err("not a valid IPv6 address")
        }
    }


    fn add_ipv4_addr(&self, addr: in_addr) {
        let sock_addr = sockaddr_in {
            sin_family: AF_INET as sa_family_t,
            sin_port: 0,
            sin_addr: addr,
            sin_zero: [0, 0, 0, 0, 0, 0, 0, 0]
        };

        let mut req = in_ifreq {
            ifr_name: self.if_name,
            ifr_addr: sock_addr,
        };

			  let res = unsafe { ioctl(self.sock, SIOCSIFADDR, &mut req) };

			  if res < 0 {
				    panic!("{}", io::Error::last_os_error());
        }
    }

    fn add_ipv6_addr(&self, addr: in6_addr) {
			  let mut req = in6_ifreq {
				    ifr6_addr: addr,
            ifr6_prefixlen: 8,
            ifr6_ifindex: self.if_index
        };
			  let res = unsafe { ioctl(self.sock, SIOCSIFADDR, &mut req) };
			  if res < 0 {
				    panic!("{}", io::Error::last_os_error());
        }
    }

	  pub fn add_address(&self, ip: CString) {
		    self.up();
        let ip6 = ip.clone();
        match self.get_in_addr(ip) {
            Ok(addr) => self.add_ipv4_addr(addr),
            Err(s) => {
                match self.get_in6_addr(ip6) {
                    Ok(addr) => self.add_ipv6_addr(addr),
                    Err(s) => panic!("We're expecting an ipv4 or ipv6 address string")
                }
            }
	      }
    }

	pub fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		assert!(buffer.len() >= MTU_SIZE);

		let len = try!(self.file.read(buffer));
		Ok(len)
	}

	pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
		self.file.write_all(data)
	}
}
