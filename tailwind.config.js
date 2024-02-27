/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      fontFamily: {
          poppins: ["Poppins", "sans-serif"]
      },
      colors: {
        'deep': '#191828',
        'light': '#252434',
      },
    },
  },
  plugins: [require("daisyui")], // add to tailwind.config.js
  daisyui: {
    themes: [
    {
      darkpurple: {
        "primary": "#312e81",
        "secondary": "#a78bfa",
        "accent": "#ede9fe",
        "neutral": "#78716c",
        "base-100": "#1f2937",
        "info": "#6b7280",
        "success": "#a5b4fc",
        "warning": "#fde047",
        "error": "#f87171",
      },
    },
    {
      lightpurple: {
        "primary": "#3730a3",
        "secondary": "#5b21b6",
        "accent": "#818cf8",
        "neutral": "#78716c",
        "base-100": "#FFFF",
        "info": "#6b7280",
        "success": "#4840BB",
        "warning": "#f97316",
        "error": "#f87171",
      },
    }],
},
}