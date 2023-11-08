import test from 'ava'
import {getCodeFile, readParsedFile} from './utils.mjs'
import { Parser } from '../index.js'

const parser = new Parser()
const parse = (file, shouldResolve = true) => parser.parse(getCodeFile(file), shouldResolve)

test('should parse es', (t) => {
  t.deepEqual(
    parse('es.js'), 
    readParsedFile(
      'es.json', 
      ['es.js'],
      {semver: 'semver/index.js'}
    )
  )
})

test('should parse ts', (t) => {
  t.deepEqual(parse("ts.ts"), readParsedFile('ts.json', ['ts.ts']))
})

test('should parse jsx', (t) => {
  t.deepEqual(
    parse('es.jsx'), 
    readParsedFile(
      'es-jsx.json', 
      ['es.jsx'],
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})

test('should parse tsx', (t) => {
  t.deepEqual(
    parse('ts.tsx'), 
    readParsedFile(
      'ts-tsx.json', 
      ['ts.tsx'],
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})