// @ts-check

import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const filename = fileURLToPath(import.meta.url);
const currentDir = path.dirname(filename);

const projectDir = path.resolve(currentDir, "..");
const rootDir = path.resolve(projectDir, "..", "..");
const crateDir = path.resolve(rootDir, "core", "gb-core-wasm");
const outDir = path.resolve(projectDir, `.gb-core`);

spawnSync("wasm-pack", ["build", crateDir, "--out-dir", outDir], {
  stdio: "inherit",
});
