<p align="left">
  <img src="https://user-images.githubusercontent.com/102238922/283716554-56ef774d-6ed0-4491-afab-93fbeba9e955.png" >
</p>

# import-tree(WIP)

A super fast tool that uses [swc](https://swc.rs/) to build JavaScript/TypeScript module `import/require`-relation tree.

## Features

- :zap: Blazing fast ESM/CJS modules scanner
- :rainbow: Interactive module relation graph view
- :hammer_and_pick: Strict `import` diagnosis based on [eslint-plugin-import](https://www.npmjs.com/package/eslint-plugin-import) rules [todo]
- :herb: Rich plugins support (vite, rollup, webpack) [todo]

## Usage

`import-tree` is not only a cli tool, you can also use it as a JavaScript API or a specific bundler plugin.

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

|repo|file type|total files|import links|import depth|ops/sec|average time(ns)|
|---|----|-----|----|----|----|---|
|[axios@1.6.2](https://github.com/axios/axios/tree/v1.6.2)|`.js`|59|132|3|81|**12**,249,262.50|
|[rxjs@8.0.0-alpha.12](https://github.com/ReactiveX/rxjs/tree/8.0.0-alpha.12)|`.ts`|205|877|3|2|**351**,465,279.17|

## Roadmap

### v0.1.x [WIP]

- `rust parser`
  - construct import tree with specific depth
  - parse esm static `import/export` syntax
  - resolve internal/external module path
  - compute circular dependencies
  - handle exception robustly 
- `npm core`
  - cross-platform fast Nodejs api
- `npm cli`
  - output import tree in json & html format
  - check circular dependencies
- `module graph`
  - graph chart renderer :heavy_check_mark:	
  - dark mode :heavy_check_mark:	


