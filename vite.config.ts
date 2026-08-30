import { defineConfig } from 'vitest/config';

export default defineConfig({
  build: {
    outDir: 'dist/site',
    emptyOutDir: true,
    target: 'es2022',
    cssCodeSplit: false,
  },
  test: {
    exclude: ['**/node_modules/**', '**/target/**'],
  },
});
