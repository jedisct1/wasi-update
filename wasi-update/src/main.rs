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

enum ModuleType {
    Freestanding,
    Command,
    Reactor,
}

fn guess_module_type(module: &Module) -> ModuleType {
    let mut has_start = false;
    let mut has_wasi_core_import = false;
    for section in &module.sections {
        if let Section::Standard(section) = section {
            if section.id() == SectionId::Export {
                let payload = section.payload();
                let mut reader = Cursor::new(payload);
                let count = varint::get32(&mut reader).unwrap();
                for _ in 0..count {
                    let name = varint::get_slice(&mut reader).unwrap();
                    let name = std::str::from_utf8(&name).unwrap();
                    let kind = varint::get7(&mut reader).unwrap();
                    if kind != 0 {
                        continue;
                    }
                    let id = varint::get32(&mut reader).unwrap();
                    if id != 8 {
                        continue;
                    }
                    if name == "_start" {
                        has_start = true;
                    }
                }
            } else if section.id() == SectionId::Import {
                let payload = section.payload();
                let mut reader = Cursor::new(payload);
                let count = varint::get32(&mut reader).unwrap();
                print!("import count: {}", count);
                for _ in 0..count {
                    let module_name = varint::get_slice(&mut reader).unwrap();
                    println!("imported: {}", std::str::from_utf8(&module_name).unwrap());
                    let name = varint::get_slice(&mut reader).unwrap();
                    println!("imported: {}", std::str::from_utf8(&name).unwrap());
                    let name = std::str::from_utf8(&name).unwrap();
                    let kind = varint::get7(&mut reader).unwrap();
                    if kind != 0 {
                        continue;
                    }
                    let id = varint::get32(&mut reader).unwrap();
                    if id != 8 {
                        continue;
                    }
                    println!("imported: {}", name);
                    if name == "_start" {
                        return ModuleType::Command;
                    }
                    if module_name == b"wasi_snapshot_preview1" {
                        has_wasi_core_import = true;
                    }
                }
            } else {
                continue;
            }
        }
        if has_start && has_wasi_core_import {
            break;
        }
    }
    match (has_start, has_wasi_core_import) {
        (true, true) => ModuleType::Command,
        (false, true) => ModuleType::Reactor,
        (false, false) => ModuleType::Freestanding,
        (true, false) => ModuleType::Command,
    }
}

fn main() -> Result<(), Error> {
    let matches = clap::command!()
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .num_args(1)
                .value_name("FILE")
                .help("Input file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .num_args(1)
                .value_name("FILE")
                .help("Output file")
                .required(true),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let module_bin = std::fs::read(input_file).unwrap();
    let module = {
        let mut reader = Cursor::new(&module_bin);
        Module::deserialize(&mut reader).unwrap()
    };
    let module_type = guess_module_type(&module);
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
        .module(&module_bin)
        .unwrap()
        .realloc_via_memory_grow(true)
        .adapter(IMPORTED_MODULE_NAME, adapter)
        .unwrap()
        .validate(true)
        .encode()
        .unwrap();

    let mut fp = File::create(output_file).unwrap();
    fp.write_all(&component_bin).unwrap();
    Ok(())
}
