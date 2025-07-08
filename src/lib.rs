use bincode::{self, Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use mlua::{Lua, StdLib, LuaOptions};
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct AfeFile {
    pub version: u32,
    pub app_name: String,
    pub app_version: String,
    pub lua_script: String,
    pub permissions: String, // rwxr-xr-x format
    pub using:String
}

pub struct AfeBuilder {
    app_name: String,
    app_version: String,
    lua_script: String,
    permissions: String,
}

impl AfeBuilder {
    pub fn new(app_name: &str, app_version: &str) -> Self {
        AfeBuilder {
            app_name: app_name.to_string(),
            app_version: app_version.to_string(),
            lua_script: String::new(),
            permissions: "rwxr-xr-x".to_string(),
        }
    }

    pub fn set_permissions(&mut self, permissions: &str) -> &mut Self {
        self.permissions = permissions.to_string();
        self
    }

    pub fn add_lua_function(&mut self, name: &str, args: Vec<&str>, body: &str) -> &mut Self {
        let args_str = args.join(", ");
        let lua_func = format!("function {}({})\n{}\nend\n", name, args_str, body);
        self.lua_script.push_str(&lua_func);
        self
    }

    pub fn add_lua_code(&mut self, code: &str) -> &mut Self {
        self.lua_script.push_str(code);
        self.lua_script.push('\n');
        self
    }

    pub fn build(&self,using:String) -> AfeFile {
        AfeFile {
            version: 1,
            app_name: self.app_name.clone(),
            app_version: self.app_version.clone(),
            lua_script: self.lua_script.clone(),
            permissions: self.permissions.clone(),
            using:using,
        }
    }

    pub fn save(&self, path: &str,using_condition:String) -> Result<(), std::io::Error> {
        //STANDART LIBS
        let lua = unsafe {

            Lua::unsafe_new_with(StdLib::ALL, LuaOptions::default())
        };
        let afe = self.build(using_condition);
        let encoded = bincode::encode_to_vec(&afe, bincode::config::standard())
            .expect("Encode failed");

        let mut file = File::create(Path::new(path))?;
        file.write_all(&encoded)?;
        Ok(())
    }
}