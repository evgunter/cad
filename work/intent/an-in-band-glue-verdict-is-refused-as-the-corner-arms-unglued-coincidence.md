---
id: an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence
kind: issue
title: A pair the glue door reads in band is refused at the corner arm as an unglued coincidence, naming the corner instead of the in-band glue verdict
status: open
opened: 2026-10-11
priority: P2
cost: M
---


Found by stage 4 F (`declared-pairs-retire`), on the rung a declared
`Rest` used to bridge.

## What

`crates/editor-core/tests/docm2_part_interval.rs`,
`a7_the_width_ladder_of_the_split_of_a_widened_box`, at the ε/16 rung:
the corpus `part_select` document at the `Interval` scalar, with the
split body's height widened by ε/16. The union of the split's two halves
refuses
`Escalated{Coincidence(Carriers, Settles(Rest))}` with predicate
`carrier_unglued_coincidence` (`crates/topo/src/boolean/recl.rs`, the
corner arm), carrying the enclosure `[0, 0]`.

The cause is the glue door, not the corner. The glue door
(`boolean::glue::decided_declarations`) reads the cut faces' plane
offset at `[−1.5e-9, 1.5e-9]` (`bool_plane_offset`, among the
refusal's escalations), inside the sliver band (zero `1e-9`). It leaves
the pair unglued, which D10 says it must: the band refuses. The corner
then reads the pair one carrier over its own arm, finds it unglued, and
refuses with that reason. So the refusal names a Zero reading at the
corner when the deciding fact is an in-band margin over the faces'
extent: the user is told the pair is coincident and unglued, not that
its coincidence is too close to call.

## Owed

Have a pair the glue door leaves in band refuse with the glue door's
reading (`Coincide::Carriers` with the in-band margin, the geometry and
the tolerance as recourse) wherever a later arm meets it. Do not have
the corner arm report an unglued Zero for such a pair. The a7 ε/16 rung
is the witness, pinned as `Rung::Escalates { node: union, predicate:
"carrier_unglued_coincidence" }`, and that pin moves with the fix.

## Evidence from FLUSH (`flush/wedge-flush-reads-zero`, 2026-10-11)

FLUSH's
`work/flush/a-wedge-flush-on-a-block-top-refuses-every-op-undeclared-though-its-face-reads-zero`
is the same structure: the glue door reads a planar pair in band over
both faces (`bool_plane_parallel` 3.25ε to 5.42ε) and leaves it unglued,
and the corner then refuses with its own reading: `Coincidence(Sectors)`
from `vtxfac.rs`'s coplanar lump (`bool_sector_coplanar`), quoting the
wedge's bounds' Zero margin. There, the corner's reading is the one a
smaller tolerance has to clear. With the geometry fixed, the ops still
refuse at the corner just under the offer the glue door's margin would
make (≈5.4e-10). Just under the corner's offer (1.7e-11) they build or
meet a refusal with its own offer. So "refuse with the glue door's
reading wherever a later arm meets the pair" would quote a false offer
on those poses (pinned: `offer_rows`'
`the_in_band_wedge_is_in_band_at_the_glue_door`).

The same family also reaches this row's own shape. In the ignored grid
`crates/topo/tests/wedge_flush_review_probes.rs` (`probe_other_poses`,
ε 1e-9), every pose at `k = 0.5` (θ = 5e-10) refuses all six ops
`Coincidence(Carriers)` (`carrier_unglued_coincidence`) while the glue
door reads the pair in band: 20 poses, standing and sunk, φ ∈ {2°, 5°,
20°, 60°, 90°}, big and tight blocks, wedge length 1. The quoted margins
are rounding residue, or exact zero:

- ±1.110e-16 at φ = 2°, 5° and 20°;
- 5.170e-26 and 2.585e-26 at φ = 60° and 90°;
- exactly 0 at φ = 90° standing.

The glue door reads `bool_plane_parallel` 1.04 to 1.36ε, or on the
tight block at 60° and 90° `bool_plane_reach` (`Undecided`, 1.30ε at
60°). A rule keyed on the corner's margin being exactly Zero misses all
but one of these. Quoting the smaller of the two offers would quote
about 5e-27 m, which is true but useless.
