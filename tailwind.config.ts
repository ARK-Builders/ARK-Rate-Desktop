import type { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  darkMode: 'selector',
  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        primary: {
          '50': '#F6F5FF',
          '100': '#EDEBFE',
          '200': '#DCD7FE',
          '300': '#CABFFD',
          '400': '#AC94FA',
          '500': '#9061F9',
          '600': '#7E3AF2',
          '700': '#6C2BD9',
          '800': '#5521B5',
          '900': '#4A1D96',
        },
      },
    },
  },
  plugins: [require('@tailwindcss/typography'), require('flowbite/plugin')],
} as Config;
