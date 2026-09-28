# CONTACT-5 — the backstop decides a meeting pair by the probe and the touch analysis

Carries `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`
(P0) and `a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses`
(P0). Files `declared-only-meetings-clear-at-the-census-gate-unread`, now P0
and narrowed to the residue this unit leaves.

## The logical change

**Arm 2's box gate answers containment, and only per shell.** Before
this change, a pair whose two whole-solid hulls both separated on the
box was cleared before any finding between the two solids was read.
Two things were wrong with that:

- The gate read whole-solid hulls. A solid may have several outer
  shells: `boolean::union` of two disjoint cubes is one solid with two
  shells. One lump can sit inside the other instance while its sibling
  pokes out.
- Box separation says nothing about two boundaries that meet.

Now `sweep_cross_solid_backstop` does this per pair:

- **It reads the gate per shell.** Every shell's vertex hull is tested
  against the other solid's reach box. A shell that is not definitely
  separated "reaches".
- **It asks whether the boundaries meet on record.** They meet if a
  standing finding names one entity of each solid (`meets_found`), or a
  declared v-on-f, v-v, curve or patch record names one of each
  (`meets_declared`).
- **Nothing on record, every shell separated:** the pair clears at the
  gate.
- **Otherwise the probe runs.** Every vertex of each solid is tested
  against the other's material, in both orderings.
  - An `In` vertex is a decided interference.
  - If both orderings are clear, `blocks` reads every finding about the
    pair. A declared-only pair is the exception; see below.

The site states why this decides the pair. Let `U` be the overlap of
the two interiors. If every meeting is a rest, no point where the
boundaries meet is in `U`'s closure. So `∂U` splits into points of `∂A`
inside `B` and points of `∂B` inside `A`. Each part is open and closed
in its own boundary, so it is a union of whole shells. Therefore `U` is
non-empty only if some shell of one solid lies wholly inside the other,
and then that shell's vertices are strictly inside:

- With nothing on record, that shell's hull also lies inside the other
  solid's reach box, so the per-shell gate would not have separated it.
- Otherwise the probe finds the shell's vertices.

The argument assumes the census is complete for planar boundaries.
That is why `blocks` refuses on an escalation or on an entity left
unexamined against everything. A pair with nothing on record clears at
the gate even beside an escalation; the escalation stands as the
body's refusal.

**Declared-only pairs.** For a pair whose only meetings are declared,
the probe runs and `declared_crossing` refuses a declared v-on-f or v-v
touch that decidedly crosses. `blocks` is not run there. Running it
refuses ratified acceptance rows: the M9-2 declared curved boss as
`TouchUnreadable`, and nineteen declared planar seats as
`DeclaredFacePair`. So two things remain taken on the records' word:

- a declared touch the analysis cannot read;
- the events a declared face pair backs.

An overlap those hide with no vertex strictly inside clears. No such
pose has been built. This residue is the re-scoped P0 row.

**One finding→pair mapping.** `Named::of` maps every standing finding
to the entities it names and what it says of them (`Said`). Both
`meets_found` and `blocks` read it.

**One third-solid rule.** A finding blocks a pair only if it names one
entity of each of the pair's solids. A pierce, an arm-1 refusal or an
unsupported face pair between one of them and a third solid is that
other pair's finding and is read there. Three kinds block beyond that,
each because the argument needs every meeting of the pair on record:

- An escalation names no entity. The event that escalated may be this
  pair's meeting.
- An entity left unexamined against everything may meet either solid
  unseen.
- A pierce or edge cross between two entities of one solid is that
  solid's boundary crossing itself. No placement can be read against
  that material.

`bool4r2_probes::a_pierce_between_a_and_c_blocks_the_a_b_material_test`
pinned the old conservative arm "so the cost is on record". It is
re-signed as `a_pierce_between_a_and_c_leaves_the_a_b_pair_to_its_own_findings`:

- The bracket's material is its own and well formed.
- The bracket × part pair clears exactly as it does alone.
- The bracket × piercer pair refuses, as `InstanceInterference`.

**A crossing of two edges is a touch site.** `TouchSite::EdgeCross(a, b)`
maps `CensusContact::EdgeEdgeCross` through the same `of`, `entities`
and `verdict` path as every other kind. Its site is the crossing point,
from `ee_cross_point`, which the crossing lane shares. Its cones are
the two edges' dihedral wedges there.

