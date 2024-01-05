const colors = require('tailwindcss/colors');

module.exports = {
  content: ['./src/renderer/**/*.{js,jsx,ts,tsx,ejs}'],
  daisyui: {
    themes: ["light", "dark", "night"],
  },
  variants: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
