/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: [
      "*.html", 
      "./src/**/*.rs",
      "node_modules/preline/dist/*.js"
    ],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('preline/plugin'),
  ],
}
