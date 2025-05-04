/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        extend: {
            colors: {
                'theme-background': 'var(--theme-color-background)',
                'theme-text': 'var(--theme-color-text)',
                'theme-primary': 'var(--theme-color-primary)',
                'theme-secondary': 'var(--theme-color-secondary)',
                'theme-accent': 'var(--theme-color-accent)',
                'theme-border': 'var(--theme-color-border)',
                'theme-sidebar-bg': 'var(--theme-color-sidebar-bg)',
            },
            spacing: {
                'theme-spacing-unit': 'var(--theme-spacing-unit)',
            },
            fontFamily: {
                sans: ['var(--theme-font-sans)', 'sans-serif'],
            },
        },
    },

    plugins: [
        // require('@tailwindcss/forms'), // Example plugin
    ],
    prefix: 'tw-'
};