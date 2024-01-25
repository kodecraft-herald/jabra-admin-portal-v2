/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      fontFamily: {
          poppins: ["Poppins", "sans-serif"]
      },
      colors: {
        'deep': 'rgba(11,9,26,255)',
        'light': '#252434',
      },
    },
  },
  plugins: [require("daisyui")], // add to tailwind.config.js
}