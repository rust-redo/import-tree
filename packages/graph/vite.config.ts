import { defineConfig } from 'vite'
import {presetAttributify, presetUno} from 'unocss'
import UnoCSS from 'unocss/vite'
import preact from '@preact/preset-vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    UnoCSS({
      presets: [
        presetUno({variablePrefix: 'ig-'}),
        presetAttributify(),
      ]
    }),
    preact()
  ],
})
