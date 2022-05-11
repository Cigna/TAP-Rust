# TAP: Test Anything Protocol

`->` Lives on crates.io as the [`testanything`](https://crates.io/crates/testanything) crate.

This Rust library provides facilities for the generating and emitting results in the [Test Anything Protocol](https://en.wikipedia.org/wiki/Test_Anything_Protocol). Please feel free to see [testanything.org](http://testanything.org/) for more information.

## Usage

Please see the examples in the `examples` folder.

Simple:

```
1..2
ok 1 Panda Bamboo
not ok 2 Curry Noodle
# Tree
# Flower
```

### Use with `alloc` only (`#[no_std]`)

To use this crate with alloc in `#[no_std]`, use:
  testanything = { version = "*", default-features = false, features = ["alloc"] }

## Testing

```shell
cargo test
```

## License

[Apache License Version 2.0](https://spdx.org/licenses/Apache-2.0.html)
