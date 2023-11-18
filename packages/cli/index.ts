import {resolve, isAbsolute} from 'node:path'
import {writeFileSync, readFileSync} from 'node:fs'
import { Command } from 'commander'
import { Parser } from 'import-analysis.core'
import { version } from './package.json'

const program = new Command()

program
  .name('import-analysis')
  .description('CLI to some JavaScript string utilities')
  .version(version);

program
  .requiredOption('-t, --target <file>', 'target file\'s relative or absolute path')
  .option('-r, --root <directory>', 'target codebase root directory', './')
  .option('-d, --depth <number>', 'separator character', '2')
  .option('-o, --output <file>', 'parsing result\'s file path', './import.json')
  .action((options) => {
    const { root, depth, output, target } = options
    const absRoot = isAbsolute(root) ? root : resolve(process.cwd(), root)
    const parser = new Parser({ root: absRoot })
    const buf = parser.parse(target, {depth: Number(depth), buffer: true}) as Buffer

    writeFileSync(output, output.endsWith('.html') ? computeHtml({name: guessName(absRoot), buf, target}) : buf)
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
  const graphJs = readFileSync(require.resolve('import-analysis.graph/dist/index.js')).toString()
  const graphStyle = readFileSync(require.resolve('import-analysis.graph/dist/index.css')).toString()
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