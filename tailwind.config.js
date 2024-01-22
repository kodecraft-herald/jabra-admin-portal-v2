/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      fontFamily: {
          poppins: ["Poppins", "sans-serif"]
        }
    },
  },
  plugins: [require("daisyui")], // add to tailwind.config.js
}