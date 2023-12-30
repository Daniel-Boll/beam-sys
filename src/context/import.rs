use crate::utils::chunk::encode::encode_chunk;

use super::{atom::Atom, Context};

pub struct ImportTable {
  parent_module_name: String,
  pub(crate) imports: Vec<Import>,
}

pub struct Import {
  pub(crate) module: Atom,
  pub(crate) function: Atom,
  pub(crate) arity: u32,
}

impl ImportTable {
  pub(crate) fn new(parent_module_name: String) -> Self {
    Self {
      imports: vec![],
      parent_module_name,
    }
  }

  pub(crate) fn encode(&self) -> Vec<u8> {
    let mut import_chunk: Vec<u8> = vec![];
    import_chunk.extend((self.imports.len() as u32).to_be_bytes());

    for import in self.imports.iter() {
      import_chunk.extend((import.module.index + 1).to_be_bytes());
      import_chunk.extend((import.function.index + 1).to_be_bytes());
      import_chunk.extend(import.arity.to_be_bytes());
    }

    encode_chunk("ImpT", &mut import_chunk)
  }

  pub(crate) fn get_import_function_index(&self, import_function: String) -> Option<u32> {
    let (module, function, arity) = ImportTable::parse_function_signature(import_function.as_str());

    for (i, import) in self.imports.iter().enumerate() {
      if import.module.name == module && import.function.name == function && import.arity == arity {
        return Some(i as u32);
      }
    }

    None
  }

  pub fn base_functions(&mut self) {
    let context = Context::get(self.parent_module_name.clone());
    let atom_table = context.atom_table_mut();

    let functions = vec!["erlang:+/2", "erlang:-/2"];

    for function in functions {
      let (module, function, arity) = Self::parse_function_signature(function);
      let module = atom_table.get(module).clone();
      let function = atom_table.get(function).clone();

      self.imports.push(Import {
        module,
        function,
        arity,
      });
    }
  }

  fn parse_function_signature(function: &str) -> (&str, &str, u32) {
    let (module, function) = function.split_once(':').unwrap();
    let mut parts = function.splitn(2, '/');
    let function = parts.next().unwrap();
    let arity = parts.next().unwrap().parse::<u32>().unwrap();

    (module, function, arity)
  }
}
