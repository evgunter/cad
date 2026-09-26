# CONTACT-5 — the backstop clears a meeting pair only through the touch analysis

Carries `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`
(P0) and `a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses`
(P0). Files `declared-only-meetings-clear-at-the-census-gate-unread` (P1).

## The logical change

**Arm 2's box gate answers containment and nothing else.** Before, a
pair whose two orderings both separated on the box was cleared before
any finding between the two solids was read. Now
`sweep_cross_solid_backstop` asks first whether the two boundaries
meet — any standing finding with one entity on each solid (`meets`).

- No finding between them: the gate clears, as before.
- Findings, and a box that could contain the other: the material
  probe runs in both orderings, then `blocks`, as before.
- Findings, both orderings separated: `blocks` alone decides. Any
  meeting that is not a rest refuses with its reason; an all-rest pair
  clears with no probe.

The site states why no probe is needed. If every meeting is a rest,
`U = int A ∩ int B` has no meeting point in its closure. So `∂U` splits
into parts of `∂A` inside `B` and parts of `∂B` inside `A`, each open
and closed, and each part is a union of whole shells. Filling the voids
reduces this to the outer shells: one solid's outer shell sits inside
the other's filled region, hence inside its reach box, so that ordering
would not have separated. The argument assumes the census is complete.
An escalation breaks that, and so does a declared-only meeting (the
row below).

**A crossing of two edges is a touch site.** `TouchSite::EdgeCross(a, b)`
maps `CensusContact::EdgeEdgeCross` through the same `of / entities /
verdict` path as every other kind. Its site is the crossing point
(`ee_cross_point`, the one formula the crossing lane now shares). Its
cones are the two edges' dihedral wedges there. `Cone::wedge` now
takes the touch point and levers its faces at it: the crossing point
for a cross, and the vertex for a vertex-on-edge (before, the edge's
start). A cross of two edges of ONE of the pair's solids still blocks
as `Crossing`. A cross between one of the pair and a third solid no
longer blocks the pair; it is that other pair's touch, like every
other touch kind.

**"Coplanar" is decided where the finding is made.** An `EdgeEdgeCross`
is pushed only after `pm_census_ee_gap` decides that the two lines
meet, and it escalates in band there. So every cross finding's edges
span one plane. There is no "non-coplanar" cross for the site to screen
out. A cross whose materials pass into each other reads as a crossing
through its wedges (the plus-shaped slabs below). A cross with no
shared face plane, such as two ridges crossed edge on edge, is a real
rest, and the wedge test decides it exactly.

## Base vs head

"Refused" means a placement finding on the solid pair. The undeclared
touches stand as findings in every row.

