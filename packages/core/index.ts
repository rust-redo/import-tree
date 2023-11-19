import { isAbsolute, resolve } from 'node:path'
import fg from 'fast-glob'
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
  root: string
  constructor({root = './', alias}: {root?: string, alias?: string} = {}) {
    const absRoot = isAbsolute(root) ? root : resolve(process.cwd(), root)
    this.root = root
    this.parser = new CoreParser(Buffer.from(absRoot), alias ? Buffer.from(alias) : undefined)
  }

  parse(
    files: string | string[], 
    {
      depth,
      resolve,
      buffer,
    }: {
      depth?: number,
      resolve?: boolean
      buffer?: boolean
    } = {}
  ): Record<string, ImportNode> | Buffer  {
    const fileArr = (Array.isArray(files) ? files : [files]).reduce((acc, file) => {
      if(fg.isDynamicPattern(file)) {
        acc.push(...fg.sync(file, {cwd: this.root}))
      } else {
        acc.push(file)
      }
      return acc
    }, [] as string[])

    const parsed = this.parser.parse(Buffer.from(fileArr.toString()), depth, resolve)
    return buffer ? parsed : JSON.parse(parsed.toString())
  }
}

