---
id: a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim
kind: issue
title: A ray in band of an edge a curved face bounds, or of a planar face's curved edge, is still set aside by each face's own trim walk
status: closed
opened: 2026-10-19
priority: P2
cost: H
branch: flush/curved-edge-ray
pr: 5697
closed: 2026-10-25
---


Filed by the edge-meeting ray unit
(`a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face`), which
built D10's edge-meeting ray for line edges between two planar faces.
The curved half of that row is unbuilt.

## What is left

- `boolean::solid_contain::ray_crossings` decides an edge once, for both
  faces, only where it is a line edge and both of its faces are planar
  (`edge_meets`, through `census::line_passes`). A planar face with a
  curved edge (a cylinder's cap), or one beside a curved face, still
  takes its own in-face walk (`point_in_face`, `point_in_loop_projected`),
  and every curved face its own trim (`wall_hit`,
  `point_on_cone_in_face`, the sphere and torus regions). A crossing in
  band of such a boundary sets the ray aside, as before.
- The row's two curved readings are owed:
  - two faces on one carrier (a seam split) share one comparison of the
    root against the seam parameter;
  - an edge between two carriers needs one signed offset of the ray from
    the edge near the crossing, per edge and ray.
  Where a carrier pair cannot give that number soundly, the ray refuses
  typed, never approximates.
- The curved aims' exact ties at a nominal stay with it
  (`the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`):
  `cut_cylinder`'s reader still reads 90 exact zeros at its nominal
  (`curve_foot_branch` 12, `point_in_arc_loop_side` 6, the rest structural
  wall and chart reads), measured by
  `crates/editor-core/tests/revalue_corpus.rs`
  `reader_ties_at_the_nominal`.

## The rule that carries over

A face the ray meets at two of its edges is a wedge and reads nothing;
an edge's half crossings span its faces' plane crossings (`RayHit`,
`Crossings::push_span`); a pair's parties are faces where the ray
crosses their carriers (`RayHit::pair`). A curved edge's reading has to
keep all three. Without the wedge the planar build reads a point outside
a turned needle `In` (`narrow_needle_containment`); without spans (as
the first build measured), a pyramid's result reads the wrong material
side at ε 1e-6
(`partners_along_the_face_build_sound_against_the_germ`); with an edge's
closest point as a pair's party, two cubes 12ε apart refuse on a pair
the census reads definite (`two_edges_of_faces_definite_apart_are_no_pair`).

## Measured (main at `28e9e447`)

Probes: `crates/sweep/tests/curved_edge_ray_probes.rs` (ignored), through
`topo::test_support::{aimed_rays, each_aim, edge_frames, ray_at}`. Each
body is read both ways out: its own aims (every face's, not only until
one decides), the reader's answer, and a ray aimed at the middle of every
edge, into each face, at offsets 0, Kε/3 and Kε (within the band) and
3Kε and 30Kε (clear of it).

- The ball and the torus are closed groups, whose seams no ray reads,
  and read every ray before and after.
- **The reader falls to the volume on none of them**, at ε 1e-9, 1e-6
  and 1e-12: a cylinder (two and three arcs), a boss on a box, a bored
  block, a filleted edge, a cone frustum (alone and unioned on a box), a
  ball, a hemisphere, a torus, a half torus, and discs 12ε, 21ε and 30ε
  thick. A face's own aim sits away from its edges (a planar face's
  certified interior point, a curved face's chart middle), so the first
  aimed ray reads. Aims set aside, both ways out: the bored block's two
  planar faces (a graze), the half torus's two end caps (a graze), the
  12ε disc's two (a crossing in band).
- **Rays within the band of a curved edge are set aside on a crossing in
  band of a face's own boundary**: of the rays at 0, Kε/3 and Kε, both
  ways out, at ε 1e-9, 14–27 of 72 read on a nominal body (25 of 108 on
  the three-arc cylinder; 15–17 of 48 on the fillet and the hemisphere),
  the rest set aside, 22–59 of them on such a crossing; the thin discs
  read 0–10 of 72. ε 1e-6 and 1e-12 alike (the PR's table).
- `cut_cylinder`'s reader reads **62** exact zeros at its nominal, not
  90: `curve_foot_branch` 12, `point_in_arc_loop_side` 6,
  `bool_ruled_region_roots_coaxial` 44 (`reader_ties_at_the_nominal`).
  The ruled region's rework (PR 5664) moved the rest.

## Fix

- **Every edge is decided once, for both faces it bounds**
  (`boolean/solid_contain.rs`, `ray_crossings`; the new
  `boolean/solid_contain/curved_edges.rs`).
  - A line edge between two crossed planes keeps the planar reading
    (`edge_meets`).
  - Every other edge between two faces that read their own carrier
    crossings (a crossed plane, or a trimmed curved face's roots from
    `carrier_roots`) is read from those crossings. One definitely off
    the edge's carrier (`Curve3::clearance`) is clear of it there. One
    that is not is carried to the ray's closest approach by alternate
    feet (`Curve3::foot`), and read on the edge's span as the census
    reads it (`census::edge_span`, `Foot::on_carrier`, `Span::place`).
  - On the carrier, or in the band's gap, inside the span, the ray meets
    the edge.
  - A seam inside one face is no boundary of it, and is decided as one.
- **Half a crossing for each face, at the carrier crossing it stands
  for**: a planar face's plane crossing, or a curved face's root decided
  nearest the passage from both ends of the passage's span. A met edge
  stands for a crossing only where the edge represents it; otherwise it
  reads nothing there. It represents a crossing that:
  - no other met edge stands for (keyed by surface and root order, so a
    root two half-walls share is one crossing);
  - belongs to a face whose every edge is decided;
  - on a face not every edge of which is a line edge read exactly, lies
    near the passage (`side_at_infinity_met_edge_represents` /
    `bool_point_in_solid_met_edge_represents`) or is definitely held by
    the face.

  The hit spans the passage and both carrier crossings. A planar face
  with a met edge offers no crossing of its own; a curved face offers
  none at that root.
- **The wedge keeps PR 5422's reading**: a face met at two of its edges
  (`EdgeMeets::meet`), and one edge met at two places along the ray reads
  nothing there. A face met at one edge is read again at its other edges
  from both ends of the passage (`curved_edges::narrow`).
- **A seam split**: two faces on one carrier cross it at one root.
  - Near their seam, the ray meets the seam once, half a crossing for
    each face with one sign.
  - Elsewhere, a face that definitely holds the root holds it alone. The
    others' readings, in band, faulted or not, decide nothing; a fault
    stands only where no face holds the root. The reading is order-free.
- **A fault does not turn on face order.** Where faces' readings of the
  carriers fault, the ray refuses with the highest-ranked fault
  (`RayFault::rank`, the order `Evidence` ranks by: fatal, blocked, in
  band, graze), and of two of one rank the lesser face's. A broken face
  is the query's refusal whatever another face holds.
- Vertices of curved edges are corners, as a line edge's are, save a
  closed edge's conventional vertex.

## Review (PR 5697, dual concurrent, H)

One review found two MAJORs, each a ray that reads the wrong side on a
legal body.
- **A planar face's half was read at a plane crossing outside the face.**
  The ray is shallow to the face and passes another, curved edge of it
  that read clear at the crossings: the kinked bore,
  `curved_edge_corpus::kinked_bore`; 140 of 3042 rays were wrong.
- **Two met rims each stood for one root two half-walls share**: the
  thin discs.

Both are fixed by the "represents" rule above. The other review found a
seam inside a face unread (now decided), and found the random pin blind
to both MAJORs. The pin now reads every crossing of a ray against an
analytic truth.

## Measured with the fix

- Rays at 0, Kε/3 and Kε into each face, both ways out, at ε 1e-9.
  - Nominal bodies read 40–67 of 72 (68 of 108 on the three-arc
    cylinder, 36–38 of 48 on the fillet and the hemisphere), with
    **no** crossing in band of a face's own boundary. On main they read
    14–27 of 72, with 22–59 set aside that way.
  - Discs 21ε and 30ε read 36 and 38 of 72; the 12ε disc, under 2Kε, 8.
  - ε 1e-6 and 1e-12 alike.
  - The "represents" rule costs the reads where the half would have
    counted right across the met edge itself (the cylinder's rim from
    the wall side reads 10 of 24 one way out, 16 before the rule):
    `a-ray-shallow-to-both-faces-of-a-curved-edge-reads-it-clear-at-their-crossings`.
- **No ray reads the wrong side**, at any ε:
  - every crossing of rays about every edge of the curved corpus
    (`curved_edge_corpus`), at `CAD_FUZZ_EFFORT` 1, 30 and 100;
  - 25 920 uniformly aimed rays per ε (`probe_uniform_rays_about_the_corpuss_edges`).
- The reader falls to the volume on none of the probe bodies.
- `cut_cylinder`: 62 exact zeros, of 1827 reader decisions (1719 on
  main), retired by none of this
  (`the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`).
- The five batteries, line-diffed against the merge base: see the PR.

## Pins

`crates/sweep/tests/a_ray_meets_a_curved_edge_once_for_both_faces.rs`,
over `curved_edge_corpus.rs`. Each body is asserted at tier 3′ with no
contacts at every ε row. Every crossing of a ray is held to the body's
analytic membership on either side, in two face orders, which must
also refuse with one fault kind:
- `every_crossing_of_a_ray_about_a_curved_edge_counts_right` (shape 1):
  the corpus's cheap part, the thin discs and the kinked bore included;
- `every_crossing_of_a_ray_about_the_corpuss_edges_counts_right`
  (shape 1): the whole corpus, revolves and booleans;
- `a_ray_shallow_to_a_face_past_its_curved_edge_reads_right`: the first
  review's scan of the kinked bore, 3042 rays;
- `a_root_two_half_walls_share_is_counted_once` (slow set): the thin
  discs on a pinned seed (16, 14 and 16 wrong on the first head at the
  three ε);
- `one_root_two_rims_stand_for_reads_nothing_twice`: the first review's
  ray, the fast pin of the same fault;
- `rays_in_band_of_a_curved_edge_read_it_once`: aimed rays at Kε/3 and
  Kε read the right side or nothing, never set aside on a face's own
  boundary, and at least 40% read.

## Residue

- `a-ray-shallow-to-both-faces-of-a-curved-edge-reads-it-clear-at-their-crossings`
  (FLUSH, P3), rewritten: what clear-at-the-crossings misses, and what
  the "represents" rule costs.
- `a-uniform-ray-about-a-curved-edge-is-still-set-aside-on-a-faces-own-boundary`
  (FLUSH, P3).
- `the-side-readers-aims-decide-exact-ties-at-a-documents-nominal`
  (FLUSH, P2, unparked, H).

## Resolution

Closed by PR 5697 (2026-10-25). A ray in band of any edge between two faces that read their own carrier crossings now meets that edge once, for both faces. That covers a curved edge, an edge a curved face bounds, and a seam split. The read goes through the census's own reads (`census::edge_span`, `Curve3::foot`, `Foot::on_carrier`, `Curve3::clearance`, `Span::place`).
- A met edge stands for a crossing only where it represents it: near the passage, or held definitely by the face. Otherwise it grazes.
- A carrier's root (surface, ordinal) stands once, however many faces' met edges claim it.
- The ray's fault is the highest-ranked of its faces' faults, whatever order they are read in, and a `Fatal` always propagates.

Dual review (concurrent, letter H). One review found two wrong-side readings that base set aside: a plane's half read outside the face, on a kinked bore; and a shared root counted twice, on the 30ε disc. Both were fixed, and the pins now read every crossing of a ray against an analytic truth and go red on the first head at every ε. The delta checks by both reviewers approved. Batteries 0 lines moved; suites full at 3 ε.

The aims' exact ties do not retire: `cut_cylinder` reads 62 exact zeros on base and head. `the-side-readers-aims-decide-exact-ties-at-a-documents-nominal` is unparked and re-priced (P2, H). Residue filed in FLUSHTAIL:
- `a-ray-shallow-to-both-faces-of-a-curved-edge-reads-it-clear-at-their-crossings` (P3), whose stands rule costs lost reads, never wrong ones;
- `a-uniform-ray-about-a-curved-edge-is-still-set-aside-on-a-faces-own-boundary` (P3).
