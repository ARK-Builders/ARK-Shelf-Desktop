/** @type {import('tailwindcss').Config} */
export default {
    content: [
        './index.html',
        './src/**/*.{js,ts,jsx,tsx,svelte}',
        './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
    ],
    theme: {
        extend: {
            colors: {
                'neutral-850': 'rgb(28,28,28)',
            },
        },
    },
    plugins: [require('flowbite/plugin')],
};
