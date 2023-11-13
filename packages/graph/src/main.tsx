import { render } from 'preact'
import 'virtual:uno.css'
import '@unocss/reset/normalize.css'
import { App } from './app.tsx'
import './index.css'

render(<App />, document.getElementById('app')!)
