// tailwind.config.js
const plugin = require("tailwindcss/plugin");

function withOpacityValue(color) {
  return ({ opacityValue }) => {
    if (opacityValue === undefined) {
      return `${variable}`;
    }
    return `${variable}_${opacityValue}`;
  };
}

module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "../../packages/pivotable/src/**/*.rs",
  ],
  media: false,
  theme: {
    extend: {
      fontFamily: {
        roboto: ["Roboto*, Helvetica, Arial, sans-serif"],
        fa: ["solid", "brands"],
      },
      spacing: {
        3: "3px",
      },
      colors: {
        gray: {
          // Profile background
          1: "#273541",
          750: "#2d3945",
        },
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
