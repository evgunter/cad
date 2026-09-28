# CONTACT-5 — the backstop decides a meeting pair by the probe and the touch analysis

Carries `partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`
(P0) and `a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses`
(P0). Files `declared-only-meetings-clear-at-the-census-gate-unread`, now P0
and narrowed to the residue this unit leaves.

## The logical change

**Arm 2's box gate answers containment, per outer shell.** Before
this change, a pair whose two whole-solid hulls both separated on the
box was cleared before any finding between the two solids was read.
Two things were wrong with that:

- The gate read whole-solid hulls. A solid may have several outer
  shells: `boolean::union` of two disjoint cubes is one solid with two
  shells. One lump can sit inside the other instance while its sibling
  pokes out.
- Box separation says nothing about two boundaries that meet.

Now `sweep_cross_solid_backstop` does this per pair:

- **It reads the gate per outer shell.** Every non-void shell's vertex
  hull is tested against the other solid's reach box. A shell that is
  not definitely separated "reaches".
  - A solid's only shell is outer.
  - Among several, a shell's role is the sign of its own volume
    (`validate::shell_role`, now `pub(crate)`).
  - A shell whose role does not read is gated as outer.
- **It asks whether the boundaries meet on record.** They meet if a
  standing finding names one entity of each solid (`meets_found`), or a
  declared record names one of each (`recorded`).
- **Nothing on record, every outer shell separated:** the pair clears
  at the gate.
- **Otherwise the probe runs.** Every vertex of each solid is tested
  against the other's material, in both orderings.
  - An `In` vertex is a decided interference.
  - Otherwise `blocks` reads the findings about the pair. It does so
    whether or not the probe decided. If it gives a reason, that is
    the pair's one refusal; if not, the probe's own refusals stand.

**The argument is stated once, at the arm-2 loop.** The module and arm
docs point to it. In short:

- If every meeting is a rest, the overlap `U` is bounded by whole
  shells, each lying inside the other solid's material.
- The outermost shell around a component of `U` has `U`, and so its
  own solid's material, on its bounded side. It is therefore an outer
  shell.
- Its vertices are strictly inside the other solid, and its hull lies
  inside the other's reach box.

The loop names the premises and what checks each one:

- Shells are connected: tier 2's `validate_closed` runs before the
  census, and a solid that crosses itself is refused in `blocks`.
- A shell's role is the sign of its volume: tier 3's check 10 holds
  the winding.
- The census is complete for planar boundaries: an escalation or an
  unexamined entity blocks, and curved meetings are arm 1's.

**Declared-only pairs.** A pair whose only meetings are declared is
probed whether or not it reaches. `blocks` reads it with
`records_on_their_word`: a declared v-on-f or v-v touch that decidedly
crosses refuses, and nothing else in its records does. It never reaches
the face-pair tail on reach alone. Reading the records fully refuses
ratified acceptance rows: the M9-2 declared curved boss as
`TouchUnreadable`, and nineteen declared planar seats as
`DeclaredFacePair`. So two things remain taken on the records' word:

- a declared touch the analysis cannot read;
- the events a declared face pair backs.

An overlap those hide with no vertex strictly inside clears. No such
pose has been built. This residue is the re-scoped P0 row.

**One mapping each for findings and records.** `Named::of` maps every
standing finding to the entities it names and what it says of them
(`Said`), and both `meets_found` and `blocks` read it. `recorded` maps
every declared record naming one entity of each solid to a `Recorded`
(a touch site, or a face pair), and both the meeting test and `blocks`
read it.

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
- **dd95fcb**: the first fix pass.
- **head**: the last pass.

Where dd95fcb is not shown, it matches head.

