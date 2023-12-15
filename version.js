//open the src-tauri/tauri.conf.json file and read the 0.package.version value and print it to the console
const fs = require("fs");
const path = require("path");

const tauriConfPath = path.join(__dirname, "./src-tauri/tauri.conf.json");
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
console.log(tauriConf.package.version);
