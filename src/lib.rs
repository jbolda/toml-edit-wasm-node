use std::str::FromStr;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn edit(toml: String, path: String, new_value: String) -> Result<String, String> {
    let mut toml = toml_edit::Document::from_str(&toml).map_err(|e| e.to_string())?;
    let mut path = path.split('>');
    let key = path
        .next()
        .ok_or_else(|| "path must at least contain one component")?
        .trim();
    let mut key = toml
        .get_mut(key)
        .ok_or_else(|| format!("'{key}' couldn't be found"))?;
    for k in path {
        let k = k.trim();
        key = key
            .get_mut(k)
            .ok_or_else(|| format!("'{k}' couldn't be found"))?;
    }
    *key = toml_edit::Item::Value(toml_edit::Value::from(new_value));
    Ok(toml.to_string())
}
