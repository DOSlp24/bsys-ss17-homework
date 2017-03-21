//! hw0 - task1
//!
//! First task! outputs given name the given amount of times!

pub fn multiple_hello(name: &str, n: i32){
	let mut i = 0;
	let mut done = false;
	while !done {
		println!("Hello, {}!", name);
		i += 1;
		if i==n {
			done = true;
		}
		
	}
}

fn main() {
    multiple_hello("Ferris", 2);
}
