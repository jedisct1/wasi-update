mod error;
mod wasm_module;

use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{BufReader, Cursor, Write},
};
use wasm_module::*;
use wit_component::*;

fn is_command(module: &Module) -> bool {
    for section in &module.sections {
        if let Section::Standard(section) = section {
            if section.id() != SectionId::Export {
                continue;
            }
            let payload = section.payload();
            let mut reader = Cursor::new(payload);
            let count = varint::get32(&mut reader).unwrap();
            for _ in 0..count {
                let name = varint::get_slice(&mut reader).unwrap();
                let name = std::str::from_utf8(&name).unwrap();
                let _kind = varint::get7(&mut reader).unwrap();
                let id = varint::get32(&mut reader).unwrap();
                if id != 8 {
                    continue;
                }
                if name == "_start" {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let input_file = "/tmp/testapp.wasm";
    let output_file = "/tmp/testapp.component";
    let module_bin = std::fs::read(input_file).unwrap();
    let module = {
        let mut reader = Cursor::new(&module_bin);
        Module::deserialize(&mut reader).unwrap()
    };
    let is_command = is_command(&module);

    let adapter = include_bytes!("wasi_snapshot_preview1.command.wasm");
    let component_bin = ComponentEncoder::default()
        .module(&module_bin)
        .unwrap()
        .realloc_via_memory_grow(true)
        .adapter("wasi_snapshot_preview1", adapter)
        .unwrap()
        .adapter("wasi_snapshot_preview1", adapter)
        .unwrap()
        .validate(true)
        .encode()
        .unwrap();
    let mut fp = File::create(output_file).unwrap();
    fp.write_all(&component_bin).unwrap();
}
