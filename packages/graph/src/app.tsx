import type { ComponentChild } from 'preact'
import { useContext, useEffect, useRef } from 'preact/hooks'
import { use, init as echartsInit } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { GraphChart } from 'echarts/charts'
import {
  LegendComponent,
  TooltipComponent,
} from 'echarts/components'
import { Context, graphTheme, useDark, useGlobalStyle, useGraph } from './hooks'

use([
  CanvasRenderer,
  GraphChart,
  TooltipComponent,
  LegendComponent,
])

export function App() {
  useGlobalStyle();
  const [dark, setDark] = useDark()

  return (
    <Context.Provider value={{ dark, actions: {setDark} }}>
      <NavBar />
      <ImportGraph />
    </Context.Provider>
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
  const {actions: {setDark}} = useContext(Context)
  return <div className="bg-primary flex flex-justify-between flex-items-center p-x-6">
    <h2 className="text-primary">{window.repoName ?? window.targetFile}</h2>
    <Button
      className="i-carbon-sun dark:i-carbon-moon important-w-[2rem] important-h-[2rem] text-primary"
      onClick={() => setDark(dark => !dark)}
    />
  </div>
}

function ImportGraph() {
  const chartRef = useRef<HTMLDivElement>(null)
  const chartInstanceRef = useRef<ReturnType<typeof echartsInit> | null>(null)
  const chartRect = chartRef.current ? chartRef.current.getBoundingClientRect() : { width: 0, height: 0 }
  const { option } = useGraph(
    { x: chartRect.width / 2, y: chartRect.height / 2 }
  )

  useEffect(() => {
    chartInstanceRef.current = echartsInit(chartRef.current, graphTheme)
  }, [])

  useEffect(() => {
    // chartInstanceRef.current?.clear()
    chartInstanceRef.current?.setOption(option)
  }, [option, chartInstanceRef.current])

  return <div ref={chartRef} className="w-100vw h-[calc(100vh-60px)]">
  </div>
}