import { StateUpdater, useEffect, useRef, useState } from "preact/hooks"
import echarts from 'echarts/core'
import type { ImportNode } from 'import-analysis.core'

declare global {
  interface Window {
    repoName?: string
    importMap: Record<string, ImportNode>
  }
}

export const useGlobalStyle = () => {
  useEffect(() => {
    const root = document.querySelector('html')!
    root.style.fontSize = '12px'
    document.body.className += ' bg-primary'
  }, [])
}

export const useDark = () => {
  const [dark, setDark] = useState(false)

  useEffect(() => {
    const root = document.querySelector('html')!
    if (dark) {
      root.className += ' dark'
    } else {
      root.className = root.className.replace(' dark', '')
    }
  }, [dark])

  return [dark, setDark] as [boolean, StateUpdater<boolean>]
}

export const useGraph = () => {
  const graphInstance = useRef()
  const [importMap, setImportMap] = useState(window.importMap)
  const [option, setOption] = useState({
    tooltip: {},
    legend: [
      // {
      //   // selectedMode: 'single',
      //   data: graph.categories.map(function (a) {
      //     return a.name;
      //   })
      // }
    ],
    series: [

      {
        name: window.repoName,
        type: 'graph',
        layout: 'force',
        // data: graph.nodes,
        // links: graph.links,
        // categories: graph.categories,
        roam: true,
        label: {
          position: 'right'
        },
        emphasis: {
          focus: 'adjacency',
          lineStyle: {
            width: 4
          }
        },
        force: {
          repulsion: 100
        }
      }
    ]
  })

  function computeGraphData(importMap: Window['importMap']) {
    const data = []
    const links = []
    for (const id in importMap) {
      data.push({ id, name: id })
      links.push(...(importMap[id].import?.map(link => ({ source: id, target: link.id })) ?? []))
    }
    console.log(data.length)
    return { data, links }
  }

  useEffect(() => {
    Object.assign(option.series[0], computeGraphData(importMap))
    setOption({...option})
  }, [importMap])

  return { option, setOption }
}