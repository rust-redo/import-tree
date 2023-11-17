import test from 'ava'
import {parser, readParsedFile} from './utils.mjs'

test('should parse es', (t) => {
  t.deepEqual(
    parser.parse('es.js'), 
    readParsedFile(
      'es.json', 
      {'es.js': 'es.js'},
    )
  )
})

test('should parse ts', (t) => {
  t.deepEqual(parser.parse("es.ts"), readParsedFile('es-ts.json', {'es.ts': 'es.ts'},))
})

test('should parse jsx', (t) => {
  t.deepEqual(
    parser.parse('es.jsx'), 
    readParsedFile(
      'es-jsx.json', 
      {'es.jsx': 'es.jsx'},
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})

test('should parse tsx', (t) => {
  t.deepEqual(
    parser.parse('es.tsx'), 
    readParsedFile(
      'es-tsx.json', 
      {'es.tsx': 'es.tsx'},
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})