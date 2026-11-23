---
id: the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict
kind: unit
title: the embedding question reads tier 3's class and the pair's extent from the verdict, and decides neither
status: spec
opened: 2026-11-22
priority: P0
cost: H
---

## Ruling

Ev on PR 7456 (2026-11-22): "1 sounds good. 2 is also probably good,
though i'm less sure that there won't be some issue later. we can
proceed with it though". The text is `docs/DESIGN.md`
`[topology.tiers]`: where faces meet, the question reads from the
verdict what tier 3 decided there (an edge's class and turn, an end, a
pole or apex vertex) and decides none of it; over each cell it reads
the rate at which the two faces part, the faces' rates summed, levered
at how far the narrower of them runs from what they share, measured
across it, never at a cell of the question's own subdivision; where
tier 3 holds no fact, that reading is the only one. Three designers
agreed on it after five rounds (fork-log row 304; the PR body holds
their final texts and the worked cases).

It also builds the hand-over Ev approved on PR 6893, which nobody
built: the strict question takes tier 3's verdict as its input, and
`AtRestBody` keeps tier 3's per-entity facts.

## Spec

Stacked on `embed/remeter` (PR 7329), which levers the departure
margin at the faces' reach, recomputed per cell pair.

1. **The verdict keeps, per shared edge:** its class as check 4 decided
   it (a crease and its turn, a smooth seam, an end) and the pair's
   extent: how far the narrower of the two faces runs from the edge,
   measured across it. Per face, its pole or apex vertex by key (check
   6 places it). Per vertex two faces share alone, the same kind of
   extent from the vertex (`reach_from_vertex` takes the farther face's
   today). Each is read once and never recomputed per cell pair.
2. **The question takes the finished body** and never classes an edge.
   The class decides which arguments may run: the other face's own
   surface is a separator only across a crease; across an end no
   first-order separator certifies, the bisecting plane included, and
   the pair goes to the second-order arm; a smooth seam is proved
   through a separator both faces leave squarely. Where the question
   proves a sign at a crease it must agree with the turn the verdict
   holds; a mismatch refuses.
3. **The cell's reading is the two faces' rates off the separator,
   summed**, times that extent, against the band. Under the carrier arm
   one face contributes rate 0.
4. **Pins owed** (measured first on `embed/wedge-measure`): the curling
   sliver (a 1 mm cylinder face 1e-7 m wide ending 5e-12 m from a 1 m
   plane) and its flat twin (sine 5e-5) refuse; the cone that owns its
   apex, cut through its axis, holds; a crease read under the carrier
   arm and under the bisecting plane decides alike; the bodies whose
   verdict moves between PR 7329's reach and the narrower run are
   listed with each one's narrower face.
5. **Not this unit's:** whether check 4 itself takes the narrower
   face's extent, and whether its fold is the right meter (RESTFRONT:
   `check-4-levers-a-wedge-at-the-farther-faces-run-…`,
   `check-4s-fold-reads-a-crease-where-a-face-curves-back-…`). Until
   check 4 moves, the question can refuse a body check 4 passed (the
   flat strip): the right verdict at the wrong door, to be said in the
   PR. The run is measured across the edge, which HONEONE's
   `the-wedge-reach-counts-a-faces-run-along-its-edge` proposes; if
   that lands otherwise this unit says so on that row.

For the reviewer: the summed-rate bound assumes a separator whose
gradient has unit length (exact for a plane, close for a face's own
surface near the edge, unchecked for the edge-graph separator).

