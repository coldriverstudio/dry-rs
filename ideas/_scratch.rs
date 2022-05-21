fn main() {
  for_each!($number in [1, 2, 3, 4, 5] {
    for_each!($letter in ['a', 'b', 'c', 'd', 'e'] {
      println!("{} {}", $number, $letter);
    });
  });

  // TODO: This is a natural result of the way macros are expanded in Rust, but
  // looks wrong and isn't how the regular `for` loop works.
  for_each!($number in [$letter, 2, 3, 4, 5] {
    for_each!($letter in ['a', 'b', 'c', 'd', 'e'] {
      println!("{} {}", $number, $letter);
    });
  });

  for_each!($number in [$letter, 2, 3, 4, 5] {
    for_each!($letter in [$number, 'b', 'c', 'd', 'e'] {
      println!("{} {}", $number, $letter);
    });
  });

  let_!($message = "Hello, world!" {
    println!($message);
    println!("{} (again)", $message);
  });

  let_!(
    $struct = ( // <-- wrap curly braces inside parentheses
      struct Struct {
        awesome: bool,
      }
    ),
    $print = println!("{}", Struct { awesome: ($value) }.awesome)
  {
    {
      $struct
      let_!($value = false { $print })
    }
    {
      $struct
      let_!($value = true { $print })
    }
  });

  {
    macro_rules! my_struct {
      () => {
        struct Struct {
          awesome: bool,
        }
      };
    }

    macro_rules! my_print {
      ($value:expr) => {
        println!("{}", Struct { awesome: ($value) }.awesome);
      };
    }

    {
      my_struct!();

      macro_rules! my_value {
        () => {
          false
        };
      }
      my_print!(my_value!());
    }
    {
      my_struct!();

      macro_rules! my_value {
        () => {
          true
        };
      }
      my_print!(my_value!());
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

// TODO: This is much more useful with tuples.

// TODO: Saving the subsitutions for later use in multiple places? Maybe have
// wrap_for_each do that. No, because it could be used elsewhere too so it
// should be its own macro. It actually composes well because macros are
// applied outside-in.

// substitute!($variants = [A, B, C, D, E] {
//   wrap_for_each!(match x {
//     for_each!($Variant in $variants {
//       Enum::$Variant => None,
//     })
//   })
// });

// If you want to duplicate an module-level item, you can use the attribute
// macro `item_for_each` to avoid a bit of indentation. Your call!
// #[item_for_each($id in [AX, BX, CX, DX, EX])]
// struct $id {
//   many_fields: bool,
//   so_many_fields: bool,
//   impossibly_many_fields: bool,
// }

// Sometimes you want to repeat something somewhere that isn't a macro
// invocation position so we can't use the function-like macro `for_each`. If
// that something is a single item, you're in luck: you can use the attribute
// macro `item_for_each`.
// struct Struct {
//   // for_each!($id in [a, b, c, d, e] { // <-- this doesn't work
//   #[item_for_each(id in [a, b, c, d, e])]
//   id: u32,
//   // })
// }

// fn attribute_macro_on_statement() {
//   // Unfortunately, using attribute macros on statements is not supported by
//   // Rust yet: https://github.com/rust-lang/rust/issues/54727
//   //
//   //     #[item_for_each($number in [1, 2, 3, 4, 5])] // <-- this doesn't work
//   //     println!("{}", 2);
// }
