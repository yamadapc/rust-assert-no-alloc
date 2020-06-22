use assert_no_alloc::*;

#[global_allocator]
static A: AllocDisabler = AllocDisabler;

fn main() {
	println!("Alloc is allowed. Let's allocate some memory...");
	let mut vec_can_push = Vec::new();
	vec_can_push.push(42);

	println!();

	let fib5 = forbid_alloc(|| {
		println!("Alloc is forbidden. Let's calculate something without memory allocations...");

		fn fib(n: u32) -> u32 {
			if n<=1 { 1 }
			else { fib(n-1) + fib(n-2) }
		}

		fib(5)
	});
	println!("\tSuccess, the 5th fibonacci number is {}", fib5);
	println!();

	forbid_alloc(|| {
		println!("Alloc is forbidden. Let's allocate some memory...");
		let mut vec_cannot_push = Vec::new();
		vec_cannot_push.push(42); // panics
	});

	println!("this will not be executed due to the panic above");
}
