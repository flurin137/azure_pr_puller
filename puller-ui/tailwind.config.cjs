module.exports = {
    content: ["./src/**/*.{svelte,js,ts}"],
    plugins: [require("tailwind-scrollbar")],
    safelist: [
      'badge-success',
      'badge-error',
      'badge-info',
      'badge-warning',
    ],
};
