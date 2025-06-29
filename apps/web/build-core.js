// @ts-check

import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const filename = fileURLToPath(import.meta.url);
const currentDir = path.dirname(filename);

const rootDir = path.resolve(currentDir, "..", "..");
const crateDir = path.resolve(rootDir, "core", "gb-core-wasm");
const outDir = path.resolve(currentDir, `.gb-core`);

spawnSync("wasm-pack", ["build", crateDir, "--out-dir", outDir], {
  shell: true,
  stdio: "inherit",
});
