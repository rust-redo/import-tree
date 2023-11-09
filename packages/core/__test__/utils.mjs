import { readFileSync } from 'node:fs'
import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))

export const getCodeFile = (file) => join(__dirname, 'fixture/code', file)
export const getNodeModules = (file) => join(__dirname, 'fixture/code/node_modules', file)
export const readParsedFile = (file, codeFiles = {}, nodeModules = {}, resolve = true) => {
  let content = readFileSync(join(__dirname, 'fixture/parsed', file)).toString()
  Object.keys(codeFiles).forEach((f) => {
    content = content.replaceAll(`{${f}}`, resolve ? getCodeFile(codeFiles[f]) : codeFiles[f])
  })
  Object.keys(nodeModules).forEach((f) => {
    content = content.replaceAll(`{${f}}`, resolve ? getNodeModules(nodeModules[f]) : nodeModules[f])
  })
  return JSON.parse(content)
}