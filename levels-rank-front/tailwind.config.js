/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: "#2b2b2b",
        secondary: "#272525",
      },
    },
  },
  plugins: [],
};
