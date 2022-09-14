/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["static/*.{tera, js, html}"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography')
  ],
}
