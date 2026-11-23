---
id: a-plane-cell-holding-both-sides-of-a-thin-face-is-read-only-below-its-width
kind: issue
title: present_plane read one line or one corner, so a cell holding a thin plane face's whole end (its short side with both neighbours) was read only once split below the face's width - it now reads the end, the end's cap decided before any Vacuous
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
pr: 7334
branch: embed/strip
---


## Finding

Traced by the planar lane of the per-solid census
(`embed/trace-planar`, 2026-11-21), on `embed/gate-measure` at
`1ca743418c` with diagnostic exit lines (`embed/trace-planar-measure`).

`topo::injective`'s `present_plane` (`crates/topo/src/injective.rs`)
presents a plane face's part of a cell only when the cell holds one
line edge, or two line edges that meet at a vertex at a convex corner.
It returns `Unpresented::Local` for two line edges with no common end
(the `[e1, e2]` arm's `let Some(&vtx) = … else`) and for three or more
(the `_ =>` arm). That is right for a cell that a split can still
narrow onto one edge or one corner. On a thin face it cannot.

A face of width `w` holds both of its long sides in every cell that
reaches across it, and its short side with both neighbours in every
cell at that side. `split_by` cuts the metrically longest side
(`Axes::Metric`), so a cell is narrower than `w` only after it has
been cut down to about `w` along the face's length. The two failures
are these:

- **at a short shared edge** (the end of a thin slab), each cell of the
  pair needs about `log₂(L / w)` cuts along `L` before the first one
  across. For `L = 1`, `w = 2·10⁻⁸` that is 26 cuts a cell, 52 for the
  pair, past `max_cell_depth = 40`. The pair refuses
  `Uncertified { arm: Departure }` at an edge.
- **along a long shared edge** (a needle's or sliver's side), every
  piece of the edge splits until it is `w` long. That is `L / w` pairs,
  and the walk refuses `Budget(Pairs { max_cell_pairs: 65536 })` before
  any depth floor.

The departure margin is never reached: no sign is read on a face that
is not presented. So this is not the sliver concession's margin
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`,
PR 7095), though a fix will meet that margin once the strip is
presented. What the presented pair reads is in "Measured and landed,
2026-11-22" below.

## Trace

Cheapest: `editor-core::all m10_3_r1_probes_interval::a_one_leaf_budget_still_covers_the_box_exactly`
(0.09 s). It is a 1 × 1 slab `2·10⁻⁸` thick, 20 ε at ε = 10⁻⁹. The
D10 rows (`sliver_shell_role::a_thin_wall_refuses_on_a_pair_below_k_eps_and_builds_above_it`)
build walls this thin, and so does every op the family drives. The gate
refuses
`Uncertified { a: 3v1, b: 4v1, shared: Edge(6v1), arm: Departure }`.
Face 3 is the side `y = 0` and face 4 the side `x = 1`, and edge 6 is
their common vertical edge, `2·10⁻⁸` long. All 41 asks of the pair exit
at the `_ =>` arm on both faces, with three edges in each cell
(`xl = [0, 2, 3]`: the shared edge and the slab's top and bottom
edges). The finest cells are `u ∈ (0.4999990, 0.5)` by the whole
thickness `v ∈ (−10⁻⁸, 10⁻⁸)`. The metric split has not yet cut across.

Second: `sweep::all review_arm_vs_reach_probes::review_lengths` has a
step `3·10⁻⁷` wide. It refuses the same way, with cells `1.2·10⁻⁴` by
`3·10⁻⁷`.

## Count

On the census of main `c2a0c41f40`, **departure, edge, plane/plane:
40 of the 111 traced** (of 116). The refusing pair's cells hold three
edges on both faces at a width below 10⁻⁶ (10⁻⁸ to 3·10⁻⁷). The
families are the `m10_*_interval` drivers, `shell_role_reader`,
`missed_ray_side`, `dsc_checks`, `sliver_shell_role`,
`review_arm_vs_reach_probes`, `a_long_bound_beside_a_short_arm` and
`fillet_h6_cap_rim`. Ten more refuse with the same three-edge exit
beside a thin curved shoulder or strut, `10⁻⁵` to `2·10⁻⁶` wide:
`wedge_end_doors` (5), `review_must_carry_rule_r*_probes` (2),
`a_short_shallow_strut_between_wide_walls_is_a_corner` (2) and
`wedge_end_doors::a_cut_across_a_tangent_seam_describes_each_piece_by_the_rule`.
Their other face exits elsewhere (`leaves_line`'s corner departure,
`decide`d negative at the vertex), so they are probably this cause
with a second one. Unchecked.

**`Budget(Pairs)`: 24 of the 98 traced** (of 124). Here the pair
budget is spent on two to four touching pairs of plane faces of a
needle, a sliver, a rod or a strut. Each runs 20,000 to 39,000 asks
that exit both faces with two line edges and no common end (the
`[e1, e2]` arm's `vtx` exit, in 8 traced tests) or past a short edge's
end (the span exit, `sliver_shell_role`'s needle). The other 16 have
the same pair counts as a traced test from their family. The cheapest
is `topo::all review_m3_pr3_consumer::tiny_real_sliver_not_wrongly_refused`
(183,590 exits at the `vtx` exit in one run). The families are
`shell_role_reader`'s needles (4), `contact9_side_codes` (6),
`germ_torus_doors`' rods (5), `sliver_shell_role` (2),
`a_corner_reads_only_its_own_sectors`, `must_carry_rule`,
`review_m4_pr2_transform`, `review_fillet_h6_r1_probes`,
`refusal_concision_chains` and `band_closed_mixed_chain_shapes`. A
rod's face is not ε-thin. `germ_torus_doors::a_rod_inside_the_tube_is_not_passed_silently`
reads cells 3·10⁻³ by 4·10⁻³ on a face about 1.3·10⁻³ wide, so what
matters is the face's width against its length, not against ε.

**`Budget(Depth)`, probably.** In 3 of the 10 depth-bucket tests re-run,
the deepest touching pairs exit at the same two arms. One is
`band_one_cycle_members::a_cup_with_a_handle_and_a_u_tube_refuse_wholly_inside`,
with 573 of 828 asks at the `vtx` exit. The census reads
`Budget(Depth)` there because a distance pair beside them reaches the
floor first. Not checked by a flip.

## What a fix needs

A planar presentation of a strip: the cell clipped to the face between
two line edges across it from each other (parallel or not), or at a
short edge between its two neighbours. The face there is an intersection of half-planes, convex,
so the same affine reading serves, with the shared side on `y = 0` and
the far lines as the rectangle's bounds. `present_chart`'s clip sides
already read several lines on one axis (`run_on`,
`sides_are_the_faces`). The plane's own arm is the one that does not.

## Measured and landed, 2026-11-22

`embed/strip`, PR 7334. `present_plane` reads one thing: the face's
part of the cell where the face is convex there, the intersection of
the half-planes of the line edges the cell holds. It is read inside a
parallelogram two of whose sides lie on one edge or on a convex
corner's two edges. The cases:
- one edge;
- a corner;
- **a strip's end**: an edge with a neighbour at each end, read at
  either corner whose other neighbour (the cap) is not shared. The cap
  bounds the far side at the highest its line stands over the cell's
  reach.

Convexity is decided by the leans alone. Each corner in the cell is
convex, and the cap and the edge it caps stand on each other's face
side over the reach, at `SELF_INTERSECTION_CORNER`, read linearly in
the base's own frame. **Each reading decides that convexity before it
may answer `Vacuous`.** A frame's "the cell is off my face side" is
true only where nothing else in the cell bounds the face. The review
of PR 7334 found it read first: a coplanar fold past a slot's far side
certified (`a_face_laid_over_a_slots_far_side_is_not_certified`).

**The strip arm was built, measured at zero, and removed.** PR 7334
first also read two line edges with no common end (a strip's middle),
and a wedge whose tip was clear of the cell, as a strip capped at the
far side. It held the review's first two probes of the `Vacuous` hole.
Measured through the gate on the 42 census tests that hold:

| mutant | what is off | the 42 | body rows (slab, needle) |
|---|---|---|---|
| M11 | the strip arm (two lines with no common end, the wedge fallback) | 42 pass | green |
| M11b | the end arm (three lines) | 4 pass, 38 fail | red |

The four that pass under M11b pass under M11 too: the end arm alone
carries them. So no measured test needs the strip arm. Two facts make
that structural:
- a thin rectangle's first split leaves two halves that each hold a
  whole end;
- of two plane faces along an edge, only one needs presenting, because
  the other's carrier separates them; a thin face whose partner reads
  at its own end is never asked across its middle.

A chamfered hexagon prism, built to need it, did not (its side
rectangles read at their ends). **What would bring the strip arm back:**
a body with two thin plane faces along one edge, neither with a
single-edge end within reach of the cells along that edge, which
refuses with the strip arm off. Rebuild it only with that body as its
pin. `a_strip_middle_is_not_read` pins its absence.

**The census tests of this cause** (61 of the census of `c2a0c41f40`,
picked by the families above from the 116 departure/edge/plane-plane
and 124 `Budget(Pairs)` rows, run through the gate on
`embed/strip-measure`). Before: 0 pass, 37 `Uncertified { Departure }`
at an edge and 24 `Budget(Pairs)`. After: 39 pass and 22 fail. What the
22 read now, per `STRIP_TRACE` (every unpresented plane exit and every
departure margin the band does not decide):

- **In band by construction (6)**, the pair presented and its own
  width below Kε:
  - `sliver_shell_role`'s needle and wedge (2, `Budget(Pairs)`): the
    wedge's far end is 8.4e-8 wide, so cells near its tip read the
    strip's width in band. One presented pair's margin is −9.70e-9:
    rate −2.12 times extent 4.57e-9, the strip's own reach across.
  - `sliver_shell_role::a_thin_wall_refuses_on_a_pair_below_k_eps_and_builds_above_it` (1).
  - `missed_ray_side` (3): slabs 10ε (= Kε) thick, whose cap leans at
    the end read the width at the band.
- **contact9's dipping chords (4)**: the strip arm is reached, and
  its leans or the departure read widths of 1e-16 to 1e-15. One
  margin is −4.5e-15: rate −9.33 times extent 4.8e-16. These are
  near-coincident faces by construction. One of them,
  `a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`,
  moved from `Budget(Pairs)` to `Crosses` at a station pair 4.8e-10
  apart, below ε. That is a distance verdict, not a reading.
- **Elsewhere (10)**:
  - `a_long_bound_beside_a_short_arm` (3, depth and vertex) and
    contact9's `a_near_coincident_pair_at_a_corner_refuses_typed`:
    their undecided margins have rate 0 (coplanar faces, a
    separator that does not depart). Their unpresented exits are
    mostly the corner's lean.
  - `review_fillet_h6_r1_probes` (corner lean).
  - `fillet_h6_cap_rim`, `germ_torus_doors::subtract_and_intersect…`
    and `band_closed_mixed_chain_shapes`: the undecided margins
    are on curved faces.
  - `review_arm_vs_reach_probes::review_tilted` and
    `shell_role_reader::a_needle_channel…` (depth, four-edge plane
    cells).
- **Not the gate (2)**:
  `a_corner_reads_only_its_own_sectors::an_unglued_pair_parallel_at_the_arm_refuses_on_the_arms_tilt`
  and `m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`
  fail with the gate off, and the first fails on main as well.

So 6 of the 22 stop at a width in the band, the re-metering's ground,
and none of the 39 that then held needed the margin changed. (These
figures are the first head's; the fix pass's are below.)

**Not checked here**: the ten thin curved shoulder and strut rows
(`wedge_end_doors` and the rest). They are not in the 61.

**After the review's fix pass** (the merged head, on
`embed/strip-measure`), 42 of the 61 pass:
- `missed_ray_side`'s three slabs (10ε = Kε thick) now hold. The cap's
  lean over the base no longer picks up the reach's slack per chart
  coordinate.
- The ∩ of `contact9_side_codes::a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`,
  a wedge whose two faces share a 1 mm edge and part at 5e-8 rad (so
  stay within ε of each other for about 20 mm), reads `Embedded`. The
  verdict door read `Crosses` at the 4.78e-10 station pair; before this
  unit the gate refused it on the pair budget. The departure that
  certifies it multiplies the faces' 5e-8 rad parting by the cell's
  reach, the margin PR 7095 re-meters.
- The test still fails, on its − (`Departure` at an edge).

**Last pass (re-review of `a54717c96`, approve with fixes).**
- The end's cap reads one lean: the base's points on the cap's face
  side over a reach from the corner. Two checks went as implied, each
  after its mutant survived:
  - the cap's rise: its height is 1 at the corner, and the lean keeps
    its line off the base;
  - the run check: an undecided height bounds nothing, and the far side
    is then the cell's reach.
- A structural refusal from either corner stands.
- Rows pin the cap's band, the reach starting at the corner, and a
  structural end.
- The reviewer's fuzz of three-edge chains is a counterexample search
  in `injective`'s rows (`a_presented_end_holds_the_faces_part_at_random`).
  At effort 15 with seed `0x1234` it finds failures for three mutants:
  the cap's lean dropped, a reach from the cell instead of the corner,
  and `Vacuous` read before the cap. It does not catch a looser cap,
  which is still sound. 56,545 cells at 1e-9 in the reviewer's run
  read 578 three-edge cells `Vacuous` where main read every
  three-edge cell `Local`, and all 578 are sound.


**Landing pass (merged with `embed/wave3-land` `8d34076a9e`).**
- The 61 census tests of this cause were run through the gate on this
  unit's final `present_plane` (`embed/strip-measure` `25095ebaff`):
  42 pass and 19 fail, each test's verdict identical to the reviewed
  fix pass's. This replaces the argued claim that the last pass's
  removals cannot move them.
- `contact9_side_codes`' sector-sliver meet read `Budget(Pairs)` with
  its millimetre faces unread. This unit presents the wall's end, so it
  now reads `Embedded`, an in-band pair certified by the reach lever.
  That is not the ratified reading (`[topology.tiers]` levers at the
  narrower face's run). It is re-pinned at what it reads, as
  `the_sector_slivers_meet_is_certified_by_the_reach_lever_once_its_faces_are_read`,
  and `the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict`
  owes the flip to the refusal.
