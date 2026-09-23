---
id: topo-src-cyl-sheet-is-one-construction-twice-and-not-the-tests-one
kind: issue
title: census.rs and chart_region.rs build the same cylinder sheet twice, and the tests/ door is that construction with a pub(crate) scar
status: open
opened: 2026-09-19
priority: P1
cost: E
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

## Why it was not folded with the `tests/` half

`cyl_wall_sheet` (`crates/topo/src/test_support_fixtures.rs`, PR for
`the-cylindrical-patch-rim-builder-is-written-nine-times`) is **the
same construction**, reached through a door a `tests/` binary cannot
open. Read line by line the two are one thing: same Euler skeleton,
same rim closure with the same `ccw` / reversed-axis / `radial(u1)`
branch, same `Intersection { s1, s2, witness }` at the same midpoint.
One difference, and it is a visibility scar rather than a design:

| | `src` pair | the shared door |
| --- | --- | --- |
| rim plane's surface | `Body::add_surface` (`pub(crate)`) | scaffold `mvfs` + `set_face_surface` |
| arena counts | 1 solid, 2 faces, 4 vertices | 3 solids, 4 faces, 6 vertices |

The scaffold exists because `add_surface` is `pub(crate)` and a
`tests/` binary cannot call it. Nothing else differs.

**The scar is inert to every row that uses the shared door**, measured
2026-09-19: switching the door's rim planes to `add_surface` reds only
the door's own arena row and leaves all 617 `topo` integration rows
green at both lanes. So no suite on the `tests/` side is defending the
scaffolds. What stops the fold being free is the other side —
`census.rs`'s rows are about solids and faces, and removing the scar in
the other direction moves what they count. That is a full-tier unit
under this program's review posture, not a declaration-site move, which
is why it was dispositioned out of the folding unit rather than swept
into it.

## What a unit here owes

1. Decide which of the two doors both sides use. The scaffolds are
   already measured inert on the `tests/` side (above), so the open
   half is what `census.rs`'s solid-and-face rows mean to assert.
2. Re-take the count first — `census.rs` and `chart_region.rs` are
   live files and this was measured 2026-09-19.

## Why this row is not on `curved`'s slate

`scripts/work.py territory` reads `crates/topo/src/census.rs` as
`curved`'s. The finding is a duplication between two fixture
builders, which is this program's charter and not `curved`'s; it is
filed here so the class stays with the other members, and `curved`
owns the file whenever it wants the row.
