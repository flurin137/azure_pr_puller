import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default {
    compilerOptions: {
        warningFilter: (warning) => !warning.filename?.includes("node_modules") && !warning.code.startsWith("a11y"),
    },
    preprocess: vitePreprocess(),
};
