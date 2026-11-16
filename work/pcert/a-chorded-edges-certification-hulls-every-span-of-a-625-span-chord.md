---
id: a-chorded-edges-certification-hulls-every-span-of-a-625-span-chord
kind: issue
title: Certifying a cylinder pair's chorded edges costs 5.5–7.6 s per boolean in release: the span hulls of 625-span chords
status: closed
closed: 2026-11-11
opened: 2026-11-10
priority: P1
cost: M
refs: [a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses]
---

## What

Filed by SPIRICARM's cylinder-pair cost lane (branch
`spiricarm/cylinder-pair-cost`, 2026-11-10), measuring the P1 row
`a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses`.

A skew or unequal-radius cylinder boolean (the fenced row's poses in
`sweep/tests/verbs_germarms2.rs`, `the_fenced_poses_keep_their_own_outcomes`;
reproduced by `cargo run -p sweep --release --example
cylinder_pair_cost_probe`) carries its section as spline chords of
625–666 cubic Bézier spans each. Certifying those chorded edges is now
the largest single step of the operation. Release, timing spans
(wall, one op each, the box shared):

| op | whole op | `topo::boolean::join::bool_connect` | of which `Body::certify_edge_spec` | `certify_edge_spec` after the join |
|---|---|---|---|---|
| unequal ∪ | 15.8 s | 8.9 s | 5.5 s (16 calls) | 0.7 s (20 calls) |
| unequal ∩ | 15.7 s | 9.4 s | 5.8 s | 0.5 s |
| skew ∪ | 15.4 s | 8.9 s | 7.6 s | 0.6 s |
| skew ∖ | 14.5 s | 7.2 s | 6.1 s | 0.8 s |

`perf` (release, line tables, frame pointers) places it: `certify_edge_spec`
(`crates/topo/src/euler.rs`) → `geom_brep::certify::run_checks` →
`edge_nurbs::analytic_edge_limbs` → `pcurve_cache::twin_span_hulls` /
`spans_of` (through `span_memo::read`) and `limb2_offset_sup` →
`projected::offset_rounds` → `ssi::certify::subdivide_until`; inside them
`geom_core::spline::blossom::span_restrict` (`pyramid_after`, `level`)
takes 29% of all CPU samples and `canonical_composite` /
`to_bezier_spans_extra` 16%. `limb2_offset_sup` hands `twin_span_hulls`
`projected::INCIDENCE_CUTS` = 8 parts per knot span, so a 625-span chord
is restricted 5,000 times per hull read, the count a 1-span net reads
at 8. The rotational volume lane had the same shape (16 pieces a span)
and spreads its count over the spans now; whether the incidence read
can do the same (its spans are already 1/625 of the chord) is the
question this row asks, measured against the 0.3% check of
`projected-check4-incidence-can-falsely-refuse-in-the-sliver-below-the-band`.

**A coarser piece can refuse where a finer one read.** The rotational
lane's first cut at this (PR 6805) spread its pieces as a share of the
span count, and its review found the class: a long span, or one passing
the chart's axis or a pole, read in one piece refused at round 0 where 16
pieces had read. So a cut that spreads `INCIDENCE_CUTS` should be read
off each span's own geometry (its turn, its distance from the axis),
not a share of a count, with rows on a long span and one by the axis.

## Why it matters

The P1 row's operations take 7–17 s each in release; this step is
about 40–50% of each.

## Closed

Branch `pcert/chord-hulls-by-span-geometry`. Limb 2 reads a net first
with each knot span in the parts its own geometry needs
(`pcurve_cache::Parts::ByTurn`): the fewest, a power of two up to
`INCIDENCE_CUTS`, on each of which the part's control box spans at most
`PIECE_TURN` (τ/16, the rotational lane's) times its distance from the
chart's axis. The count search is the rotational lane's own
(`props::taylor::pieces_where`), now shared with its turn constant. A
first reading that clears is the limb's; anything else goes to the
rounds at eight parts a span, so every refusal is read as before.

A 625-span chord's first reading is 625 parts, where it was 5,000, and
every chord of the cylinder pair clears on it. Release, one op a
process (`cylinder_pair_cost_probe`): `certify_edge_spec` 2.2–3.2 s per
op, from 5.4–6.3 s; the ops 9.9–11.5 s, from 11.4–14.3 s. Rows:
`pcurve_cache/by_turn_rows.rs` (a long span, a span by the pole, short
spans far from both, and a first reading that does not clear).

Check 4's projected door reads the same hulls at eight parts a span and
never halves, so it is filed, not cut here:
`check-4s-projected-door-reads-every-span-of-a-chord-in-eight-parts`.
