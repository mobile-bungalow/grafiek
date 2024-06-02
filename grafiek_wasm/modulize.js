import { readFileSync, writeFileSync } from "fs";

const dirName = "./grafiek_wasm/pkg/"; 

const content = readFileSync(dirName + "package.json");

const packageJSON = JSON.parse(String(content));
packageJSON["type"] = "module";

writeFileSync(dirName + "package.json", JSON.stringify(packageJSON));