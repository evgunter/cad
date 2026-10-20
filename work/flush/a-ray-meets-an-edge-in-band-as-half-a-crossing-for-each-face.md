---
id: a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face
kind: unit
title: A ray that passes within the band of an edge meets it, half a crossing for each face the edge bounds, and reads the role off its signed count (D10, Ev on PR 5119)
status: closed
opened: 2026-10-15
priority: P2
cost: H
branch: flush/edge-meeting-ray
pr: 5422
closed: 2026-10-20
---


The ray half of Ev's ruling on the needle class (PR 5119, 2026-10-15,
fork-log row 176). The volume half landed in PR 5119 itself.

**Ratified text.** D10 "Booleans" says: "A ray that passes within the
band of an edge meets it, half a crossing for each face the edge bounds
(Q1), and reads the role as the sign of its count of crossings; it
refuses only on two faces that do not meet whose crossings lie in band
of each other, a pair, and a ray that meets a corner or a wedge reads
nothing." Q1 says an instrument does not represent a tie-break the band
forbids it to make.

**Today.**
- Each face decides for itself whether a crossing lies in it. The
  in-face walk refuses a crossing within Kε of a loop boundary:
  `point_in_loop` (`crates/topo/src/splitting/containment.rs:1883`),
  reached through `point_in_loop_projected`
  (`crates/topo/src/boolean/solid_contain.rs:3930`).
- So a ray aimed at a face narrower than 2Kε is set aside, and so is
  one landing near any edge. The reader
  (`crates/topo/src/side_reader.rs:439`, `side_at_infinity`) then
  falls back to the volume.
- The same walk serves `point_in_solid`
  (`crates/topo/src/boolean/solid_contain.rs:3967`), so needle
  operands lose rays at the boolean's containment fallback too.

**What is owed.**
1. **Planar faces first.** A crossing within the band of an edge meets
   that edge, and counts half a crossing for each face the edge bounds;
   a crossing within the band of a vertex reads nothing. The ray reads
   the role as the sign of its signed count (+1 entering material, −1
   leaving it); crossings in band of each other are summed, not
   ordered.
   - It refuses only on a pair: two faces that do not meet
     (`side_reader.rs:347`, `meet`).
   - The designers measured an edge-by-edge probe reading all 20
     needle cases (w ∈ {10.5, 12, 15, 21, 30}ε, ε 1e-9 and 1e-6, both
     orientations). It is on branch `flush/needle-design-x` (head `feb5b660e2`) (fork-log
     row 176).
2. **Curved faces.**
   - Two faces on one carrier (a seam split) share one comparison of
     the root against the seam parameter.
   - An edge between two carriers needs one number per edge and ray:
     the ray's signed offset from the edge near the crossing.
3. **`point_in_solid` reads through the same walk,** so its needle
   operands keep their rays. That is CLEAVE's
   `point-in-solid-sets-a-ray-aside-near-a-narrow-faces-edge`, which
   this unit closes when it lands; settle the split with CLEAVE before
   dispatch.
4. **Re-measure:**
   - the needle row's fixtures;
   - the `brick ∩ brick` needle poses;
   - the 1e-12 torus (`p3b_a_band_scale_major_radius_through_the_revolve_door`);
   - the five batteries.

   Re-pin whatever moves, and say what moved.

**Done when** a ray within the band of an edge meets it; set-asides
are left only at vertices, wedges and real pairs; and the fixtures
above read by ray rather than by the volume fallback.

**It also retires the aims' exact ties at symmetric nominals**
(`work/flush/the-side-readers-aims-decide-exact-ties-at-a-documents-nominal.md`):
a planar aim's `point_in_loop_arm` and a wall aim's `curve_foot_branch`
read exactly 0 at `die_chamfer`'s and `cut_cylinder`'s nominals, read
apart today as the reader's family.

## Measured (main at `f717fb9bb`)

