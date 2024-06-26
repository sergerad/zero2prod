# Zero To Production in Rust

This repo is based on a walkthrough of the book *Zero to Production in Rust: an opinionated introduction to backend development* by Luca Palmieri.

The book is a phenomenal resource and I highly recommend it to anyone beginning their Rust journey. Here is a list of all the books I found helpful:
* [Zero to Production](https://github.com/LukeMathWalker/zero-to-production)
* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Async Rust](https://rust-lang.github.io/async-book/)
* [Programming Rust (Systems)](https://www.amazon.com.au/Programming-Rust-Fast-Systems-Development/dp/1492052590)
* [Rust for Rustaceans](https://rust-for-rustaceans.com/)
* [Rust in Action](https://www.rustinaction.com/)

Make sure to do the [rustlings](https://github.com/rust-lang/rustlings) challenges.

I also recommend watching Rust content on Youtube. There are some excellent channels that will get you set up and show you how to write idiomatic Rust.

Jon Gjengset (author of Rust for Rustaceans) has a [channel](https://www.youtube.com/@jonhoo) which does a great "decrusting crate {name}" series as well as some long-form coding challenge videos which are a great way to learn from an experienced Rustacean.

### The Source

While I have followed the book closely, I have made some noteworthy deviations from its instructions for the sake of interest and learning, namely:
* Postgres (incl. migrations) and redis bootstrap implemented in Rust instead of BASH;
* The use of a cargo workspace for additional crates;
* Go no further than implementation, testing, and CI (no CD).

### Usage

Have a look at `main.yaml` to understand how all the source is validated.

To run the server locally, or run the tests, we must first run Postgres and Redis via docker:
```sh
cargo run --bin stores run &
```

Then we can run tests:
```sh
cargo test
```

And we can run the API server:
```sh
cargo run --bin zero2prod
```

Have a look at the YAML files in `configuration/` if you want to change any default values. The default port is `8080`:

```sh
curl -v http://localhost:8080/health
```