| pose | base | first pass | head |
|---|---|---|---|
| half-overlapping cubes | cleared (wrong) | `MixedTouch` | `MixedTouch` |
| two cubes face to face | cleared | cleared | cleared |
| beam across two supports | cleared at the gate | cleared (8 crosses read as rests) | cleared |
| beam sunk 1 mm | cleared at the gate | `Crossing` ×2 | `Crossing` ×2 |
| beam tilted in band | cleared | `Unexamined` ×2 | `Unexamined` ×2, the findings' reason reported over the probe's |
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
| obtuse-sector part dipping 2 or 5·zero (in band) | cleared (wrong) | refused | `Unexamined` (floor, part) |
| **last-pass M1** hollow part (cavity `[4,6]³`) seated declared in a U channel or an L corner | `Ok` | `Ok` (dd95fcb: `DeclaredFacePair`) | `Ok` |
| same with a solid part | `Ok` | `Ok` | `Ok` |
| hollow part floating in the channel with a 0.1 gap | `Ok` | `Ok` | `Ok`, cleared at the gate |
| block floating in a part's cavity | `Ok` | `Ok` | `Ok` |
| two-lump solid, one lump seated declared on the channel floor, sibling far or floating in the channel | not measured | dd95fcb: `DeclaredFacePair` (the seated lump's hull is inside the channel's reach) | `Ok` |
| same, sibling sunk in the channel floor | not measured | dd95fcb: refused | `InstanceInterference` |
| all corners on the other's boundary (`bool4_material_containment`) | `AllOn` | `AllOn` | `MixedTouch`: the slab is inside the cube, and the touch analysis says so |
| a witness in band of a wall (`bool4_material_containment`) | too close to place ×2 | same | `Unexamined` ×1 |

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
- Dips in band are refused as left unchecked: the probe cannot place
  the corners, and the sweeps' escalations give the reason.

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
The last pass moved two rows relative to dd95fcb:

- In `kitchen_sink`, (4,1) "every corner on the boundary" becomes
  (1,4) `MixedTouch`. When the probe cannot decide, the findings are
  now read, and they give that reason.
- In `cut_cylinder`, `VolumeUncertified` both ways becomes a single
  `Unexamined`. Arm 1's refusals of the curved faces are the reason.

Relative to base:

- `kitchen_sink` gains `Unexamined` refusals for solid pairs (1,8),
  (3,8) and (4,8). Solid 8 is curved, and arm 1 refused its faces
  against theirs, which are unexamined meetings between those pairs.
- The first pass's (3,4) refusal is gone. Solids 3 and 4 are the two
  halves split at z = 0.625, and all of their own findings are rests.
  They had refused only on other pairs' arm-1 findings; the third-solid
  rule ends that false refusal.
- `cut_cylinder` gains one `Unexamined` refusal of the solid pair.
- `kitchen_sink`'s (4,1) `AllOn` is now (1,4) `MixedTouch`.

All of these bodies already refused.

**The general pattern, beyond the goldens.** Every assembly with a
curved face within reach of another solid gains one solid-pair
`Unexamined` beside arm 1's face-pair refusals, for example the
`sweep` #1032 plate × boss (12 and 7). The pair meets through
candidates arm 1 left unexamined, so it is not cleared past them. The
refusal is redundant with arm 1's: no body changed verdict.

**Behaviour relaxed on purpose.** A declared seat whose part lies
inside the partner's reach refused as `DeclaredFacePair` at dd95fcb,
and at base whenever the whole solid's hull did. Such a pair is now probed and its records are taken on their
word, as for every declared-only pair. The two-lump rows above are the
measurement.

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

**Not matched:** the declared-only residue, filed as the P0 row. The
last pass also files
`work/contact/a-same-solid-self-overlap-seen-only-as-touches-is-not-a-self-crossing.md`
(P3, found by the delta review): a solid whose own shells overlap while
meeting only in touches is not caught by the self-crossing exception.

## Territory seam

Most of the change is in `crates/topo/src/census.rs` and
`crates/topo/tests/` (contact's ground). Two `editor-core` test
expectations move: the docm6 allowlist and the three `perf12` goldens.

## Local results

- `cargo test -p topo --lib --test all`: green at eps unset, 1e-6 and
  1e-12.
- `editor-core`, `sweep` and `pncad` under nextest at eps unset, after
  merging `main`: 4044 of 4044 passed. The #1032 measurement row was
  re-baselined in the pass before.
- The moved `editor-core` rows (`perf12`, docm6) and the `sweep`
  boss-union and #1032 rows: green at all three eps rows.
- `cargo clippy -p topo --all-targets -D warnings`: clean.
- Rustdoc for topo with the doc gate's lints and
  `--document-private-items --features sweep-testing`: clean.
- Every `scripts/gates/*.sh`, `check-ci-mirror-parity` and
  `work.py lint`: clean. `check-python-lint` skipped itself because the
  local ruff is not the pinned version; no python changed.

The hosted run is the record.
