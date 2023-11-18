import { defineConfig, UserConfig } from 'vite'
import {presetAttributify, presetUno, presetIcons} from 'unocss'
import UnoCSS from 'unocss/vite'
import preact from '@preact/preset-vite'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    rollupOptions: {
      output: {
        entryFileNames:  '[name].js',
        assetFileNames: '[name][extname]'
      }
    }
  },
  plugins: [
    UnoCSS({
      shortcuts: {
        'bg-primary': 'bg-white dark:bg-[#121212]',
        "text-primary": 'text-[#121212] dark:text-white'
      },
      presets: [
        presetUno({variablePrefix: 'ig-'}),
        presetAttributify(),
        presetIcons(),
      ]
    }),
    preact()
  ],
}) as UserConfig
