use crate::utils::chunk::encode::encode_chunk;

use super::atom::Atom;

#[allow(dead_code)]
pub struct ExportTable {
  parent_module_name: String,
  pub(crate) exports: Vec<Export>,
}

pub struct Export {
  function: Atom,
  arity: u32,
  label_id: u32,
}

impl Export {
  pub(crate) fn new(function: Atom, arity: u32, label_id: u32) -> Self {
    Self {
      function,
      arity,
      label_id,
    }
  }
}

impl ExportTable {
  pub(crate) fn new(parent_module_name: String) -> Self {
    Self {
      exports: vec![],
      parent_module_name,
    }
  }

  pub fn export_function(&mut self, function: Export) {
    self.exports.push(function);
  }

  pub(crate) fn encode(&self) -> Vec<u8> {
    let mut export_chunk: Vec<u8> = vec![];
    export_chunk.extend((self.exports.len() as u32).to_be_bytes());

    for export in self.exports.iter() {
      export_chunk.extend(export.function.index.to_be_bytes());
      export_chunk.extend(export.arity.to_be_bytes());
      export_chunk.extend(export.label_id.to_be_bytes());
    }

    encode_chunk("ExpT", &mut export_chunk)
  }
}
