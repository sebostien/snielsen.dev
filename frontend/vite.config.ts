import { defineConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [sveltekit()],
  resolve: {
    alias: {
      '@': path.resolve('/src')
    }
  },
})
