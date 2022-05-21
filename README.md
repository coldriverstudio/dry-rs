# `dry` — Don't Repeat Yourself

[![Latest version](https://img.shields.io/crates/v/dry.svg)](https://crates.io/crates/dry)
[![Documentation](https://docs.rs/dry/badge.svg)](https://docs.rs/dry)
[![License: MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](COPYRIGHT)

<!-- Shows "MIT OR Apache-2.0" for some reason even though it doesn't for `lazy-static` or `clippy`, for example. >
<!-- [![License: MIT/Apache-2.0](https://img.shields.io/crates/l/dry.svg)](COPYRIGHT) -->

Rust macros for idiomatic deduplication of code. Use whenever `macro_rules!`
are too powerful and clunky.

```toml
[dependencies]
dry = "0.1.1"
```

## Usage

### `macro_for!`

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

### `macro_wrap!`

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

Note that because `macro_wrap!` calls `macro_for!` directly in order to make
this work, you don't need to `use dry::macro_for` if you're not using it
anywhere else.

### Features

The `nightly` feature (disabled by default) enables functionality that uses the
unstable [`proc_macro_span`] rustc feature. It enables better syntax checking
(disallows spaces between the "$" and the substitution variable names) and emits
more source code hints on errors (though quick-fixes for macros aren't
available even on nightly yet).

If you're running Rust nightly, you can enable it:

```toml
[dependencies]
dry = { version = "0.1.1", features = ["nightly"] }
```

[`proc_macro_span`]: https://github.com/rust-lang/rust/issues/54725

## About This Crate

### Dependencies

The only dependency is [`proc-macro-error`], for those sweet, sweet, friendly
error messages across Rust versions. In turn, it depends on [`quote`] and
[`proc-macro2`]. However, we don't depend on [`syn`] at all so `dry` should be
really light on compile times.

[`proc-macro-error`]: https://docs.rs/proc-macro-error
[`quote`]: https://docs.rs/quote
[`syn`]: https://docs.rs/syn
[`proc-macro2`]: https://docs.rs/proc-macro2

### Caution

You should try to use an abstraction like looping, traits, or generics whenever
possible and practical. But when it's not, `dry` makes it as painless and
pleasant as possible to avoid repeating yourself.

### Prior Art

#### For Each Loops

- [`duplicate`](https://crates.io/crates/duplicate): The most popular by far
  and works more or less like a regular `for` loop with tuple destructuring,
  except for the very foreign syntax. It offers an attribute syntax which
  avoids some nesting, but at the cost of clarity, in my opinion. The
  function-like syntax can be used wherever the attribute sytnax is valid, plus
  "$"-prefixed identifiers are invalid Rust and therefore not possible to
  implement in an attribute syntax.
- [`akin`](https://crates.io/crates/akin): Overloads the `let` syntax with an
  implicit [for
  comprehension](https://docs.scala-lang.org/tour/for-comprehensions.html).
  Avoids nesting for large numbers of substitution variables but feels too
  magical to feel at home in most Rust codebases, in my opinion.
- [`ct-for`](https://crates.io/crates/ct-for): Almost there! However it uses
  `in ... do` syntax instead of the more familiar `in ... {}` syntax Rustaceans
  are used to. This also makes it more difficult for editors to correctly
  indent the loop body. Finally, the `ct` in `ct_for` is not very descriptive.

All of them use bare identifiers instead of "$"-prefixed identifiers like `dry`
and `macro_rules!` do, which make it clear when macros are being used versus
standard language features within the loop body. None of them support
replicating struct fields, enum cases, or match arms as far as I'm aware.

#### For Each Loops Over Macros

- TODO: https://github.com/Wandalen/wTools/tree/master/module/rust/for_each

#### N Repetitions

- [`repeated`](https://crates.io/crates/repeated): Not quite the same thing,
  but alerted the author to the macro invocation position (e.g. match arms,
  etc.) problem and one way to solve it. In fact, a `match` with many arms for
  an enum in an external crate without generic traits is what spurred initial
  development of `dry`!
- [`seq_macro`](https://crates.io/crates/seq_macro): Inspired the syntax used
  in `macro_for!`. Great if you want to idiomatically iterate over a range of
  numeric or character values instead of a list of tokens at compile time.

## License

`dry` is licensed under the [MIT License](LICENSE-MIT) and the [Apache 2.0 License](LICENSE-APACHE), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contributing

Please [open a pull
request](https://github.com/coldriverstudio/dry-rs/pulls/new) if you'd like to
see anything fixed or added, or [create an
issue](https://github.com/coldriverstudio/dry-rs/issues/new) if you see a
problem or missing feature and you're not sure how or don't have the time to
fix it.

A great place to start is the roadmap below. The first few ones are small and
well-defined enough that they'd make a great first pull request. We'd love to
have you on board!

## Roadmap to 1.0

- [x] Idiomatic `for`-like syntax.
- [x] Helpful compiler error messages and hints, modelled after rustc's errors
      for the equivalent runtime constructions.
- [x] Wrapper for uses where macro invocations are illegal (e.g. struct fields,
      enum cases, match arms): `macro_wrap`.
- [ ] Fix bug where adding stuff after the last `}` is ignored. Should be an
      error instead.
- [ ] Ignore trailing comma (don't generate an additional empty substitution).
      Empty substitution can be generated with `()`.
- [ ] Make it an error to have two consecutive commas. Instead, require a `()`
      in between for an empty substitution.
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
- [ ] How to deal with substitutions with repeated items of groups? `duplicate` solves this with what they call [parametrized substitution](https://docs.rs/duplicate/latest/duplicate/#parameterized-substitution)
