---
id: a-discard-names-a-bordered-stretch-by-its-two-end-vertices
kind: issue
title: A discard names a bordered or held stretch by its two end vertices, so a stretch whose ends several result edges join is credited to all of them
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [4777]
---


## What

Found by PR 4777's dual review (FLUSH, branch `flush/pinch-residue`),
both reviewers. `topo::DiscardRow::bordered` and `::held`
(`crates/topo/src/boolean/discard.rs`) record each stretch as its two
end vertices, in result keys before the zip's fusions. Readers settle
the ends through `BooleanNaming::fused_into` and look the edge up by its
end pair: `crates/editor-core/src/names/borders.rs` (`by_ends` /
`any_by_ends`, where it pushes every edge on the pair into the
discard's seams) and `crates/topo/tests/boolean_discards.rs`
`bordered_edges` (a map keyed by end pair, so the last edge wins).

Where one pair of result vertices is joined by more than one edge (a
face passing a pinch vertex twice: an arc and a line between the same
two points), the stretch names every edge on the pair.
PR 4777's `finish::twin_copy` reads the kept copy off its place on the
section face's twin, then hands it on as a vertex pair, so the defect
is one layer down from the one it fixed.

Measured on PR 4777's head (release, `curved_notch_pinch`): 9 of the 15
bodies that PR builds have 4 bordered stretches each on an end pair more
than one result edge joins (`0 3 2 ab S`/`ba S`, `1 6 0 ab S`/`ba S`,
`3 1 0 ab S`/`ba S`, `6 1 0 ab I`/`ba U`); the other six and the older
pinned poses have none. Pinned at `3 1 0 ba S` (4) in
`curved_notch_pinch::a_curved_notch_at_a_corner_pinch_builds_bodies_the_census_cannot_read_yet`.

## Why it is not a record-type change

The record would carry the edge (or half-edge), but the edge it knows
is a pre-zip key: the twin's half-edge, which the zip either keeps or
kills into the A seam edge (`ZipReport::edge_merges`). `BooleanNaming`
carries the zip's vertex fusions (`vertex_merges`) but not its edge
merges, and the output stage's joins move edges again
(`stretch_through_joins`). So either the naming gains the zip's edge
merges, or topo settles the edge itself before it emits the row (through
`Descendants`, as the contact carry does). That is a choice about the
naming's shape, not a mechanical change of one record's type.
