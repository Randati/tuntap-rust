#![feature(env, fs, io, path, process)]

use std::env;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use std::io::Write;


fn main() {
	let src_dir_str = env::var_os("CARGO_MANIFEST_DIR").unwrap();
	let src_dir = Path::new(&src_dir_str);

	let dst_dir_str = env::var_os("OUT_DIR").unwrap();
	let dst_dir = Path::new(&dst_dir_str);
	
	let compiler = env::var("CC").unwrap_or("gcc".to_string());
	let executable = dst_dir.join("rust-constants");

	let c_src = src_dir.join("src/constants.c");
	let rust_dst = dst_dir.join("constants.rs");

	// Compile C code
	let mut cmd = Command::new(&compiler);
	cmd.arg("-o").arg(&executable);
	cmd.arg(&c_src);
	run(&mut cmd);

	// Run compiled binary and capture output
	let output = get_output(&mut Command::new(&executable));
	let mut f = File::create(&rust_dst).unwrap();
	f.write_all(output.as_bytes()).unwrap();
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

	String::from_utf8(output.stdout).unwrap()
}
