// @ts-check

import { spawnSync } from "node:child_process";

console.log("Installing rustup");
spawnSync(
  "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
  {
    shell: true,
    stdio: "inherit",
  },
);

console.log("Configuring cargo env");
spawnSync('. "$HOME/.cargo/env"', {
  shell: true,
  stdio: "inherit",
});

console.log("Installing wasm-pack");
spawnSync("npm install -g wasm-pack", {
  shell: true,
  stdio: "inherit",
});
