---
id: viewer-items-unreferenced-at-wasm32
kind: issue
title: Two viewer items are dead code at wasm32, and they are why the new wasm row cannot deny warnings
status: open
opened: 2026-09-09
---

Found by CIW unit 4 (PR 2263) while adding the first CI row that
compiles this crate for `wasm32-unknown-unknown`. Reported rather than
fixed: `crates/viewer/src/*` is VIEW's and CHROME's, not CIW's.

## What

At `--target wasm32-unknown-unknown --features app`, two items in
`crates/viewer/src/app.rs` are unreferenced and warn under the default
`dead_code` lint:

- `WINDOW_TITLE` (`app.rs:114`). Its only use is `eframe::run_native`'s
  first argument in `run` (`app.rs:1866`), and `run` is
  `#[cfg(not(target_family = "wasm"))]`. The browser build's title is
  the page's, so there is nothing for the constant to do there.
- `ViewerApp::deliver_status` (`app.rs:1066`). Its two call sites
  (`:1219`, `:1236`) are both inside `#[cfg(not(target_family =
  "wasm"))]` blocks — the Open…/Save… arms, which are compiled out on
  wasm because the browser build links no file dialog (#1125's posture:
  the control still shows, disabled, with its reason). It arrived with
  `status-line-writers-bypass-the-ranking` (PR 2026) as the `&mut self`
  shorthand beside `apply_status`.

Neither is a bug at any target this repo ships today. What they are is a
warning that a `-D warnings` row would red on.

## Why it is filed rather than left alone

`.github/workflows/ci.yml`'s `wasm32 check (viewer app feature - the
browser entry point)` is `cargo check` and **not** `-- -D warnings`,
precisely because of these two. Its comment says so at the row. So this
crate's wasm arms are now compiled by CI but are the only viewer rows in
the workflow that cannot fail on a warning — every other viewer row
(`clippy (viewer app feature - eframe + wgpu)`, the app-feature test
row, `clippy (viewer, all features)` in the nightly) denies them.

**The flip is the close condition.** When both items are resolved, that
row can become `cargo clippy -p viewer --features app --target
wasm32-unknown-unknown -- -D warnings` (or keep `check` with
`RUSTFLAGS=-Dwarnings`), and CIW will take that edit on request. Without
this file nothing schedules it, and the asymmetry becomes permanent by
default.

## Two shapes, and this file does not choose between them

1. **`#[cfg(not(target_family = "wasm"))]` on the items themselves**,
   matching the `cfg` already on every one of their users. Cheapest,
   and it says what is true.
2. **A wasm user that was lost.** `deliver_status` is the ranked-status
   shorthand; if the browser build ought to be delivering status for a
   refusal it currently swallows, the missing call is the finding and
   the warning is the symptom. That is a question about
   `status-line-writers-bypass-the-ranking`'s model, which is why this
   is filed on VIEW's slate rather than CHROME's.

`WINDOW_TITLE` is almost certainly (1). `deliver_status` is worth the
look.

## Home

VIEW and CHROME declare identical `paths`
(`crates/viewer/src/*, crates/viewer/tests/*, crates/viewer/README.md`),
so the directory is a judgement and not a derivation: it is here because
shape (2) is a question about a VIEW unit's own model. If CHROME is the
right owner, re-home by moving the file (`work/README.md`).