Probes: `crates/topo/tests/edge_meeting_ray_probes.rs` (ignored; the side
reader's aimed rays per solid through `test_support::aimed_rays`, and
`point_in_solid` at points about each needle) and
`crates/sweep/tests/shell7_r2_probes.rs` `p3b_probe_the_tori_aimed_rays`.
- **Needles** `brick (0,1)×(0,w)×(0,w)`, along the axes and turned, both
  ways out, at ε 1e-9 and 1e-6: at 10.5ε, 12ε and 15ε no face offers an
  aim (`planar_interior_point` certifies no point more than Kε inside); at
  21ε and 30ε every ray but one is set aside on a crossing in band of a
  face's boundary (`point_in_loop` `Boundary`). The aimed rays read the
  turned 21ε needle, both ways out, and at ε 1e-6 the inside-out turned
  30ε; the volume reads the rest.
- **`brick ∩ brick` needles**, the four poses of `shell_role_reader`: at
  2Kε no ray reads (no aim, or a crossing in band of a boundary); at 3Kε
  one pose of four reads at ε 1e-9, three at ε 1e-6.
- **The 1e-12 torus** (R 1e-10, r 2e-11): both aims are set aside in band
  of the ray's quartic ladder (`bool_ray_torus_split`), not of an edge.
- **`point_in_solid`** about the needles: see CLEAVE's
  `point-in-solid-sets-a-ray-aside-near-a-narrow-faces-edge`.

## Fix

- **One walk, edge by edge** (`boolean/solid_contain.rs`,
  `ray_crossings`, `edge_meets`): each line edge between two planar faces
  whose planes the ray crosses is decided once, for both faces, through
  the census's closed form for two lines' closest approach, moved behind
  a door both call (`census::line_passes`). The ray meets an edge it
  passes within the band of. Each face the edge bounds counts half a
  crossing: `(s_A + s_B)/2`, where each `s` is ±1. A face with a met edge
  offers no crossing of its own. A face clear of all its edges is
  crossed where the turn of its loops says (`crosses_by_turn`), which
  casts no auxiliary direction. A face with an edge no such reading
  decides keeps its in-face walk.
- **A corner or a wedge reads nothing** (`RayHit::mark`, `Mark`). A
  vertex in band of the ray is a corner, and one on it a graze. A face
  the ray meets at two of its edges is a wedge (`SetAsideWhy::Wedge`): the
  half rule would count that face once at each edge, where the ray
  crosses it once or not at all. A mark matters only where the count
  reaches it. What the wedge prevents, measured on the fix pass with the
  rule switched off (both reviews of PR 5422 measured it first, and
  found the first build's stated reason, the inside-out 30ε needle,
  false: that needle reads `In` without the rule once spans are in):
  - `point_in_solid` reads a point outside a turned needle `In`: one
    point wrong in each of 12 of R1's 72 fuzz rows (the 12ε, trapezoid
    and 21ε needles and the sliver wedge, R1's poses 1 and 2, both ways
    out); pinned by `narrow_needle_containment`'s
    `r1s_witnesses_outside_a_turned_needle_read_outside`, red without
    the rule;
  - `partners_along_the_face_build_sound_against_the_germ` fails at
    ε 1e-6.
  The review's narrowing is taken: two met edges sharing a vertex of
  different faces are no wedge (R2 measured no fixture that needs that
  clause, and this pass built none).
- **An edge's crossing spans its faces' plane crossings**
  (`Crossings::push_span`). On a face the ray meets obliquely, the plane
  crossing lies bands away from the edge, and anything between the two
  is summed with them. A met edge counts only with both its planes
  crossed ahead of `q`.
