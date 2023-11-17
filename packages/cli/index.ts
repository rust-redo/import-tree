import {resolve, isAbsolute} from 'node:path'
import {writeFileSync} from 'node:fs'
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
    const parser = new Parser({ root: isAbsolute(root) ? root : resolve(process.cwd(), root) })
    const buf = parser.parse(target, {depth: Number(depth), buffer: true}) as Buffer

    writeFileSync(output, buf)
  });

program.parse();