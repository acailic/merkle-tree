use std::env::{args, Args};

//println! - is a macro
//macros are like functions, but they are expanded before the compilation
fn main() {
    let mut args: Args = args();
    let first = args.nth(0).unwrap();
    println!("{:?}", first);
    println!("Hello, world!");
}
