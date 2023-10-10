import test from 'ava'
import {getCodeFile, readParsedFile} from './utils.mjs'
import { Parser } from '../index.js'

const parser = new Parser()
test('should parse es', (t) => {
  t.deepEqual(JSON.parse(parser.parse(Buffer.from(getCodeFile("es.js")), true).toString()), readParsedFile('es.json', ['es.js']))
})

