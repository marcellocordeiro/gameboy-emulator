// @ts-check

import { spawnSync } from "node:child_process";
import path from "node:path";

const currentDir = import.meta.dirname;
const projectDir = path.resolve(currentDir, "..");
const rootDir = path.resolve(projectDir, "..", "..");
const crateDir = path.resolve(rootDir, "core", "gb-core-wasm");
const outDir = path.resolve(projectDir, `.gb-core`);

spawnSync("wasm-pack", ["build", crateDir, "--out-dir", outDir], {
  stdio: "inherit",
});
