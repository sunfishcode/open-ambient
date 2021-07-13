<div align="center">
  <h1><code>open-ambient</code></h1>

  <p>
    <strong>Ambient Authority</strong>
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

See [here] for instructions on how to configure clippy for this purpose.

[ambient-authority]: https://crates.io/crates/ambient-authority
[cap-std]: https://crates.io/crates/cap-std
[here]: https://github.com/sunfishcode/ambient-authority/blob/main/clippy.toml#L14
