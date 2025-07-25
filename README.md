# tiny-three-pass-compiler
This repository contains a clean and improved solution to the [Codewars](https://www.codewars.com/) [Tiny Three-Pass Compiler](https://www.codewars.com/kata/5265b0885fda8eac5900093b) kata, as it was an interesting small challenge that went beyond just parsing expressions.

The final result only makes use of Rust's standard library, and my original kata solution only imported [itertools](https://crates.io/crates/itertools) for some smaller peekable iterator shenanigans, while still implementing a simple recursive descent parser for the compiler first stage.
