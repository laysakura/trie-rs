# {{crate}}

{{readme}}

## Versions
trie-rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

## Rust Version Supports

trie-rs is continuously tested with these Rust versions in Travis CI:

- 1.33.0
- Latest stable version

So it expectedly works with Rust 1.33.0 and any newer versions.

Older versions may also work, but are not tested or guaranteed.

## Contributing

Any kind of pull requests are appreciated.

### Guidelines

- `README.md` is generated from `$ cargo readme` command. Do not manually update `README.md` but edit `src/lib.rs` and then `$ cargo readme > README.md`.
- Travis CI automatically does the following commit & push to your pull-requests:
    - `$ cargo readme > README.md`
    - `$ cargo fmt --all`

## License

{{license}}
