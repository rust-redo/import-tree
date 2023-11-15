import { join } from 'node:path'
import { Parser } from 'import-analysis.core'
import { Bench } from 'tinybench';

const repos = [
  ['axios', 'lib/axios.js', 3],
  ['rxjs', 'src/index.ts', 3]
]


async function run() {
  for (const [name, target, depth] of repos) {
    const bench = new Bench({ time: 100  });
    const parser = new Parser({
      root: join(process.cwd(), `../repos/${name}`)
    })

    bench
      .add(`${name} without resolve`, () => {
        parser.parse(target, { resolve: false })
      })

    for (let i = 1; i <= depth; i++) {
      bench.add(`${name} with depth=${i}`, () => {
        parser.parse(target, { depth: i })
      })
    }

    bench.todo('unimplemented bench')

    await bench.run();

    console.table(bench.table());
  }
}

run()