pub fn main() {
	// Sometimes, bad thing happen in your code, and there's nothing you can do about it.
	// When the `panic!` macro executes, your program will print a failure message,
	// unwind and clean up the stack, and then quit.
	// This most commonly occurs when a bug of some kind has been detected
	// and it's not clear to the programmer how to handle the error.
	panic!("crash and burn");
	// We can use the backtrace of the functions the `panic!` call came from
	// to figure out the part of our code that is causing the problem.
}

fn backtrace() {
	// In C, attempting to read beyond the end of a data structure is undefined behavior.
	// You might get whatever is at the location in memory that would correspond to
	// that element in the data structure, even though the memory doesn't belong to that structure.
	// This is called a "buffer overread" and can lead to security vulnerabilities if
	// an attacker is able to manipulate the index such a way to read data they shouldn't
	// be allowed to that is stored after the data structure.
	let v = vec![1, 2, 3];

	v[99];
	// If you try to read an element at an index that doesn't exist,
	// Rust will stop execution and refuse to continue.

	// We can set the `RUST_BACKTRACE` environment variable to get a backtrace
	// of exactly what happened to cause the error.
	// A "backtrace" is a list of all the functions that have been called
	// to get to this point.

	// In order to get backtraces with this information, debug symbols must be enabled.
	// Debug symbols are enabled by default when using `cargo build`
	// or `cargo run` without the `--release` flag.
}