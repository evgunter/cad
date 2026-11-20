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
- `a-solved-value-the-body-stores-is-admitted-by-its-check`

## The solved value (Ev, PR 7176, 2026-11-20)

Ruled: `crates/geom-core/README.md`, DL5.2's paragraphs after the table. A closed form stays in `T`; otherwise one door admits an `f64` candidate by the check at `T`, bounded `Certifies`, the one caller of `from_computed`, and only for a value the body stores. Round 15 traced each value forward and found three or four storing functions (`spline_face::settle`, `composite::crossings`, `replace_face::carrier_foot`, and by one reading `SplineWall::paths`); the five that only evaluate at the value are places and belong to `a-place-is-an-f64-argument-answered-on-the-value-channel`. `replace_face::corner_of_root` is `f64`-only by its lane.

Units added on that ruling:

- `a-solved-value-the-body-stores-is-admitted-by-its-check` (the door, and the gate that `from_computed` has one caller)
- `a-stored-solved-parameter-is-an-enclosure-of-each-members-root` (Ev's B: not now, likely needed for booleans over wide boxes)

## Also found, not yet filed anywhere else

- `SplineWall` finds its feet on an `f64` copy of its chart, which goes round `project`'s `CertifiedBounds` fence.
- `ExtentBall::of_overlap` has no callers.
- Two brackets decide outside `decide` (the census unit `census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection` re-sorts them).
