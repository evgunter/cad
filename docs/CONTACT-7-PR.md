# CONTACT-7: a touch is read in metres over the touch point's star

This file is the PR body for the landing. The orchestrator folds it into
that PR and deletes it at merge.

Carries three rows, all closed on this branch:
- `work/contact/touch-cone-readings-are-levered-directions-not-face-distances` (P1);
- `work/contact/census-touch-cones-are-a-third-vertex-sector-builder` (P1);
- `work/contact/zero-dihedral-conflates-flat-with-slit-in-touch-cones` (P3).

Spec: `docs/CONTACT-7-SPEC.md` with its amendment of 2026-09-28 (S2′,
S3.3′, S5′, S6′). The amendment exists because the first build, which
read each star face whole, refused CONTACT-1's L-bracket rests (below).

## What changed, and why

The census's touch analysis (`census.rs`, arm 2's clear) used to read
each sign as a unit-direction reading times a lever. A levered reading
has the right sign wherever it is definite. Its Zero moves with the
lever, and where the analysis read a Zero as a verdict ("this ray is on
the plane", "this edge is flat"), a face's far points could leave the
plane by many band widths behind it. Five review rounds found that at
five sites. It is now read in metres.

### S1: every verdict is a vertex's distance from a plane

Every sign the analysis decides as a verdict is
`Margin::of(n·(q − p))`:
- `q` is a vertex of a star face's piece;
- `p` is the touch point;
- `n` is a candidate plane's unit normal.

Directions (face normals, edge directions, a face's direction off an
edge, their cross products) only propose candidate planes.

Two readings of unit normals stay, each of magnitude about 1 wherever it
is asked, and only its sign is read:
- an On face's outward normal against the candidate's
  (`census_touch_normal`);
- an edge's two faces' normals, aligned or opposed, where neither order
  of the convexity reading decides.

Both go through `geom_brep::classify_material_pairing`. The first goes
through a new twin, `classify_material_pairing_as`, under its own name,
so it does not pool with `material_wedge_side`. The existing function
now delegates to the twin under its old name, bit for bit.

### S2 and S2′: the finite star, each face through its piece

`Star` is built from the census snapshot. No orbit is walked and no
sector is built:
- at a vertex, the faces in `vertex_faces` and the line edges ending
  there;
- inside an edge, the edge's two faces;
- inside a face, that face.

`FaceGeo` gains `loops` (each loop's vertices in walk order) and
`polygon` (every loop walked, every edge a line).

Each face is read through its **piece** at `p` (`face_piece`): the part
of the face visible from `p`. Holes' edges block and holes' vertices
cast windows.
- It is built as a fan: the face's vertices are sorted by their turn
  about `p`.
- Between each two neighbouring turns, the edge a ray from `p` meets
  first is read on the bisecting ray and cut at the two turns. No edge
  can end strictly between two neighbouring turns.
- Its vertices are face vertices and window points on edges, all real
  points.
- Every choice is decided in metres (`census_touch_piece_side`,
  `census_touch_piece_reach`).
- A choice in band builds no piece, which is the smallest piece and
  costs a refusal.
- A turn read Zero is one direction, and an edge read as blocking within
  ε blocks, so both only shrink the piece.

The piece is star-shaped from `p`, so its vertices' distances bound
every point of it.

Deleted: `Cone`, `ConeKind`, `RayKind`, `ConeShape`, `Cone::fan`,
`Cone::wedge`, `Cone::vertex`, `Cone::within`, `touch_lever`, and the
census's use of `sector_face::resolve` and `sector_shape`. The boolean
and splitting lanes keep their own sector code.

### S3 and S3.3′: the readings

`Star::within(n)` is S3.2:
- Each face is read from its piece (all Zero On, all non-negative with
  one Positive Above, any Negative Below).
- The star lies in the closed half-space iff no face is Below.
- The material side is then read from an On face's normal, else from an
  edge on the plane whose faces are Above (by its convexity), else from
  the shape class.
- Disagreeing On faces are a fold.

`Star::dihedral` is S3.3′:
- The far face's piece vertices are read against the near face's plane
  through `p`, in both orders, and a definite side in either order
  decides.
- Where neither order decides and one reads On, aligned normals are a
  flat seam and opposed normals are a fold (a slit). A fold refuses the
  star as `Degenerate`, and is named rather than read as flat.

At a vertex, the shape class also reads each face's corner at `p`
(`census_touch_corner`). A face turning away there makes the corner a
saddle.

### S4: a rest has one door

`TouchVerdict::Rest(rest::Rest)`. `Rest` has a private field in
`mod rest`, and only `rest::certify` mints one, after running
`Star::within` against a candidate:
- a plane, with both stars on either side;
- or the complement of a co-convex or flat star.

A sixth round of the lever class now has to get past two guards. The
source row `census::tests::the_touch_analysis_levers_only_its_candidates`
reads this analysis's section for a `levered` door and admits exactly
one, `touch_candidates`' span test, where a Zero only skips a candidate.
Minting a rest outside `certify` does not compile.

### S5′: a crossing only on decided readings

A decided Below at a piece vertex is exact local evidence: the segment
from `p` to that vertex lies in the face and leaves the half-space. The
first build's reach-back refusal is gone. A candidate refused in band,
on a saddle, on contradictory readings or by a piece that did not build
leaves the touch `InBand`, `Degenerate` or `Unanalysed`, never
`Crossing`. `blocks` is unchanged.

### S6′: the tolerance of arm 2's argument

This is written at arm 2's loop. Within the ball about `p` that reaches
the nearest boundary of the pieces read, any overlap is confined to a
slab at most 2·`zero` thick, which is coincidence under D4. Beyond it,
an overlap deeper than the band is a dip of a face, and it shows either
at a vertex the probe reads or at a crossing that stands as a finding.

### An edge in a face is read inside the overlap

`TouchSite::EdgeInFace` used to read both stars at the edge's first end
`p0`. That point is not inside the edge, so the edge star there is not
the edge's, and it need not be inside the face. It now reads them at
the midpoint of the first overlap cell, the same cells
`ef_overlap_lane` reports (`ef_overlap_cells`, factored out of it).

### Refusal text

`TouchUnanalysed` used to say only "a corner that is neither convex nor
concave". What reaches it now also includes a face that meets itself at
the touch, or whose piece does not build on decided readings. The
message now names both. It keeps the no-way-through form of its
siblings, since nothing about it is a kernel defect.

## Rows that moved, and why

**Re-signed.**
- `census::tests::an_obtuse_sector_is_read_through_its_rays`: the corner
  read **Rest** at 30× zero, and now reads **not Rest** at 2, 5, 30 and
  500× zero. Its face is convex, so its piece is the whole face and the
  far dip is read. The pair-level wedge still refuses at every dip.
- `census::tests::a_dihedral_is_read_at_its_far_face_on_a_real_corner`:
  it read **convex at the bottom, flat at the top**, and now reads
  **convex at both ends**. The long side's far vertices decide in one
  order, and a definite side in either order decides. The old pin
  encoded the defect: its verdict depended on which way the orbit was
  walked.
- `census::tests::a_slit_reads_degenerate`: this was `Within::Degenerate`
  on a hand-built fan. It is now the fold named at the star (a
  `Degenerate` star) through `classify_material_pairing`, beside a flat
  seam that reads `Flat`.

**Rebuilt on the new types, same verdict.**
- `a_dihedral_is_levered_at_its_face` became
  `a_shallow_pit_is_read_at_its_far_vertices`: a shallow pit's apex on a
  floor, `Crossing` as before.
- `a_corner_out_of_scale_refuses_on_scale` now builds `Star::vertex`,
  and reads `ChordScale` as before.

**Renamed, assertions unchanged.** Their names and docs described
levers:
- `a_bisector_is_levered_at_its_face` →
  `a_reflex_notch_dipping_far_off_is_never_a_rest`;
- `a_convex_corner_is_levered_at_its_faces` →
  `a_convex_corner_dipping_far_off_is_never_a_rest`;
- `a_wedge_in_face_ray_is_levered_at_its_face` →
  `an_edge_in_a_face_dipping_far_off_is_never_a_rest`;
- `ordinary_rests_stay_rests_under_the_face_levers` →
  `ordinary_rests_stay_rests`;
- `a_ray_is_read_at_the_larger_of_its_faces` →
  `a_tip_tilted_about_one_chord_is_never_a_rest`.

Rows asserting a rest now spell it `is_rest()`, since `Rest` carries its
certificate.

**New.**
- `the_touch_analysis_levers_only_its_candidates`: the S4 source row.
- `an_obtuse_sector_on_a_synthetic_star_is_not_a_rest`: the synthetic
  fan at 500× zero, `Crossing`.
- `a_notched_bracket_rests_against_the_wall`: the L-bracket with a notch
  cut into its leg's end, and the wall pose. The bottom face's piece at
  `(1, 3)` holds neither the foot's far corner nor the corner `(0, 3)`
  the notch shadows, and every touch is a rest.
- `a_holed_block_rests_beside_a_brick`: a drilled block beside a brick.
  The top face's piece at `(0, 0, 2)` stops at the hole, and a hole
  corner casts a window on the far edge. It clears.
- `a_wall_rest_off_by_a_band_width_never_rests`: the wall pose turned
  so its far corners stand off or into the wall by the geometric mean
  of the band's thresholds. The bracket's faces reach back across the
  exact separating plane, no touch reads a rest, and the pair does not
  clear.
- `contact7_touch_sweeps` (integration): the two rebuilt sweeps below.

**Every other touch row answers as before.** This covers CONTACT-1's
kinds (`every_touch_kind_reads_rest_and_crossing` and the eight
`contact1_touch_cones` rows), CONTACT-5's edge cross and brick grid,
the ordinary-rest battery at three rotations, and the `bool4` rows.
The four L-bracket rows that the first, whole-face build refused are
rests again:
- the block against the wall;
- the block flush with the arm top;
- the tilted block on the corner;
- the wall pose in the battery.

**Goldens.** No golden moved; see Local results for the `perf12` census
goldens and `docm6`.

## The first build, and why the spec was amended

Read whole, a face that is not convex can reach back across the only
separating plane far from the touch. At the L-bracket's corner `(1, 3)`
the only separator of the block against the wall is `x = 1`, and the
L's floor runs to `x = 3`. That refused three `contact1_touch_cones`
rests and the battery's wall pose, all `TouchUnanalysed`, which
contradicted the spec's rows. The lane stopped at `2b080332d`. Both
designers then amended the spec: read each face through its piece
(S2′).

## Sweeps

Each sweep counts wrong clears (materials overlap, no placement
finding) and false refusals (no overlap, a placement finding), head
against base (`origin/main` at the merge, `5bf6dac0b`). The same rows
were run at base. All three ε rows gave the same numbers on each side.

| sweep | poses | base: wrong clears / false refusals | head: wrong clears / false refusals |
|---|---|---|---|
| CONTACT-5's brick grid (`a_grid_of_brick_pairs_clears_exactly_the_rests`) | 1000 | 0 / 0 | 0 / 0 |
| rotated prisms (`a_rotated_bracket_and_brick_sweep_clears_no_overlap`) | 432 | 0 / 12 | 0 / 12 |
| crossed ridges (`a_crossed_ridge_sweep_clears_no_overlap`) | 135 | 0 / 0 | 0 / 0 |

**The two rebuilt sweeps.** Neither reviewer's harness was committed,
and their records give only counts. They are rebuilt from the
descriptions in CONTACT-1's and CONTACT-5's Closed sections and are now
committed rows.
- **Rotated prisms.** The L-bracket against a brick on a grid of spans
  (beside its walls, in its inner corner, on and under it, sunk into
  it), under the identity and two generic rotations. The bracket is
  chosen because its floor and ceiling are faces that are not convex.
  Ground truth is exact: the bracket is the union of two boxes.
- **Crossed ridges.** Two 45°-turned bars crossed ridge on ridge.
  - Five crossing points, two at or past the lower bar's end.
  - Three angles.
  - Three shifts, one putting the crossing at the upper bar's end.
  - Three lifts (sunk 1 cm, resting, 1 cm off).
  - Ground truth is the separating-axis depth of the two convex bars.

**The 12 false refusals are the same on both sides.** They are the
brick seated in the bracket's inner corner on the floor. That brick
meets the saddle corner `(1, 1)`, which no test certifies (filed:
`a-touch-at-a-saddle-corner-refuses-unanalysed`). The row pins them by
shape and count.

These sweeps do not separate head from base. The lever design had no
end-to-end wrong clear in any measured pose (CONTACT-1's closing note),
and the difference is local. The local rows above show it.

## Sweep for the class (discipline §5)

The shape is a levered reading of directions whose Zero is taken as a
verdict. Pattern: `Margin::levered` / `levered_inv` / `classify_dihedral`
/ `classify_material_pairing` in `census.rs`.
- The touch analysis: fixed. The one levered door left,
  `touch_candidates`' `census_touch_span`, only skips a candidate, and
  the source row holds it there.
- `pair_edge_edge`'s `pm_census_ee_parallel` (`levered(ncross.norm(),
  arm)`): not this unit's. A Zero routes the pair to the collinear lane,
  which then decides the overlap on metre distances. It is a routing
  decision, not a verdict read off a direction.
- `ee_crossing_lane`'s `levered_inv(d·n̂ × …)`: a distance between two
  lines in metres, not this class.
- `ee_cross_backed`'s `classify_dihedral` and `classify_material_pairing`:
  a sign of magnitude about 1 on a pair already decided Smooth, not this
  class.

Second pass, the same shape outside the census:
`boolean/sectors.rs` (×5), `splitting/neighborhood.rs`,
`boolean/vtxfac.rs` (×2). These are the boolean and splitting lanes' own
sector readings, owned by
`work/contact/boolean-side-codes-lever-a-chord-direction-where-a-zero-is-read-as-on.md`.
They are not touched here.

**Not matched, stated:** a lever spelled as a bare product (`x * arm`),
and a levered reading reached through a call into another module. The
source row reads the door's name, not provenance. I checked the touch
section by eye for both. Its only products of a dot and a length are
the piece's crossing-point interpolations, which are constructions and
decide nothing. The two `classify_material_pairing_as` calls are the
two readings S1 keeps.

## Territory seam

Everything is `crates/topo/src/census.rs`, `crates/topo/tests/`,
`docs/` and `work/contact/`, except one additive change in
`crates/geom-brep/src/dihedral.rs` and its `lib.rs` export:
`classify_material_pairing_as`. Its owner should know that the census
now calls it under its own predicate name.

## Local results

All under nextest, with the lane's own target directory:
- `-p topo -p sweep`: 3300 of 3300 at ε unset, 1e-6 and 1e-12. After the
  last text and clippy edit, it was re-run at ε unset (3300 of 3300),
  and the census and contact rows at all three ε (100 of 100).
- `editor-core`, the concision rows, `perf12_census_*` and
  `docm6_seam_declarations`: 26 of 26 at all three ε. No `perf12`
  golden moved.
  - `refusal_concision_at_rest` first went red on the reworded
    `TouchUnanalysed` (88 words against a budget of 75). The text was
    cut to fit.
- `-p test-utils`: 79 of 79. `-p geom-brep`: 782 of 782.
- `cargo clippy -p topo -p geom-brep --all-targets --all-features -- -D
  warnings`: clean.
- rustdoc (`-D warnings -A rustdoc::private_intra_doc_links`, `-p topo
  --document-private-items --all-features`): clean.
- every `scripts/gates/*.sh`: OK. `python3 scripts/work.py lint`: 0
  problems.
- Mutation: a `Margin::levered` added to `Star::within` turns the S4
  source row red. The first build read faces whole, which is the
  mutation that drops the piece. It turned red the four L-bracket rows,
  which the notch row now shares.

The hosted run is the record.
