# import-analysis(WIP)

A fast tool that uses [swc](https://swc.rs/) to comprehensively analyze JavaScript/TypeScript modules dependencies.

## Features

- [ ] Blazing fast ESM/CJS modules scanner
- [ ] Readable scanning output (json & html)
- [ ] Interactive dependency network preview
- [ ] Strict `import` diagnosis based on [eslint-plugin-import](https://www.npmjs.com/package/eslint-plugin-import) rules
- [ ] Rich plugins support (vite, rollup, webpack)


## Usage

`import-analysis` is not only a cli tool, you can also use it as a JavaScript API or a specific bundler plugin.

## Benchmark & Demo

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

|repo|total files|import links|depth|ops/sec|average time(ns)|demo|
|---|----|-----|----|----|----|
|[axios@1.6.2](https://github.com/axios/axios/tree/v1.6.2)|59|132|3|81|12,249,262.50|
|[rxjs@8.0.0-alpha.12](https://github.com/ReactiveX/rxjs/tree/8.0.0-alpha.12)|205|877|3|2|351,465,279.17|


