import test from 'ava'
import {join, dirname} from 'node:path'
import { fileURLToPath } from 'node:url'

import { sum, Parser } from '../index.js'

const __dirname = dirname(fileURLToPath(import.meta.url))

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

const parser = new Parser()

parser.parse(join(__dirname, "./fixture/es.js"), true)