# sparkline-svg

Sparkline SVG is a sparkline generator that compiles to a Wasm component.

Try the [CLI example][cli] to generate sparkline SVGs using the Homestar runtime.

## Build

Build with a Wasm target.

```sh
cargo build --target wasm32-unknown-unknown
```

## Componentize

Componentize the Wasm binary.

```sh
wasm-tools component new target/wasm32-unknown-unknown/debug/sparkline_svg.wasm -o output/sparkline_svg.wasm
```

[cli]: examples/cli/README.md

## Features

The `vector-effects` feature uses `vector-effect="non-scaling-stroke"` and is set by default. If your use case does not support vector effects, compile with:

```sh
cargo build --target wasm32-unknown-unknown --no-default-features
```
