import { StateUpdater, useEffect, useState } from "preact/hooks"

export const useRem = () => {
  useEffect(() => {
    const root = document.querySelector('html')!
    root.style.fontSize = '12px'
  }, [])
}

export const useDark = () => {
  const [dark, setDark] = useState(false)

  useEffect(() => {
    const root = document.querySelector('html')!
    if(dark) {
      root.className += ' dark'
    } else {
      root.className = root.className.replace(' dark', '')
    }
  }, [dark])
  
  return [dark, setDark] as [boolean, StateUpdater<boolean>]
} 