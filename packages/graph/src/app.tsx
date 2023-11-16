import type { ComponentChild } from 'preact'
import { useEffect, useRef, useState } from 'preact/hooks'
import { use, init as echartsInit } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { GraphChart } from 'echarts/charts'
import type {
  SingleAxisComponentOption,
  TooltipComponentOption,
} from 'echarts/components'
import {
  LegendComponent,
  TooltipComponent,
} from 'echarts/components'
import { useDark, useGlobalStyle, useGraph } from './hooks'

use([
  CanvasRenderer,
  GraphChart,
  TooltipComponent,
  LegendComponent,
])

export function App() {
  useGlobalStyle();

  return (
    <>
      <NavBar />
      <ImportGraph />
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

function ImportGraph() {
  const chartRef = useRef(null)
  const chartInstanceRef = useRef<ReturnType<typeof echartsInit> | null>(null)
  const {option} = useGraph()
  
  useEffect(() => {
    chartInstanceRef.current = echartsInit(chartRef.current)
  }, [])

  useEffect(() => {
    chartInstanceRef.current?.setOption(option)
  }, [option, chartInstanceRef.current])

  return <div ref={chartRef} className="w-100vw h-[calc(100vh-40px)]">
  </div>
}