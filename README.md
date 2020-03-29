# Subnet - Calculator

A simple CLI application to calculate subnet with IPv4 and IPv6 address.
## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

I you want to build the project from the sources, you must have Rust install.
More information in [Rustup](https://rustup.rs/).

```
curl https://sh.rustup.rs -sSf | sh
```
If you are on Windows, you just need to launch [Rustup-init.exe](https://rustup.rs/).

### Installing

To install this application, you need to build the project with cargo.


```
cargo build --release
```
After that, cargo build the project on the target/realease folder.
If you are on linux OS you can add the file create in /bin folder.

## Running the tests

With cargo you can make automatic test.

### Unit test

You can test all function, with prebuilt test. I made 2 tests for every function, one simple and one more complex.

```
cargo test
```



## Built With

* [Rust](https://www.rust-lang.org/) - A fast and secure programming language
* [Clap](https://clap.rs/) - Fast. Configurable. Argument Parsing for Rust
* [Cargo](https://github.com/rust-lang/cargo) - The Rust packet manager



## Authors

* **Léo Huteau** - *Initial work* - [ZeProf10T](https://github.com/ZeProf10T)

See also the list of [contributors](https://github.com/ZeProf10T/Subnet-calculator/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
