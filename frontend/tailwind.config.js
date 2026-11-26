/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        specimen: {
          bg: '#0b0d12',
          surface: '#141721',
          hover: '#1c202e',
          border: '#262b3d',
          accent: '#7c5cff',
          'accent-hover': '#6945f8',
          text: '#f0f2f5',
          muted: '#8c93a8',
          success: '#10b981',
          warning: '#f59e0b',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      }
    },
  },
  plugins: [],
}
