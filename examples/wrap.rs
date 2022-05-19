enum Container {
  I32(i32),
  F32(f32),
  Special,
}

impl Container {
  fn eq(&self, other: &Container) -> bool {
    #[wrap_for_each]
    match self {
      // TODO: If it's not possible to unambiguously detect where the
      // wrapping structure ends, you can try wrapping the for-loop-like
      // inner structure with a $(...).
      for_each!(($Variant, $Value) in [
        (I32, i32),
        (F32, f32),
      ] {
        $Variant(value) => {
          match other {
            $Variant(other_value) => value == other_value,
            _ => false,
          }
        }
      })
      Special => {
        match other {
          Special => true,
          _ => false,
        }
      }
    }
  }
}

fn main() {}
