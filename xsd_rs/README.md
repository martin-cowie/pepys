# Contents

Each package in this space holds code generated from the XML Schemata and WSDL files in `../schemata`.
Rust source is generated using the tools from https://github.com/lumeohq/xsd-parser-rs, which is a self
described work in process (if a good one).

Once generated the code is amended to satisfy dependencies, and encapsulated as a package.

## Package Creation

This illustrates the process uses to create `xsd_rs/xmlmime --lib` though the process is broadly the same for each.

1. `cargo new xsd_rs/xmlmime --lib` to createa a library package skeleton.
1. Add `xsd_rs/xmlmime` to `workspace.members` in the root `Cargo.toml`.
1. `xsd-parser -i ./schemata/xmlmime.xsd ./xsd_rs/xmlmime/src/lib.rs` to generate the code
1. Preface the generated code with
```
#![allow(clippy::derive_partial_eq_without_eq)]

// External dependencies
use yaserde_derive::{YaDeserialize, YaSerialize};
use xsd_macro_utils::*;
use xsd_types::types as xs;

// Local dependencies
use validate::Validate;

// Runtime dependencies
use std::str::FromStr;

// Generated from xmlmime.xsd hereon ---------------------------------
```
5. Add the following dependcies to the new package's `Cargo.toml`
```
validate = { path = "../../validate" }
xml-rs.workspace = true
yaserde.workspace = true
yaserde_derive.workspace = true
xsd-macro-utils.workspace = true
xsd-types.workspace = true
```
5. Run `cargo clippy` and remove any unused `use` statements, and any related dependencies from `Cargo.toml`
5. Resolve any missing dependencies: generated code can include commented type hints such as `// pub type Filter = FilterType;` which are the most likely solution to missing types.
5. Type hints resolving to a simple type such as `bool` or `String` may not satisfy YaSerde, and must be replaced with an empty struct, e.g.
```
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct FixedTopicSet {}
```