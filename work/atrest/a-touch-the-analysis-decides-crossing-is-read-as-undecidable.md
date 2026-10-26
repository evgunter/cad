---
id: a-touch-the-analysis-decides-crossing-is-read-as-undecidable
kind: issue
title: A touch the analysis decides is a crossing (the materials' interiors meet there) is reported as undecidable, not as interference
status: open
opened: 2026-10-26
priority: P1
cost: M
refs: [two-copies-filling-one-volume-certify-at-rest]
---


Found by `two-copies-filling-one-volume-certify-at-rest`'s sweep (refs
that row) for other paths that report a real overlap as
`CensusUndecidable`.

## What

The touch analysis (`census.rs` `rest::judge`, `TouchVerdict`) has a
DECIDED verdict `TouchVerdict::Crossing`: "the two materials' interiors
meet at the touch, on decided readings". Interiors are open, so that
is an overlap of positive volume. But `TouchVerdict::refusal` maps it
to `Undecided::MixedTouch`, and arm 2 of `sweep_cross_solid_backstop`
reports it through `blocks` as `CensusUndecidable` ("one passes into
the other where they touch"). `census_interference_solids` does not
read that, so between two copies at rest the at-rest door treats a
decided overlap as could-not-look. Since stage 5 C that means it never
refuses at `Error`, and no assertion can say the overlap is intended.

The planar coincident subset of this path (copies whose faces lie on
one plane facing the same way and share interior) is now decided
before `blocks` runs, as `SameSideInterference`. The rest is what
`MixedTouch` still carries: an overlap with no vertex strictly inside,
no pierce, no same-side crossing and no aligned coplanar face pair,
whose crossing shows only at a touch. One witness in the tree is the
pyramid pair `a_vertex_read_by_two_sector_passes.rs` admits ("a pyramid
notched by another", its `three_prime` allowlist).

## What closing it takes

Report a decided `Crossing` touch between two solids as interference
that `census_interference_solids` reads, naming the touch's two cells,
and keep `MixedTouch` for nothing else, or retire it. Check first that
`Crossing` is never minted from an in-band reading (its doc says
decided readings only).
