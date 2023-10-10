import { readFileSync } from 'node:fs'
import {join, dirname} from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))

export const getCodeFile = (file) => join(__dirname, 'fixture/code', file)
export const readParsedFile = (file, codeFiles = [], nodeModules = []) => {
  let content = readFileSync(join(__dirname, 'fixture/parsed', file)).toString()
  codeFiles.forEach((f) => {
    content = content.replaceAll(`{${f}}`, getCodeFile(f))
  })
  nodeModules.forEach((f) => {
    content = content.replaceAll(`{${f}}`, getCodeFile(f))
  })
  return JSON.parse(content)
}