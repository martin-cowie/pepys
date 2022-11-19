# README

This crate generates Rust source from input XML. Generated Rust sources
are mostly generate canned 'known' responses for testing. Most instances
of the `ExampleData` trait start this way.

It uses crates in /wsdl_rs and /xsd_rs for the heavy lifting to generate
Rust source which is imperfect but useful, often requiring manual editing
to match the intended Rust struct.