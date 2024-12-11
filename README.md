# Guard-Clause [![Cloud-CI](https://github.com/Radiator-Labs/guard-clause-rs/actions/workflows/cloud-ci.yml/badge.svg)](https://github.com/Radiator-Labs/guard-clause-rs/actions/workflows/cloud-ci.yml)

Guard-clause library, providing syntactic sugar to improve readability of
[guard-clauses](https://en.wikipedia.org/wiki/Guard_(computer_science))
in Rust code.

## Intention

The Rust programming language provides the [`assert`](https://doc.rust-lang.org/core/macro.assert.html)
macro in the core library (starting in 1.6.0, assert was originally in the standard library.)
The `assert` allows values to be checked and, if not as desired, panics the application.
Guard-clauses attempts to deliver a similar functionality, but returns a value instead of panicking.

Original Code:

```rust
if (value > threshold) {
    return Err(Error::OverThreshold);
}
```

Code with guard-clause:

```rust
guard!(value <= threshold, Err(Error::OverThreshold));
```

This expresses the guard clause in a more concise manner, improving readability.

### Development goals

The guard-clause is intended to have no dependencies and operate in no-std code
bases. (Dev-dependencies will be allowed, if necessary.)

Pedantic clippy is used to drive a more idiomatic codebase.

### Other options

It has also been discussed to make the guard-clause even more tailored, with the following options:

* Return a Result::Error, as `guard_err(value <= threshold, Error::OverThreshold);`
  * Perhaps this should be the default, or only option.
* Return an Option::None, as `guard_none(value <= threshold);`
* Return a Result::Error<&'static str>, as `guard_str(value <= threshold, "Value is over threshold");`
  * Should this return a String instead, and use a formatter to insert values in the String?

The current thinking is that these options do not offer enough benefit over the version
that just returns a generic type.

## Attribution

Work creating this library was performed as part of commercial development
by [Kelvin](https://kel.vin/) (formerly Radiator Labs), a green energy company
dedicated to decarbonizing the world's legacy buildings.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.83 and up. It *might*
compile with older versions but that may change in any new patch release.
Changes in Clippy support are the main factor driving the version dependency.

See [here](../docs/msrv.md) for details on how the MSRV may be upgraded.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## end
