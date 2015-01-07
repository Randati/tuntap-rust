use std::io::{IoResult, IoError, File, Open, ReadWrite};
use std::os::unix::prelude::AsRawFd;
use std::string::String;
use libc::c_int;
use libc::consts::os::bsd44::{AF_INET6, SOCK_DGRAM};
use libc::funcs::bsd43::socket;
use libc::funcs::bsd44::ioctl;
use libc::funcs::posix88::unistd::close;
use libc::types::os::common::bsd44::in6_addr;
use c_interop::*;


const DEVICE_PATH: &'static str = "/dev/net/tun";

// TODO Make not a constant
const MTU_SIZE: uint = 1500;


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

impl TunTap {
	pub fn create(typ: TunTapType) -> TunTap {
		TunTap::create_named(typ, "")
	}

	pub fn create_named(typ: TunTapType, name: &str) -> TunTap {
		let (file, if_name) = TunTap::create_if(typ, name);
		let (sock, if_index) = TunTap::create_socket(if_name);

		TunTap {
			file: file,
			sock: sock,
			if_name: if_name,
			if_index: if_index
		}
	}

	fn create_if(typ: TunTapType, name: &str) -> (File, [u8; IFNAMSIZ]) {
		if name.len() >= IFNAMSIZ {
			panic!("Interface name too long, max length is {}", IFNAMSIZ - 1);
		}

		let path = Path::new(DEVICE_PATH);
		let file = match File::open_mode(&path, Open, ReadWrite) {
			Err(why) => panic!("Couldn't open tun device '{}': {}", path.display(), why.desc),
			Ok(file) => file,
		};

		let mut req = ioctl_flags_data {
			ifr_name: str_as_buffer(name),
			ifr_flags: match typ {
				TunTapType::Tun => IFF_TUN,
				TunTapType::Tap => IFF_TAP
			}
		};

		let res = unsafe { ioctl(file.as_raw_fd(), TUNSETIFF, &mut req) };
		if res < 0 {
			panic!("{}", IoError::last_error());
		}

		(file, req.ifr_name)
	}

	fn create_socket(if_name: [u8; IFNAMSIZ]) -> (c_int, c_int) {
		let sock = unsafe { socket(AF_INET6, SOCK_DGRAM, 0) };
		if sock < 0 {
			panic!("{}", IoError::last_error());
		}
		
		let mut req = ioctl_ifindex_data {
			ifr_name: if_name,
			ifr_ifindex: -1
		};

		let res = unsafe { ioctl(sock, SIOCGIFINDEX, &mut req) };
		if res < 0 {
			let err = IoError::last_error();
			unsafe { close(sock) };
			panic!("{}", err);
		}

		(sock, req.ifr_ifindex)
	}

	pub fn get_name(&self) -> String {
		unsafe { String::from_raw_buf_len(self.if_name.as_ptr(), IFNAMSIZ) }
	}

	pub fn up(&self) {
		let mut req = ioctl_flags_data {
			ifr_name: self.if_name,
			ifr_flags: 0
		};


		let res = unsafe { ioctl(self.sock, SIOCGIFFLAGS, &mut req) };
		if res < 0 {
			panic!("{}", IoError::last_error());
		}

		if req.ifr_flags & IFF_UP & IFF_RUNNING != 0 {
			// Already up
			return;
		}

		req.ifr_flags |= IFF_UP | IFF_RUNNING;

		let res = unsafe { ioctl(self.sock, SIOCSIFFLAGS, &mut req) };
		if res < 0 {
			panic!("{}", IoError::last_error());
		}
	}

	pub fn add_address(&self, ip: &[u8]) {
		self.up();

		if ip.len() == 4 {
			panic!("IPv4 not implemented");
		}
		else if ip.len() == 16 {
			let mut req = in6_ifreq {
				ifr6_addr: in6_addr {s6_addr: [
					(ip[ 1] as u16) << 8 | ip[ 0] as u16,
					(ip[ 3] as u16) << 8 | ip[ 2] as u16,
					(ip[ 5] as u16) << 8 | ip[ 4] as u16,
					(ip[ 7] as u16) << 8 | ip[ 6] as u16,
					(ip[ 9] as u16) << 8 | ip[ 8] as u16,
					(ip[11] as u16) << 8 | ip[10] as u16,
					(ip[13] as u16) << 8 | ip[12] as u16,
					(ip[15] as u16) << 8 | ip[14] as u16
				]},
				ifr6_prefixlen: 8,
				ifr6_ifindex: self.if_index
			};

			let res = unsafe { ioctl(self.sock, SIOCSIFADDR, &mut req) };
			if res < 0 {
				panic!("{}", IoError::last_error());
			}
		}
		else {
			panic!("IP length must be either 4 or 16 bytes, got {}", ip.len());
		}
	}

	pub fn read<'a>(&mut self, buffer: &'a mut [u8]) -> IoResult<&'a [u8]> {
		assert!(buffer.len() >= MTU_SIZE);

		let len = try!(self.file.read(buffer));
		Ok(buffer.slice_to(len))
	}

	pub fn write(&mut self, data: &[u8]) -> IoResult<()> {
		self.file.write(data)
	}
}
