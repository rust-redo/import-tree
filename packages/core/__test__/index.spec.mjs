import {join} from 'node:path'
import test from 'ava'
import {getCodeFile, readParsedFile} from './utils.mjs'
import { Parser } from '../index.js'

const parser = new Parser({root: join(process.cwd(), '__test__/fixture/code')})

test('should parser.parse es', (t) => {
  t.deepEqual(
    parser.parse('es.js'), 
    readParsedFile(
      'es.json', 
      {'es.js': 'es.js'},
      {semver: 'semver/index.js'}
    )
  )

  t.deepEqual(
    parser.parse('es.js', {recursion: false}), 
    readParsedFile(
      'es.json', 
      {'es.js': 'es.js'},
      {semver: 'semver/index.js'}
    )
  )

  t.deepEqual(
    parser.parse('es.js', {recursion: false, resolve: false}), 
    readParsedFile(
      'es.json', 
      {'es.js': 'es.js'},
      {semver: 'semver'},
      false
    ) 
  )
})

test('should parser.parse ts', (t) => {
  t.deepEqual(parser.parse("ts.ts"), readParsedFile('ts.json', {'ts.ts': './ts.ts'},))
})

test('should parser.parse jsx', (t) => {
  t.deepEqual(
    parser.parse('es.jsx'), 
    readParsedFile(
      'es-jsx.json', 
      {'es.jsx': './es.jsx'},
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})

test('should parser.parse tsx', (t) => {
  t.deepEqual(
    parser.parse('ts.tsx'), 
    readParsedFile(
      'ts-tsx.json', 
      {'ts.tsx': './ts.tsx'},
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})