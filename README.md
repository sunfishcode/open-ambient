<div align="center">
  <h1><code>open-ambient</code></h1>

  <p>
    <strong>Open files and directories with constant paths</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/open-ambient/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/open-ambient/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/open-ambient"><img src="https://img.shields.io/crates/v/open-ambient.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/open-ambient"><img src="https://docs.rs/open-ambient/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

One of the uses for [ambient-authority] is to mark places in the code which
may be opening files or other resources in ways that may be influenced by
untrusted inputs. Paths or other identifiers which are constant and known
at compile time are safe. This crate provides macros for use with [cap-std]
which open files and directories in a way that requires the paths to be
constant and in a way that allows them to be ignored in a clippy scan for
use of dynamic ambient authority.

To use, it add `#![deny(clippy::disallowed_method)]` to your code and copy
[the clippy.toml file], as described [here], for example:

```rust
#![deny(clippy::disallowed_method)]

use open_ambient::open_ambient_file;

fn main() {
    let ok = open_ambient_file!("Cargo.toml").unwrap();
    // ... do stuff with `ok`
    drop(ok);

    let ambient = std::fs::File::open("Cargo.toml").unwrap();
    // ... do stuff with `ambient`
    drop(ambient);
}
```

And run clippy configured with [these instructions]. The above code
gets just one error:

```
error: use of a disallowed method `std::fs::File::open`
  --> test.rs:10:19
   |
10 |     let ambient = std::fs::File::open("Cargo.toml").unwrap();
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
```

The `open_ambient_file!` line does not get an error, while the
`std::fs::File::open` line does.

[ambient-authority]: https://crates.io/crates/ambient-authority
[cap-std]: https://crates.io/crates/cap-std
[here]: https://github.com/sunfishcode/ambient-authority/blob/main/clippy.toml#L5
[these instructions]: https://github.com/sunfishcode/ambient-authority/blob/main/clippy.toml#L18
[the clippy.toml file]: https://github.com/sunfishcode/ambient-authority/blob/main/clippy.toml
