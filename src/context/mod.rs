use singleton_manager::SingletonManager;

use crate::utils::chunk::encode::encode_chunk;

pub mod atom;
pub mod code;
pub mod export;
pub mod import;
pub mod string;

#[allow(dead_code)]
pub struct Context {
  module_name: String,
  atom_table: atom::AtomTable,
  import_table: import::ImportTable,
  export_table: export::ExportTable,
  string_table: string::StringTable,
  code: code::Code,
}

impl Context {
  pub fn create(module_name: String) -> &'static mut Context {
    SingletonManager::instance()
      .set(
        &module_name.clone(),
        Context {
          atom_table: atom::AtomTable::new(module_name.clone()),
          code: code::Code::new(module_name.clone()),
          import_table: import::ImportTable::new(module_name.clone()),
          export_table: export::ExportTable::new(module_name.clone()),
          string_table: string::StringTable::new(module_name.clone()),
          module_name,
        },
      )
      .expect("Failed to create context")
  }

  /// Get or create a new context
  pub fn get(module_name: String) -> &'static mut Context {
    SingletonManager::instance()
      .get::<Context>(&module_name)
      .expect("Failed to get context")
  }

  pub fn add_basic_bif(&mut self) {
    self.import_table.base_functions();
  }

  /// Get the atom table
  /// Give the reference to the atom table to be mutaded
  pub fn atom_table_mut(&mut self) -> &mut atom::AtomTable {
    &mut self.atom_table
  }

  pub fn atom_table(&self) -> &atom::AtomTable {
    &self.atom_table
  }

  /// Get the import table
  #[allow(dead_code)]
  pub fn import_table(&self) -> &import::ImportTable {
    &self.import_table
  }

  #[allow(dead_code)]
  pub fn import_table_mut(&mut self) -> &mut import::ImportTable {
    &mut self.import_table
  }

  pub fn export_table(&self) -> &export::ExportTable {
    &self.export_table
  }

  pub fn export_table_mut(&mut self) -> &mut export::ExportTable {
    &mut self.export_table
  }

  /// Get the code
  pub fn code(&mut self) -> &mut code::Code {
    &mut self.code
  }

  pub fn encode(&mut self) -> Vec<u8> {
    let mut content: Vec<u8> = vec![];
    content.extend(b"BEAM");

    content.append(&mut self.atom_table.encode());
    content.append(&mut self.import_table.encode());
    content.append(&mut self.export_table.encode());
    content.append(&mut self.string_table.encode());
    content.append(&mut self.code.encode());

    encode_chunk("FOR1", &mut content)
  }
}
