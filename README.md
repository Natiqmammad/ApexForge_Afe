# apexforge_afe

A Rust library for generating `.afe` (ApexForge Executable) files, embedding Lua scripts with minimal metadata. Designed for simplicity and portability, it uses `bincode` and `serde` for efficient serialization and `mlua` for Lua 5.4 script validation.

---

## 📦 Features

- **Minimal Metadata**: Embed only the essentials — app name, version, permissions, and Lua code.
- **Builder API**: Chain methods to create `.afe` packages easily.
- **Lua 5.4 Integration**: Validate scripts using `mlua` with standard libraries.
- **Binary Serialization**: Compact `.afe` files via `bincode`.

---

## 🚀 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
apexforge_afe = "0.1.1"
```

Then build your project:

```bash
cargo build
```

---

## 🛠️ Quick Start

Create and save a simple `.afe` file:

```rust
use apexforge_afe::AfeBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = AfeBuilder::new("HelloWorld", "1.0");
    
    builder
        .add_lua_function("greet", vec!["name"], r#"print("Hello, "..name)"#)
        .add_lua_code("greet('ApexForge')")
        .set_permissions("rwxr-xr-x");

    builder.save("hello.afe", "Printing 'Hello World!' Keyword".to_string())?;
    println!("Created hello.afe");
    Ok(())
}
```

---

## 🔍 Structs & Methods

### `AfeFile`

```rust
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct AfeFile {
    pub version: u32,
    pub app_name: String,
    pub app_version: String,
    pub lua_script: String,
    pub permissions: String, // e.g., rwxr-xr-x
    pub using: String,       // e.g., lua54
}
```

### `AfeBuilder`

```rust
pub struct AfeBuilder {
    app_name: String,
    app_version: String,
    lua_script: String,
    permissions: String,
}
```

### Methods

```rust
impl AfeBuilder {
    pub fn new(app_name: &str, app_version: &str) -> Self;

    pub fn set_permissions(&mut self, permissions: &str) -> &mut Self;

    pub fn add_lua_function(&mut self, name: &str, args: Vec<&str>, body: &str) -> &mut Self;

    pub fn add_lua_code(&mut self, code: &str) -> &mut Self;

    pub fn build(&self, using: String) -> AfeFile;

    pub fn save(&self, path: &str, using: String) -> Result<(), std::io::Error>;
}
```

---

## 📚 Use Cases

- **Embedded Lua Apps**: Package small Lua utilities or tools.
- **Lightweight Executables**: Distribute logic + metadata in one `.afe` file.
- **Custom Lua Runtimes**: Match Lua version/environment explicitly using `using` field.

---

## 🧪 Notes

- The `save()` method validates Lua code by loading it with `mlua`.
- `using` is a free-form string (e.g., `"Printing Hello World"` or `"write hello world"`).
- Permissions are string-based (`rwxr-xr-x`), not enforced automatically.

---

## 📜 License

Dual-licensed under **MIT** or **Apache-2.0**.

- MIT: https://opensource.org/licenses/MIT  
- Apache: https://opensource.org/licenses/Apache-2.0

---

*Made with 🦀 & ❤️ by the ApexForge community.*