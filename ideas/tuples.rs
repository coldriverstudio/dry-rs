// TODO

// use dry::*;

// // This is a contrived example. In this case it would be best to actually
// // duplicate the code, it's almost shorter and certainly easier to read. Even
// // if there were more cases, it would actually be better to put the repeated
// // values in an array and iterate over that using a regular for loop.
// //
// // But you can imagine cases where neither would be possible or desirable, and
// // other abstractions like traits or generics won't work. For such cases,
// // `dry` to the rescue!

// fn handle_keys() {
//   dry!(($Key, $Sign, $Axis) in [
//     (W, -, Z),
//     (S, +, Z),
//     (A, -, X),
//     (D, +, X),
//   ] {
//     if keys.pressed(KeyCode::$Key) {
//       direction $Sign= Vec3::$Axis;
//     }
//   })
// }

fn main() {}
