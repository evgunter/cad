---
id: the-edge-join-records-no-coincidence-for-the-class-it-joins-on
kind: issue
title: The boolean's edge join makes two edges one on a class reading and records no coincidence
status: closed
opened: 2026-11-04
priority: P1
cost: M
pr: 6512
closed: 2026-11-04
---


## What

D3 `[verdicts.consumers]` names the edge join as a structural act
("Zero acts and is recorded"), and D10 `[coincidence.door]` says "a
door that makes cells one on a class it read (the boolean's edge join)
records it" (Ev on `[ev]` PR 5113, fork-log row 144: "the edge join
records its class Zeros (i agree with the lean)"). The join does not.

`crates/topo/src/boolean/edge_join.rs`, `joinable` (its `locus_of`
closure): a `Chart` edge's and an `OnPair` edge's locus is read off
`geom_brep::chart_iso_family` (`crates/geom-brep/src/pcurve_cache.rs`,
`chart_iso_family`), a class reading that returns the family and drops
the margin that decided it. Where both edges read one family and
`singular_at` reads the vertex regular, `joinable` returns
`join(false)` and `join_all` kills the vertex. `join_stage` writes only
substitution rows into the op's `Descendants`; nothing reaches the
op's coincidences (`boolean::ops::cite_rows`, the rows the
`unproven-coincidence` lint reads through `editor-core`'s
`coincide::name_rows`), so a result whose two edges were made one on
that reading carries no row naming them, and the lint never sees the
coincidence.

Contrast the glue (`boolean::glue::section_door`, its `row` closure)
and the vertex fusion (`boolean::ops::cite_rows`,
`DecisionSite::VertexFusion`), which push a `crate::Coincidence` per
Zero with its cells, relation, site and margin.

## Done when

A boolean whose join makes two edges of its inputs one on an
iso-family reading records one row naming both, at its own site, with
the reading's margin; the in-band reading refuses with the decision's
lever; a definite reading keeps the edges apart.

## Resolution

PR 6512. `geom_brep::chart_iso_zero` reports the Zero a class reading
stands on (the largest of the class gates `derive_class` decided Zero
and the incidence test's margin, which `Incidence::On` now carries);
`chart_iso_family` is unchanged. `edge_join::join_all` hands back a
`ClassZero` for each join made on an iso family, and
`boolean::ops::join_rows` records it at every output path as one
`DecisionSite::IsoJoin` row, `Relation::SameOriented`, naming the two
input edges. A join of two pieces of one input edge is that edge read
twice and records nothing; a join on equal descriptions (`Intersection`,
`TangentIntersection` on one `SurfacePair`) is the pair's locus read
twice and records nothing.

Through today's boolean no join of two different input edges reaches the
iso arm: two faces on one surface key meet along a ruling only on a
closed wall, and continuing one across operands is a closed run the
merge keeps cut by design (sweep README, "Walls"; `PeriodClosure`). So
the record path is pinned at `topo`'s unit level
(`edge_join::class_zero_rows`, `ops::join_row_rows`), and no golden
moves. Done when's second part (the in-band reading refuses with the
decision's lever) is met only for the cylinder ruling, the one class
reading `chart_iso_family` hands the join as `Undecided`: there both
doors end on a lever, the join door's the wrong one
(`the-joins-in-band-class-reading-ends-two-ways`). On a sphere, cone or
torus the in-band gate reaches the join as an image refusal and ends
"no way through yet", no lever at all
(`a-non-ruling-class-reading-in-band-at-the-join-ends-not-yet`).
Residues: `a-join-onto-an-op-minted-edge-records-no-row`,
`the-join-outside-the-boolean-records-no-class-zero`,
`a-non-ruling-class-reading-in-band-at-the-join-ends-not-yet`,
`the-joins-in-band-class-reading-ends-two-ways`.
