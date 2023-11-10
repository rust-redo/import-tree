import test from 'ava'
import { parser, readParsedFile } from './utils.mjs'

test('should parse with resolve and recursion', (t) => {
  t.deepEqual(
    parser.parse('resolve.js'),
    readParsedFile(
      'resolve.json',
      { 
        'resolve.js': 'resolve.js',
        'foo.js': 'nested/foo.js'
      },
      { semver: 'semver/index.js' }
    )
  )
})

test('should parse without recursion', (t) => {
  t.deepEqual(
    parser.parse('resolve.js', { recursion: false }),
    readParsedFile(
      'resolve.json',
      { 
        'resolve.js': 'resolve.js',
        'foo.js': 'nested/foo.js'
      },
      { semver: 'semver/index.js' }
    )
  )
})

test('should parse without resolve', t => {
  t.deepEqual(
    parser.parse('resolve.js', { resolve: false }),
    readParsedFile(
      'resolve.json',
      { 
        'resolve.js': 'resolve.js',
        'foo.js': './nested/foo'
      },
      { semver: 'semver' },
      false
    )
  )
})

test('should parse without recursion & resolve', (t) => {
  t.deepEqual(
    parser.parse('resolve.js', { recursion: false, resolve: false }),
    readParsedFile(
      'resolve.json',
      { 
        'resolve.js': 'resolve.js', 
        'foo.js': './nested/foo'
      },
      { semver: 'semver' },
      false
    )
  )
})