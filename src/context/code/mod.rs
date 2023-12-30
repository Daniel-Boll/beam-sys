use crate::utils::chunk::encode::encode_chunk;

use self::{opcode::Opcode, tag::Tag};

use super::{export::Export, Context};

pub mod opcode;
pub mod tag;

pub struct Code {
  parent_module_name: String,

  chunk: Vec<u8>,
  label_count: u32,
  function_count: u32,

  instruction_set: u32,
  opcode_max: u32,
  sub_size: u32,
}

impl Code {
  pub(crate) fn new(parent_module_name: String) -> Self {
    Self {
      parent_module_name,
      chunk: vec![],
      label_count: 1,
      function_count: 0,
      instruction_set: 0,
      opcode_max: 182,
      sub_size: 16,
    }
  }

  pub fn build_function_block(&mut self, function_name: String, arity: u32) -> FunctionBlock {
    self.function_count += 1;
    FunctionBlock::new(self, function_name, arity)
  }

  pub(crate) fn encode(&mut self) -> Vec<u8> {
    self.chunk.push(Opcode::IntCodeEnd as u8);
    let mut metadata = vec![];
    metadata.extend(self.sub_size.to_be_bytes());
    metadata.extend(self.instruction_set.to_be_bytes());
    metadata.extend(self.opcode_max.to_be_bytes());
    metadata.extend(self.label_count.to_be_bytes());
    metadata.extend(self.function_count.to_be_bytes());
    metadata.extend(&self.chunk);

    encode_chunk("Code", &mut metadata)
  }
}

pub struct UnsignedValue(Vec<u8>);
pub struct IntValue(Vec<u8>);
pub struct XRegister(Vec<u8>);

pub struct FunctionBlock<'a> {
  code: &'a mut Code,
  instructions: Vec<u8>,
  function_name: String,
  arity: u32,
  label_id: Option<u32>,
}

impl<'a> FunctionBlock<'a> {
  pub(crate) fn new(code: &'a mut Code, function_name: String, arity: u32) -> Self {
    Self {
      code,
      instructions: vec![],
      function_name,
      arity,
      label_id: None,
    }
  }

  pub fn function_metadata(&self) -> Export {
    let context = Context::get(self.code.parent_module_name.clone());
    let atom_table = context.atom_table();

    let function = atom_table
      .find(&self.function_name)
      .expect("The function name should already exists");

    Export::new(
      function,
      self.arity,
      self.label_id.expect("The label id should exists"),
    )
  }

  pub fn build_const_uint(&mut self, value: u32) -> UnsignedValue {
    UnsignedValue(Tag::Unsigned.encode(value as i32))
  }

  pub fn build_const_int(&mut self, value: i32) -> IntValue {
    IntValue(Tag::Integer.encode(value))
  }

  /// The erlang:+/2 bif add function for integers
  /// This function expects lhs and rhs as IntValues
  pub fn build_add_int(&mut self, lhs: IntValue, rhs: IntValue, dest: u32) {
    let context = Context::get(self.code.parent_module_name.clone());
    let import_table = context.import_table();

    let bif_add_function = import_table
      .get_import_function_index("erlang:+/2".to_string())
      .expect("The function erlang:+/2 should be always present") as i32;

    self.build_move_to_x_register(lhs, 0);
    self.build_move_to_x_register(rhs, 1);

    self.instructions.push(Opcode::GcBif2 as u8);
    self.instructions.extend(Tag::Label.encode(0)); // 0 being the label to jump when the bif fails
    self.instructions.extend(Tag::Unsigned.encode(2)); // 🤷
    self
      .instructions
      .extend(Tag::Unsigned.encode(bif_add_function));
    self.instructions.extend(Tag::XRegister.encode(0)); // lhs
    self.instructions.extend(Tag::XRegister.encode(1)); // rhs
    self.instructions.extend(Tag::XRegister.encode(dest as i32)); // destination
  }

  pub fn build_move_to_x_register(&mut self, value: IntValue, x_register: u32) {
    self.instructions.push(Opcode::Move as u8);
    self.instructions.extend(value.0);
    self
      .instructions
      .extend(Tag::XRegister.encode(x_register as i32));
  }

  pub fn build_return(&mut self) {
    self.instructions.push(Opcode::Return as u8);
    self.encode_function_into_code();
  }

  fn encode_function_into_code(&mut self) {
    let module = Context::get(self.code.parent_module_name.clone());
    let atom_table = module.atom_table_mut();

    self.code.chunk.push(Opcode::Label as u8);
    self
      .code
      .chunk
      .extend(Tag::Unsigned.encode(self.code.label_count as i32));
    self.code.label_count += 1;
    self.code.chunk.push(Opcode::Line as u8);
    self.code.chunk.extend(Tag::Unsigned.encode(1));
    self.code.chunk.push(Opcode::FuncInfo as u8);
    self.code.chunk.extend(
      Tag::Atom.encode(
        (atom_table
          .get_index(&self.code.parent_module_name)
          .expect("The module name atom should always exists")
          + 1) as i32,
      ),
    );
    self
      .code
      .chunk
      .extend(Tag::Atom.encode((atom_table.add(&self.function_name).index + 1) as i32));
    self
      .code
      .chunk
      .extend(Tag::Unsigned.encode(self.arity as i32));

    self.code.chunk.push(Opcode::Label as u8);
    self
      .code
      .chunk
      .extend(Tag::Unsigned.encode(self.code.label_count as i32));
    self.label_id = Some(self.code.label_count);
    self.code.label_count += 1;

    self.code.chunk.extend(&self.instructions);
  }
}