`Cone::wedge` levers its faces at a point on its own edge:

- For a cross, the crossing point.
- For a vertex-on-edge, the vertex.
- For a collinear overlap, the overlap's midpoint (`ee_overlap_midpoint`)
  for both wedges. Before this change it was each edge's start. In the
  pass before this one, edge `b`'s wedge was levered at `a`'s start,
  which can lie off `b`.

**"Coplanar" is decided where the finding is made.** An `EdgeEdgeCross`
is pushed only after `pm_census_ee_gap` decides that the lines meet,
and that predicate escalates in band. So there is no "non-coplanar"
cross finding.

- A cross whose materials pass into each other reads as a crossing
  through its wedges: the plus-shaped slabs.
- Ridges crossed edge on edge share no face plane and are a real rest.

## Base vs head

"Refused" means a placement finding on the solid pair. Three columns:

- **base**: `main` before CONTACT-5.
- **first pass**: `3af4ebc`.
- **head**: this fix pass.

| pose | base | first pass | head |
|---|---|---|---|
| half-overlapping cubes | cleared (wrong) | `MixedTouch` | `MixedTouch` |
| two cubes face to face | cleared | cleared | cleared |
| beam across two supports | cleared at the gate | cleared (8 crosses read as rests) | cleared |
| beam sunk 1 mm | cleared at the gate | `Crossing` ×2 | `Crossing` ×2 |
| beam tilted in band | cleared | `Unexamined` ×2 | corners too close to place, both pairs |
| ridges crossed, resting / sunk | cleared / cleared | cleared / `Crossing` | cleared / `Crossing` |
| **M1** two-lump solid, lump 1 inside B, lump 2 touching B | cleared (wrong) | cleared (wrong) | `InstanceInterference`, lump solid inside B |
| **M1** same, lump 2 far off (nothing meets) | cleared (wrong) | cleared (wrong) | `InstanceInterference` |
| **M1** same, touching and declared | cleared (wrong) | cleared (wrong) | `InstanceInterference` |
| **M2** prism dipping 0.1 into a wall, 4 corners declared v-on-f | `Ok(())` (wrong) | `Ok(())` (wrong) | `InstanceInterference` |
| **M2** declared patch seat with a keel 0.5 deep | `Ok(())` (wrong) | `Ok(())` (wrong) | `InstanceInterference` |
| same seat, flat | validates | validates | validates |
| one solid crossing itself (plus of two shells) beside a resting block | cleared | `Crossing` | `Crossing` |
| two interpenetrating cubes (`m3_pr6_tier3prime`) | cleared | `Crossing` | `InstanceInterference` |
| obtuse-sector part dipping 30·zero | cleared (wrong) | refused | `InstanceInterference` (floor, part) |
| obtuse-sector part dipping 2 or 5·zero (in band) | cleared (wrong) | refused | corners too close to place, (floor, part) |

The beam row's premise did not reproduce end to end. At base the beam
is gate-separated and cleared before `blocks` read its crosses. The
refusal the row described is what `blocks` would have said had it run.

## Sweeps

- **Brick grid**
  (`contact5_gate_and_beam::a_grid_of_brick_pairs_clears_exactly_the_rests`):
  the cube `[0,2]³` against every brick on the grid −1…3, 1000 poses.
  Exact ground truth comes from interval overlap: 512 poses overlap and
  488 are rests. Base: 422 wrong clears and 0 false refusals. Head: 0
  and 0, at eps unset, 1e-6 and 1e-12.
- **The reviewers' 480-pose rotated-prism sweep**: 0 wrong clears and 0
  false refusals at head.
- **The reviewers' crossed-ridge sweep** (135 poses) and **coplanar
  cross sweep** (12): 0 mismatches.

**The obtuse-sector re-check.**
`census::tests::an_obtuse_sector_never_clears_a_dip_where_the_gate_separates`
now pins the pair and the reason:

- Dips decidedly below the floor are the part's corners inside the
  floor.
- Dips in band are corners the probe cannot place.

No dip clears at any of the three eps rows.

## Mutations

- `EdgeCross` forced to read `Rest`: `an_edge_cross_reads_rest_and_crossing`
  goes red. It pins the plus-slab crosses as crossings and a bar's
  crosses over a block's edges as rests. The end-to-end rows stay
  green, because in every overlapping pose measured the overlap also
  shows as an edge lying in a face, a pierce or a vertex inside.
  Whether an overlap can show only at crosses is not settled, and the
  row says so.
