# NAND2Tetris
Implementation of [NAND2Tetris](https://www.nand2tetris.org/) with some additional features (linter, diagnostics).
## Setup

```sh
# downloads emulators and test suites
$ nix run .#setup
```

## Test

```sh
$ nix develop # enter nix shell
$ ./test.sh
$ cargo test --workspace
```

## Compile Jack

```sh
# generate `runtime` directory
$ ./build.sh jack-compiler/tests/fixtures/{Project}
```

## Diagnostics

More examples found in [uitest](./jack-compiler/tests/ui).

<p align="center">
    <img src="./assets/diagnostic.png">
</p>
