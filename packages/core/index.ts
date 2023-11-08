import {Parser as CoreParser} from './core'

export class Parser {
  parser: CoreParser
  constructor(root?: string) {
    this.parser = new CoreParser(typeof root === 'string' ? Buffer.from(root) : undefined)
  }

  parse(file: string, shouldResolve: boolean) {
    return JSON.parse(this.parser.parse(Buffer.from(file), shouldResolve).toString())
  }
}

