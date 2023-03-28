const { edit } = require("./pkg/toml_edit_wasm_node");
const { readFileSync, writeFileSync, mkdirSync } = require("fs");

let file = readFileSync("./__fixtures__/Cargo_old.toml", "utf8");
let file_edit_1 = edit(file, "package > name", "new name");
let file_edit_2 = edit(file_edit_1, "package > version", "1.0.0");

console.log(`
\x1b[33mBefore\x1b[0m
${file}

\x1b[33mAfter\x1b[0m
${file_edit_2}
`);

try {
  mkdirSync(".tmp");
} catch (error) {
  // no-op
}
writeFileSync("./.tmp/Cargo.toml", file_edit_2);
