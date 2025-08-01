//! Conditionally compiling tests with std and our executable with no_std.
//!
//! Rust's built in unit testing framework requires the standard library,
//! but we need to build our final executable with no_std.
//! The testing framework also generates a `main` method, so we need to only use the `#[entry]`
//! annotation when building our final image.
//! For more information on why this example works, see this excellent blog post.
//! https://os.phil-opp.com/unit-testing/
//!
//! Running this example:
//!
//! Ensure there are no targets specified under `[build]` in `.cargo/config`
//! In order to make this work, we lose the convenience of having a default target that isn't the
//! host.
//!
//! cargo build --example test_on_host --target thumbv7m-none-eabi
//! cargo test --example test_on_host

#![cfg_attr(test, allow(unused_imports))]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

#[cfg(not(test))]
#[entry]
fn main() -> ! {
    loop {
        // your code goes here
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn foo() {
    println!("tests work!");
    assert!(2 == add(1,1));
  }
}
