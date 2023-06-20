[![License](https://img.shields.io/crates/l/source-chain.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/source-chain.svg)](https://crates.io/crates/source-chain)
[![Docs.rs](https://docs.rs/source-chain/badge.svg)](https://docs.rs/source-chain)

<!-- cargo-sync-readme start -->

# source-chain

Formats StdError with it's source chain

```rust
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("unknown file {1}")]
    UnknownFile(#[source] std::io::Error, &'static str),
}

fn file_error() -> Result<String, Error> {
    let filename = "unknown-file.txt";
    std::fs::read_to_string(filename).map_err(|e| Error::UnknownFile(e, filename))
}

let err = file_error().unwrap_err();
assert_eq!(
    source_chain::to_string(&err),
    "unknown file unknown-file.txt\nCaused by:\n\tNo such file or directory (os error 2)"
);

let dyn_err: Box<dyn std::error::Error> = Box::new(err);
assert_eq!(
    // notice dereferencing
    source_chain::to_string(&*dyn_err),
    "unknown file unknown-file.txt\nCaused by:\n\tNo such file or directory (os error 2)"
);
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
