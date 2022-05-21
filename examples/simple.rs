#![allow(dead_code)]
use for_each_macro::{for_each, wrap_for_each};

fn main() {
  // You know the trusty `for` loop.
  for number in [1, 2, 3, 4, 5] {
    println!("{}", number);
  }

  // Use `for_each!` to duplicate the code, not just run the code multiple
  // times.
  for_each!($number in [1, 2, 3, 4, 5] {
    println!("{}", $number);
  });

  // Everything between the square brackets is blindly separated by commas;
  // each substitution doesn't have to be an expression.
  for_each!($statement in [
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
for_each!($id in [A, B, C, D, E] {
  struct $id {
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
// invocation position in `wrap_for_each!` and then use `for_each!` as usual.
wrap_for_each!(enum Enum {
  // ↓ this doesn't work without `wrap_for_each!`
  for_each!($Variant in [A, B, C, D, E] {
    $Variant,
  })
});

// Another common use case where `wrap_for_each!` is needed is match arms.
fn match_arms(x: Enum) -> i32 {
  wrap_for_each!(match x {
    // ↓ this doesn't work either without `wrap_for_each!`
    for_each!($Variant in [A, B, C, D, E] {
      Enum::$Variant => 1,
    })
  })
}
