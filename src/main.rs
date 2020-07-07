/* Usage of this crate for error handling to facilitate error type creation */
extern crate derive_more;

use std::process;

mod alg;
mod error;
mod input;

fn main() {
    let (start, target, len) = input::retrieve().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    });
    let start: alg::Constellation = alg::starify(start);
    let target: alg::Constellation = alg::starify(target);

    println!("{}", alg::resolve(start, target, len));
}