Seams: `validate.rs` (what tier 3's verdict keeps: TOPO, RESTFRONT),
`finish.rs` (EMBEDGATE: `AtRestBody`), `wedge_reach.rs` (HONEONE,
ONEROUND).

Tier: dual (the verdict's shape is broad and hard to change later).
Letter H.


## Design as built (lane `embed/verdict-read`, 2026-11-23)

Approved by the EMBED orchestrator on 2026-11-22 with the rulings
marked **R**.

**The definition (R).** A face's run from what it shares is the
supremum, over the face, of the distance from the shared locus (a
shared edge's whole carrier line or circle; a shared vertex). The
pair's extent is the lesser run. Every reading built is a lower bound
of that, never more; each bound is argued at its site
(`geom_brep::BoundaryStations`, `geom_brep::spanned_curve_run`).

**What tier 3 keeps** (`topo::Tier3Facts`, `crates/topo/src/rest_facts.rs`):
- per edge, its class as check 4's fold read it: `EdgeClass::Crease`
  with its turn, `Seam`, `End` with its end. An edge check 4 exempts
  by kind (beside a spline face) has none. A lamina, a mixed edge and
  an in-band reading refuse at tier 3, so the type cannot hold them;
- per face, its run from each edge of its boundary (check 2, which
  reads every certified edge before check 4's exemption) and from each
  vertex more than three edges meet at (the only vertices two faces
  share with no edge on them);
- per face, each pole or apex check 6 placed on a vertex, by key.

The spec's item 1 says "the pair's extent" per edge. Each face's run is
kept instead and the lesser taken where the two are read: the spanned
bound below is read on the question's side and needs each face's own.

`AtRestBody` holds the facts beside its sign certificate and carries
them through an isometry. `topo::geometric(&Body, tol)` runs tier 2,
the session box and tier 3's local battery (check 7 not made) and
hands back the same bundle for a body that is not finished (PR 6893:
"a caller holding an unfinished body runs the local battery to get the
value").

**How the question receives it.**
`topo::clearance::embedded(&Geometric, faces, band, budgets)`. The
facts are crossed once, where each face's window is built
(`injective::Facts::of`), and the readings take them from the lifted
boundary. `Side::windows_of(&Geometric, faces, which)` is the same
windows for a caller that drives the sweep itself; a touching pair
whose windows carry no facts refuses typed (`Unsupported`, "read with
no tier-3 verdict").

**The run, exactly.** The face's stations are each boundary edge's two
ends and its middle, and each lone vertex: points of its boundary.
- line edge: the station's perpendicular distance from the whole line;
- circular edge: its distance from the whole circle;
- any other carrier (an ellipse, a spiric, a spline): the nearest the
  station stands to the edge's own span, covered in 64 pieces. Here
  the locus is the edge's arc, so a run along the edge past an end
  still counts: more than the definition, stated at the door;
- shared vertex: the station's distance from it.
- **the spanned bound (R, 8a)**: a face whose whole boundary is one
  loop on one closed planar curve with a centre (a circle, an ellipse,
  a spiric oval) spans it, so it meets the line through the centre
  square to the curve's plane, and runs at least as far from the curve
  as that line stands from it. One edge is one curve; several edges
  are one conic by `injective::one_conic` and no second equality. The
  run from the edge's middle reads a disc at its diameter, past its
  true run, and is no lower bound.

Every distance is a cover's reading (`Cover::range_about`,
`Cover::distance_range`); the door takes carriers and spans and places
the stations itself.

**The class decides which separators run.**

| class | other face's carrier | plane through the edge, conic's own plane | edge graph | second order |
|---|---|---|---|---|
| crease | runs; the side proved must be the verdict's turn, else `TurnContradicted` | runs | runs | never |
| seam | barred | runs | runs | never |
| end | barred | barred | barred | refused `SecondOrder` at once |
| none | as before | as before | as before | the engine's own `cusp` reading, kept only here |

A plane or the graph parts the faces of a wedge θ and of 2π − θ alike,
so it carries no turn: turn agreement is checked in the carrier arm
only. An apex is read by the verdict's key.

**The reading (R, 8b).** Each face's own side of the separator is the
sign of its sine's enclosure, through `Margin::sign_of` under
`Band::enclosure_sign`, with no band held against it. The banded
quantity is the two faces' sines summed, times `inf|∇ψ| / sup|∇ψ|`
(one for a plane and for a carrier, whose ψ is a distance; the graph's
`|m − t·γ′|` over its slopes), levered at the extent, at
`self_intersection_departure`. A face with no zero side is read by
ψ over it, a distance, at the band. Under the carrier arm the
separator's own face brings no sine.

**The shell's cavity (R, 8c)** runs the local battery on its moved
clone to ask; a refusal there is `ShellError::NotValid`. This retires
with EMBEDGATE's lane C.
