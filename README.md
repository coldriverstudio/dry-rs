# `dry` — Don't Repeat Yourself

Rust macros for idiomatic deduplication of code. Use whenever `macro_rules!`
are still too powerful and clunky.

```toml
[dependencies]
dry = "0"
```

## `macro_for!`

You know the trusty `for` loop:

```rust
for number in [1, 2, 3, 4, 5] {
  println!("{}", number);
}
```

Use `macro_for!` to iterate over tokens at compile time:

```rust
macro_for!($Struct in [A, B, C, D, E] {
  struct $Struct {
    many_fields: bool,
    so_many_fields: bool,
    impossibly_many_fields: bool,
  }
});
```

Compared to using `macro_rules!`:

```rust
macro_rules! my_struct {
  ($Struct:ident) => {
    struct $Struct {
      many_fields: bool,
      so_many_fields: bool,
      impossibly_many_fields: bool,
    }
  };
}
my_struct!(A);
my_struct!(B);
my_struct!(C);
my_struct!(D);
my_struct!(E);
```

See the [examples](examples) for more details.

## `macro_wrap!`

Allows you to use the other macros in this crate in places where macro
invocations are illegal (e.g. struct fields, enum cases, match arms).

Wrap the closest syntax tree ancestor that is in a macro invocation position
and you're good to go:

```rust
macro_wrap!(match x {
  // ↓ can't usually call macros here, but `macro_wrap!` makes it work
  macro_for!($Variant in [A, B, C, D, E] {
    Enum::$Variant => 1,
  })
})
```

## Features

The `nightly` feature (disabled by default) enables functionality that uses the
unstable [`proc_macro_span`] rustc feature. It enables better syntax checking
(disallows spaces between the "$" and the substitution variable names) and emits
more source code hints on errors (though quick-fixes for macros aren't
available even on nightly yet).

If you're running Rust nightly, you can enable it:

```toml
[dependencies]
dry = { version = "0", features = ["nightly"] }
```

[`proc_macro_span`]: https://github.com/rust-lang/rust/issues/54725

## Dependencies

The only dependency is [`proc-macro-error`], for those sweet, sweet, friendly
error messages across Rust versions. In turn, it depends on [`quote`] and
[`proc-macro2`]. However, we don't depend on [`syn`] at all so `dry` should be
really light on compile times.

[`proc-macro-error`]: https://docs.rs/proc-macro-error
[`quote`]: https://docs.rs/quote
[`syn`]: https://docs.rs/syn
[`proc-macro2`]: https://docs.rs/proc-macro2

## Caution

You should try to use an abstraction like looping, traits, or generics if at
all possible. But when it's not, `dry` makes it as painless and pleasant as
possible to avoid repeating yourself.

## Roadmap

- [x] Idiomatic `for`-like syntax.
- [x] Helpful compiler error messages and hints, modelled after rustc's errors
      for the equivalent runtime constructions.
- [x] Wrapper for uses where macro invocations are illegal (e.g. struct fields,
      enum cases, match arms): `macro_wrap`.
- [x] Fix bug where adding stuff after the last `}` is ignored. Should be an
      error instead.
- [ ] Better documentation
- [ ] Testing
- [ ] Support multiple substitution variables using a tuple-destructuring-like
      syntax
- [ ] Support commas in substitutions by wrapping in parentheses (and support
      parentheses by doubling them)
- [ ] Figure out minimum Rust version
- [ ] Nesting with scoped substitution variables (currently substitution
      variables are expanded outside-in, not inside-out like you would expect in a
      regular `for` loop)
- [ ] `macro_let` macro for idiomatic substitutions (replaces `macro_rules!`
      without syntax arguments)
- [ ] Investigate joining substitutions with syntax elements in the loop body?
      Like identifiers (`$variable~_suffix`), or operators (`variable $op~= change`). This is meant to be a straightforward replacement for
      `macro_rules!` in simple cases, though. How does it solve this problem?
      See `paste` crate.
- [ ] Can `macro_wrap` expand macros outside of this crate, too? Probably not,
      but let's investigate. Maybe we can let other macro crates plug into it
      if we can't do it automatically.
