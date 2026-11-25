---
id: a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs
kind: issue
title: A curved keel's slit or ring fails tier 3′: its two coincident arcs sit on two curve entries and no record backs the overlap
status: dispatched
opened: 2026-10-19
priority: P1
cost: M
branch: heldrecords/split-on
---


Found by `cleave/keel-slit-ring`, which builds the split's zero-area
ring (`work/cleave/a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section.md`).

## What

A split along a curved keel in the plane gives the side round it two
coincident arcs, floor | wall a and floor | wall b: a slit of the
section's outline where the keel is open at one end (PR 4622), a
zero-area ring where both ends are wrapped. The two arcs are on two
curve entries. Tier 3′ with no records refuses that side with one
`UnrecordedContact { EdgeEdgeOverlap }` naming the two arcs: census pass
5′ (`crates/topo/src/census.rs`, `sweep_curved_edge_edge`) owes a curved
overlap its edge-edge record, and the split returns none. Measured at
the default ε:

- the ring, on PR 5350's first head: both normals, the V's tip at 0
  and `±0.6 ε`. The split now refuses that trace typed instead
  (`SplitJoinError::CurvedSlitTrace`, pinned by
  `crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
  `a_curved_keel_closed_at_both_ends_refuses_as_a_construction_gap`), so
  the ring form builds only once this row is fixed;
- the one-end slit on main: every row of
  `a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`
  refuses the same way on the walls' side.

- the Boolean's twin, on `cleave/boolean-builds-the-slit-form`'s first
  head: a 270° annular slab about `y` less a 150° revolved V whose tip
  rests on its floor, turned 60° (a ring) and −20° (a slit of the
  floor's outline), built both forms at the closed-form volume, tiers
  1, 2 and 3 passing, and 3′ refused one `UnrecordedContact {
  EdgeEdgeOverlap }` naming the two arcs. The Boolean now refuses
  `EdgeInFace { gap: Curved }` before parting instead, with the split's
  words (pinned by
  `crates/sweep/tests/a_curved_keel_on_the_floor_refuses_its_slit_form.rs`).

Straight slits and rings pass: two collinear edges whose bounds are one
point key each are one line by the census's reconstruction rule
(`census.rs` module docs, the D3 rule). Two arcs between two shared
points need not be one arc. The split's door gates at tier 3, so it
returns the one-end slit's sides
(`the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`).

## Owed

One of:
- **One curve entry.** The two arcs share one curve entry (tier 1
  bars sharing for null-scaffold curves only, pass 13), and the census
  reads a curved pair on one entry bounded by shared point keys as
  structural. That extends `crates/topo/README.md`'s "Structural sharing
  (same surface or point key) needs no record" to curve keys, which is
  Ev's.
- **A record.** The split returns its own contact records for the
  pinches it builds.

Neighbour: `work/contacthold/two-coincident-curved-edges-touching-record-no-edge-edge-row.md`
(the boolean's curved edge-edge rows).

Once this lands, lift `CurvedSlitTrace` from `Sweep::section_kind`, and
`EdgeInFaceGap::Curved` from `topo::boolean::slit::part_riders`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: straight slits pass tier 3′ by the reconstruction rule and curved ones have no record: a special case that should be handled uniformly, P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).

## Decided (2026-10-28, Ev on PR 5967)

The slit's two coincident arcs are two cells, each with its own face pair's description, so one curve entry is not an option ([edges], tier 3 check 4). The split records them as an edge–edge pair, citing a new `SplitOn` row (the operand edge ON the plane). The census confirms the record as a curve touch. `CurvedSlitTrace` and `EdgeInFaceGap::Curved` come out once the record lands.

## 2026-10-26 — rows PR 5793 moved onto this gap (CLEAVE)

PR 5793 reads arcs lying in curved faces, so these rows, which built
the one-arc rider form on main, refuse `EdgeInFace` (`Curved`) until the
curved slit form is built; each is owed back then, with the records it
pinned (`sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`):

- `a_tube_through_the_domes_base_builds_every_op_undeclared`, `t ∖ d`:
  census `(7, 12, 8, 1)`, two v-e records of the rim's ends on the
  seam ruling;
- `a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared`,
  `t ∖ d`: censuses `(7, 12, 8, 1)` (two walls) and `(9, 18, 12, 1)`
  (four), records `[v-v, v-f, v-e, e-e]` = `[0, 2, 0, 2]`,
  `[0, 2, 0, 4]` and, on the rulings, `[0, 0, 2, 2]`;
- `a_turned_rim_on_a_wall_bounded_by_an_ellipse_builds_every_op_undeclared`,
  `t ∖ d`: census `(7, 12, 8, 1)`, records `[0, 2, 0, 2]`;
- `a_rod_rim_on_the_dome_across_its_seam_meridian_builds_every_op_undeclared`,
  `d ∖ a`: census `(6, 10, 7, 1)`, two v-f records and one or two e-e
  records at the seam meridian.

## 2026-11-20 — a torus less a disc on a parallel reaches the same gate (SECT)

Measured on `sect/closed-edge-on-torus` (main `d44f46d3`), at the
default ε, 1e-6 and 1e-12: the torus `R = 2, r = 1` revolved about `y`
from two semicircles, less the puck `r = 2.5`, `y ∈ [0, √3/2]`, whose
top rim is the torus's parallel at tube angle `π/3`, refuses
`EdgeInFace { gap: Curved }`, raised by `slit.rs` `part_riders`
(`refused(EdgeInFaceGap::Curved)` after `edge_is_line`). The result is
pinched along the rim: the puck's corner removes a quadrant whose wall
and top both run into the tube there, leaving two wedges of material
(30° and 60° in the meridian half-plane) that meet only along the
circle. The other five ops build `SOUND` at their Pappus forms
(`∩ = 13.381483883`).

## 2026-11-23 — built, and the shared-bounds exemption measured off (`heldrecords/split-on`)

**Built.** The split records every coincident pair of edges it mints
over an operand edge lying in the plane, citing a new `SplitOn` row of
that edge (`Relation::OnCarrier`, the operand edge and the tool, decided
at its two ends' ON verdicts jointly): a slit's two edges, straight or
curved, an area polygon's or a trace's (`splitting::join::Tree::slit`),
and a straight pinch's two copies of an operand line
(`splitting::finish::Sides::minted_contacts`). `CurvedSlitTrace` is gone,
and the ring form and the one-end slit build and pass tier 3′ against
their records, both normals, the tip at 0 and `±0.6 ε`, at the default
ε, 1e-6 and 1e-12
(`crates/sweep/tests/a_curved_keel_in_the_plane.rs`
`a_curved_keel_slit_or_ring_passes_tier_3_prime_on_its_records`). The
Boolean's parting of an arc lying in a plane face mints its second edge
on the arc's own carrier and records the pair, citing the records that
put the arc or its ends on the face; `EdgeInFaceGap::Curved` is gone,
and `Unbacked` refuses an arc nothing recorded on the face. Its twin
builds both forms at the closed-form volume with 3′
(`a_curved_keel_on_the_floor_builds_its_slit_form.rs`), and so do the
closed rims in a plane face (`a_closed_rider_lying_in_a_face.rs`). Arcs
in a curved face now stop at `CurvedFace`
(`work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`),
the rows PR 5793 moved onto this gap among them, and the SECT torus less
a disc on a parallel with them.

**The exemption off.** `census.rs` `ee_collinear_lane` with `recorded`
read from the records alone, the `ci` profile of topo, sweep and
editor-core at the default ε: 27 rows red as the tests stand (topo 18,
sweep 6, editor-core 3; the review's count), 20 with the split's tests
reading each side's own records. None is a pair the split mints: the
editor-core three refuse on their own records too, their operand a
Boolean's unrecorded pair.

- The Boolean's straight lanes (filed:
  `the-booleans-straight-slit-and-pinch-lanes-record-no-edge-edge-pair`):
  `a_face_inside_an_edges_wedge_refuses_at_tier_3_prime` (2),
  `a_figure_eight_hole_cut_through_its_pinch`,
  `a_vertex_read_by_two_sector_passes::an_edge_through_two_blocks_glued_faces_builds_at_rest`,
  the union pinch through `common/touch_union.rs` (3), the bore's slit
  forms (`a_keel_on_a_bores_ruling`, `a_slit_tip_on_a_curved_face…`, red
  as the tests stand), and the splits of a Boolean's unrecorded pair
  (`split_pinch_keeps`, 3).
- The exemption's own pins: `census::tests::a_tip_copy_on_its_own_bit_equal_point_is_an_unrecorded_contact`,
  `m3_pr6_tier3prime` (2), `union_flush_onto_edge_contact::a_pinch_without_its_edge_edge_row_reads_its_shared_ends`.
- Tests that hand no records where the body holds its own:
  `m3_pr3_split` (3, `pseudomanifold_door`); and fixtures built
  unrecorded: `a_doubled_line_does_not_revert`,
  `a_slit_tip_at_a_cone_apex_reads_its_full_turn` (3).

Not reached by that run: the split's curved pinch along a closed keel
(filed: `a-splits-curved-pinch-along-a-closed-keel-records-no-edge-edge-pair`).
A later cut along a recorded slit now refuses `Carry { Parted }`
(`work/heldrecords/a-record-the-split-parts-has-no-consumption-rule.md`).
