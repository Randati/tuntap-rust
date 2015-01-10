#include <stdio.h>
#include <sys/ioctl.h>
#include <net/if.h>
#include <linux/if_tun.h>

#define RUST_CONST(name, type, printf_type) printf("pub const " #name ": " #type " = " printf_type ";\n", name);

int main() {
	RUST_CONST(TUNSETIFF, c_int, "%lu")
	RUST_CONST(SIOCSIFADDR, c_int, "%d")
	RUST_CONST(SIOCGIFINDEX, c_int, "%d")
	RUST_CONST(SIOCGIFFLAGS, c_int, "%d")
	RUST_CONST(SIOCSIFFLAGS, c_int, "%d")

	RUST_CONST(IFF_TUN, c_short, "%d")
	RUST_CONST(IFF_TAP, c_short, "%d")
	RUST_CONST(IFF_UP, c_short, "%d")
	RUST_CONST(IFF_RUNNING, c_short, "%d")

	RUST_CONST(IFNAMSIZ, usize, "%d")

	return 0;
}
