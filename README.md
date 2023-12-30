# Beam-sys

## Overview

`beam_sys` is a Rust library designed for the creation and manipulation of BEAM (Erlang VM) bytecode. It provides an intuitive API to generate `.beam` files, abstracting the intricacies of lower-level details. This project is inspired by the [Tsoding](https://github.com/tsoding) [bada project](https://github.com/tsoding/bada) and incorporates ideas from the [`llvm-sys`](https://lib.rs/crates/llvm-sys) and [`cranelift`](https://lib.rs/crates/cranelift) crates.

## Features

- **Shared Context Management:** Uses a shared context (`beam_sys::context::Context`) for enhanced code generation.
- **High-Level Abstractions:** Offers high-level constructs to build functions, manage export tables, and handle bytecode encoding.
- **Erlang VM Compatibility:** Generated bytecode seamlessly integrates with Erlang systems.

## Usage Examples

### Basic Example

To create a `.beam` file:

```rust
use beam_sys;

fn main() {
    let module = beam_sys::context::Context::create("sys".to_string());
    module.add_basic_bif();

    let code = module.code();
    let mut function_block = code.build_function_block("empty".to_string(), 0);
    {
        function_block.build_return();
    }

    let function_metadata = function_block.function_metadata();
    module.export_table_mut().export_function(function_metadata);

    std::fs::write("sys.beam", module.encode()).unwrap();
}
```

Verify with Erlang shell:

```erl
1> beam_disasm:file(sys).
{beam_file,sys,
           [{'-',0,2}],
           [],[],
           [{function,empty,0,2,
                      [{label,1},
                       {line,1},
                       {func_info,{atom,sys},{atom,empty},0},
                       {label,2},
                       return]}]}
```

### Context Reusability Example

Demonstrating the shared context's reusability across different functions:

```rust
use beam_sys;

fn main() {
    initialize_module();
    add_function();
    // ... more operations
}

fn initialize_module() {
    let module = beam_sys::context::Context::create("sys".to_string());
    module.add_basic_bif();
    // ... additional initializations
}

fn add_function() {
    let module = beam_sys::context::Context::get("sys".to_string());
    let code = module.code();
    let mut function_block = code.build_function_block("test_func".to_string(), 0);
    {
        // ... build the function
        function_block.build_return();
    }

    let function_metadata = function_block.function_metadata();
    module.export_table_mut().export_function(function_metadata);
}
```

In this example, the shared context is created in `initialize_module` and reused in `add_function` without the need to pass the context around, simplifying the API usage.

## Development Status

`beam_sys` is in active development. The API is evolving and may undergo significant changes.

## Contributions

Contributions are welcome! Feel free to submit pull requests or open issues to propose features or discuss bugs.

## License

This project is under the [MIT License](LICENSE).