| pose | base | head |
|---|---|---|
| half-overlapping cubes `[0,2]³`, `[1,3]×[0,2]²` | cleared at the gate (wrong) | refused `MixedTouch` |
| two cubes side by side, face to face | cleared | cleared (8 rests read) |
| beam across two supports (the row's pose) | cleared at the gate | cleared through the analysis (8 edge crosses, all rests) |
| same beam sunk 1 mm | cleared at the gate (pierces stand) | refused `Crossing` ×2 |
| beam tilted in band about one bottom edge | cleared at the gate (escalations stand) | refused `Unexamined` ×2 |
| two ridges crossed edge on edge | cleared | cleared |
| same ridges sunk 1 cm | cleared at the gate | refused `Crossing` |
| two interpenetrating cubes (`m3_pr6_tier3prime`) | cleared at the gate | refused `Crossing` |
| plus of two slabs (`review_m3_pr6::r2_coplanar_plus_overlap_detected`) | cleared at the gate | refused `MixedTouch`, read at its coplanar edge crosses |
| obtuse-sector part dipping 2–30·zero into the floor, 9 poses | all cleared at the gate (wrong) | all refused |
| brick grid, 1000 poses (below) | 422 wrong clears, 0 false refusals | 0 wrong clears, 0 false refusals |

The beam row's premise did not reproduce end to end. At the base the
beam pose is gate-separated, so it cleared before `blocks` read its
crosses. The refusal the row describes is what `blocks` would have done
had it run. The pose now clears for the right reason, and its sunk and
tilted variants refuse.

Moved rows:

- `bool4r2_probes::two_half_overlapping_cubes_are_cleared_at_the_gate` →
  `two_half_overlapping_cubes_refuse_as_a_mixed_touch`.
- `m3_pr6_tier3prime::hand_built_self_intersection_is_undeclared` and
  `review_m3_pr6::r2_coplanar_plus_overlap_detected` now expect the
  pair's one refusal beside the findings.
- `editor-core` `docm6_seam_declarations::unattributed_is_only_a_finding_no_declaration_answers_for`:
  the penetrating seat now carries the instance arm's `MixedTouch`, a
  solid-pair refusal no declaration answers for. The allowlist admits
  it.
- `editor-core` `perf12_census_goldens`, re-blessed at all three ε
  rows. `kitchen_sink` gains four `Unexamined` solid-pair refusals and
  `cut_cylinder` gains one. Those pairs meet and are separated at the
  box, and arm-1 findings stand naming their solids. Both bodies
  already refused. This is the census deciding more of the pairs it
  used to clear unread.

## The sweep (`contact5_gate_and_beam::a_grid_of_brick_pairs_clears_exactly_the_rests`)

The cube `[0,2]³` is tested against every brick whose sides run
between the grid values `−1, 0, 1, 2, 3` on each axis. That gives 1000
poses with coplanar faces, collinear edges and in-plane crosses
wherever the grid lines coincide. For bricks, ground truth is exact:
the materials overlap iff the intervals overlap in positive length on
all three axes. This equals grid sampling of the shared interior with
no sampling error. 512 poses overlap and 488 are rests.

- Base: 422 wrong clears (every overlapping pose the gate separates)
  and 0 false refusals.
- Head: 0 wrong clears and 0 false refusals, at ε unset, 1e-6 and
  1e-12.

**The obtuse-sector re-check (the lever gap).** The part from
`an_obtuse_sector_is_read_through_its_rays` already sits on the floor
in a gate-separated pose.
`an_obtuse_sector_never_clears_a_dip_where_the_gate_separates` runs it
end to end. Dips are 2, 5 and 30 times the zero threshold. Sectors run
from 0.6° short of flat down to 300 band widths short of it, scaled to
the run's band. No dip clears at any of the three ε rows. All nine poses are gate-separated, so
the base cleared every one at the gate (measured on the first). A lifted face (a true rest) refuses while
its lift is in band. It also refused at 1e-6 in one measured pose,
`δ = 0.03`, 30·zero: a false refusal, which the redesign row
`touch-cone-readings-are-levered-directions-not-face-distances` owns.

## Sweep for the class (discipline §5)

The shape is a box test that CLEARS a pair on containment grounds
before the pair's boundary evidence is read. The second part of the
class is a finding kind that `TouchSite::of` leaves without a site.

Pattern 1: `decide("…(contain|gate|separat|disjoint|box)…")` across
`crates/`.

- `census.rs` `census_backstop_containment`: fixed.
- `census.rs` `census_backstop_gap` (arm 1): not this class. It is a
  separation test on sound reach boxes, so separated boxes cannot
  meet.
- `shell.rs` `shell_footprint_separation`: not this class. It is a
  separation test on grown boxes, and its direction only refuses.

Pattern 1 cannot match a box skip spelled as an `Aabb` overlap test or
an unnamed comparison. Second pass: `.overlaps(` / `.intersects(` /
`.later(` in `topo`, `editor-core` and `geom-brep` sources. Hits:

- `census.rs` vertex, edge, reach and extent pre-filters.
- `separation.rs` ×2.
- `boolean/reduce.rs`.
- `boolean/ops.rs` ×4.
- `editor-core/src/eval/wire.rs`.

Every hit skips on DISJOINT boxes, which cannot meet. None clears on
containment, so none is this unit's.

Pattern 2: every `CensusContact` arm of `TouchSite::of` that returns
`None`.

- `EdgeEdgeCross`: fixed.
- `EdgeFacePierce`: a transverse crossing, and correctly `Crossing`.
- `ConformalPatch`: a curved touch, and correctly `TouchUnreadable`.

Consumers of `EdgeEdgeCross` outside the census (`validate.rs`
display, `pncad-py` tag, `test_support_samples`) only render it.

**Not matched, stated:** meetings that leave no finding. Declared
v-on-f, v-v and face-pair records are not read by `meets`. Counting
them refused the `sweep` M9-2 acceptance row (a declared curved boss,
`TouchUnreadable`) and nineteen declared planar-seat rows
(`DeclaredFacePair`). Filed as
`work/contact/declared-only-meetings-clear-at-the-census-gate-unread.md`
with the evidence. A measured bound: the half-overlap cannot be
declared finding-free, because a vertex on an edge has no record type.

## Territory seam

Everything is `crates/topo/src/census.rs` and `crates/topo/tests/`
(contact's ground, `work.py territory`: 0 paths elsewhere), except
two moved expectations in `crates/editor-core/tests/`
(`docm6_seam_declarations.rs` and the three `perf12_census_*.txt`
goldens). Those move only because the census now decides differently,
and their owners should read the kitchen-sink and cut-cylinder rows
above.

## Local results

- `cargo test -p topo --lib --test all`: green at ε unset, 1e-6 and
  1e-12.
- `editor-core`, `sweep` and `pncad` under nextest at ε unset; the
  moved `editor-core` rows at all three.
- `cargo clippy -p topo --all-targets -D warnings`.
- `cargo doc -p topo --no-deps --document-private-items --features
  sweep-testing` under the doc gate's lints (`-D warnings -A
  rustdoc::private_intra_doc_links`) is clean. Plain
  `RUSTDOCFLAGS='-D warnings' cargo doc -p topo --no-deps` fails on
  106 pre-existing private-link errors, none in `census.rs`.

The hosted run is the record.
