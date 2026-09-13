---
id: wasm-only-doc-comments-are-checked-by-nothing
kind: issue
title: no browser rustdoc pass runs, so a doc comment on a wasm-only item gets neither a doc build nor a lint
status: open
opened: 2026-09-10
---


Disclosed by `viewer-docs-do-not-build-at-wasm32`'s closing unit, whose
ruling (`crates/viewer/README.md`, **Rustdoc posture**) names this gap as
its stated cost. This file is the schedule for it.

## What

A doc comment on an item the host pass renders no page for is read by
**no** rustdoc pass this repo runs. `scripts/doc-gate.sh` builds at the
host target, where the item does not exist, so the comment is never
parsed; the browser pass that would parse it is run by nothing.

The class, by name rather than by number — a count here would be a
second copy of a figure nothing keeps in step, which is the mistake this
file's first draft made:

- `crates/viewer/src/app.rs`: `WebStartupError` and each of its five
  variants (`NoDocument`, `NoCanvasElement`, `NotACanvas`, `Startup`,
  `Runner`), `run_web`, and the two wasm-only `impl` blocks beside them.
  The variants carry **no `cfg` of their own** — the `cfg` is on the
  enum — which is why the ruling's test is page existence and not an
  attribute.
- `crates/viewer/src/bin/viewer.rs`: `CANVAS_ID`, `STATUS_ID`,
  `STATUS_DETAIL_ID`, `STATUS_TITLE_ID` and `report_to_page`.

The first draft of this file said `bin/viewer.rs` had "no intra-doc link
at all" and inferred it had no doc comments in the class. Those are
different claims: it has five doc comments and no links, so it is in the
class and carries no defect today.

## Why it is worth a file

Two links in this class — `[`run`]` twice inside `run_web`'s doc — were
dead in every configuration for as long as they existed, and nothing
found them: the host gate could not see the comment, and the pass that
could had never been run.

**But the blind spot itself is not the thing to close.**
`scripts/doc-gate.sh:299-309` names the identical blind spot on the
FEATURE axis — *"a genuinely broken intra-doc link written INSIDE a
`not(feature)` half is still reported by nothing"* — and records it
**accepted permanently** (issue #1317, answered 2026-09-04). The target
axis is the same claim with `target_family` for `feature`, so
consistency accepts it here too.

What is missing is the thing that makes it acceptable there. On the
feature axis pass 3 RUNS, so a `not(F)` half still compiles in a doc
build and still gets every other rustdoc lint. On the target axis
nothing runs at all, so wasm-only items get neither. That gap, not the
blind spot, is this row.

## The shape that closes it, priced

**A browser pass with `rustdoc::broken_intra_doc_links` allowed** — the
precedent's own shape, `RUSTDOC_LINTS_INERT` pointed at
`--target wasm32-unknown-unknown`. Zero per-site cost, and it buys what
pass 3 buys: the wasm half compiles in a doc build, and every other
rustdoc lint reads it. Measured 2026-09-10: the lib-only pass with that
lint allowed is **clean**.

**Its precondition, which is the row's real work.** doc-gate's pass
shape is two invocations, and the second adds `--bins --examples`. At
`wasm32-unknown-unknown` that half does not compile:
`error[E0432]: unresolved import viewer::evalseam::ThreadEvaluator` at
`crates/viewer/examples/r1_e2e.rs:19:37` — the example imports a
host-only item unconditionally. Any browser pass at doc-gate's pass
shape needs that fixed first.

## The `cfg_attr` shape, refused, with its costing corrected

`#[cfg_attr(target_family = "wasm", allow(rustdoc::broken_intra_doc_links))]`
on the items bearing the by-construction links greens the browser pass
under FULL lints, without weakening the host pass, because the allow is
conditional. That much is confirmed. Three things refuse it:

- **Its costing was wrong.** "A CI row could then hold it" cannot be
  paid by those attributes: doc-gate's pass shape hits the `E0432` above
  first, so the same precondition is owed either way.
- **It is inconsistent with the precedent.** It closes the target-axis
  blind spot while the feature axis leaves the identical one accepted
  permanently. That may be right, but it needs an argument against
  #1317 that nobody has made.
- **doc-gate already rejected this shape one level coarser.** Its
  *per-root deny list, considered and rejected* (`:287-298`) says
  allowing the lint only where cross-half prose happens to live "makes
  the LINT SET a function of which crate currently happens to have such
  prose", so the first correct cross-half link written elsewhere reds
  for being correct — and *"a blind spot that is uniform is one
  sentence; a blind spot that is per-root is a roster"*. A per-SITE
  `cfg_attr` roster is that argument one level finer, in a crate whose
  `evalseam` carries **two seams, each with two implementations**
  (`evalseam.rs:1-6`), and which therefore mints cross-target links by
  construction.

## Fence

The CI row is `.github/workflows/ci.yml`, which is **CIW's**. The
`examples/` fix and the lint selection for a browser pass are this
crate's.
