---
id: a-one-band-sphere-faces-cdt-bulk-load-costs-twenty-times-more-per-point
kind: issue
title: A slab-cut sphere's one-band face costs its trimmed-lane CDT bulk load ~22x more per inserted point than the two-band faces did
status: open
opened: 2026-11-25
priority: P3
cost: M
---



Found by the LATHE one-band lane on `lathe/full-revolve-one-band`
(PR 7414), measuring why `route_rows::a_body_that_meshes_records_mains_k_stream`
crossed the 1 s bar (1.63 s in that PR's `test (app)` job).

## What

The route row's corpus body "ball less a slab turned 0.3"
(`crates/mesh/tests/route_rows.rs:142`, `ball_less_a_tilted_slab(0.3, 0.3)`)
is two faces on the branch and three on its base 84588ee6fa: the
sphere's remainder, two bands on base, is one band now. Its three
tessellations in the row cost 300-350 ms on the branch against ~80 ms
on base; every other body in the row is unchanged.

Callgrind, the row's tessellation calls alone (`--toggle-collect=mesh::tessellate*`):

| | base 84588ee6fa | branch |
|---|---|---|
| all tessellation | 1.73e9 Ir | 7.01e9 Ir |
| `TrimmedFace::attempt` | 4.1e8 (24 calls) | 5.72e9 (18 calls) |
| spade `try_bulk_load_cdt` | 1.49e8 | 5.25e9 |
| `single_bulk_insertion_step` | 14,373 calls, ~8.9e3 Ir each | 27,048 calls, ~1.9e5 Ir each |
| `robust::incircleadapt` | 34.0k calls, 1.10e9 | 34.0k calls, 1.10e9 |

So the points inserted roughly double (one face holds both halves' points),
but each insertion costs ~22x more: 34% of the branch's total sits in a
slice iteration inside `single_bulk_insertion_step`, which reads as
spade's bulk loader walking a long hull per point, quadratic in the
face's point count. The call is `bulk_load_cdt` at
`crates/mesh/src/trimmed.rs:1238`.

## Why it matters

A full revolve's wall is one band now, so every such face meshes through
one CDT of the whole turn. A per-point cost that grows with the hull
makes large one-band faces quadratic where the two-band split kept them
near linear. The route row was moved to the slow set
(`.config/nextest.toml`) with this cause; the cost itself is unfixed.

## Question

Is the hull walk inherent to the point order handed to spade (the
lifted chart's points along the full period, sorted by spade's own
bulk order), so that a different insertion order or chart layout cures
it, or is it spade's bulk loader on this shape of point set?
