//@compile-flags: -Znext-solver=globally
//@check-pass
#![warn(clippy::future_not_send)]
async fn run() {}
fn main() {}
