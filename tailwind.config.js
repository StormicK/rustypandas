/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/*.{js,jsx,ts,tsx}", "./src/view/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        'red-panda-text': '#FCE7D2',        // Light cream color for text
        'red-panda-background': '#3D1F16',  // Deep brown color for background
        'red-panda-link': '#B13D14',        // Rusty red color for links
        'red-panda-link-hover': '#63433A',  // Darker shade for link hover effect
        'red-panda-text-dark': '#2A140E',
        'red-panda-background-dark': '#20100B',
        'red-panda-link-dark': '#8B260B',
        'red-panda-link-hover-dark': '#472D27',
        'red-panda-accent-1': '#FF6B35',    // Vibrant orange accent color
        'red-panda-accent-2': '#A3311F',    // Deeper shade of red for accents
        'red-panda-accent-3': '#F4CDA5',    // Soft beige for additional accents
        'red-panda-accent-4': '#635956',    // Muted gray-brown for subtle details
        'red-panda-accent-1-dark': '#D64812', // Darker shade of vibrant orange accent
        'red-panda-accent-2-dark': '#802518', // Darker shade of deeper red for accents
        'red-panda-accent-3-dark': '#D5B590', // Darker shade of soft beige for additional accents
        'red-panda-accent-4-dark': '#463F3A',
      },
    },
  },
  plugins: [],
};


