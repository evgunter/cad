---
id: gui-wasm-build-is-not-gated-at-all
kind: issue
title: the GUI's wasm32 build is gated by nothing: ci.yml's wasm row excludes viewer, and default features exclude the app feature where the wasm code lives
status: open
opened: 2026-09-04
---


Found by PR 1741's style review (FIX, the `viewer` `Display` cut) and
confirmed by that unit's fix pass, which reproduced the break and then
measured what would and would not have caught it. Filed by the FIX
orchestrator; CIW is the natural claimant.

## What happened

PR 1741 shipped `error.to_string()` on `eframe::WebRunner::start`'s
`JsValue`. `wasm_bindgen::JsValue` implements `Debug` and **not**
`Display` — no `Display` impl, no inherent `to_string`, only
`as_string() -> Option<String>` — so the line is `E0599`. The PR was
**green**.

The code is `#[cfg(target_family = "wasm")]`, and the only wasm row in
the gate is `ci.yml:1646`:

```
cargo check --workspace --exclude pncad --exclude pncad-py --exclude viewer --target wasm32-unknown-unknown
```

`viewer` is excluded, so nothing in CI compiles that block.

## The two facts that make this worse than a missing exclusion

Both measured by the fix pass, not inferred:

1. **`viewer` cannot simply come off the exclusion list.** That row also
   excludes `pncad`, and `viewer` depends on it. Covering the GUI needs
   a **new row**, not a shorter exclusion list.

2. **A default-features row would not have caught this bug.** The wasm
   entry point lives behind the non-default `app` feature, so the check
   has to be `-p viewer --features app --target wasm32-unknown-unknown`.
   The fix pass ran both: default features clean, `--features app` clean
   only after the revert.

So the naive repair — delete one `--exclude` — produces a row that
passes, looks like coverage, and still would not have failed on this
commit. That is the failure mode worth naming.

## Why it matters beyond one line

A `cfg`-gated block no CI target builds is a place where **a text-driven
sweep edits code the compiler never sees.** PR 1741 was a mechanical
`{error:?}` → `{error}` sweep across a crate; every other site it
touched was compiled by the gate, and the one that was not is the one
that broke. Any future sweep over `viewer` has the same hole.

The unit's own blind-spot list — carefully written, five entries — did
not contain this one, because there is no reason a lane would think of
it. That is what makes it infrastructure rather than lane discipline.

## What the fix looks like

A row that runs `cargo check -p viewer --features app --target
wasm32-unknown-unknown`. Cost is one `check`, no test execution, and it
is compile-only by nature. Whether it joins the existing wasm job or
takes its own is CIW's call; whether it is per-push or a nightly row
depends on how much wasm-toolchain time the gate can carry, and the
`--features app` half is the part that must not be dropped for speed.

Worth checking in the same pass whether any other crate's `cfg`-gated
targets are similarly ungated — this issue names `viewer` because that
is where it was measured, not because a sweep established it is alone.

## Interim state

PR 1741 reverted the arm to `format!("{error:?}")` with the reason at
the site (the orphan rule forecloses forwarding; `as_string()` is
rejected because it answers `None` for non-string values and would drop
the browser's message), and its PR body states plainly that the lane
request it made buys determinism only — the axis the change actually
needed is not on the matrix, and the clean wasm32 result is reported
from a local check rather than from the gate.

## Home

`work/issues/` — `.github/workflows/ci.yml` is CI ground and CIW is the
open program there. Re-home by header edit.
