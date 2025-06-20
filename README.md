**apexforge\_afe**

A Rust library for generating `.afe` files for the ApexForge platform. It allows developers to embed Lua scripts with metadata such as application name, version, permissions, dependencies, environment variables, and entry points. Leveraging `bincode` and `serde` for compact binary serialization, and `mlua` for Lua 5.4 integration, `apexforge_afe` makes it easy to build, validate, and package ApexForge applications.

---

## 📦 Features

* **Fluent Builder API**: Create `.afe` files with chained methods.
* **Embed Lua Scripts**: Add functions or raw code, validated against Lua 5.4 at build time.
* **Metadata Support**:

  * Application name & version
  * File permissions in `rwxr-xr-x` format
  * Dependencies (other modules or libraries)
  * Environment variables
  * Custom entry point (default: `main`)
* **Efficient Serialization**: Compact binary output using `bincode` and `serde`.
* **Executable `.afe`**: Optionally emit a Unix shebang header so `.afe` files can be run directly.

---

## 🚀 Installation

Add to your project's `Cargo.toml`:

```toml
[dependencies]
apexforge_afe = "0.2.1"
```

Then run:

```bash
cargo fetch
```

---

## 🛠️ Quick Start

Create and save an `.afe` file:

```rust
use apexforge_afe::AfeBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize builder with app name and version
    let mut builder = AfeBuilder::new("HelloWorld", "1.0.0");

    // Add a simple Lua function and raw code
    builder
        .add_lua_function("greet", vec!["name"], r#"
            print("Hello from ApexForge, " .. name)
        "#)
        .add_lua_code(r#"
            greet("Developer")
        "#);

    // Customize permissions, dependencies, env vars, entry point (optional)
    builder
        .set_permissions("rwxr--r--")
        .add_dependency("socket");
        .set_env_var("RUST_LOG", "info")
        .set_entry_point("greet");

    // Save as executable .afe file
    let output = "hello.afe";
    builder.save(output)?;
    println!("✅ Created {}", output);
    Ok(())
}
```

This example:

1. Initializes the builder for `HelloWorld` v1.0.0.
2. Defines a Lua function `greet(name)` and appends a call.
3. Sets file permissions to `rwxr--r--`, adds a dependency, an environment variable, and custom entry point.
4. Writes a self-executable `hello.afe` file with proper shebang.

---

## 🔍 API Reference

```rust
/// Core structure representing the binary data of an .afe file.
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct AfeFile {
    pub version: u32,
    pub app_name: String,
    pub app_version: String,
    pub lua_script: String,
    pub permissions: String,
    pub dependencies: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub entry_point: String,
}

/// Builder for constructing and saving AfeFile instances.
pub struct AfeBuilder { /* fields omitted */ }

impl AfeBuilder {
    /// Create a new builder with given name and version.
    pub fn new(app_name: &str, app_version: &str) -> Self;

    /// Set Unix-style file permissions (e.g., "rwxr-xr-x").
    pub fn set_permissions(&mut self, permissions: &str) -> &mut Self;

    /// Add a dependency string to be included in metadata.
    pub fn add_dependency(&mut self, dep: &str) -> &mut Self;

    /// Assign an environment variable for execution.
    pub fn set_env_var(&mut self, key: &str, value: &str) -> &mut Self;

    /// Specify the Lua entry point function name.
    pub fn set_entry_point(&mut self, func: &str) -> &mut Self;

    /// Add a Lua function: name, argument list, and body.
    pub fn add_lua_function(&mut self, name: &str, args: Vec<&str>, body: &str) -> &mut Self;

    /// Append raw Lua code snippet.
    pub fn add_lua_code(&mut self, code: &str) -> &mut Self;

    /// Build the AfeFile struct without writing to disk.
    pub fn build(&self) -> AfeFile;

    /// Serialize and save as binary .afe file, with validation of Lua and permissions.
    pub fn save(&self, path: &str) -> Result<(), std::io::Error>;
}
```

---

## 📚 Use Cases

* **Game Modding**: Package Lua-based mods with versioning and permissions.
* **Scripting Engines**: Embed scripts into a single `.afe` artifact.
* **CI Pipelines**: Automate .afe generation in Rust build workflows.
* **Secure Distribution**: Serialize with strict schema, validate at build time.

---

## 🤝 Contributing

Contributions, suggestions, and bug reports are welcome!

1. Fork the repository on [GitHub](https://github.com/yourusername/apexforge_afe).
2. Create a feature branch: `git checkout -b feature/YourFeature`.
3. Commit your changes: `git commit -m "Add awesome feature"`.
4. Push to the branch: `git push origin feature/YourFeature`.
5. Open a Pull Request.

Please follow Rust community conventions and include tests for new functionality.

---

## 📜 License

`apexforge_afe` is dual-licensed under the MIT License and the Apache License, Version 2.0.

* MIT: [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)
* Apache-2.0: [https://opensource.org/licenses/Apache-2.0](https://opensource.org/licenses/Apache-2.0)

Select the license that best suits your project.

---

*Made with ❤️ by the ApexForge community.*
