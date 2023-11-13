import {Parser as CoreParser} from './core'

export enum ImportNodeType {
  BUILTIN = 'builtin',
  LOCAL = 'local',
  NODE_MODULES = 'node_modules'
}
export interface ImportNode {
  id: string
  type: ImportNodeType
  importer: string[] | null
  import: ImportLink[] | null
}

export enum ImportLinkType {
  STATIC = 'static',
  DYNAMIC = 'dynamic',
  REQUIRE = 'require'
}

export interface ImportLink {
  id: string
  type: ImportLinkType
  ident: {name: string, as: string}[]
}


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
  ): Record<string, ImportNode>  {
    return JSON.parse(this.parser.parse(Buffer.from(file), depth, resolve).toString())
  }
}

