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

<details>
  <summary>Prerequisites for usage without <code>cargo-make</code></summary>
  <br>
  <ul>
    <li><a href="https://github.com/cargo-generate/cargo-generate"><code>cargo-generate</code></a></li>
    <li><a href="https://rustwasm.github.io/wasm-bindgen/reference/cli.html"><code>wasm-bindgen-cli</code></a></li>
    <li><a href="https://deno.land"><code>deno</code></a></li>
    <li><a href="https://github.com/denoland/deployctl"><code>deployctl</code></a></li>
  </ul>
</details>

## Usage

> **Note**
>
> If using `cargo make`, any commands requiring `deno` will attempt to install `deno` via `cargo`, if it is not already installed and in your `$PATH`.
> If you want to install `deno` via other means and haven't already, do so before running any `cargo make` commands.

### Using this template

```sh
cargo generate gh:yoav-lavi/deno-deploy-rust-template
```

### Building your project

```sh
cargo make build-wasm
```

<details>
  <summary>Manual command</summary>
  <br>
  <pre>cargo build --release --target wasm32-unknown-unknown<br>&& wasm-bindgen target/wasm32-unknown-unknown/release/{{crate_name}}.wasm --target deno --out-dir build/</pre>
</details>

### Deploying to Deno Deploy

> **Note**
>
> Create a new token in the Deno Deploy (under "Access Tokens") and use it in place of `...` in `DENO_DEPLOY_TOKEN=...`
>
> If `~/.deno/bin` is not in your `$PATH`, you will need to add it for this command to work

```sh
DENO_DEPLOY_TOKEN=... cargo make deploy
```

<details>
  <summary>Manual command</summary>
  <br>
  <pre>deployctl deploy --token=... --project={{deno-deploy-project-name}} src/index.ts --exclude "target/"</pre>
</details>

### Running your project with Deno

```sh
cargo make run
```

<details>
  <summary>Manual command</summary>
  <br>
  <pre>deno run --allow-read --allow-net --allow-env src/index.ts</pre>
</details>

### Running tests

```sh
cargo make test-rust
cargo make test-integration
```

<details>
  <summary>Manual command</summary>
  <br>
  <pre>cargo test --target wasm32-unknown-unknown<br>deno test --allow-read --allow-net --allow-env tests/integration.ts</pre>
</details>

