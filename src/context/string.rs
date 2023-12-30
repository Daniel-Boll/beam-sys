use crate::utils::chunk::encode::encode_chunk;

#[allow(dead_code)]
pub struct StringTable {
  parent_module_name: String,
  pub(crate) strings: Vec<String>,
}

impl StringTable {
  pub(crate) fn new(parent_module_name: String) -> Self {
    Self {
      strings: vec![],
      parent_module_name,
    }
  }

  pub(crate) fn encode(&self) -> Vec<u8> {
    let mut string_chunk: Vec<u8> = vec![];

    encode_chunk("StrT", &mut string_chunk)
  }
}
