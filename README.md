<div align="center">
  <h1>
    <code>deno-deploy-rust-template</code>
  </h1>
</div>

<p align="center">
A template for creating <a href="https://deno.com/deploy">Deno Deploy</a> projects in Rust
</p>

## Prerequisites

- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) (optional, use this template and manually change the needed placeholders otherwise)
- [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) (`wasm-pack` does not have a stable version targeting Deno yet)
- [`deno`](https://deno.land)
- [`deployctl`](https://github.com/denoland/deployctl)

## Using this template

```sh
cargo generate --git https://github.com/yoav-lavi/deno-deploy-rust-template.git --name my-project
```

## Building your project

```sh
cargo build --release --target wasm32-unknown-unknown;
wasm-bindgen target/wasm32-unknown-unknown/release/{{crate_name}}.wasm --target deno --out-dir build/
```

## Running your project with Deno

```sh
deno run --allow-read --allow-net src/index.ts
```

## Deploying to Deno Deploy

> **Note**
>
> Create a new token in the Deno Deploy (under "Access Tokens") and use it in place of `...` in `--tokens=...`

```sh
deployctl deploy --token=...  --project={{deno-deploy-project-name}} src/index.ts --exclude "target/"
```
