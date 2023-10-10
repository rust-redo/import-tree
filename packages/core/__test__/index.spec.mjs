import test from 'ava'
import {getCodeFile, readParsedFile} from './utils.mjs'
import { Parser } from '../index.js'

const parser = new Parser()
const parse = (file) => JSON.parse(parser.parse(Buffer.from(getCodeFile(file)), true).toString())

test('should parse es', (t) => {
  t.deepEqual(parse('es.js'), readParsedFile('es.json', ['es.js']))
})