- Same-solid cross clause removed: `a_solid_crossing_itself_blocks_its_pair`
  goes red.

## Goldens that moved

`editor-core` `perf12_census_*` were re-blessed at all three eps rows.
Relative to base:

- `kitchen_sink` gains `Unexamined` refusals for solid pairs (1,8),
  (3,8) and (4,8). Solid 8 is curved, and arm 1 refused its faces
  against theirs, which are unexamined meetings between those pairs.
- The first pass's (3,4) refusal is gone. Solids 3 and 4 are the two
  halves split at z = 0.625, and all of their own findings are rests.
  They had refused only on other pairs' arm-1 findings; the third-solid
  rule ends that false refusal.
- `cut_cylinder` gains a `VolumeUncertified` refusal both ways. The
  pair meets, so it is probed, and the door cannot certify the curved
  part's volume.

All of these bodies already refused.

## Other moved rows

- The half-overlap row is renamed to its refusal,
  `two_half_overlapping_cubes_refuse_as_a_mixed_touch`.
- `review_m3_pr6::r2_coplanar_plus_overlap_detected` expects the pair's
  `MixedTouch`.
- `m3_pr6_tier3prime::hand_built_self_intersection_is_undeclared`
  expects the decided interference.
- The allowlist in `editor-core` `docm6_seam_declarations` admits a
  solid-pair `CensusUndecidable` only for the reason that occurs, the
  penetrating seat's `MixedTouch`.
- `sweep` `verbs_pierce_r2_probes::r2_the_1032_declaration_measurement_reproduces`
  moves from 11 and 6 to 12 and 7. It counts every `CensusUndecidable`,
  and the plate × boss pair now adds one solid-pair `Unexamined`
  refusal. The pair meets through the curved candidates arm 1 left
  unexamined, and a pair that meets is no longer cleared past them.

## Sweep for the class (discipline §5)

The class is a box test that clears on containment grounds before the
boundary evidence is read, or that reads a solid as one hull.

**Pattern 1:** `decide("…(contain|gate|separat|disjoint|box)…")` across
`crates/`.

- `census_backstop_containment`: fixed, now per shell.
- `census_backstop_gap` (arm 1): a separation test on sound reach
  boxes. Not this class.
- `shell.rs` `shell_footprint_separation`: a separation test on grown
  boxes that can only refuse. Not this class.

**Pattern 2:** `.overlaps(` / `.intersects(` / `.later(` in `topo`,
`editor-core` and `geom-brep`. The hits are:

- the census pre-filters;
- `separation.rs` ×2;
- `boolean/reduce.rs`;
- `boolean/ops.rs` ×4;
- `eval/wire.rs`.

Every one skips on disjoint boxes, which cannot meet.

**Pattern 3:** `TouchSite::of` arms that return `None`.

- `EdgeEdgeCross`: fixed.
- `EdgeFacePierce`: a crossing.
- `ConformalPatch`: a curved touch.

**Pattern 4** (the multi-shell blind spot): whole-solid hulls used as
the contained side. The hit is `solid_boxes` in arm 2, now used only by
the extent pre-filter. That filter prunes on disjoint extents, and a
union hull is sound for that.

**Not matched:** the declared-only residue, filed as the P0 row.

## Territory seam

Most of the change is in `crates/topo/src/census.rs` and
`crates/topo/tests/` (contact's ground). Two `editor-core` test
expectations move: the docm6 allowlist and the three `perf12` goldens.

## Local results

- `cargo test -p topo --lib --test all`: green at eps unset, 1e-6 and
  1e-12.
- `editor-core`, `sweep` and `pncad` under nextest at eps unset:
  3998 of 3999 passed. The one red, the #1032 measurement row above,
  was re-baselined and then passed at all three eps rows.
- The moved `editor-core` rows and the `sweep` boss-union row: green at
  all three eps rows.
- `cargo clippy -p topo --all-targets -D warnings`: clean.
- Rustdoc for topo with the doc gate's lints and
  `--document-private-items --features sweep-testing`: clean.
- Every `scripts/gates/*.sh`, `check-ci-mirror-parity` and
  `work.py lint`: clean. `check-python-lint` skipped itself because the
  local ruff is not the pinned version; no python changed.

The hosted run is the record.
