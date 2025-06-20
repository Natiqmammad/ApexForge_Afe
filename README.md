apexforge_afe
A Rust library for generating .afe files for the ApexForge platform. It allows developers to embed Lua scripts with metadata such as application name, version, and permissions, serialized into a compact binary format using bincode. With mlua integration, it supports Lua 5.4 scripts, making it perfect for building ApexForge applications.

Features

Create .afe files with a fluent builder API.
Embed Lua scripts, including functions and raw code, validated with mlua (Lua 5.4).
Customize permissions in rwxr-xr-x format.
Efficient binary serialization using bincode and serde.

Installation
Add the library to your Cargo.toml:
[dependencies]
apexforge_afe = "0.2.0"

Quick Start
The following example creates an .afe file that runs a Lua script to print "Hello, ApexForge World!":
use apexforge_afe::AfeBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut hello = AfeBuilder::new("HelloWorld", "1.0");
    hello.add_lua_code(r#"
        print("Hello, ApexForge World!")
    "#);
    
    let afe_path = "hello.afe";
    hello.save(&afe_path)?;
    
    println!("Created {}", afe_path);
    Ok(())
}

This code:

Initializes an AfeBuilder with the app name "HelloWorld" and version "1.0".
Adds a simple Lua script.
Saves the configuration as a binary .afe file.

Key Methods

AfeBuilder::new(app_name, app_version): Create a new builder instance.
add_lua_function(name, args, body): Define a Lua function with arguments and body.
add_lua_code(code): Append raw Lua code.
set_permissions(permissions): Set permissions (e.g., "rwxr-xr-x").
save(path): Save the .afe file to disk.
build(): Return an AfeFile struct without saving.

Example Use Cases

Build ApexForge applications with embedded Lua logic.
Package scripts with metadata for deployment on ApexForge platforms.
Automate .afe file generation in build pipelines.

Contributing
We welcome contributions! Please:

Submit issues or pull requests on GitHub.
Adhere to Rust coding standards.
Include tests for new features.

License
Licensed under MIT OR Apache-2.0.
Dependencies

bincode: Binary serialization.
mlua: Lua 5.4 integration.
serde: Serialization framework.

