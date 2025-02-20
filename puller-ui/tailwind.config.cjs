module.exports = {
    content: ["./src/**/*.{svelte,js,ts}"],
    plugins: [require("daisyui"), require("tailwind-scrollbar")],
    daisyui: {
        themes: [
            "light",
            "dark",
            "coffee",
            "sunset",
            "luxury",
            "cyberpunk",
            {
                gritec: {
                    primary: "#0076bd",
                    secondary: "#b9bdc4",
                    accent: "#1fb2a6",
                    neutral: "#1d232a",
                    "base-100": "#1d232a",
                    info: "#3abff8",
                    success: "#36d399",
                    warning: "#fbbd23",
                    error: "#f87272",
                    "--rounded-box": "0.5rem", // border radius rounded-box utility class, used in card and other large boxes
                    "--rounded-btn": "0.5rem", // border radius rounded-btn utility class, used in buttons and similar element
                    "--rounded-badge": "0.5rem", // border radius rounded-badge utility class, used in badges and similar
                    "--border-btn": "1px", // border width of buttons
                    "--tab-border": "1px", // border width of tabs
                    "--tab-radius": "0.5rem", // border radius of tabs
                },
            },
            {
                pinky: {
                    primary: "#F0C",
                    secondary: "#007",
                    accent: "#0F0",
                    "base-100": "#003",
                    success: "#0C0",
                    warning: "#CC0",
                    error: "#700",
                    "--rounded-box": "0.3rem", // border radius rounded-box utility class, used in card and other large boxes
                    "--rounded-btn": "0.3rem", // border radius rounded-btn utility class, used in buttons and similar element
                    "--rounded-badge": "0.3rem", // border radius rounded-badge utility class, used in badges and similar
                    "--border-btn": "1px", // border width of buttons
                    "--tab-border": "1px", // border width of tabs
                    "--tab-radius": "0.3rem", // border radius of tabs
                },
            },
        ],
    },
};
