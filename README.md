# Typhoon DSL outliner

Parser for Typhoon DSL for extracting outline information. Used by
vscode-typhoondsl to provide outline and breadcrumbs navigation.

Return a JSON list of elements together with names and locations.

Uses [the rustemo parsing library](https://github.com/igordejanovic/rustemo/)
(still not published at crates.io).

## To install

``` sh
cargo install
```

## Run tests

``` sh
cargo test
```

See tests and JSON outputs in `src/tests/{models, libraries}`


## Usage

Outliner can parse the given file or it can parse the content provided on
standard input.

Examples:

``` sh
typhoondsl-outliner -j -p 'src/tests/models/shipboard power.tse'
```

``` sh
cat 'src/tests/models/shipboard power.tse' | typhoondsl-outliner -j -p
```





