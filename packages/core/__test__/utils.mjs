import { readFileSync } from 'node:fs'
import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'
import fg from 'fast-glob'
import { Parser } from '../index.js'

const __dirname = dirname(fileURLToPath(import.meta.url))

export const getCodeFile = (file) => join(__dirname, 'fixture/code', file)
export const getNodeModules = (file) => join('node_modules', file)
export const readParsedFile = (file, codeFiles = {}, nodeModules = {}, resolve = true) => {
  let content = readFileSync(join(__dirname, 'fixture/parsed', file)).toString()
  Object.keys(codeFiles).forEach((f) => {
    content = content.replaceAll(`{${f}}`, codeFiles[f])
  })
  Object.keys(nodeModules).forEach((f) => {
    content = content.replaceAll(`{${f}}`, resolve ? getNodeModules(nodeModules[f]) : nodeModules[f])
  })
  return JSON.parse(content)
}
export const parser = new Parser({ root: join(process.cwd(), '__test__/fixture/code') })
export const getGitRepo = repo => join(__dirname, '../../../repos', repo)
export const getGitRepoFiles = (repo, pattern, ignore) => {
  const root = getGitRepo(repo)
  return fg.sync(pattern, {cwd: root, ignore})
}
export const validateParsed = (t, parsed, files) => {
  files.forEach(file => {
    t.truthy(parsed[file], `${file} not parsed`)
  })
  t.is(files.length, Object.values(parsed).filter(item => item.type==='local').length)

  for(let id in parsed) {
    if(parsed[id].importer?.length) {
      let visited = {}
      for(const importerId of parsed[id].importer) {
        t.falsy(visited[importerId], `${id} importer ${importerId} duplicated`)
        t.truthy(parsed[importerId], `${id} importer ${importerId} not exist`)
        visited[importerId] = true
      }

      visited = {}

      for(const {id: importId, ident} of (parsed[id].import ?? [])) {
        t.notDeepEqual(ident, visited[importId], `${id} import ${importId} duplicated`)
        t.truthy(parsed[importId], `${id} import ${importId} not exist`)
        visited[importId] = ident
      }
    }
  }
}