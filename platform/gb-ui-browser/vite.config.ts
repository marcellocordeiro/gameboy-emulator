import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import wasmPack from "vite-plugin-wasm-pack";
import tsconfigPaths from "vite-tsconfig-paths";

const coreCratePath = "../../core/gb-core-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    target: "esnext",
  },
  plugins: [wasmPack(coreCratePath), react(), tsconfigPaths()],
});
