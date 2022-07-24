<div align="center">
  <h1>
    <code>deno-deploy-rust-template</code>
  </h1>
</div>

<p align="center">
A template for creating <a href="https://deno.com/deploy">Deno Deploy</a> projects in Rust
</p>

## Prerequisites

- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
- [`cargo-make`](https://github.com/sagiegurari/cargo-make#installation)

OR

- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
- [`wasm-bindgen-cli`](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) (`wasm-pack` does not have a stable version targeting Deno yet)
- [`deno`](https://deno.land)
- [`deployctl`](https://github.com/denoland/deployctl)

## Usage

> **Note**
>
> If using `cargo wasm`, any commands requiring `deno` will attempt to install `deno` via `cargo`, if it is not already installed and in your `$PATH`.
> If you want to install `deno` via other means and haven't already, do so before running any `cargo make` commands.

### Using this template

```sh
cargo generate --git https://github.com/yoav-lavi/deno-deploy-rust-template.git --name my-project
```

### Building your project

```sh
cargo make build-wasm
```

<details>
<summary>Manual command</summary>
<br>
```sh
cargo build --release --target wasm32-unknown-unknown \
&& wasm-bindgen target/wasm32-unknown-unknown/release/{{crate_name}}.wasm --target deno --out-dir build/
```
</details>

### Running your project with Deno

```sh
cargo make run
```

<details>
<summary>Manual command</summary>
<br>
```sh
deno run --allow-read --allow-net src/index.ts
```
</details>

### Running tests

```sh
cargo make test-rust
cargo make test-e2e
```

<details>
<summary>Manual command</summary>
<br>
```sh
cargo test --target wasm32-unknown-unknown
deno test --allow-read --allow-net tests/e2e.ts
```
</details>


### Deploying to Deno Deploy

> **Note**
>
> Create a new token in the Deno Deploy (under "Access Tokens") and use it in place of `...` in `DENO_DEPLOY_TOKEN=...`
>
> If `~/.deno/bin` is not in your `$PATH`, you will need to add it for this command to work


```sh
DENO_DEPLOY_TOKEN=... cargo make test-e2e
```

<details>
<summary>Manual command</summary>
<br>
> **Note**
>
> Create a new token in the Deno Deploy (under "Access Tokens") and use it in place of `...` in `--token=...`
>
> If `~/.deno/bin` is not in your `$PATH`, you will need to add it for this command to work 

```sh
deployctl deploy --token=...  --project={{deno-deploy-project-name}} src/index.ts --exclude "target/"
```
</details>



