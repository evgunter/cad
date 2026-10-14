---
id: a-concave-graze-of-a-curved-wall-refuses-for-reasons-other-than-its-knife-edge
kind: issue
title: a concave graze of a curved wall refuses as a corrupt body, a degenerate cone or a join invariant instead of its knife edge
status: closed
opened: 2026-10-02
priority: P2
cost: M
branch: cleave/concave-graze
closed: 2026-10-06
pr: 4098
---


## What

A plane tangent to a hole's wall from inside has an honest refusal:
the hole's piece would meet the cut face along a knife edge the split
cannot declare, `SplitFinishError::SectionCusp`
(`wedge_end_doors::a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint`).
Rule (b) sends the graze's entries across for that reason
(`splitting/rules.rs`, `wall_graze`). Only some concave grazes reach
that refusal. The rest refuse with a payload that names something
else, and one names the body corrupt.

Measured with the fixtures of
`crates/sweep/tests/split_tangent_edge_curved.rs`. Every row below
gives the same payload with and without the convex-graze change, so
none of them comes from it:

- Round hole (radius 0.5 in a 4 × 4 plate), grazed at its seam
  (x = 0.5), either normal: `Finish(SectionCusp)`. This is the honest
  refusal.
- The same hole, grazed along a ruling (y = 0.5):
  `Join(DegenerateSection)` with +y, and
  `Join(SectionInvariant { what: "tangent section chord endpoints
  coincide along the ruling" })` with −y.
- Conical socket (`a_concave_graze_of_a_revolved_hole_refuses`),
  plane tangent along a ruling:
  - u = ±z, s = +1: `Finish(Corrupt)`, "the finish traversal failed
    (corrupt body)", on a valid revolved operand;
  - u = +x (the seam), s = −1: `Join(Section { source:
    DegenerateOperand { what: "the cone's half-angle does not
    definitely open off its axis, …" } })`, on a cone of half-angle
    atan(1/2);
  - u = +x, s = +1 and u = −x, s = +1: `Join(DegenerateSection)`;
  - otherwise: `Join(SectionInvariant)`, as for the hole.

- More poses, from CLEAVE DR-51's review rows (`review-tests/dr51`):
  - the round hole off-axis (θ ∈ {0.3, 1.1, 2, 2.9, 4, 5.5}):
    `DegenerateSection` with s = +1 and `SectionInvariant` with
    s = −1;
  - a counterbore (r = 0.5 under r = 1, revolved), plane tangent to
    the narrow bore's wall: `Finish(Corrupt)` at θ ∈ {0.3, 1.1, 2, 4};
  - a rounded-rectangle hole (fillet-door corners, r = 0.5) grazed
    coplanar with a flat (φ = 0 or π/2):
    `Finish(Euler(RechartUndescribed))` with s = +1.

`Finish(Corrupt)` is the worst of these, because it blames the
operand. The half-angle text names a property the cone does not have.

## Where to look

The raising sites have not been traced. The section the plane makes
there is a ruling line through the apex's side of the cone, so the
section lane's apex branch is the first suspect for the half-angle
text. `Finish(Corrupt)` comes from one of `finish.rs`'s
`SplitFinishError::Corrupt` sites.

## Found by

`cleave/convex-graze`'s measurement of the concave guards.

## Built (branch cleave/concave-graze)

**Re-measured on main (9d03eda), default ε.** Every pose of this row and
of DR-51's concave rows, both normals: the round hole (both loop
orientations) at the ten azimuths of `THETAS`, the conical socket and the
counterbore at the same ten, and the filleted hole at φ ∈ {0, 0.3, π/4,
1.2, π/2}, 90 poses in all. Each run of the pipeline was instrumented: the
direct run, the mirrored rerun and the third run that surfaces the direct
refusal, each wrong payload's raising site by backtrace, and rule (b)'s
concave verdicts. Main has moved since the row was written:

- the socket's seam pose (u = +x, s = −1) no longer refuses on the cone's
  half-angle. It refuses
  `Join(Euler(Certification { IntervalNotForward }))`, and so do 16
  other poses, all with s = −1 (the holes at θ ∈ {3π/2, 4, 5.5}, seven socket
  and four counterbore poses);
- the socket at θ = 2, s = +1: `Join(UnpairedLooseEnds { count: 2 })`;
- the socket at θ = 0.3: `Reduce(SliverSector)`.

