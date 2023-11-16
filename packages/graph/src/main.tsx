import { render } from 'preact'
import 'virtual:uno.css'
import '@unocss/reset/sanitize/sanitize.css'
import '@unocss/reset/sanitize/assets.css'
import { App } from './app.tsx'
import './index.css'

render(<App />, document.getElementById('app')!)
