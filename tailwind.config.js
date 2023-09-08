/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx,svelte}'],
  theme: {
    extend: {
      colors: {
        'neutral-850': 'rgb(28,28,28)',
      },
    },
  },
  plugins: [],
};
