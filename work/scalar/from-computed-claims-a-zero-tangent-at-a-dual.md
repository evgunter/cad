---
id: from-computed-claims-a-zero-tangent-at-a-dual
kind: issue
title: "Real::from_computed returns tangent zero at a dual: what a solved value is, and the units that take every other re-entry off it"
status: open
opened: 2026-11-19
priority: P1
cost: H
refs: [dl5-has-no-arm-for-a-selection-whose-result-is-the-payload, census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection]
design: true
---

Filed by the SCALAR orchestrator, 2026-11-19; restated 2026-11-20 on Ev's ruling on PR 7155.

## What

`Dual::from_computed` is `Self::constant(T::from_computed(x))`: tangent zero, the claim that the value does not move with the inputs. Of the 128 calls a dual can reach by type, Ev's ruling takes all but the solved values off `from_computed`, each as a unit with this row as its parent:

- `certification-interiors-are-closed-to-a-dual-by-a-marker-bound`
- `a-bound-to-decide-against-never-becomes-a-t`
- `a-place-is-an-f64-argument-answered-on-the-value-channel`
- `f64-built-data-is-built-at-t`
- `from-f64-is-split-by-where-the-number-comes-from`
- `the-stackups-tangents-are-checked-against-finite-differences`

## What is still open (the design question on this row)

A solved value the result is built on: a root, a foot, a crossing, about 9 calls (`curve_face/composite.rs` and `spline_face.rs` `settle`, `replace_face.rs` `corner_of_root`, the two polishes). Ev: "there's also no obvious way to convert to f64 and back in a way that's valid". The designers walked two sites scalar by scalar; their recommendation is with Ev as its own `[ev]` PR, on `scalar/solved-value`: a closed form stays in `T`, and otherwise one door admits an `f64` candidate by the check at `T`, bounded `Certifies`.

What the walks found, read in the code:

- `composite::crossings` isolates a root in `f64`, checks the residual at `T` at that exact number, and returns `T::from_computed(t)`; its doc calls it "a parameter whose point reads ON the surface, not an enclosure of the true crossing's parameter". True of the number at `f64` and `Interval`, an opaque unknown at `Sym`, false at a dual.
- `spline_face::settle` holds a certified box per meeting and returns its midpoint as a point: at `Interval` the returned `T` drops the box.
- No dual reaches either today: both sit behind `NurbsLane`, and `replace_face`'s corner root behind `SectionLane::f64()`. The one solved value a dual reaches on the production path is `topo::Link::lowest_on`, which has its roots in `T` and discards them (ATREST's row).

## Also found, not yet filed anywhere else

- `SplineWall` finds its feet on an `f64` copy of its chart, which goes round `project`'s `CertifiedBounds` fence.
- `ExtentBall::of_overlap` has no callers.
- Two brackets decide outside `decide` (the census unit `census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection` re-sorts them).
