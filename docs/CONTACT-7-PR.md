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

Every sign the analysis decides as a verdict is a point's signed
distance from a plane, in metres, `n·(q − o)`. On a candidate plane:
- `q` is a vertex of a star face's piece;
- `o` is the touch point `p`;
- `n` is the candidate's unit normal.

It comes through one door, `mod metric`: its `Distance` has a private
field and one constructor, from a point and a plane (a point on it and
its unit normal), and `metric::sign` is the only place a `Distance` is
decided (`Margin::of`). A lever, a product or any other quantity has no
way to become a verdict: it cannot be made into a `Distance`.

Directions (face normals, edge directions, a face's direction off an
edge, their cross products) only propose candidate planes.

Two readings of unit normals stay, each of magnitude about 1 wherever it
is asked, and only its sign is read:
- an On face's outward normal against the candidate's
  (`census_touch_normal`);
- an edge's two faces' normals, aligned or opposed, where neither order
  of the convexity reading decides.

Both go through a new twin of `geom_brep::classify_material_pairing`,
`classify_material_pairing_as`, under the census's own names
(`census_touch_normal`, `census_touch_fold`), so neither pools with
`material_wedge_side`. The existing function now delegates to the twin
under its old name, bit for bit.

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
- Each gap between two neighbouring turns is then checked empty: no
  vertex lies strictly inside it, on decided readings. The sort's
  comparisons are not transitive within ε, and the check refuses where
  the sort misplaced a vertex.
- In each gap, the edge a ray from `p` meets first is read on the
  bisecting ray. No edge ends inside the gap, so that edge is first on
  every ray through it. It is cut where the rays through the gap's two
  bounds meet it, and each meeting is checked to lie on that edge
  (refused, not clamped, where it does not).
