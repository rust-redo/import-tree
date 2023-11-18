import test from 'ava'
import { Parser } from '../index.js'
import { getGitRepo, getGitRepoFiles, validateParsed } from './utils.mjs'

test('axios', t => {  
  const parser = new Parser({root: getGitRepo('axios')})
  const data = parser.parse('lib/axios.js', {depth: 3})
  const files = getGitRepoFiles('axios', 'lib/**/*.js', [
    '**/null.js', 
    '**/deprecatedMethod.js',
    "**/env/classes/FormData.js",
    "**/platform/browser/**",
  ])
  validateParsed(t, data, files)
})

test('rxjs', t => {  
  const parser = new Parser({root: getGitRepo('rxjs')})
  const data = parser.parse('src/index.ts', {depth: 3})
  const files = getGitRepoFiles('rxjs', 'src/**/*.ts', [
  "**/{webSocket,ajax,fetch,operators,testing}/index.ts",
  "**/internal/{ajax,testing}/**",
  "src/internal/operators/partition.ts",
  "src/internal/util/workarounds.ts",
  "src/internal/observable/dom/{fetch,WebSocketSubject,webSocket}.ts"
  ])
  validateParsed(t, data, files)
})

test('nextui', t => {  
  const parser = new Parser({root: getGitRepo('nextui/packages/components')})
  const data = parser.parse('**/src/index.ts', {depth: 3})
  const files = getGitRepoFiles('nextui/packages/components', '**/src/**/*.{ts,tsx}', [
  ])
  validateParsed(t, data, files)
})