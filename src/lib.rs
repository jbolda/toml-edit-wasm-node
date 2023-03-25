use std::str::FromStr;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn edit(toml: String) -> Result<String, String> {
    let mut toml = toml_edit::Document::from_str(&toml).map_err(|e| e.to_string())?;
    let package = toml
        .get_mut("package")
        .ok_or_else(|| "error: `package` couldn't be found")?;
    let name = package
        .get_mut("name")
        .ok_or_else(|| "error: `name` couldn't be found")?;
    *name = toml_edit::Item::Value(toml_edit::Value::from("Mutated Name"));
    Ok(toml.to_string())
}
