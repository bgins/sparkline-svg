This example generates sparklines by running Sparkline SVG on the Homestar runtime.

## Setup

Install the following:

- [Homestar][homestar]
- [IPFS Kubo][kubo]
- [wasm-tools][wasm-tools]
- [jq][jq]

## Build and publish compute

Build the Sparkline SVG Wasm component.

```sh
cargo build --target wasm32-unknown-unknown
wasm-tools component new ../../target/wasm32-unknown-unknown/debug/sparkline_svg.wasm -o ./sparkline_svg.wasm
```

The first command builds a Wasm binary, and the second command componentizes it.

Start the Kubo daemon and add the Wasm component.

```sh
ipfs dameon
ipfs add --cid-version 1 sparkline_svg.wasm
```

The `ipfs add` command will return a CID for the Wasm component.

## Prepare workflow

`workflow.json` describes the computation we would like to run.

The `rsc` field contains the CID for the Wasm component we added to IPFS. If you have altered source code, you will want to update this CID to the one Kubo reported.

The `args` are the data that will be used to generate the SVG.

```json
"args": [
  [1, 0, 5, 4, 8, 10, 15, 10, 5, 4], // data points
  "An SVG Title", // SVG title for accessibility
  "A SVG Description", // SVG description for accessibility
  500, // width
  200, // height
  "#2e4374", // line color
  "#7c81ad" // fill color, set to transparent for no fill
],
```

Update `args` to generate the different SVGs as desired.

## Run workflow

Start the Homestar runtime.

```sh
homestar start
```

In a separate terminal window, run the workflow

```sh
homestar run -w workflow.json
```

The Homestar node will log a message that looks like:

```sh
ts=2024-01-01T02:53:07.324951Z level=debug target=homestar_runtime::event_handler::notification message="emitting receipt to WebSocket" subject=notification.receipt category=notification cid=bafyrmihiazhc3lk2eyvaoom7fdmgv7trwchecslqbqjpydnop7hffazl2m
```

Take the `cid` from the message and retrieve the computed receipt from IPFS.

```sh
ipfs dag get bafyrmihiazhc3lk2eyvaoom7fdmgv7trwchecslqbqjpydnop7hffazl2m | jq ."out[1]" --raw-output > output.svg
```

We use `jq` to extract the SVG from the receipt `out` field and write it to an `output.svg` file.

[homestar]: https://docs.everywhere.computer/getting-started/setup-your-local-node#installing-homestar
[kubo]: https://docs.ipfs.tech/install/command-line/#install-official-binary-distributions
[wasm-tools]: https://github.com/bytecodealliance/wasm-tools#installation
[jq]: https://jqlang.github.io/jq/download/
