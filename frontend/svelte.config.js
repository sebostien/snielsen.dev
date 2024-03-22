import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

export default {
  kit: {
    alias: {
      "@/*": "./src/*",
    },
    adapter: adapter({
      pages: "build",
      assets: "build",
      strict: true,
      fallback: "200.html"
    })
  },
  preprocess: vitePreprocess(),
}

