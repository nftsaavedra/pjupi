import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

const getPdfVendorChunk = (id: string) => {
  const packageMatch = id.match(/node_modules\/(?:\.pnpm\/)?((?:@[^/]+\/[^/]+)|[^/]+)/);
  const packageName = packageMatch?.[1] ?? null;

  if (packageName?.startsWith('@react-pdf/')) {
    return `pdf-${packageName.split('/')[1].replace(/[^a-z0-9-]/gi, '-')}`;
  }

  if (
    packageName === 'pdfkit'
    || packageName === 'png-js'
    || packageName === 'crypto-js'
    || packageName === 'dfa'
    || packageName === 'clone'
  ) {
    return 'pdf-engine';
  }

  if (
    packageName === 'fontkit'
    || packageName === 'unicode-properties'
    || packageName === 'unicode-trie'
    || packageName === 'tiny-inflate'
    || packageName === 'restructure'
    || packageName === 'brotli'
    || packageName === 'linebreak'
  ) {
    return 'pdf-fonts';
  }

  if (packageName === 'yoga-layout') {
    return 'pdf-layout';
  }

  return null;
};

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    rolldownOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules/recharts')) {
            return 'charts-vendor';
          }

          if (id.includes('node_modules/exceljs')) {
            return 'excel-vendor';
          }

          if (id.includes('@tauri-apps/api/core')) {
            return 'tauri-vendor';
          }

          const pdfChunk = getPdfVendorChunk(id);
          if (pdfChunk) {
            return pdfChunk;
          }

          return undefined;
        },
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
