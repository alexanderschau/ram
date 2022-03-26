module.exports = {
  content: [
    "./public/**/*.html",
    "./src/**/*.{astro,js,jsx,svelte,ts,tsx,vue}",
  ],
  // more options here
  theme: {
    extend: {
      colors: {
        primary: "#3C1EF4",
        text: "#010F0A",
      },
      fontFamily: {
        lato: ["Lato", "sans-serif"],
      },
    },
  },
};
