<p align="left">
  <img src="https://github.com/rust-redo/import-analysis/assets/102238922/56ef774d-6ed0-4491-afab-93fbeba9e955" >
</p>

# import-analysis(WIP)

A super fast tool that uses [swc](https://swc.rs/) to build JavaScript/TypeScript module `import/require`-relation tree.

## Features

- :zap: Blazing fast ESM/CJS modules scanner [WIP]
- :rainbow: Interactive module relation [graph view](https://rust-redo.github.io/import-analysis/axios.html) [WIP]
- :hammer_and_pick: Strict `import` diagnosis based on [eslint-plugin-import](https://www.npmjs.com/package/eslint-plugin-import) rules [todo]
- :herb: Rich plugins support (vite, rollup, webpack) [todo]

## Install

```shell
$ npm i -g import-analysis
```

## Usage

<!-- `import-analysis` is not only a cli tool, you can also use it as a JavaScript API or a specific bundler plugin. -->

```shell
Usage: import-analysis [options]

CLI to some JavaScript string utilities

Options:
  -V, --version           output the version number
  -t, --target <file>     target file's relative or absolute path
  -r, --root <directory>  target codebase root directory (default: "./")
  -d, --depth <number>    import relation tree's depth (default: "2")
  -o, --output <file>     parsing result's file path (default: "./import.json")
  -h, --help              display help for command
```

### parse target file with custom import-tree depth

```shell
$ it -t ./src/index.js -d 3
```

### parse target file in specific root directory

```shell
$ it -t ./src/index.js -r ../root
```

### parse target files of glob pattern

```shell
$ it -t ./**/index.js -r ../root
```

### parse target file and output graph html or json file

```shell
$ it -t ./src/index.js -o ./graph.html
$ it -t ./src/index.js -o ./import.json
```

## Benchmark

`import-analysis` tested its Nodejs api's benchmark in multiple popular third-party git repos.

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
|[axios@1.6.2](https://github.com/axios/axios/tree/v1.6.2)|`.js`|59|132|3|**12**,249,262.50|[view](https://rust-redo.github.io/import-analysis/axios.html)|
|[rxjs@8.0.0-alpha.12](https://github.com/ReactiveX/rxjs/tree/8.0.0-alpha.12)|`.ts`|205|877|3|**351**,465,279.17|[view](https://rust-redo.github.io/import-analysis/rxjs.html)|
|[nextui@2.0.0](https://github.com/nextui-org/nextui)|`.ts` `.tsx`|230|13510|3|**4429**,009,233.62|[view](https://rust-redo.github.io/import-analysis/nextui.html)|

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
  - check circular dependencies
- `module graph`
  - graph chart renderer :heavy_check_mark:	
  - dark mode :heavy_check_mark:	
  - circular import highlight


