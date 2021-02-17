// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use omc_bytecode_source_map::source_map::SourceMap;
use common::types::account_address::AccountAddress;
use omc_ir_to_bytecode::{compiler::compile_module, parser::parse_module};
use omc_ir_types::location::Loc;
use std::{fs, path::Path};
use omv_core::file_format::CompiledModule;

pub fn do_compile_module(
    source_path: &Path,
    address: AccountAddress,
    dependencies: &[CompiledModule],
) -> (CompiledModule, SourceMap<Loc>) {
    let source = fs::read_to_string(source_path)
        .with_context(|| format!("Unable to read file: {:?}", source_path))
        .unwrap();
    let file = source_path.as_os_str().to_str().unwrap();
    let parsed_module = parse_module(file, &source).unwrap();
    compile_module(address, parsed_module, dependencies).unwrap()
}