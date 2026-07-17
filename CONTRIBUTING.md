# Contributing to QuickChat

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution.

## Code of Conduct

This project and everyone participating in it is governed by the [QuickChat Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

- Ensure the bug was not already reported by searching on GitHub under [Issues](https://github.com/aaryanrwt/QuickChat/issues).
- If you're unable to find an open issue addressing the problem, open a new one. Be sure to include a title and clear description, as much relevant information as possible, and a code sample or an executable test case demonstrating the expected behavior that is not occurring.

### Suggesting Enhancements

- Open a new issue and detail your feature request.
- Explain *why* this enhancement would be useful to most users.

### Pull Requests

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes (`cargo test --all-targets --all-features`).
5. Ensure your code is formatted (`cargo fmt --all`).
6. Make sure your code passes clippy (`cargo clippy --all-targets --all-features -- -D warnings`).
7. Issue that pull request!

## Styleguide

- Use idiomatic Rust. Follow the guidelines presented by `clippy`.
- Document public APIs using Rustdoc comments (`///`).
- Keep asynchronous boundaries clean and avoid blocking operations in `async` blocks.
