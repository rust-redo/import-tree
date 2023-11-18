#!/usr/bin/env node

import {resolve} from 'node:path'
import {writeFileSync, readFileSync} from 'node:fs'
import { Command } from 'commander'
import ora from 'ora'
import { ImportNode, Parser } from 'import-tree.core'
import { version } from './package.json'
import { performance } from 'node:perf_hooks'

const program = new Command()

program
  .name('import-tree')
  .description('CLI to some JavaScript string utilities')
  .version(version);

program
  .requiredOption('-t, --target <file>', 'target file\'s relative or absolute path')
  .option('-r, --root <directory>', 'target codebase root directory', './')
  .option('-d, --depth <number>', 'import relation tree\'s depth', '2')
  .option('-o, --output <file>', 'parsing result\'s file path', './import.json')
  .action((options) => {
    const now = performance.now()
    const { root, depth, output, target } = options
    const parser = new Parser({ root })
    const spinner = ora('').start(`parsing ${target} in ${parser.root}...`);

    
    Promise.resolve().then(() => {
      return parser.parse(target, {depth: Number(depth), buffer: true}) as Buffer
    }).then((buf: Buffer) => {
      spinner.color = 'yellow';
      spinner.text = `writing parsed data to ${output}...`;
  
      writeFileSync(output, output.endsWith('.html') ? computeHtml({name: guessName(parser.root), buf, target}) : buf)
  
      const timeCost = parseInt(`${performance.now() - now}`)
      const {files, links} = computeParsedFiles(buf)

      setTimeout(() => {
        spinner.succeed(`parsed total ${files} files, ${links} imports in ${timeCost} ms`)
        spinner.succeed(`data saved to ${output}\n`)
      }, 500)
    })

  });

program.parse();

function guessName(root: string) {
  let name = 'import-tree'
  try {
    name = require(resolve(root, 'package.json'))?.name
  } catch(_) {}

  return name
}

function computeHtml({name, buf, target}: any) {
  const graphJs = readFileSync(require.resolve('import-tree.graph/dist/index.js')).toString()
  const graphStyle = readFileSync(require.resolve('import-tree.graph/dist/index.css')).toString()
  const tree = buf.toString()

  return `
  <!doctype html>
  <html lang="en">
  
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Import Tree</title>
    <script type="module" crossorigin>{{GRAPH_JS}}</script>
    <style>{{GRAPH_STYLE}}</style>
  </head>
  
  <body class="bg-primary">
    <div id="app"></div>
    
    <script>
      window.repoName = "{{REPO_NAME}}"
      window.targetFile = "{{TARGET_FILE}}"
      window.importTree = {{IMPORT_TREE}}
    </script>
  </body>
  </html>
  `.replace('{{IMPORT_TREE}}', tree)
  .replace("{{REPO_NAME}}", name)
  .replace("{{TARGET_FILE}}", target)
  .replace("{{GRAPH_JS}}", graphJs)
  .replace("{{GRAPH_STYLE}}", graphStyle)
  ;
}

function computeParsedFiles(buf: Buffer) {
  const parsed = JSON.parse(buf.toString()) as Record<string, ImportNode>
  const files = Object.keys(parsed)
  return {
    files: files.length,
    links: files.reduce((acc, file) => {
      return acc + (parsed[file].import?.length ?? 0)
    }, 0)
  }
}