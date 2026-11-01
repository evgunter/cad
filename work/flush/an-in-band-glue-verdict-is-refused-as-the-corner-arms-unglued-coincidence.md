---
id: an-in-band-glue-verdict-is-refused-as-the-corner-arms-unglued-coincidence
kind: issue
title: A pair the glue door reads in band is refused at the corner arm as an unglued coincidence, naming the corner instead of the in-band glue verdict
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: flush/corner-names-glue
pr: 5008
closed: 2026-10-14
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

## Measured (`flush/corner-names-glue`, 2026-10-13, main at 4dce8298)

Every witness refuses at an unglued corner that reads the pair one
carrier, on the corner's own offset margin, which no tolerance clears:

- **a7 ε/16** (`docm2_part_interval.rs`) refuses at `recl.rs`
  `require_same` (`ReadOn::Faces`): the corner's offset is the enclosure
  ±2.5e-10, its first sector's bounds read `[0, 2.5e-10]` off the
  other's plane (no size), and the glue door read the cut faces'
  `bool_plane_offset` at ±1.5e-9 (an enclosure across zero). The rung
  that clears it is ε/32, where the door's enclosure falls in the zero
  band and glues the pair.
- **The 20 `k = 0.5` poses** (`wedge_flush_review_probes.rs`, θ =
  5e-10) refuse at `vtxfac.rs`' coplanar lump: the sector's tilt reads
  Zero at its span (1), so the carrier ladder reads one plane with the
  offset a rounding residue (±1.1e-16, 5.2e-26, 0). The glue door reads
  `bool_plane_parallel` 1.00 to 1.52e-9 (`bool_plane_reach` 1.30 to
  1.34e-9 on the tight block at 60° and 90°). The sector's bounds stand
  off the block's plane by θ·sin φ (1.745e-11, 4.358e-11, 1.710e-10,
  4.330e-10, 5.000e-10 at φ = 2°, 5°, 20°, 60°, 90°).
- **ε ladder** on all 20 (θ fixed at 5e-10; ε 2e-10 down to 5e-13): just
  under the glue door's offer (1.2e-10) every op still refuses at the
  corner, `Coincidence(Sectors)` on θ·sin φ; then `VertexOnFace` on the
  same margin while ε > θ·sin φ / K; standing poses build once ε is
  below θ·sin φ / K, sunk ones meet `CrossingInsertion` (and one sunk
  90° op a `Merge`) with their own offers. So the binding reading is the
  bounds', not the door's and not the corner's offset.
- **The wedge-flush pin** (`the_in_band_wedge_is_in_band_at_the_glue_door`,
  k = 2) already refuses on the bounds (its tilt is in band, PR 4709's
  branch), and is unchanged.

## Fix

The glue door keeps what it read of each pair it leaves in band
(`Glue::in_band`, `glue.rs`), and a corner that reads an unglued pair
one carrier refuses through `boolean::unglued_corner` (`mod.rs`, called
from `vtxfac.rs` and `recl.rs`): the sector's bounds' reading
(`BoolSector::bounds_off`, `sectors.rs`, the one home of the steeper
bound's departure) where a tolerance a run admits decides it, else the
door's in-band reading, else the corner's own. The door's reading never
offers a tolerance (a margin in band shows no size), so the two are
never weighed against each other. One admission rule
(`boolean::sectors_read`) serves every `Coincide::Sectors` refusal. a7 now names the
door's `bool_plane_offset` (pin moved); the wedge poses name the bounds,
and `offer_rows`' `flat_wedge_*` rows execute the offer (true at every
opening, T1 standing, T2 sunk past a true `CrossingInsertion` offer).

## Found beside it (the review of PR 5008)

A pose whose pair glues refuses its union `ResultVolumeImplausible` on
rounding-sized excess. That is CLEAVE's
`a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop`
shape, but that row lives only on unmerged branches (`flush/given-gate`,
`contacthold/fold-step-face-face-row`). So the evidence is recorded here,
for whoever lands the row to carry it over.

- Standing wedge, block `[0,3] × [-2,2.5] × [0,1]`, wedge lengths 1,
  measured on `e0fed17b`.
- At ε 1e-9, θ 2e-12, every φ ∈ {2, 5, 20, 60, 90}° refuses in both
  orders (`got` 14.366025403784464 against `bound` 14.36602540378444 at
  60°; 14.500000000000034 against 14.5 at 90°).
- At ε 2e-11, θ 5e-12 it refuses the same way (got minus bound 5e-15 to
  8e-14).
- The glue closes the wedge face's Zero-band tilt onto the block's top,
  and the union backstop (`volume_backstop`, `ops.rs`) has no slack for
  the volume that closes.
