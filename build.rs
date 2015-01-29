#![allow(unstable)]

use std::os::getenv;
use std::io::Command;
use std::io::File;

fn main() {
	let src_dir = Path::new(getenv("CARGO_MANIFEST_DIR").unwrap());
	let dst_dir = Path::new(getenv("OUT_DIR").unwrap());
	
	let compiler = getenv("CC").unwrap_or("gcc".to_string());
	let executable = dst_dir.join("rust-constants");

	let c_src = src_dir.join("src/constants.c");
	let rust_dst = dst_dir.join("constants.rs");

	// Compile C code
	let mut cmd = Command::new(compiler);
	cmd.arg("-o").arg(&executable);
	cmd.arg(c_src);
	run(&mut cmd);

	// Run compiled binary and capture output
	let output = get_output(&mut Command::new(executable));
	let mut f = File::create(&rust_dst).unwrap();
	f.write_str(output.as_slice()).unwrap();
}

fn run(cmd: &mut Command) {
	let status = match cmd.status() {
		Ok(status) => status,
		Err(e) => panic!("failed to spawn process: {}", e),
	};

	if !status.success() {
		panic!("nonzero exit status: {}", status);
	}
}

fn get_output(cmd: &mut Command) -> String {
	let output = match cmd.output() {
		Ok(output) => output,
		Err(e) => panic!("failed to spawn process: {}", e),
	};

	if !output.status.success() {
		panic!("nonzero exit status: {}", output.status);
	}

	String::from_utf8(output.output).unwrap()
}
