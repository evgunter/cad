---
id: the-edge-join-records-no-coincidence-for-the-class-it-joins-on
kind: issue
title: The boolean's edge join makes two edges one on a class reading and records no coincidence
status: open
opened: 2026-11-04
priority: P1
cost: M
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
