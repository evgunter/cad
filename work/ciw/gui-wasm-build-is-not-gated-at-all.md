---
id: gui-wasm-build-is-not-gated-at-all
kind: issue
title: the GUI's wasm32 build is gated by nothing: ci.yml's wasm row excludes viewer, and default features exclude the app feature where the wasm code lives
status: review
opened: 2026-09-04
branch: ciw/unreachable-roots
pr: 2263
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

`viewer` is excluded, so nothing in CI compiles that block. (Re-read
2026-09-06: the row has moved to `ci.yml:1936` and now also carries
`--features interval`; the `--exclude viewer` is unchanged and so is this
finding.)

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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/ciw/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Disposition (2026-09-09, PR 2263)

**Built, as a seed-keyed row.** `.github/workflows/ci.yml`'s `fmt` job
now carries `wasm32 check (viewer app feature - the browser entry
point)`: `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo check -p
viewer --features app --target wasm32-unknown-unknown`, gated on
`needs.filter.outputs.run_viewer_toolkit` — the same axis the three
toolkit rows above it read, and the treatment Ev confirmed in chat on
2026-09-09 as the one that needs no further ruling. `RUSTFLAGS` is
required, not decoration: `getrandom` refuses to build for wasm32 until
a backend is named in both halves. Mirrored in `local-scripts/ci-local.sh`
as `wasm32 check (viewer app)`, unconditional there.

**The `--features app` half is kept**, which is what this item asked
for: `run_web` lives behind that non-default feature, so the naive
default-features row would have been green on the defect.

**Reproduce-then-catch, hosted, in that order.** Run `34373755002`
(`ff658559`, PR 1741's `E0599` planted back, no new row) concluded
**success** across all 37 jobs — and the viewer axis was TRUE on that
run, so `clippy (viewer app feature - eframe + wgpu)` ran `-D warnings`
over the same crate and passed. Run `34375557117` (`121890d9`, same
plant, row present) failed at exactly one step with
`error[E0599]: JsValue doesn't implement std::fmt::Display`,
`crates/viewer/src/app.rs:1990`. The plant is removed on this branch.

**Cost when the key fires**, measured on this PR's own runs: the STEP
is 55 s green (`34377712872`) and 57 s red (`34375557117`); the JOB is
5m42s without the row (`34373755002`) and 6m00s with it, i.e. +18 s,
because `rustdoc (gate)` in the same job swings tens of seconds between
runs. Both readings round to 6 billed minutes, so the billed cost on
that pair is +0 — not quoted as a flat +0, because the same swing can
put the job over a boundary on another day. Nothing at all on runs where
the axis is false.

**No nightly re-take, and the reason is written at the row.** The three
toolkit rows above it defer their skipped coverage to `nightly.yml`;
this row does not need to, because every diff that can break it either
seeds `viewer` (the code is under `crates/viewer/src`) or is a
Cargo.toml/Cargo.lock edit classifying TIER=all — and both make the axis
true. What is left is a wasm-specific break in a crate `viewer` depends
on but which is outside {viewer, pncad, bvh}; every such crate is one
the workspace wasm row above already compiles for this target. That
argument is stated at the row so it is re-checked, not re-derived, if
`viewer` grows a new edge.

**The sweep this item asked for, its hit list, and its blind spot.**
Pattern: `grep -rln 'target_family = "wasm"\|target_arch = "wasm32"'`
over `crates/ demos/ tools/ benches/ interval-transcendentals/`. Eight
files, all in `crates/viewer`, and every one is now compiled at that
target by this row:

- `src/app.rs` — the `run_web` entry point and `WebStartupError`. The
  defect's own site. **Fixed** (this row compiles it).
- `src/bin/viewer.rs` — six `cfg(target_family = "wasm")` items,
  including the wasm `main` and `report_to_page`. **Fixed**, and
  verified rather than assumed: `cargo check -p viewer` selects lib
  AND bins, the bin's `required-features = ["app"]` is satisfied by
  this row's `--features app`, and a deliberate `E0308` planted in
  `report_to_page` failed the row (`could not compile viewer (bin
  "viewer")`).
- `src/lib.rs`, `src/prefs.rs`, `src/evalseam.rs`, `src/frame.rs` —
  `cfg(not(target_family = "wasm"))` and one `cfg!` runtime branch.
  **Fixed** in the sense that matters: the negated arms are what this
  target *stops* compiling, and the row is what proves the remainder
  still builds without them.
- `tests/eval_seam.rs` — five negated arms in a test target. **Not this
  unit**: `cargo check` without `--all-targets` builds no test targets,
  and a wasm test lane is GUI-5's, not a compile guard's.
- `Cargo.toml` (viewer, and `crates/pncad/Cargo.toml`) —
  `cfg(target_arch = "wasm32")` dependency tables, not code. **Not a
  defect**; they are the stanzas this row's `RUSTFLAGS` pairs with.

What the pattern cannot match: a `cfg` written through `cfg_attr`, a
feature-gated module whose contents are platform-specific without naming
a target, and any target family other than wasm — nothing here
establishes that a `cfg(windows)` or `cfg(target_os = "macos")` block is
compiled by anything in this repo.

