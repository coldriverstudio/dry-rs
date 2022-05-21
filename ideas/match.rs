// TODO

// enum Container {
//   I32(i32),
//   F32(f32),
// }

// impl Container {
//   fn eq(&self, other: &Container) -> bool {
//     match_for_each!(self {
//       ($Variant, $Value) in [
//         (I32, i32),
//         (F32, f32),
//       ] {
//         $Variant(value) => {
//           match other {
//             $Variant(other_value) => value == other_value,
//             _ => false,
//           }
//         }
//       }
//     })

//     #[for_each_inside(($Variant, $Value) in [
//       (I32, i32),
//       (F32, f32),
//     ])]
//     match self {
//       ${
//         $Variant(value) => {
//           match other {
//             $Variant(other_value) => value == other_value,
//             _ => false,
//           }
//         }
//       }
//     }
//   }
// }

fn main() {}
