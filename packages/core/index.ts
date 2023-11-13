import {Parser as CoreParser} from './core'

export class Parser {
  parser: CoreParser
  constructor({root}: {root?: string} = {}) {
    this.parser = new CoreParser(typeof root === 'string' ? Buffer.from(root) : undefined)
  }

  parse(
    file: string, 
    {
      depth,
      resolve,
    }: {
      depth?: number,
      resolve?: boolean
    } = {}
    ) {
    return JSON.parse(this.parser.parse(Buffer.from(file), depth, resolve).toString())
  }
}

