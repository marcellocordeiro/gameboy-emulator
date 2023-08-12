import wasm from "vite-plugin-wasm";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import wasmPack from "vite-plugin-wasm-pack";
import tsconfigPaths from "vite-tsconfig-paths";

const coreCratePath = "./core-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  root: "ui-browser",
  build: {
    target: "esnext",
  },
  plugins: [wasmPack(coreCratePath), wasm(), react(), tsconfigPaths()],
});
