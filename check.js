const assert = require("assert");
const { readFileSync } = require("fs");

let file_old = readFileSync("./__fixtures__/Cargo_new.toml", "utf8");
let file_new = readFileSync("./.tmp/Cargo.toml", "utf8");

assert(file_old === file_new);
console.log("the files match");
