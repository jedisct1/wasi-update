mod error;
mod wasm_module;

pub use anyhow::{anyhow, bail, ensure, Error};
use clap::Arg;
use std::{
    fs::File,
    io::{Cursor, Write},
};
use wasm_module::*;
use wit_component::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum ModuleType {
    Freestanding,
    Command,
    Reactor,
}

impl std::fmt::Display for ModuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleType::Freestanding => write!(f, "Freestanding"),
            ModuleType::Command => write!(f, "WASI-core command"),
            ModuleType::Reactor => write!(f, "WASI-core reactor"),
        }
    }
}

fn guess_module_type(module: &Module) -> Result<ModuleType, Error> {
    let mut has_start = false;
    let mut has_wasi_core_import = false;
    for section in &module.sections {
        if let Section::Standard(section) = section {
            if section.id() == SectionId::Export {
                let payload = section.payload();
                let mut reader = Cursor::new(payload);
                let count = varint::get32(&mut reader)?;
                for _ in 0..count {
                    let name = varint::get_slice(&mut reader)?;
                    let name = std::str::from_utf8(&name)?;
                    let kind = varint::get7(&mut reader)?;
                    let _id = varint::get32(&mut reader)?;
                    if kind == 0 {
                        println!("Exported:\t{}", name);
                        if name == "_start" {
                            has_start = true;
                        }
                    }
                }
            } else if section.id() == SectionId::Import {
                let payload = section.payload();
                let mut reader = Cursor::new(payload);
                let count = varint::get32(&mut reader)?;
                for _ in 0..count {
                    let module_name = varint::get_slice(&mut reader)?;
                    let module_name = std::str::from_utf8(&module_name)?;
                    let name = varint::get_slice(&mut reader)?;
                    let name = std::str::from_utf8(&name)?;
                    let kind = varint::get7(&mut reader)?;
                    let _id = varint::get32(&mut reader)?;
                    if kind == 0 {
                        println!("Imported:\t{}#{}", module_name, name);
                        if module_name == "wasi_snapshot_preview1" {
                            has_wasi_core_import = true;
                        }
                    }
                }
            }
        }
    }
    let res = match (has_start, has_wasi_core_import) {
        (true, true) => ModuleType::Command,
        (false, true) => ModuleType::Reactor,
        (false, false) => ModuleType::Freestanding,
        (true, false) => ModuleType::Command,
    };
    Ok(res)
}

fn main() -> Result<(), Error> {
    let matches = clap::command!()
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .num_args(1)
                .value_name("FILE")
                .help("Input file (regular module)")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .num_args(1)
                .value_name("FILE")
                .help("Output file (component)")
                .required(true),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    println!("Reading module from:\t[{}]", input_file);
    let module_bin = std::fs::read(input_file)?;
    let module = {
        let mut reader = Cursor::new(&module_bin);
        Module::deserialize(&mut reader)?
    };
    println!("Module size:\t{} bytes", module_bin.len());

    let module_type = guess_module_type(&module)?;
    println!("Module type:\t{}", module_type);

    static IMPORTED_MODULE_NAME: &str = "wasi_snapshot_preview1";
    let adapter = match module_type {
        ModuleType::Command => include_bytes!(concat!(
            "precomp/",
            "wasi_snapshot_preview1",
            ".command.wasm"
        ))
        .as_ref(),
        ModuleType::Reactor | ModuleType::Freestanding => include_bytes!(concat!(
            "precomp/",
            "wasi_snapshot_preview1",
            ".reactor.wasm"
        ))
        .as_ref(),
    };

    let component_bin = ComponentEncoder::default()
        .module(&module_bin)?
        .realloc_via_memory_grow(true)
        .adapter(IMPORTED_MODULE_NAME, adapter)?
        .validate(true)
        .encode()?;

    println!("Verifying component");
    Module::deserialize(&mut Cursor::new(&component_bin))?;

    println!("Writing component to:\t[{}]", output_file);
    let mut fp = File::create(output_file)?;
    fp.write_all(&component_bin)?;
    println!(
        "Component size:\t{} bytes (overhead: {} bytes)",
        component_bin.len(),
        component_bin.len() - module_bin.len()
    );

    Ok(())
}