- **The count** (`ray_walk::Crossings::first_count`): crossings in band
  of each other are summed, and the sign of the first nonzero running
  count is the side. A pair (`RayHit::pair`) is two faces that do not
  meet (`meet`), each crossed, met in band of each other along the ray,
  each party read where the ray meets its faces: a face's crossing, or
  where the ray passes an edge; an edge passed without crossing (count
  0) is no party. Not where the ray crosses their carriers: on a shallow
  ray a carrier crossing lands far off its face, beside another face
  (R2's re-check of `2be7c1ad14`: a wall, a gap of 3Kε–8Kε, a box whose
  top edge the ray passes; 8 of 9 rays refused on the wall and the top's
  carrier, ~3ε apart; `an_oblique_carrier_crossing_beside_another_face_is_no_pair`,
  all 9 `In` at both ε). Two cubes 12ε–19ε apart, whose top edges a ray
  passes in band across the gap's mouth, are no pair
  (`two_edges_of_faces_definite_apart_are_no_pair`). A corner or wedge in the same
  cluster reads before a pair: D10 refuses only on a pair, a
  restriction of refusal and not a demand for it, and a ray meeting a
  corner or a wedge reads nothing
  (`the_count_reads_its_first_turn_and_a_mark_before_a_pair`).
- **Aims** (`side_reader.rs`): a line-bounded planar face aims at its
  loop's mean point, which asks nothing. The incidence is 10° off the
  normal, not 60°, so a narrow face is as wide to the ray as it is.
- `point_in_solid` reads through the same walk and count (`cast_ray`).

## Measured with the fix

- **Needles**, at ε 1e-9 and 1e-6 alike: every needle 21ε and 30ε
  across, along the axes and turned, both ways out, reads by ray. Under
  2Kε every ray through a needle meets a wedge and reads nothing, except
  the inside-out needle along the axes at 10.5ε to 15ε, which reads `In`.
  The volume reads the rest, as on main. Every reading is right.
- **`brick ∩ brick`**: every 3Kε pose reads `Out` by ray at both ε. At
  2Kε every ray meets a wedge, and the volume reads the result, as on
  main.
- **The torus**: unchanged, in band of the quartic
  (`bool_ray_torus_odd`). Filed as
  `a-small-toruss-aims-land-on-its-faces-shared-seam-where-the-quartic-reads-in-band`.
- **The five batteries** (`join1_delta_arc_battery`,
  `corner_pairs_battery`, `pinch_runs_battery`, `dbl_battery`,
  `review_4737_span_battery`), release, line-diffed against the merge
  base: 0 lines move (nextest's `SLOW` timing lines aside).
- **The aims' exact ties**: `die_chamfer`'s 2 exact `point_in_loop_arm`
  zeros are gone; `cut_cylinder`'s curved ones remain (that row's
  `## Measured on the edge-meeting ray`).

## Not built

The curved faces. A seam split's one comparison and a carrier pair's
signed offset are
`a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim`.

## Resolution (2026-10-20, PR 5422)

The planar half landed:
- A ray within the band of a line edge between two planar faces meets it once, half a crossing for each face. The read is the census's `ee_lines_closest`, behind `census::line_passes`.
- A face met at two of its edges is a wedge and reads nothing; a vertex in band is a corner.
- A pair's parties are read where the ray meets their faces.
- `point_in_solid` reads through the same walk, which closes CLEAVE's point-in-solid row.
- The needles of 21ε and 30ε and `brick ∩ brick` at 3Kε read by ray. Needles under 2Kε are read by the volume, as D10 allows where no ray reads.

**Review.** A dual concurrent review found no MAJOR. Its fix pass:
- the wedge rule's false reason replaced, the rule narrowed and pinned on R1's witnesses;
- two false pairs fixed and pinned: definite-apart cubes, and an oblique carrier crossing;
- marks before pairs documented;
- the vertex read enclosed.

The second reviewer's re-checks approved. Batteries: 0 lines moved.

**Residues:**
- `a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim` (P2, H), the curved half;
- `a-small-toruss-aims-land-on-its-faces-shared-seam-where-the-quartic-reads-in-band` (FLUSHTAIL);
- `d10-says-no-word-on-what-a-wedge-is-or-how-far-a-rays-refusal-reaches` (FLUSHTAIL, design): the wedge's definition, the ~3Kε refusal reach and the winding-number question, for Ev;
- the aims' exact ties, parked on the curved half.