- An edge whose two ends lie decidedly behind `p` along the probe ray
  is skipped before anything else is asked of it: it can never meet the
  ray. (This is the fix pass's MAJOR, below.)
- Its vertices are face vertices and window points on edges, all real
  points.
- Every choice is decided in metres, through `metric`, under five K
  rows, one per quantity: `census_touch_piece_side` (a point from a line
  through `p`), `_turn` (a point along the reference ray), `_front` (an
  edge's end or crossing along a probe ray), `_reach` (one crossing
  against another along it) and `_meet` (a meeting against an edge's
  end).
- A choice in band, or readings that contradict each other within ε,
  build no piece. That is the smallest piece and costs a refusal,
  `TouchPieceInBand`, with its own wording.
- A turn read Zero is one direction, and an edge read as blocking within
  ε blocks, so both only shrink the piece.
- The loops are read with the face's interior on their left about its
  outward normal (outer loop counterclockwise, holes clockwise), as the
  snapshot walks them; `face_piece`'s doc says so.

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

The shape class is read from the edges' convexity alone. The first
build also read each face's corner at `p` (`census_touch_corner`), and
the fix pass deleted it: no row could observe it, because a corner over
180° cannot occur beside edges that all read one way. The faces about
`p` trace a simple spherical polygon whose turns are the edges'
dihedrals. If every turn is convex (or flat), the polygon is convex and
no side exceeds 180°. The same holds of the complement when every edge
is reflex. So a reflex corner always comes with a saddle's mixed edges.
The argument is at `Star::shape`.

### S4: a rest has one door

`TouchVerdict::Rest(rest::Rest)`. `Rest` has a private field in
`mod rest` and is neither `Copy` nor `Clone`, so a certificate cannot be
replayed at another site. Only `rest::certify` mints one, after running
`Star::within` against a candidate:
- a plane, with both stars on either side;
- or the complement: one star within every face's outer half-space of
  the other. The first build accepted only a co-convex or flat `outer`
  here. The fix pass drops the guard, with the proof at the site: a
  direction into any polyhedral cone's material fails some face's outer
  half-space. Walk the great circle from it to the boundary: at the
  face it crosses outward, `m·v = A·cos s + B·sin s` has an upward zero
  in `(0, π)`, which forces `A = m·d < 0`. The test is sound whatever
  `outer`'s shape, and the reviewers' mutant (f) no longer has a guard
  to break.

Two guards stand against a sixth round of the lever class:
- `census::tests::the_touch_analysis_decides_only_through_its_doors`
  reads this analysis's section. It holds that every `Margin` door and
  every `decide` call lies inside `mod metric` (exactly one
  `Margin::of`) or `touch_candidates` (exactly one `Margin::levered`,
  the span test, where a Zero only skips a candidate). It holds that the
  unit-normal classifier is called only from `pairing`, that nothing
  reads a bare sign (`sign_within`), and that `Distance`'s field is
  private.
- Minting a rest outside `certify` does not compile.

The row states what it cannot see: `Distance::of` takes its normal on
trust. A normal scaled by a length is a lever spelled as a plane, and a
point built as `p + (q − p)·k` is a product spelled as a point. Every
caller passes a face's outward normal or a normalized direction, and a
real face point or a point interpolated on a face's edge. That is
checked by reading, not by the row.

### S5′: a crossing only on decided readings

A decided Below at a piece vertex is exact local evidence: the segment
from `p` to that vertex lies in the face and leaves the half-space. The
first build's reach-back refusal is gone. A candidate refused in band,
on a saddle, on contradictory readings or by a piece that did not build
leaves the touch `InBand`, `Degenerate`, `Unanalysed` or
`PieceInBand`, never `Crossing`. `blocks` is unchanged.

### S6′: the tolerance of arm 2's argument

This is written at arm 2's loop. Within the ball about `p` that reaches
the nearest boundary of the pieces read, any overlap is confined to a
slab at most 2·`zero` thick, which is coincidence under D4. Beyond it,
an overlap deeper than the band is a dip of a face, and it shows either
at a vertex the probe reads or at a crossing that stands as a finding.

**Measured by the dual review.** A Rest tolerates exactly that slab: a
dip of 2·`zero` clears, and it cleared at base too. Beyond the slab,
neither reviewer's sweep (17,052 and 50,624 poses) found a wrong clear.

### An edge in a face is read inside the overlap

`TouchSite::EdgeInFace` used to read both stars at the edge's first end
`p0`. That point is not inside the edge, so the edge star there is not
the edge's, and it need not be inside the face. It now reads them at
the midpoint of the first overlap cell, the same cells
`ef_overlap_lane` reports (`ef_overlap_cells`, factored out of it). A
refusal raised while the cells are re-derived refuses the touch
(`TouchInBand`) rather than being dropped. That covers a declared
record's site too, where no finding was made first.

This re-derives the cells once per edge-in-face site. It is linear in
the face's boundary per site, not quadratic per pair, and carrying the
cell's point on the finding would change `CensusContact`, so it is left
as is.

### Refusal text

`TouchUnanalysed` used to say only "a corner that is neither convex nor
concave". What reaches it now also includes a face that folds back on
itself at the touch, or whose piece does not build on decided readings.
The message names both, within the 75-word budget. A piece that fails
IN BAND has its own reason, `TouchPieceInBand` ("a face of one, seen
from the touch, has corners or edges too nearly in line to trace at
this tolerance"), so that "too close to call" is not claimed of a
construction. Both keep the no-way-through form of their siblings,
since neither is a kernel defect.

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
- `the_touch_analysis_decides_only_through_its_doors`: the S4 source
  row (the fix pass rebuilt it on the `metric` door; see S4).
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
- `contact7_touch_sweeps` (integration): the rebuilt sweeps below.

**New in the fix pass.**
- `a_brick_on_a_u_channel_rests_past_an_edge_behind_the_touch` and
  `a_brick_on_a_comb_rests_past_edges_behind_the_touch`: the reviewers'
  two repros of the MAJOR. Each has a probe ray running along a
  collinear edge behind the touch point, at an edge-in-face site (both)
  and at a vertex-on-face site (the comb). Every touch is a rest and the
  pair clears.
- `a_guest_seated_into_a_reflex_edge_is_not_a_rest`: the reviewers' guest
  prism seated into the L's reflex edge. The material side is read at
  an edge on the plane through its convexity, and the touch reads
  `Crossing`, never `Rest`.
- `contradictory_convexity_orders_refuse`: a convex wedge, and the same
  wedge with one face's normal turned over. The two orders then read
  convex and reflex, and the edge refuses as `Degenerate`.
- `either_order_decides_a_slivers_convexity`: a 3 cm face bent
  `15·zero`/m from a 1 m face, listed both ways round. It reads convex
  either way, because only one order decides.
- `a_rotated_comb_and_channel_sweep_clears_no_overlap`: below.

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

## The dual review's mutants, and the rows that kill them

Each mutant was applied alone to `census.rs`, and the census and contact
rows were run under nextest.

| mutant | rows it turns red |
|---|---|
| (a) `within` reads an on-plane Reflex edge as Convex | `a_guest_seated_into_a_reflex_edge_is_not_a_rest` |
| (b) `Star::shape` ignores a reflex face corner | no longer exists: the corner reading is deleted (S3 above: unobservable, argued at `Star::shape`) |
| (c) `dihedral` reads contradictory orders as Convex | `contradictory_convexity_orders_refuse` |
| (d) the dihedral read in one order only | `either_order_decides_a_slivers_convexity`, `contradictory_convexity_orders_refuse` |
| (e) `corner` skipped | no longer exists (as (b)) |
| (f) the Complement accepts a saddle `outer` | no longer exists: the guard is deleted, and the test is proved sound for any `outer` at the site (S4 above) |
| MAJOR restored (no behind-`p` skip) | `a_brick_on_a_u_channel_rests_past_an_edge_behind_the_touch`, `a_brick_on_a_comb_rests_past_edges_behind_the_touch` |
| a `Margin::over_lever` in `within` | `the_touch_analysis_decides_only_through_its_doors` |
| `decide(…, Margin::of(x·k), …)` in `within` | `the_touch_analysis_decides_only_through_its_doors` |
| each face read whole, not through its piece | 9 rows: `a_notched_bracket_rests_against_the_wall`, `a_holed_block_rests_beside_a_brick`, `every_touch_kind_reads_rest_and_crossing`, `ordinary_rests_stay_rests`, four `contact1_touch_cones` rows, and a `contact7` sweep |

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
against base (the merge base with `origin/main`, `afaf7fec3`, for the
fix pass). The same committed rows were run on both sides, at ε unset,
1e-6 and 1e-12, and all three rows gave the same numbers on each side.

| sweep | poses | base: wrong clears / false refusals | head: wrong clears / false refusals |
|---|---|---|---|
| CONTACT-5's brick grid (`a_grid_of_brick_pairs_clears_exactly_the_rests`) | 1000 | 0 / 0 | 0 / 0 |
| rotated prisms (`a_rotated_bracket_and_brick_sweep_clears_no_overlap`) | 432 | 0 / 12 | 0 / 12 |
| crossed ridges (`a_crossed_ridge_sweep_clears_no_overlap`) | 135 | 0 / 0 | 0 / 0 |
| comb and channel (`a_rotated_comb_and_channel_sweep_clears_no_overlap`, fix pass) | 927 | 0 / 18 | 0 / 18 |

**The false refusals are the same poses on both sides, each with a named
reason, and each row pins them by shape, reason and count.**
- Rotated prisms, 12: a brick seated in the bracket's inner corner on
  the floor. It meets the saddle corner `(1, 1)`, which no test
  certifies (filed: `a-touch-at-a-saddle-corner-refuses-unanalysed`).
- Comb and channel, 18: a brick filling a slot (the channel's `[1, 2] ×
  [1, 2]`, the comb's gap `[0.5, 1.5] × [1, 2]`).
  - 12 of them sit on the slot's floor at its inner corners, the same
    saddle.
  - 6 are off the floor, with every corner on the host's boundary. The
    probe cannot place them (`AllOn`).

**The dual review's false-refusal MAJOR.** Before the fix pass, an edge
collinear with a probe ray's line but lying behind the touch point
refused the piece as in band. The reviewers measured +534 and +344
false refusals against base on their sweeps, all of this class. The
comb-and-channel sweep is built on the shapes their repros use. Every
touch there on the collinear slot floor or tooth roots is a probe ray
running along an edge behind the point, and head now equals base on it.

**What the rebuilt sweeps are, and are not.** Neither reviewer's harness
was committed, and their records give only counts. The two sweeps are
rebuilt from the descriptions in CONTACT-1's and CONTACT-5's Closed
sections and are now committed rows. They are NOT the original sweeps:
- CONTACT-5's R1 rotated-prism sweep was 480 poses with 0 false
  refusals at its head. This one is 432 poses of an L-bracket against a
  brick. Its 12 false refusals (the same at base) are the floor-corner
  saddle poses, which that sweep either did not pose or did not count.
- CONTACT-1's R2 falsifier was about 5.7k poses of bricks, mirrored L's
  and tilted parallelepipeds. None of it is reproduced here beyond the
  L-bracket against bricks under three rotations.
- CONTACT-5's R2 crossed-ridge sweep was 135 poses and so is this one.
  The pose set (crossing points, angles, shifts, lifts) is my reading of
  its description, not its code.
- The dual review of this unit ran its own falsifiers: 17,052 and
  50,624 poses, and 114,260 and 76,136 random pieces. They found 0
  wrong clears beyond the stated 2·`zero` slab, and 0 containment or
  neighbourhood violations.

These sweeps do not separate head from base. The lever design had no
end-to-end wrong clear in any measured pose (CONTACT-1's closing note),
and the difference is local. The local rows and the mutant table above
show it.

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
