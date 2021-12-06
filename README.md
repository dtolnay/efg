Conditional compilation expressions
===================================

Conditional compilation using boolean expression syntax, rather than *any()*,
*all()*, *not()*.

```toml
[dependencies]
efg = "0.0"
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
