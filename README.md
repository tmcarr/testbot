# testbot

Toying around with Rust and the [serenity library](https://github.com/serenity-rs/serenity). It's docs are [located here](https://docs.rs/serenity/)

![CI](https://github.com/tmcarr/testbot/workflows/CI/badge.svg?branch=master)

### Notes for M1 Macs:

You need to use rustup to target x86 since some of diesel deps dont really like ARM yet.

```
rustup default nightly-x86_64-apple-darwin
```
