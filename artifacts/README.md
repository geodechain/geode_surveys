# Surveys build artifacts

Built from `lib.rs` on this branch with `cargo contract build --release` (ink! 5.1.1).

| File | What it is |
|---|---|
| `surveys.wasm` | Contract code: upload this with `contracts.uploadCode` (Technical Committee account) |
| `surveys.json` | Metadata / ABI: use it with the code hash to instantiate and call |
| `surveys.contract` | Code + metadata bundle |

- **Code hash (blake2-256 of the wasm):** `0xae0b91d50c0896b5e2e91bac568db85b1ed195fd39f6311636f425ef9d04edba`
- **SHA-256:** see `SHA256SUMS`
- **E2E:** this exact code passed `e2e/e2e.mjs` (60/60) on a local Geode dev node, runtime 20261018. See `e2e/e2e-results.txt`.
- **Constructor:** `new()`; the deploying account becomes root.
