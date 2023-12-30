// /usr/lib/erlang/lib/compiler-8.4.1/src/beam_opcodes.hrl
// -define(tag_u, 0).
// -define(tag_i, 1).
// -define(tag_a, 2).
// -define(tag_x, 3).
// -define(tag_y, 4).
// -define(tag_f, 5).
// -define(tag_h, 6).
// -define(tag_z, 7).
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum Tag {
  Unsigned = 0,
  Integer = 1,
  Atom = 2,
  XRegister = 3,
  YRegister = 4,
  Label = 5,
}

impl Tag {
  pub(crate) fn encode(self, n: i32) -> Vec<u8> {
    match n {
      0..16 => {
        vec![((n as u8) << 4) | (self as u8)]
      }
      16..2048 => {
        vec![
          (((n >> 3) & 0b11100000) | self as i32 | 0b00001000) as u8,
          (n & 0xff) as u8,
        ]
      }
      _ => todo!("Currently support only for the interval 0 <= n < 2048"),
    }
  }
}
