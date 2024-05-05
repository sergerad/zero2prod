# Zero To Production in Rust

This repo is based on a walkthrough of the book *Zero to Production in Rust: an opinionated introduction to backend development* by Luca Palmieri.

The book is a phenomenal resource and I highly recommend it to anyone beginning their Rust journey. Here is a list of all the books I found helpful:
* [Zero to Production](https://www.zero2prod.com/index.html?country=New%20Zealand&discount_code=OC20)
* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Async Rust](https://rust-lang.github.io/async-book/)
* [Programming Rust (Systems)](https://www.amazon.com.au/Programming-Rust-Fast-Systems-Development/dp/1492052590)
* [Rust for Rustaceans](https://rust-for-rustaceans.com/)
* [Rust in Action](https://www.rustinaction.com/)

I also recommend watching Rust content on Youtube. There are some excellent channels that will get you set up and show you how to write idiomatic Rust.

### The Source

While I have followed the book closely, I have made some noteworthy deviations from its instructions for the sake of interest and learning, namely:
* Implementation of Postgres database start up and migrations in Rust, rather than BASH;
* The use of a cargo workspace (in part to accomodate the previous point);
* Go no further than implementation, testing, and CI (no CD);
* ...

There are two binary crates in the workspace: `server` and `pg`.

The `pg` crate is used to launch Postgres locally with Docker for the sake of local deployment and testing.

The `server` is the REST API.

### Usage

Have a look at `main.yaml` to understand how all the source is validated.

To run the server locally, or run the tests, we must first start up a Postgres instance:
```sh
cargo run --bin pg run &
```

Then we can run tests:
```sh
cargo test
```

And we can run the API server:
```sh
cargo run --bin server
```

Have a look at the YAML files in `configuration/` if you want to change any default values. The default port is `8080`:

```sh
curl -v http://localhost:8080/health
```
