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
  actions: {}
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

export const useGraph = (center: {x: number, y: number}) => {
  const {dark} = useContext(Context)
  const [importTree, setImportTree] = useState(window.importTree)
  const [option, setOption] = useState({
    tooltip: {},
    legend: [{
      left: '2%',
      bottom: '5%',
      orient: 'vertical',
      data: [],
      textStyle: {},
    }],
    series: [
      {
        name: window.repoName,
        type: 'graph',
        layout: 'force',
        symbolSize: 12,
        edgeSymbol: ['none', 'arrow'],
        edgeSymbolSize: 5,
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
    const categoryMaps: Record<string, boolean>= {}

    Object.keys(tree).forEach(id => {
      const dir = id.replace(/\/?[^/]+\.\w+$/, '')
      if(categoryMaps[dir] === undefined) {
        categoryMaps[dir] = true
      }
      data.push({id, name: id, category: dir})
      links.push(...(tree[id].import?.map(link => ({ source: id!, target: link.id })) ?? []))
    })

    const categories = Object.keys(categoryMaps).sort().map(cate => ({name: cate}))
    data.forEach(node => {
      node.category = categories.findIndex(cate => cate.name === node.category)
    })
      
    return { data, links, categories: categories }
  }

  useEffect(() => {
    Object.assign(option.series[0], computeGraphData(importTree))
    // @ts-expect-error
    option.legend[0].data = option.series[0].categories
    console.log(option)
    setOption({...option})
  }, [importTree, center.x, center.y])

  useEffect(() => {
    option.legend[0].textStyle = {
      color:  dark ? '#fff' : '#121212'
    }
    setOption({...option})
  }, [dark])

  return { option, setOption }
}