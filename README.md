# Bevy TMX Map Parser and Code Generator

This project demonstrates how to parse a TMX file, generate corresponding Rust code, and integrate it into a Bevy game. The generated code is written to a file during the build process and can be used within the main game code.

## Overview

The main goal of this project is to:

1. Parse a TMX file containing map data.
2. Generate Rust structs and data from the parsed map.
3. Output the generated code to `target/debug/build/<project_name><your_project_hash>/out/generated_code.rs`.
4. Include the generated code in the main game and use it.

## Dependencies

- `bevy` - A data-driven game engine built in Rust.
- `proc-macro2` - A library for working with Rust's procedural macro API.
- `quote` - A library for generating Rust code.
- `xml` - A library for parsing XML files.
- `once_cell` - A library for lazily-initialized static variables.
