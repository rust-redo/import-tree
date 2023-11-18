import { createContext } from "preact"
import { type StateUpdater, useEffect, useState, useContext } from "preact/hooks"
import type { ImportNode } from 'import-analysis.core'

declare global {
  interface Window {
    repoName: string
    targetFile: string
    importTree: Record<string, ImportNode>
  }
}

interface GraphNode {
  id: string,
  name: string,
  category?: number | string
  x?: number,
  y?: number
  symbolSize?: number | number[]
  itemStyle?: {
    borderWidth?: number,
    borderColor?: string
  }
  fixed?: boolean
}

interface GraphLink {
  source: string
  target: string
}

export const Context = createContext({
  dark: false,
  actions: {},
} as {
  dark: boolean
  actions: {
    setDark: StateUpdater<boolean>
  }
})

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

export const useGraph = (center: { x: number, y: number }) => {
  const { dark } = useContext(Context)
  const [importTree] = useState(window.importTree)
  const [option, setOption] = useState({
    tooltip: {},
    legend: [{
      left: '1%',
      bottom: '2%',
      orient: 'vertical',
      data: [],
      itemWidth: 12,
      itemHeight: 10,
      textStyle: {},
    }],
    series: [
      {
        name: window.repoName,
        type: 'graph',
        layout: 'force',
        symbolSize: 16,
        edgeSymbol: ['none', 'arrow'],
        edgeSymbolSize: 6,
        roam: true,
        label: {
          // show: true
          position: 'right'
        },
        emphasis: {
          focus: 'adjacency',
          lineStyle: {
            width: 4
          }
        },
        force: {
          repulsion: 150,
        }
      }
    ]
  })

  function computeGraphData(tree: typeof importTree) {
    const data: GraphNode[] = []
    const links: GraphLink[] = []
    const categoryMaps: Record<string, boolean> = {}

    Object.values(tree).forEach(node => {
      const dir = {
        node_modules: 'external',
        builtin: 'builtin',
        local: node.id.replace(/\/?[^/]+\.\w+$/, '')
      }[node.type]
      categoryMaps[dir] = true
      data.push({ id: node.id, name: node.id, category: dir })
      links.push(...(tree[node.id].import?.map(link => ({ source: node.id, target: link.id })) ?? []))
    })

    const categories = Object.keys(categoryMaps).sort().map(cate => ({ name: cate }))
    data.forEach(node => {
      node.category = categories.findIndex(cate => cate.name === node.category)
    })

    return { data, links, categories: categories }
  }

  useEffect(() => {
    Object.assign(option.series[0], computeGraphData(importTree))
    // @ts-expect-error
    option.legend[0].data = option.series[0].categories

    setOption({ ...option })
  }, [importTree, center.x, center.y])

  useEffect(() => {
    option.legend[0].textStyle = {
      color: dark ? '#fff' : '#121212'
    }
    setOption({ ...option })
  }, [dark])

  return { option, setOption }
}

export const graphTheme = {
    "color": [
        "#2ec7c9",
        "#b6a2de",
        "#5ab1ef",
        "#ffb980",
        "#d87a80",
        "#8d98b3",
        "#e5cf0d",
        "#97b552",
        "#88bb88",
        "#dc69aa",
        "#07a2a4",
        "#9a7fd1",
        "#588dd5",
        "#f5994e",
        "#c05050",
        "#59678c",
        "#c9ab00",
        "#7eb00a",
        "#6f5553",
        "#c14089"
    ],
   
}