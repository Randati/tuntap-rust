#include <stdio.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <linux/if_tun.h>

#define RUST_CONST(name, type, printf_type) printf("pub const " #name ": " #type " = " printf_type ";\n", name);

int main() {
	RUST_CONST(TUNSETIFF, c_ulong, "%lu")
	RUST_CONST(SIOCSIFADDR, c_ulong, "%d")
	RUST_CONST(SIOCGIFINDEX, c_ulong, "%d")
	RUST_CONST(SIOCGIFFLAGS, c_ulong, "%d")
	RUST_CONST(SIOCSIFFLAGS, c_ulong, "%d")

	RUST_CONST(IFF_TUN, c_short, "%d")
	RUST_CONST(IFF_TAP, c_short, "%d")
  RUST_CONST(IFF_NO_PI, c_short, "%d")
  RUST_CONST(IFF_UP, c_short, "%d")
	RUST_CONST(IFF_RUNNING, c_short, "%d")

	RUST_CONST(IFNAMSIZ, usize, "%d")

	return 0;
}
