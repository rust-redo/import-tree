import type { ComponentChild } from 'preact'
import { useState } from 'preact/hooks'
import preactLogo from './assets/preact.svg'
import viteLogo from '/vite.svg'
import { useDark, useRem } from './hooks'
import './app.css'

export function App() {
  const [count, setCount] = useState(0)

  useRem();

  return (
    <>
      <NavBar />
      <div bg="blue-400 hover:blue-500 dark:blue-500 dark:hover:blue-600"
        text="sm white"
        font="mono light"
        p="y-2 x-4"
        m="l-1em"
        border="2 rounded blue-200">
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} class="logo" alt="Vite logo" />
        </a>
        <a href="https://preactjs.com" target="_blank">
          <img src={preactLogo} class="logo preact" alt="Preact logo" />
        </a>
      </div>
      <h1 font="mono light">Vite + Preact</h1>
      <div class="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/app.tsx</code> and save to test HMR
        </p>
      </div>
      <p class="read-the-docs">
        Click on the Vite and Preact logos to learn more
      </p>
    </>
  )
}

function Button({
  className,
  onClick,
  children
}: {
  className?: string
  onClick?: () => void,
  children?: ComponentChild
}) {
  return <button
    className={className}
    cursor="pointer"
    onClick={onClick}>{children}</button>
}


function NavBar() {
  const [_, setDark] = useDark()
  return <div className="bg-primary">
    <Button
      className="i-carbon-sun dark:i-carbon-moon important-w-[2rem] important-h-[2rem] text-primary"
      onClick={() => setDark(dark => !dark)}
    />
  </div>
}