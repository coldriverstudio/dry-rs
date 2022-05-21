#![allow(dead_code)]
use dry::{macro_for, macro_wrap};

fn main() {
  // You know the trusty `for` loop.
  for number in [1, 2, 3, 4, 5] {
    println!("{}", number);
  }

  // Use `for_each!` to iterate over tokens at compile time.
  macro_for!($number in [1, 2, 3, 4, 5] {
    println!("{}", $number);
  });

  // Everything between the square brackets is blindly separated by commas;
  // each substitution doesn't have to be an expression.
  macro_for!($statement in [
    print!("hello");,
    print!("world");
  ] {
    $statement
    print!(",");
  });
  println!("");
}

// For example, you can repeat the same struct multiple times with different
// names.
macro_for!($Struct in [A, B, C, D, E] {
  struct $Struct {
    many_fields: bool,
    so_many_fields: bool,
    impossibly_many_fields: bool,
  }
});

// Sometimes the thing you want to repeat isn't in a macro invocation position.

// enum Enum {
//   // for_each!($Variant in [A, B, C, D, E] { // <-- this doesn't work
//   // #[item_for_each($Variant in [A, B, C, D, E])] // <-- this doesn't work either, we checked
//   $Variant,
//   // })
// }

// For those cases, you can wrap the closest ancestor that is in a macro
// invocation position in `macro_wrap!` and then use `for_each!` as usual.
macro_wrap!(enum Enum {
  // ↓ this doesn't work without `macro_wrap!`
  macro_for!($Variant in [A, B, C, D, E] {
    $Variant,
  })
});

// Another common use case where `macro_wrap!` is needed is match arms.
fn match_arms(x: Enum) -> i32 {
  macro_wrap!(match x {
    // ↓ this doesn't work either without `macro_wrap!`
    macro_for!($Variant in [A, B, C, D, E] {
      Enum::$Variant => 1,
    })
  })
}
