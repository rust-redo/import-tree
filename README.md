<p align="left">
  <img src="https://github.com/rust-redo/import-tree/assets/102238922/56ef774d-6ed0-4491-afab-93fbeba9e955" >
</p>

# import-tree(WIP)

A super fast tool that uses [swc](https://swc.rs/) to detect and build JavaScript/TypeScript module graph.

## Features

- :zap: Blazing fast ESM/CJS modules scanner [WIP]
- :rainbow: Interactive module relation [graph view](https://rust-redo.github.io/import-tree/axios.html) [WIP]
- :hammer_and_pick: Strict `import` diagnosis based on [eslint-plugin-import](https://www.npmjs.com/package/eslint-plugin-import) rules [todo]
- :herb: Rich plugins support (vscode, vite, rollup, webpack) [todo]

## Why import-tree

Javascript module graph is very useful for

- dead code elimination
- code shift
- code quality diagnosis
- codebase structure analysis

and just like [madge](https://github.com/pahen/madge) and [dependency-cruiser](https://github.com/sverweij/dependency-cruiser), `import-tree` is a more powerful and faster tool to get you there.

## Screenshot

![import](https://github.com/rust-redo/import-tree/assets/102238922/0d363ae9-91fc-4d5b-8214-ac2bc484ae3a)

## Install

```shell
$ npm i -g import-tree
```

## Usage

<!-- `import-tree` is not only a cli tool, you can also use it as a JavaScript API or a specific bundler plugin. -->

```shell
Usage: import-tree [options]

CLI to some JavaScript string utilities

Options:
  -V, --version           output the version number
  -t, --target <file>     target file's relative or absolute path
  -r, --root <directory>  target codebase root directory (default: "./")
  -d, --depth <number>    import relation tree's depth (default: "2")
  -a, --alias <alias>     module path alias
  -o, --output <file>     parsing result's file path (default: "./import.json")
  -h, --help              display help for command
```

```shell
$ it -t ./src/index.js -d 3 # parse target file with custom import-tree depth, depth starts from 0
$ it -t ./src/index.js -r ../root # parse target file in specific root directory
$ it -t ./**/index.js -r ../root # parse target files of glob pattern
$ it -t ./src/index.js -o ./graph.html # parse target file and output graph html 
$ it -t ./src/index.js -o ./import.json # parse target file and output json file
```

## Benchmark

`import-tree` tested its Nodejs api's benchmark in multiple popular third-party git repos.

```shell
System:
  OS: macOS 13.6
  CPU: (12) arm64 Apple M2 Pro
  Memory: 68.25 MB / 16.00 GB
  Shell: 3.2.57 - /bin/sh
Binaries:
  Node: 18.17.1 
  Yarn: 1.22.19 
  npm: 9.6.7 
  pnpm: 8.8.0 
```

|repo|file type|parsed files|import links|import depth|average time(ns)|graph|
|---|----|-----|----|----|----|---|
|[axios@1.6.2](https://github.com/axios/axios/tree/v1.6.2)|`.js`|59|132|3|**12**,249,262.50|[view](https://rust-redo.github.io/import-tree/axios.html)|
|[rxjs@8.0.0-alpha.12](https://github.com/ReactiveX/rxjs/tree/8.0.0-alpha.12)|`.ts`|205|877|3|**351**,465,279.17|[view](https://rust-redo.github.io/import-tree/rxjs.html)|
|[nextui@2.0.0](https://github.com/nextui-org/nextui)|`.ts` `.tsx`|230|886|3|**57**,522,687.48|[view](https://rust-redo.github.io/import-tree/nextui.html)|
|[antd@5.11.2](https://github.com/ant-design/ant-design)|`.ts` `.tsx`|658|2282|3|**332**,391,816.69|[view](https://rust-redo.github.io/import-tree/antd.html)|

## Roadmap

### v0.1.x [WIP]

- `rust parser`
  - construct import tree with specific depth :heavy_check_mark:	
  - parse esm static `import/export` syntax
  - resolve internal/external module path
  - compute circular dependencies
  - handle exception robustly 
- `npm core`
  - cross-platform fast Nodejs api
  - parse target files of glob pattern :heavy_check_mark:	
- `npm cli`
  - output import tree in json & html format :heavy_check_mark:	
  - meaningful process log
  - check circular dependencies
- `module graph`
  - graph chart renderer :heavy_check_mark:	
  - dark mode :heavy_check_mark:	
  - circular import highlight


