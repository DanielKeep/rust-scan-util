# rust-scan-util

[![Build Status](https://travis-ci.org/DanielKeep/rust-scan-util.svg?branch=master)](https://travis-ci.org/DanielKeep/rust-scan-util)

This is the runtime support library for the [`rust-scan`](https://github.com/DanielKeep/rust-scan) package.  If you want to *use* the `scan*` family of macros, go read the documentation there.  You only need to worry about this package if you intend to:

- Implement a `Scanner` for a type manually, *without* using the `scanner!` macro.
- Implement a custom `Tokenizer`, `Whitespace` or `CompareStrs` type for use at runtime (you sadly cannot provide a custom implementation of these at compile time).
- You want to use one of the utility routines this package happens to have.

## License

This package is provided under the MIT license.
