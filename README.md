## `heatmap-wasm`

Rust → WebAssembly metrics library intended to be **installed as an npm package** and imported from a **module Web Worker**.

Currently exported metrics:
- `profit_factor(pnls: Float64Array) -> number`
- `profit_factor_batch(gross_profits: Float64Array, gross_losses_abs: Float64Array) -> Float64Array`

### Build an installable npm package

This repo uses [`wasm-pack`](https://github.com/rustwasm/wasm-pack). Build output goes to `pkg/` (that folder is what gets published to npm).

```bash
# For React apps using a bundler (Vite / webpack / CRA-style builds):
wasm-pack build --release --target bundler

# If you need a "no-bundler" build (served directly on the web):
# wasm-pack build --release --target web
```

### Publish to npm

```bash
wasm-pack publish --access public
```

### Consume from a module Web Worker

In your frontend repo:

```bash
npm i heatmap-wasm
```

In your worker (must be an **ES module worker**):

```js
import init, { profit_factor, profit_factor_batch } from "heatmap-wasm";

let ready;
function ensureReady() {
  if (!ready) ready = init(); // instantiates the .wasm once
  return ready;
}

self.onmessage = async (e) => {
  await ensureReady();

  const { pnls, grossProfits, grossLossesAbs } = e.data;
  const pf = profit_factor(pnls);
  const pfs = profit_factor_batch(grossProfits, grossLossesAbs);

  self.postMessage({ pf, pfs });
};
```

In your React app code (Vite/webpack-style):

```js
const worker = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });
```

### License

Dual-licensed under Apache-2.0 and MIT (see `LICENSE_APACHE` and `LICENSE_MIT`).
