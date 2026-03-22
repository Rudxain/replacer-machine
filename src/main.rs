#![feature(slice_split_once)]
use std::{
	fs,
	io::{self, Write},
	process::ExitCode,
};

mod util;
#[allow(clippy::wildcard_imports, reason = "can't expect")]
use util::*;

fn main() -> ExitCode {
	let mut out = io::stdout().lock();
	let mut err = io::stderr().lock();

	let mut argv = std::env::args_os().skip(1);
	let code = fs::read(if let Some(a) = argv.next() {
		a
	} else {
		#[expect(clippy::unwrap_used, reason = "duh")]
		writeln!(err, "missing path to program code").unwrap();
		return ExitCode::FAILURE;
	})
	.unwrap();
	// TO-DO: handle \n \= \#
	let code: Box<_> = code
		.split(|&c| c == b'\n')
		.filter(|line| !line.trim_ascii_start().is_empty())
		// comments
		.filter(|line| !line.trim_ascii_start().starts_with(b"#"))
		.map(|entry| entry.split_once(|&c| c == b'=').unwrap_or((entry, &[])))
		.collect();

	let mut m = Machine::new(
		&code,
		fs::read(if let Some(a) = argv.next() {
			a
		} else {
			#[expect(clippy::unwrap_used, reason = "duh")]
			writeln!(err, "missing path to input file").unwrap();
			return ExitCode::FAILURE;
		})
		.unwrap(),
	);
	while let Some(s) = m.next() {
		#[expect(clippy::unwrap_used, reason = "duh")]
		write!(out, "{}", String::from_utf8_lossy(s)).unwrap();
	}
	ExitCode::SUCCESS
}
