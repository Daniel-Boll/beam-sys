// /usr/lib/erlang/lib/compiler-8.4.1/src/beam_opcodes.erl
#[derive(Copy, Clone)]
#[repr(u8)]
pub(crate) enum Opcode {
  Label = 1,
  FuncInfo = 2,
  IntCodeEnd = 3,
  Return = 19,
  Move = 64,
  GcBif2 = 125,
  Line = 153,
}
