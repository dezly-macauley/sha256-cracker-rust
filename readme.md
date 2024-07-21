# SHA256 Cracker Rust
## A password cracker written in Rust

<img src="images/hacker_crab.jpg" 
    alt="Hacker Crab" width="300" height="300"/>

---

### About
This is an interactive command line app that can be used to reveal the plain
text version of a password that has been encrypted in a sha256 hash.

---

### Usage Guide

The number after `main` is the sha256 hash of the password you are trying to
crack.

`cargo run -q --bin main 9e861941ad8bf5bcb649e5fde92d712528200a216018c2437371498e6ab7683d`

---

### Technical Details
- The uses the sha2 crate from crates.io to handle the decryption
- The env module from Rust's standard library is used to run the program 
with an argument.
- The io module is usd to handle file reading
- Data structures like Vector's are used to process input, and a for loop
is used to repeatedly attempt to crack the password.

---
