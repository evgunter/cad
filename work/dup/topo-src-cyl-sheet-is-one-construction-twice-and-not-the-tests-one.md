---
id: topo-src-cyl-sheet-is-one-construction-twice-and-not-the-tests-one
kind: issue
title: census.rs and chart_region.rs build the same cylinder sheet, and it is not the construction the tests/ door now shares
status: open
opened: 2026-09-19
---


## Finding

Two in-`src` test modules grow the same cylinder-wall sheet:
`crates/topo/src/census.rs`'s `cyl_sheet` (with `cyl_sheet_b` beside
it) and `crates/topo/src/chart_region.rs`'s `cyl_sheet`, the latter
with its rim factored into `rim_spec`. **Measured byte-identical** at
equal arguments (`u ∈ [0.2, 1.4] × z ∈ [0, 1]`, canonical unit
cylinder): a sorted dump of every vertex, edge, half-edge, loop, face,
shell, solid, point, curve, surface, surface source and pcurve row
diffed empty, and `census.rs`'s extra `set_face_sense(face, true)` at
the end moves nothing. So they are one construction written twice, and
`chart_region.rs`'s `rim_spec` is the factoring the other lacks.

## Why it is not the door `crates/topo/tests/` now shares

`cyl_wall_sheet` (`crates/topo/src/test_support_fixtures.rs`, PR for
`the-cylindrical-patch-rim-builder-is-written-nine-times`) is the same
Euler sequence but a **different construction**, and the difference is
measurable rather than stylistic:

| | `src` pair | the shared door |
| --- | --- | --- |
| rim plane's surface | `Body::add_surface` | scaffold `mvfs` + `set_face_surface` |
| arena counts | 1 solid, 2 faces, 4 vertices | 3 solids, 4 faces, 6 vertices |

The scaffold exists because `add_surface` is `pub(crate)` and a
`tests/` binary cannot call it, so the two constructions diverged
along the crate boundary, not along intent. Folding the `src` pair
onto the shared door — or the door onto them — therefore **changes
what a suite measures**: `census.rs`'s rows are about solids and
faces. That is a full-tier unit under this program's review posture,
not a declaration-site move, which is why it was dispositioned out of
the folding unit rather than swept into it.

## What a unit here owes

1. Decide which construction is right for both, and say what the
   scaffold solids are worth at the predicate doors (the door's
   rustdoc claims they are inert; nothing measures that).
2. Re-take the count first — `census.rs` and `chart_region.rs` are
   live files and this was measured 2026-09-19.

## Why this row is not on `curved`'s slate

`scripts/work.py territory` reads `crates/topo/src/census.rs` as
`curved`'s. The finding is a duplication between two fixture
builders, which is this program's charter and not `curved`'s; it is
filed here so the class stays with the other members, and `curved`
owns the file whenever it wants the row.
