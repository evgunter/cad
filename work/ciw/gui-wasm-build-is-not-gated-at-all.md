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

**Reproduce-then-catch, hosted, in that order** — the lib-target
evidence is hosted; the bin-target evidence below is LOCAL, and the two
are marked apart deliberately rather than reported under one word. Run `34373755002`
(`ff658559`, PR 1741's `E0599` planted back, no new row) concluded
**success** across all 37 jobs — and the viewer axis was TRUE on that
run, so `clippy (viewer app feature - eframe + wgpu)` ran `-D warnings`
over the same crate and passed. Run `34375557117` (`121890d9`, same
plant, row present) failed at exactly one step with
`error[E0599]: JsValue doesn't implement std::fmt::Display`,
`crates/viewer/src/app.rs:1990`. The plant is removed on this branch.

**Cost when the key fires:** the STEP is 53-57 s across four hosted
runs. **No job-level delta is quoted, and the first version of this
disposition was wrong to quote one.** The four `fmt` jobs ran 342 s (no
row), 339 s (red, aborted at the row), 360 s and 307 s — and 307 s is a
WITH-row reading, 35 s *below* the no-row one. `rustdoc (gate)` alone
moved 106-126 s across the same runs, so a before/after pair on this job
measures that noise, not this step. Nothing at all on runs where the
axis is false.

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

## Three premises this row falsified, and what happened to each

A row that compiles something nothing compiled before makes claims about
that thing false. All three were found by the fix pass's reviewer, not
by the lane.

1. **`ci.yml`'s "WHAT IS NOW UNGUARDED" paragraph**, ninety lines above
   the new row in the same file: *"the `pncad`/`pncad-py` façade under
   `--cfg getrandom_backend="wasm_js"`"*. `viewer` depends on `pncad`,
   so on every axis-true run the new row compiles `pncad` at that
   target. **Fixed in this PR**: the paragraph now names `pncad-py`
   alone and says the `pncad` guard is conditional on a row it may not
   assume ran.
2. **`crates/viewer/README.md`'s browser-spike section**: *"It is also
   not CI-guarded: the wasm32 step excludes `viewer` … so a dependency
   bump can break this build with every check green."* A dependency bump
   classifies TIER=all, which makes the axis true, which fires the row —
   precisely the case that sentence calls uncovered. **Not fixed**:
   `crates/viewer/README.md` is CHROME's and VIEW's. Reported in the PR
   body. The `cfg`-pattern sweep above could not have caught this — it
   matches source, and this is prose. That is the sweep's blind spot,
   stated where the sweep is.
3. **`ci.yml`'s `fmt` job header**: *"WHY THESE THREE AND NOT SOME OTHER
   SET … Two of them still read nothing from the filter."* The job's
   shared property has not been "workspace-wide and filter-blind" for
   two changes now, and this row is the latest reader of the viewer
   axis. **Fixed in this PR**: the header states what the set actually
   is and what property admits a row into it, and tells the reader to
   grep the key rather than trust a count.

## A fourth premise, in three of the repo's own documents

`RUSTFLAGS='--cfg getrandom_backend="wasm_js"'` is **not required** at
the pinned `getrandom` 0.3.4. That version's `src/backends.rs` takes its
final wasm32 arm under `cfg(feature = "wasm_js")`; the `compile_error!`
still reading *"enabling the `wasm_js` feature flag alone is
insufficient"* sits in that arm's ELSE, i.e. it is what a reader hits
with the FEATURE off. Measured: the row is green with the cfg dropped.

The flag stays — it is free, the row's subject is the build
`serve-wasm.sh` performs, and getrandom's own diagnostic still asserts
it is needed — but three documents assert it is load-bearing.
`local-scripts/serve-wasm.sh` is CIW's and is **corrected in this PR**;
`crates/viewer/README.md` and `crates/viewer/Cargo.toml`'s wasm stanza
are CHROME's and VIEW's and are **reported, not edited**.

## Residue disclosed, with its file

- `work/view/viewer-items-unreferenced-at-wasm32.md` — the two dead-code
  warnings that are why this row is `check` and not `-D warnings`, filed
  on the owner's slate, carrying the flip as its close condition. The
  row's own comment names that file, so the debt is readable from the
  code as well as from the tracker.
- `work/ciw/mirror-pairs-env-divergence-unchecked.md` — this row's
  `RUSTFLAGS` prefix is invisible to `check-ci-mirror-parity.py`
  (`:1304` discards every token before `cargo`), so the parity pass this
  PR cites proves the `--features` value matches and nothing about the
  prefix. Recorded there as a measured instance with the population
  count that item asked for; deliberately not built here.

