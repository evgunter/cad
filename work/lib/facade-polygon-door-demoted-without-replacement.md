---
id: facade-polygon-door-demoted-without-replacement
kind: issue
title: API gap — the facade polygon door was demoted with no replacement scheduled; 11 demo call sites route around it
status: open
opened: 2026-08-20
github: 759
refs: [S79]
---

## From GitHub issue 759

Opened 2026-08-20; 0 comments.

Found by a style-lane scan of `demos/` (out of scope for the original SMELL-SCAN per `docs/SMELL-SCAN-2026-08.md` §B). Filed per **Protocol v5 / A1** — this is the clearest instance in the batch of a *disclosed but unscheduled* deviation, which is exactly what A1 says now owes a schedule.

## The gap

`crates/pncad/src/authoring.rs:115-127` is a comment block where the door used to be. It records the demotion, the reason (the old `polygon(&[(f64, f64)]) -> ProfileLoop` minted a raw vertex table with no junction classification, deferring to validate what should be decided at authoring), and then:

> A façade-level lattice-backed `polygon` is a reasonable future door, **fenced out of this unit**.

No issue number, no named plan unit. Per A1 that is "unscheduled" stated as the schedule — this issue is the schedule.

## What exists and what does not

Polygon doors exist at other layers, so this is **not** "there is no polygon anywhere":

| Layer | Door |
|---|---|
| `crates/profile/src/lib.rs:266`, `:287` | trait method over `Point2<T>` |
| `crates/editor-core/src/program.rs:1213` | `pub fn polygon` over `(f64, f64)`, returns `Result<_, DimensionError>` |
| `crates/pncad-py/src/py/doc.rs:594` | the Python binding |

What is missing is the **`pncad` façade** door — the one a direct Rust library user reaches for, and the one that was removed.

## What that costs

Eleven demo call sites now go through a demo-hosted fold over `Open.at(..).line_to(..)…line_to(Start)`: `demos/tour/src/paths.rs`'s `path_polygon`, used from `bodies.rs:100,252`, `bool_bodies.rs:36`, `bossplate.rs:36`, `skinned.rs:202`, `az.rs:76`, `lily.rs:603`, `letterforms.rs:52,67,104`.

So the shared thing lives inside one of its consumers — the shape C11/Q1 flags as the one that drifts. And the helper's own doc says it *"Mirrors `pncad::authoring::polygon`'s `(f64, f64)` slice signature"*, which now describes nothing: that function is gone. A reader chasing the reference finds a comment about its removal.

## Why this is a library finding

Per `memories/demo-purpose.md` (ratified): *demos demonstrate REAL natural usage, and awkwardness is a library finding to record, never to hide.* Eleven call sites is not one demo being awkward; it is the most-used shape in the tour having no door.

## Not asserted

The demotion looks right on its merits — junction classification at authoring rather than at validate is the better design, and the comment argues it well. The question is only whether the lattice-backed replacement gets built, and when. If the answer is "not soon", the honest close is to say so at `authoring.rs:115` and point the demo helper's stale reference somewhere real.

## Home

LIB: the missing door is in `crates/pncad/src/authoring.rs`, the program's own `crates/pncad/*` territory, and the LIB register fold placed this issue in its category A (the F1 curation-gap class).

## Question for Ev (2026-09-06, LIB orchestrator; `[ev]` PR)

Whether the lattice-backed façade `polygon` gets built, or the
demotion is made permanent at the site. The orchestrator's
recommendation first:

- **(A) Build it.** `pncad::authoring::polygon(&[(f64, f64)], tol)
  -> Result<ProfileLoop, PathError>`, spelled through the PATHS
  lattice exactly as `demos/tour/src/paths.rs::path_polygon` spells it
  today (`Open.at(p0)`, a `line_to` per vertex, `line_to(Start)` as
  the seam), so a within-band-tangent or cusped corner refuses at
  authoring. The one piece of vocabulary it needs is the refusal for
  fewer than three vertices, which `PathError` has no honest arm for
  today — one new variant. Then the tour's helper is deleted and its
  thirteen call sites use the door, which is what a demo is for.
- **(B) Say no at the site.** Rewrite the comment block at
  `crates/pncad/src/authoring.rs` from "a reasonable future door,
  fenced out of this unit" to a decision: the lattice IS the façade's
  polygon spelling, and the tour's helper is the demo of it. Re-point
  the helper's doc, which today says it "mirrors
  `pncad::authoring::polygon`" — a function that no longer exists.

Recommendation: (A). Thirteen call sites in the tour reach for one
shape and route around the façade to get it; `memories/demo-purpose.md`
says that is a library finding, and the cost of (A) is one fallible
door plus one `PathError` arm. The trait-level `polygon` in
`crates/profile` is NOT this door: it mints the raw vertex table with
no junction classification, which is exactly what the demotion
removed from the façade.

## Ruled (Ev, PR 2017, 2026-09-06): **(A) — build it**

`pncad::authoring::polygon(&[(f64, f64)], tol) -> Result<ProfileLoop, PathError>`
spelled through the PATHS lattice as the tour's `path_polygon` spells it
(a `line_to` per vertex, `line_to(Start)` as the seam), refusing at
authoring; one new `PathError` arm for fewer than three vertices; the
tour's helper deleted and its thirteen call sites moved onto the door;
the comment block at the demoted site replaced by the door. The
`PathError` arm crosses to Python through `path_error_tag` like every
other arm, with its `TAG_INVENTORY` row. Dispatchable as a LIB unit.
