use for_each_macro::for_each;

fn main() {
  for_each!($number in [1, 2, 3, 4, 5] {
    println!("{}", $number);
  });

  // #[for_each($number in [1, 2, 3, 4, 5])]
  // println!("{}", $number);

  // For reference, here is the syntax for the regular `for` loop.
  for number in [1, 2, 3, 4, 5] {
    println!("{}", number);
  }
}
