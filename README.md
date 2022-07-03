<p align="center">
    <code>deno-deploy-rust-template</code>
</p>

<p align="center">
A template for creating <a href="https://deno.com/deploy">Deno Deploy</a> projects in Rust
</p>

## Prerequisites

- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) (optional)
- [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) (`wasm-pack` does not have a stable version targeting Deno yet)
- [`deno`](https://deno.land)
- [`deployctl`](https://github.com/denoland/deployctl)

## Using this template

```sh
cargo generate --git https://github.com/yoav-lavi/deno-deploy-rust-template.git --name my-project
```

## Build

```sh
cargo build --release --target wasm32-unknown-unknown;
wasm-bindgen target/wasm32-unknown-unknown/release/{{crate_name}}.wasm --target deno --out-dir build/
```

## Run

```sh
deno run --allow-read --allow-net src/index.ts
```

## Deploy

```sh
deployctl deploy --token=...  --project={{deno_deploy_project_name}} src/index.ts --exclude "target/"
```
