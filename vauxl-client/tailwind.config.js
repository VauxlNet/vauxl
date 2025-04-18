module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
        "./src/**/*.html",
        "./src/**/*.css",
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                // Discord color palette
                discord: {
                    primary: '#7289da',
                    dark: '#36393f',
                    darker: '#2f3136',
                    darkest: '#202225',
                    light: '#ffffff',
                    success: '#43b581',
                    danger: '#f04747',
                    warning: '#faa61a',
                    mention: 'rgba(250, 166, 26, 0.1)',
                    mentionText: '#faa61a',
                    link: '#00b0f4',
                    channelText: '#8e9297',
                    channelHover: '#dcddde',
                },
                // Add more theme color palettes here
            },
            spacing: {
                '18': '4.5rem',
                '22': '5.5rem',
                '72': '18rem',
                '80': '20rem',
                '96': '24rem',
                '100': '25rem',
                '120': '30rem',
            },
        },
    },
    plugins: [],
}