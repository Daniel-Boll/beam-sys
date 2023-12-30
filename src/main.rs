#![feature(exclusive_range_pattern)]

fn main() -> anyhow::Result<()> {
  let module = beam_sys::context::Context::create("sys".to_string());
  module.add_basic_bif();

  let code = module.code();
  let mut function_block = code.build_function_block("add".to_string(), 0);
  {
    // let lhs = function_block.build_const_int(1);
    // let rhs = function_block.build_const_int(2);
    // function_block.build_add_int(lhs, rhs, 0);
    function_block.build_return();
  }

  let function_metadata = function_block.function_metadata();
  module.export_table_mut().export_function(function_metadata);

  std::fs::write("sys.beam", module.encode())?;

  Ok(())
}
