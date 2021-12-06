Conditional compilation expressions
===================================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/efg-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/efg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/efg.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/efg)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-efg-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/efg)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/efg/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/efg/actions?query=branch%3Amaster)

Conditional compilation using boolean expression syntax, rather than *any()*,
*all()*, *not()*.

```toml
[dependencies]
efg = "0.1"
```

<br>

## Summary

Rust's `cfg` and `cfg_attr` conditional compilation attributes use a restrictive
domain-specific language for specifying configuration predicates. The syntax is
described in the *[Conditional compilation]* page of the Rust reference. The
reason for this syntax as opposed to ordinary boolean expressions was to
accommodate restrictions that old versions of rustc used to have on the grammar
of attributes.

However, all restrictions on the attribute grammar were lifted in Rust 1.18.0 by
[rust-lang/rust#40346]. This crate explores implementing conditional compilation
using ordinary boolean expressions
instead:&ensp;`&&`,&ensp;`||`,&ensp;`!`&ensp;as usual in Rust syntax.

[Conditional compilation]: https://doc.rust-lang.org/1.57.0/reference/conditional-compilation.html
[rust-lang/rust#40346]: https://github.com/rust-lang/rust/pull/40346

<table>
<tr><th><center>built into rustc</center></th><th><center>this crate</center></th></tr>
<tr><td><code>#[cfg(any(<i>thing1</i>, <i>thing2</i>, &hellip;)]</code></td><td><code>#[efg(<i>thing1</i> || <i>thing2</i> || &hellip;)]</code></td></tr>
<tr><td><code>#[cfg(all(<i>thing1</i>, <i>thing2</i>, &hellip;)]</code></td><td><code>#[efg(<i>thing1</i> &amp;&amp; <i>thing2</i> &amp;&amp; &hellip;)]</code></td></tr>
<tr><td><code>#[cfg(not(<i>thing</i>))]</code></td><td><code>#[efg(!<i>thing</i>)]</code></td></tr>
</table>

<br>

## Examples

A real-world example from the `quote` crate:

```rust
#[efg(feature = "proc-macro" && !(target_arch = "wasm32" && target_os = "unknown"))]
extern crate proc_macro;
```

and from the `proc-macro2` crate:

```rust
#[efg(super_unstable || feature = "span-locations")]
pub fn start(&self) -> LineColumn {
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
