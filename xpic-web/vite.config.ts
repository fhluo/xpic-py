import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import path from "path"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      "$lib": path.resolve("./src/lib"),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
})
