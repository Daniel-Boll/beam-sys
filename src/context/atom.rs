use crate::utils::chunk::encode::encode_chunk;

/// Atom Table
/// The atom table is a list of atoms used in the module.
/// The first atom is always the `nil` atom, the second is always the module name.
///
/// Example usage:
/// ```rust
/// let module = beam_sys::Context::get("out");
/// let atom = module.atom_table().add("atom");
/// assert_eq!(atom.name, "atom");
/// assert_eq!(atom.index, 2);
/// ```
///
/// ```rust
/// let module = beam_sys::Context::get("out");
/// let atom_table = module.atom_table();
/// assert_eq!(atom_table.get(0).name, "nil");
/// assert_eq!(atom_table.get(1).name, "out");
/// ```
#[derive(Debug)]
pub struct AtomTable {
  atoms: Vec<Atom>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Atom {
  pub name: String,
  pub(crate) index: u32,
}

impl AtomTable {
  pub(crate) fn new(module_name: String) -> Self {
    Self {
      atoms: vec![
        // Atom {
        //   name: "nil".to_string(),
        //   index: 0,
        // },
        Atom {
          name: module_name,
          index: 0,
        },
      ],
    }
  }

  pub(crate) fn encode(&self) -> Vec<u8> {
    let mut atom_chunk: Vec<u8> = vec![];
    atom_chunk.extend((self.atoms.len() as u32).to_be_bytes());

    for atom in self.atoms.iter() {
      atom_chunk.extend((atom.name.len() as u8).to_be_bytes());
      atom_chunk.extend(atom.name.as_bytes());
    }

    encode_chunk("AtU8", &mut atom_chunk)
  }

  pub fn add<'a>(&'a mut self, atom: &str) -> &'a Atom {
    let atom = Atom {
      name: atom.to_string(),
      index: self.atoms.len() as u32,
    };
    self.atoms.push(atom);
    self.atoms.last().unwrap()
  }

  /// Get an atom by name, if it doesn't exist, create it
  pub fn get<'a>(&'a mut self, atom: &str) -> &'a Atom {
    if let Some(index) = self.get_index(atom) {
      return self.get_by_index(index);
    }
    self.add(atom)
  }

  pub fn find(&self, atom: &str) -> Option<Atom> {
    for a in &self.atoms {
      if a.name == atom {
        return Some(a.clone());
      }
    }
    None
  }

  pub fn get_by_index(&self, index: u32) -> &Atom {
    self.atoms.get(index as usize).unwrap()
  }

  pub fn get_index(&self, atom: &str) -> Option<u32> {
    for a in &self.atoms {
      if a.name == atom {
        return Some(a.index);
      }
    }
    None
  }
}