The 90 on main: `SectionCusp` 10 (the hole's seams, the filleted flats
with s = −1), `DegenerateSection` 22, `SectionInvariant` 20,
`IntervalNotForward` 17, `Corrupt` 14, `SliverSector` 4,
`RechartUndescribed` 2, `UnpairedLooseEnds` 1.

**Where each wrong payload is raised.** Every pose of the hole, the socket
and the counterbore passed through rule (b)'s concave verdict
(`rules::wall_graze`, `(false, OutOfMaterial)`): 2 or 4 per run. That
verdict sent the entry opposite `S`, minting a null edge for a contact the
section has no polygon for, and the join then misread it:

- `Join(DegenerateSection)` at `Sweep::certify_section_area`: the contact
  closed a zero-area polygon. The public error is the direct run's, after
  the mirror refused differently;
- `Join(SectionInvariant { "tangent section chord endpoints coincide
  along the ruling" })` at `chord_spec`'s tangent lane
  (`split_tangent_chord_forward` decided Zero). The join asked for a chord
  along the ruling between two vertices at one point;
- `Join(Euler(Certification { IntervalNotForward }))` while certifying
  such a chord;
- `Finish(Corrupt)`, all 14 poses (socket and counterbore, s = +1), at
  one site: `below_chord_u_ref`'s `chord_u_ref(..).ok_or(Corrupt)`. The
  join had completed a "section" whose loop was the hole's rim circle, one
  vertex, one half-edge. Its area passed (the circle's excess) and the
  finish found no first chord. The operand was valid; the join's polygon
  was wrong.

The filleted hole's flat poses (φ = 0, π/2) never reach rule (b), because
the flat's sector is rule (a)'s. With s = −1 they refuse `SectionCusp`
honestly. With s = +1 they refused `Finish(Euler(RechartUndescribed))` at
the section promotion: the ring loop's face was re-charted onto the
section plane with no restatements (`&[]`), while the outer loop's face
got `section_plane_restatements`. The smooth flat/corner edge, still
described in the chart the face left, was stranded whenever it fell on
the ring side.

**What landed** (after the DR-4098 fix pass).

- One knife-edge refusal, `KnifeEdge { wall, at }` (`at` names the edge
  along the contact, or the vertex where the contact crosses a rim). It is
  carried by `SplitReduceError::KnifeEdge` and by
  `SplitFinishError::KnifeEdge`, which replaces `SectionCusp`. It has one
  Display, and `SplitError::knife_edge` reads either.
  `plane_section` refuses the same contact as
  `SectionError::KnifeEdge`, with its own text.
- Rule (a) reads every wall it finds tangent at an ON vertex, and refuses
  one that bends away from its material. That covers the off-seam
  duplicates, the seams, and the walls beside a face in the plane (the
  filleted flats, a declared cove, a notch's end), before any section is
  built. Rule (b) holds only convex grazes.
- Both faces of a promoted null pair are re-charted in one call, with
  both faces' restatements.

**After.** At default ε, 88 of the 90 poses give `Reduce(KnifeEdge)`. The
filleted flats give it along their flat/wall edge, and the seams along the
seam. The remaining 2 are the filleted hole at φ = 1.2,
`Reduce(SliverSector)`, the band artifact of
`a-convex-graze-of-a-cone-refuses-at-some-azimuths` (its evidence is added
there). At 1e-6 φ = 1.2 gives the knife edge too. At 1e-12 it gives a
certification refusal. These are pinned by
`split_tangent_edge_curved::{a_concave_graze_of_a_round_hole_refuses,
a_concave_graze_of_a_revolved_hole_refuses,
a_concave_graze_of_a_filleted_hole_refuses}` and by
`wedge_end_doors::a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint`.
The pins check the payload, the wall's kind, and the contact edge lying in
the plane.

The poses of this subject that still refuse for another reason are filed:
`an-in-band-concave-graze-refuses-at-certification-on-one-side-of-tangency`.

No concave graze can build: each one is a knife edge, which D10 holds
undeclarable for a split. Nothing here declares contact.

## Closed (PR 4098, 2026-10-06)

A knife edge the split cannot make is one decision with one payload. `KnifeEdge { wall, at }`, with
one `Display` and one recourse, is carried by the reduce phase (rule (a), before any section is built)
and by the finish phase's in-band backstop. `plane_section` tells the same fact in its own words.
Review tier: single FULL, then a delta review of the fix pass. Both returned APPROVE-WITH-FIXES with no
MAJOR, and no split main built now refuses (about 1,330 poses × 3 ε plus four crates' suites). The fix
pass also builds four along-a-face-plane splits that refused on main.
