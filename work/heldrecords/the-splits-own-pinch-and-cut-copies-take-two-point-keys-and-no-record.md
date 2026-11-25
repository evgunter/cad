---
id: the-splits-own-pinch-and-cut-copies-take-two-point-keys-and-no-record
kind: issue
title: The split's own pinch and cut copies at one point take two point keys and no contact record, so tier 3′ refuses sound sides
status: review
opened: 2026-10-28
priority: P1
cost: M
branch: heldrecords/split-keys
pr: 7421
---


Found by the tier-3′ gate measurement (`splitdoor/split-gate-3prime`, 2026-10-28), cause B. It is a blocker of `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`.

## What

Where the split pinches, or two cut copies meet at one point, the copies get two point keys. That contradicts `split`'s doc ("copies share its point key"), and the split returns no record of the touch. So tier 3′ refuses sides whose volumes are right.

- **Straight:** payload `VertexVertex` + `EdgeEdgeOverlap`.
  - `a_keel_wrapped_at_both_ends_splits_as_a_ring::a_later_cut_across_the_ring_builds_and_so_does_one_through_its_tip`: the cut at x = 2 mints 28v1 and 29v1, both at (2, 2, 0).
  - editor-core `split_pinch_keeps::a_moat_reaching_the_block_walls_at_its_corners_quarters_both_sections`.
  - editor-core `split_pinch_keeps::a_diamond_moat_reaching_the_block_wall_…`.
- **Curved:** one `EdgeEdgeOverlap` on the rim circle r = 2, plus six `CensusUndecidable MayOverlap`. Every curved instance comes from one revolved-apex split:
  - `split_pinch_keeps::{a_split_through_a_revolved_apex_keeps_its_own_rim, a_revolved_pentagons_lower_top_is_its_two_sections_merged, a_lower_half_on_the_plane_takes_a_box_hole}`;
  - `name_invariance::{a_far_feature_moves_no_other_name, a_split_planes_frame_moves_no_name}`.

The curved slit (`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`) is this cause's curved instance.

## Direction

Decide whether the copies share one key (as the doc says) or the split returns the touch as a record; the doc and the code disagree today.

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).

## 2026-11-22 — crossings on one segment share a key (`heldrecords/split-keys`)

**The framing, checked against the tree.** Pinch copies already share
the point. Every straight failure is two crossings the cut mints on two
line edges ending on one pair of point keys: a ring's or slit's two
copies, or a moat corner touching a wall. In every case the operand
holds no record (0 records on every side measured), and its two edges
pass the census only through `ee_collinear_lane`'s shared-bounds rule.
Once the cut gives each crossing its own key, the pair halves share one
bound and not the other, so the side refuses `VertexVertex` and
`EdgeEdgeOverlap`.

**Fix.** `splitting/classify.rs` `insert_crossings`: a line edge whose two
ends' point keys match an edge already crossed lands its crossing on that
crossing's point (`split_edge_onto`) and key (`share_point`). Two line
segments between one pair of points are one segment, so this reads
nothing. That is the same structural sharing as an op's copies of one
vertex (`crates/topo/README.md` tier 3′).

"No record is owed" holds only while `census.rs` `ee_collinear_lane`
exempts shared bounds. Each half-pair ends on one key at both bounds, and
that exemption is what passes it. When step 3 retires the exemption, it
owes the operand's `EdgeEdge` record of the pair, carried through the
split's lineage onto both half-pairs. Once the producers record these
pairs, the split's carry has to land that record on both.

**Each case, tier 3′ against the side's own records.** Measured by a
probe in `split`, at ε = 1e-9.

| case | before | after |
|---|---|---|
| keel ring, cut at x = 2 (`a_later_cut_across_…`) | both sides refuse: VV (28v1, 29v1) at (2, 2, 0) + EEOverlap | both Ok |
| `split_pinch_keeps::a_moat_reaching_the_block_walls_…` | above 4 VV + 4 EEOverlap; below 4 VV + 4 EEOverlap | both Ok |
| `split_pinch_keeps::a_diamond_moat_reaching_the_block_wall_…` | above VV + EEOverlap; below VV + EEOverlap | both Ok |
| the five curved tests (`split_pinch_keeps` ×3, `name_invariance` ×2) | 12 of 104 sides refuse: one EEOverlap on the rim r = 2 + `CensusUndecidable` (curved faces within reach) | unchanged, 12 of 104 |

**Curved: what this covers of step 3.** Nothing directly. The rim's
`EdgeEdgeOverlap` is between two arcs the split mints itself, not two
crossings, so it is step 3's `SplitOn` record. The conic crossing lane
still mints a key per crossing. Once step 3 records curved coincident
arcs, a later cut across them is this door's curved instance. Filed as
`a-cut-across-two-curved-or-partly-overlapping-edges-on-one-locus-mints-two-point-keys`,
with the straight recorded-but-partly-overlapping pair.
