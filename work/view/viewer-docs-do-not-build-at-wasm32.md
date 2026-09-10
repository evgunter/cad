---
id: viewer-docs-do-not-build-at-wasm32
kind: issue
title: a wasm32 rustdoc pass over viewer is red with nine unresolved intra-doc links and nothing runs it
status: open
opened: 2026-09-09
---


Found by `viewer-items-unreferenced-at-wasm32`'s lane, checking whether
the `#[cfg(not(target_family = "wasm"))]` that item's fix adds could
break a documentation build. It cannot make things worse than they
already are, and finding that out is what turned up this.

## What

```
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
RUSTDOCFLAGS='-D warnings -A rustdoc::private_intra_doc_links' \
cargo doc --no-deps --document-private-items \
  -p viewer --features app --target wasm32-unknown-unknown
```

— `scripts/doc-gate.sh`'s own lint set (`:555`) and pass shape
(`:638`), pointed at the browser target — failed at `1d29a8eeb` with
**nine** `rustdoc::broken_intra_doc_links` errors, rose to ten when
`viewer-items-unreferenced-at-wasm32` closed, and is **back to nine**:
the tenth was `apply_status`'s link to `ViewerApp::deliver_status`, and
that door is deleted (the ruling
`was-the-status-route-supposed-to-fire-for-an-absent-chooser`), so the
link went with the item it pointed at. Every remaining one is a doc
comment compiled at both targets linking an item that exists only at
the host. The nine are read off the command above on THIS tree, not
counted from a diff, and each site below is the `-->` line rustdoc
printed:

| site | the link | what gates the target away |
|---|---|---|
| `app.rs:41` | `crate::evalseam::ThreadEvaluator` | `evalseam.rs:453` |
| `app.rs:532` | `ThreadEvaluator` | `evalseam.rs:453` |
| `app.rs:552` | `StartupError::Worker` | `app.rs:495` |
| `app.rs:573` | `StartupError::Worker` | `app.rs:495` |
| `app.rs:1906` | `run` | `app.rs:1801` |
| `app.rs:1911` | `run` | `app.rs:1801` |
| `evalseam.rs:18` | `ThreadEvaluator` | `evalseam.rs:453` |
| `evalseam.rs:38` | `ThreadEvaluator` | `evalseam.rs:453` |
| `evalseam.rs:74` | `ThreadIndexer` | `evalseam.rs:453` |

The two `run` links are the sharp case: they sit in `run_web`'s own doc
(`#[cfg(target_family = "wasm")]`, `app.rs:1923`, with `pub async fn
run_web` at `:1924`), so they are
unresolvable in the **only** configuration that compiles the item they
document, and resolvable in the only configuration that does not.

## Why it is filed rather than fixed

Nothing runs this pass. `.github/workflows/ci.yml`'s rustdoc gate is
host-target, and its wasm rows are `cargo check` — so this is a
configuration whose docs no row has ever built, and the count is a
ratchet with nobody holding it. The PR closing
`viewer-items-unreferenced-at-wasm32` took it from nine to ten,
deliberately and with the number stated — `apply_status`'s doc is
compiled at both targets and linked `ViewerApp::deliver_status`, which
that PR made host-only, and de-linking a working host link to hold a
count nothing reads would have made the host docs worse for no reader.
The ratchet then fell back to nine, and **that is the shape of the
problem rather than progress on it**: the tenth went because an
unrelated ruling deleted the item it linked, not because anything held
the count.

## The two shapes

1. **Fix the links** — de-link, or re-word, the nine sites so a wasm
   rustdoc pass is clean, and then a row can hold it.
2. **Rule that the viewer's docs are host-only** and say so once, in
   `crates/viewer/README.md`, so the next lane does not re-derive this.

Either way the thing to avoid is the current state, where a reader
cannot tell which it is. Note that (1) alone buys nothing durable
without a row, which is the `-D warnings` argument
`viewer-items-unreferenced-at-wasm32` makes for `check`.
