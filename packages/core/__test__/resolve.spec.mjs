import test from 'ava'
import { getCodeFile, parser, readParsedFile } from './utils.mjs'

test('should parse with resolve and recursion', (t) => {
  t.deepEqual(
    parser.parse('resolve.js', {depth: 3}),
    readParsedFile(
      'resolve.json',
      { 
        'resolve.js': 'resolve.js',
        'foo.js': 'nested/foo.js',
        'bar.js': 'nested/bar.js',
      },
      { semver: 'semver/index.js' }
    )
  )
})

test('should parse without recursion', (t) => {
  const importJson =  readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js',
      'foo.js': 'nested/foo.js',
      'bar.js': 'nested/bar.js',
    },
    { semver: 'semver/index.js' }
  )

  importJson['nested/bar.js'].import = null
  importJson['nested/foo.js'].importer.pop()

  t.deepEqual(
    parser.parse('resolve.js', {depth: 1}),
   importJson
  )
})

test('should parse without resolve', t => {
  const importJson = readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js',
      'foo.js': 'nested/foo',
      'bar.js': 'nested/bar',
    },
    { semver: 'semver' },
    false
  )

  importJson['nested/bar'].import = null
  importJson['nested/foo'].importer.pop()

  t.deepEqual(
    parser.parse('resolve.js', { resolve: false }),
    importJson
  )
})

test('should parse without recursion & resolve', (t) => {
  const importJson = readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js', 
      'foo.js': 'nested/foo',
      'bar.js': 'nested/bar',
    },
    { semver: 'semver' },
    false
  )

  importJson['nested/bar'].import = null
  importJson['nested/foo'].importer.pop()

  t.deepEqual(
    parser.parse('resolve.js', { depth: 1, resolve: false }),
    importJson
  )
})