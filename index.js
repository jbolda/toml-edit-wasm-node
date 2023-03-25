const { edit } = require("./pkg/toml_edit_wasm_node");
const { readFileSync } = require("fs");

let file = readFileSync("./Cargo.toml", "utf8");
console.log("\x1b[33mBefore\x1b[0m");
console.log(file);
let file_edit = edit(file);
console.log("\x1b[33mAfter\x1b[0m");
console.log(file_edit);
