---
id: rim-free-loop-on-a-poleless-chart-meshes-as-a-hole
kind: issue
title: mesh: a curved face whose loop is meridians only on a chart with no pole walks to zero width and meshes as a hole
status: open
opened: 2026-09-18
priority: P0
cost: D
parent: TESS-5
---


Found by TESS-1's torus measurement (spec deliverable 4) and its sweep
(discipline §5). It is the zero-WIDTH twin of the zero-height class
TESS-1 closed, and TESS-1's structural fact does not reach it.

**Measured, 2026-09-18** (default ε, local, TESS-1's branch). The body:
`witness_bodies::one_circle_cut` (`crates/mesh/tests/common/`) on a
torus about +Z, R = 2, r = 0.5, cut along the MINOR circle at `u = 0`
(`center (2,0,0)`, `axis −Y`, `radius 0.5`), both faces on the torus —
so each face's loop is two half arcs of one meridian and nothing else.

- `topo::validate` and `validate_closed` pass; `validate_geometric`
  refuses `VolumeUncomputable { NotIsoRectangle { "curved face without
  a rim (non-sphere)" } }` — the flux lane (`props::curved_face`)
  refuses the face.
- The doors `curved::require_iso_rectangle_face` cites both ADMIT it:
  `props::require_iso_rectangle` = `Ok`, `require_one_chart_branch` =
  `Ok`. That is the door's documented posture, not a props defect: its
  doc says extent is not a shape question and "a consumer that cannot
  mesh a zero-area face refuses it on its own terms".
- `mesh::tessellate`, debug assertions on: panics at the cross-face
  census, a chord segment an edge of 0 face triangles. Assertions off
  (`CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=false`): `Ok`, patches of
  `[0, 0]` triangles, and `check_mesh` = `Ok(())`
  (`check-mesh-passes-the-empty-mesh`).

**Mechanism, by reading.** Every traversal classifies `Meridian`, so
`walk::loop_polygon` takes its rim-free arm; both arcs carry one `u`,
the polygon has zero width, `curved::require_swept_rectangle` passes
(every entry is on its degenerate box) and the CDT has no inner face.

**Why TESS-1's fact does not transfer.** TESS-1 refuses a loop with no
`TravKind::Meridian`. The mirror fact — no `TravKind::Rim` — is not a
refusal: on a sphere that loop is the pole-to-pole band, which the lane
meshes, taking its u-extent from its two pole junctions. The structural
fact this member wants is narrower — no rim AND a chart with no pole
for a meridian to end on (`Chart::poles()` is empty for a cylinder and
a torus) — and a sphere band whose two meridians sit on ONE column (a
full-height slit) has zero width with poles present, so that fact would
not close the class either. The unit that takes this has to find the
statement; TESS-1 did not guess at one.

**Reach.** Unlike the rim-only sphere cap this body is NOT tier-3
valid, and no one-loop all-meridian torus face can be: a torus cut
along one circle is an annulus, which needs two loops, and a second
loop is `RingOnCurvedFace`. So the reach is the Euler door and any
caller that meshes without validating; `tessellate`'s contract does not
re-validate. The all-RIM torus loop is refused already, typed, at the
shape door (`"torus face without a meridian"` — TESS-1's row
`a_rim_only_torus_face_refuses_at_the_shape_door_on_the_same_fact`).

Not measured: the cylinder and cone members (a loop of generators
only).

## The one-column sphere band, measured (TESS-1 review, 2026-09-19/20)

A reviewer of TESS-1 constructed the member this row had left
unconstructed, and TESS-1's fix pass landed it as a row:
`crates/mesh/tests/loops_the_meridian_guard_admits.rs`,
`the_one_seam_sphere_is_a_zero_width_band_the_census_reports`. The
body is the one-face, one-seam sphere (V2 / E1 / F1): `mvfs` at the
north pole, one `mev` along a great-circle arc to the south pole, so
the face's loop is that meridian walked down and back. Reported by the
reviewer: tier 3 refuses the body; with debug assertions on
`tessellate` panics at the cross-face census; with them off it returns
`Ok` with `patches = [0]` and `check_mesh = Ok(())`. The landed row
asserts the tier-3 refusal and the census panic (the row is gated on
debug assertions; the assertions-off half is the reviewer's
measurement and is not re-run by any row).

**This settles the question the paragraph above left open.** Both
poles are junctions of that loop, so the narrower statement floated
there — "no rim AND a chart with no pole" — would not close the class:
here there is no rim, there ARE poles, and the domain still has zero
width. The structural fact this class wants is about the COLUMNS the
meridians stand on (all of them one column), which is a coordinate
comparison unless it can be read off incidence (one edge used twice by
one loop, or every meridian incident to the same two pole vertices
with no rim between them). The unit that takes this row starts there.

The same file also pins the spur disguises of the rim-only cap (a
meridian strut from the rim toward the pole, walked up and back): they
carry a meridian, so TESS-1's guard admits them, and the row records
what answers instead.

The title's "on a chart with no pole" is therefore too narrow — the
sphere member has poles. Left as filed (the id is the file name);
read it as "a rim-free loop whose meridians share one column".

## Correction to the two `check_mesh = Ok(())` readings (TESS-4, 2026-09-22)

The two measurements above — the two-face torus (`patches = [0, 0]`)
and the one-seam sphere (`patches = [0]`), both with debug assertions
off — record `check_mesh = Ok(())`. As of TESS-4 both are
`Err(MeshError::NoTriangles)`: every patch is empty, so the whole-mesh
arm answers first. `EmptyPatch { face }` is the verdict for a mixed
mesh, which neither of these is. Nothing else in either reading moves —
the `Ok` from `tessellate`, the patch counts and the census panic with
assertions on are unchanged, and the hole is still a hole.

Not re-executed. The assertions-off run the originals needed is not
needed for this: `check_mesh`'s first act is a `triangle_count == 0`
compare, so the verdict follows from the recorded patch counts alone.
The same mesh shape is pinned hand-built by
`survives_checkmesh_refuses_the_empty_mesh`'s all-patches-empty arm, so
the claim has a row that goes red if it stops holding.

## The fact, found and enforced (TESS-5, 2026-09-22)

**The statement is about the EDGES the loop's iso sides open, not about
poles and not about columns.** A rim-free loop's whole u-extent comes
from its iso-side OPENINGS: a continuation repeats `prev_u` bitwise
(#653), so only an opening can put a new column in the polygon. And a
column belongs to an EDGE, not to a traversal —
`topo::chart_iso::classify_kind` reads the edge's carrier and its stored
span, and the walking direction touches neither, so a seam walked down
and back states one column twice, bitwise. Together: the extent is the
spread of the columns of the DISTINCT edges that open the loop's iso
sides, and a loop that opens them all on one edge has an extent of
exactly zero.

`mesh::walk::require_two_columns` refuses that state
`TessellateError::SingleColumnCurvedFace { face, surface }`, in every
profile, before anything is emitted. It reads the loop's incidence and
nothing else: no column is compared, and no width, area or triangle
count is read. The ε in it is the iso-side rule's own separation band —
the guard is exact to the resolution of the computation it guards, which
is the most a premise check can be. On a cylinder or torus chart, whose
axis lies off the surface, `Eps::separates` is still called but cannot
answer `false` for any ε below the chart's radius, so the verdict there
is the band's only in form.

**Closed by it** (rows: `crates/mesh/tests/loops_with_no_rim.rs`):

- the torus face bounded by one meridian circle — this row's own body;
- the one-face, one-seam sphere — the member the paragraph above said a
  "no rim and no pole" fact could not close, and the reason the
  edge-identity rung closes it is that both of its openings state the
  same edge's column;
- the cylinder face bounded by one generator (built here for the first
  time), which used to answer `MissingEntity { "degenerate curved
  boundary" }` from the count guard behind the walk;
- the one-face cone whose loop is one generator from the apex, also
  built here for the first time.

Positive controls, unchanged and rowed: the ball's two pole-to-pole
bands and the rimless lune, whose two openings stand on two edges.

**Not closed by it, and why that is right.** The sphere cut along a
whole GREAT CIRCLE through both poles — two hemispheres, tier-3 VALID —
is refused one door earlier, `UnsupportedCurvedShape` from props'
`require_one_chart_branch`: each bounding arc carries a pole in its
interior, where the azimuth it holds constant jumps by π. Measured, and
rowed at
`loops_with_no_rim::a_hemisphere_pair_is_refused_at_the_branch_door_not_by_the_walk`.
Its recourse is to state the poles as vertices, which turns each face
into the two-column band the lane meshes.

**What the class does NOT reach**, unchanged: the trimmed and planar
lanes' empty patches are
`trimmed-and-planar-lanes-answer-ok-on-an-empty-patch`'s, and
`check_mesh`'s verdict on a mesh of nothing is
`check-mesh-passes-the-empty-mesh`'s.

**Remaining residue: REACHABLE, and it is not this row's any more.** The
sweep named the blind spot — two openings on two edges stating one column
— and argued it would need two coincident edges, which is not a valid
body. TESS-5's reviewer built it through the Euler doors, so the argument
was wrong: a sphere slit bounded by two coincident edges satisfies the
guard's premise and still walks to zero width (census panic with
assertions on, `Ok` with `patches [0, 0]` and `check_mesh
Err(NoTriangles)` without). Filed as its own row with the missing rung
named — carrier identity, above edge identity:
`work/tess/two-coincident-edges-open-two-columns-that-are-one.md`, and
rowed where it is reached in `crates/mesh/tests/loops_with_no_rim.rs`.

**So: the four members above close with TESS-5; that member does not.**
This row closes on its four, and the coincident-edge member is tracked
next door. The corpus measurement that stood in for the argument — every
rim-free walk in the `mesh` suite, 237 of them, all with exactly 2
openings and none narrower than 0.2 rad — says the corpus has no
degenerate admitted loop, which is now a weaker claim than the row next
door and is kept only as that. It has no committed apparatus and cannot
have one: the polygon is `pub(crate)` and no seam exposes it, so the
measurement is a three-line `eprintln` at the rim-free arm plus
`cargo test -p mesh --test all -- --nocapture`, re-taken by hand.
