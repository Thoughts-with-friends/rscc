# rsvcc

Small Rust-Verilog(HDL) and C Compiler in Rust

## Design List

- [ ] Design the initial file structure
- [ ] Select crates
- [ ] Design EBNF
- [ ] Set up a test environment that can parse Rust code and compile Verilog-HDL code
- [ ] Perform a simple test of the rust parser (Rust str -> Verilog-HDL str)
- [ ] Compile numbers, local values, ..., always

## GOAL

- Ability to perform logic synthesis according to Verilog-HDL syntax
- High-level synthesis is possible

  ```rust
  Rust str -> Verilog-HDL str or
  Rust str -> C str -> Verilog-HDL str
  ```

- Compile Rust and C (or C++) with FFI and output Verilog-HDL source (if possible)

## Candidate crates

- [syn (Rust parser)](https://crates.io/crates/syn)
- [nom (manual parsing)](https://crates.io/crates/nom)

## License

MIT
