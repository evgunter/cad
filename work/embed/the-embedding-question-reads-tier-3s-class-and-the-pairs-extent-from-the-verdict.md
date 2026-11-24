---
id: the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict
kind: unit
title: the embedding question reads tier 3's class and the pair's extent from the verdict, and decides neither
status: dispatched
opened: 2026-11-22
priority: P0
cost: H
branch: embed/verdict-read
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
(`geom_brep::BoundaryStations`, `geom_brep::centre_line`,
`injective::crossings`).

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
bound below is read on the question's side, in enclosures, and raises
one face's run alone.

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
- an ellipse or a spiric oval: the nearest the station stands to the
  whole curve, covered over its turn in 64 pieces;
- a spline: the nearest the station stands to the edge's own span,
  covered in 64 pieces. Here the locus is the edge's arc, so a run
  along a spline edge past an end still counts: more than the
  definition, stated at the door;
- shared vertex: the station's distance from it.
- **the spanned bound (R, 8a)**: a face that meets the centre line of
  the closed curve one of its edges lies on (a circle, an ellipse, a
  spiric oval; the line through the curve's centre square to its
  plane, read from the curve's own points: through the middle of its
  points at parameters `0` and `π`, along its tangent at `0` crossed
  with that chord, so nothing is asked of how a frame is stored) runs
  at least as far from the curve as that line stands from it: the
  least distance from the line of the covers of the curve's whole
  turn, the radius for a circle about its axis and no more than the
  semi-minor axis for an ellipse. The face meets the line where its boundary crosses a half-plane
  the line bounds an odd number of times
  (`injective::crosses_oddly`): a disc's rim once, however it is cut
  into arcs; an annulus's two rims once each. No arc is compared with
  another, so the bound needs no reading of how carriers are stored
  and holds alike at a point and a box scalar. (The ruling took "one
  stored conic" from `injective::one_conic`; that reading admits only
  carriers stored as points at a box scalar, and `revolve` cuts every
  rim in two, so ten of the suite's rows held at `f64` and refused at
  `Interval`, and the shell's cavity gate refused revolved bodies
  there. The crossing count replaces it.) The run from the edge's
  middle reads a disc at its diameter, past its true run, and is no
  lower bound.

Every distance is a cover's reading (`Cover::range_about`,
`Cover::distance_range`); the door takes carriers and spans and places
the stations itself.

**The class decides which separators run.**

| class | other face's carrier | plane through the edge, conic's own plane | edge graph | second order |
|---|---|---|---|---|
| crease | runs; where the other face's cell lies along the edge, the side proved must be the verdict's turn, else `TurnContradicted` | runs | runs | never |
| seam | barred | runs | runs | never |
| end | barred | barred | barred | refused `SecondOrder` at once |
| none | as before | as before | as before | the engine's own `cusp` reading, kept only here |

A plane or the graph parts the faces of a wedge θ and of 2π − θ alike,
so it carries no turn: turn agreement is checked in the carrier arm
only, and only on a cell of the other face whose zero set lies along
the edge. A cell that meets the edge at a vertex alone, or not at all,
stands on whichever side of the carrier the face has turned to by
there (a cone wedge past a half turn, at its apex; a notched rod's
reflex corners), and proves nothing about the turn. An apex is read by
the verdict's key.

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


## Measured and landed (lane `embed/verdict-read`, 2026-11-23)

The unit lands on `embed/wave3-land`. Its tip was merged twice, at
`8d34076a9e` and then at `7f85375799` (main's unit directions and edge
views, `embed/strip`, the tangent corner). "Before" below is
`8d34076a9e` unless a line says otherwise, and each line says which
merge it was measured on. Every figure names its population.

**Verdicts.** None moved, on any population read:

| population | before | after |
|---|---|---|
| `crates/sweep/tests/self_intersection_proves_injectivity.rs`, default ε, 1e-6 and 1e-12, `f64` and `Interval` twins | the landing branch's rows | on the first merge 92 of 92 at each ε; on the second 94 of 95 at each ε, the one red the notched rod, which does not build on that tip (`section_site_foot_bend`, SECTCONE's). No verdict of the landing branch's rows is re-pinned |
| 24 probes: the half cone, a stepped ring and a stepped drum built off the dyadic grid, each as built, turned 0.7 rad about `(1, 2, 3)`, mirrored across a generic plane and a hundred units out, at `f64` and `Interval` | 23 embedded, 1 refused | the same 23 and the same 1, with the same receipts but the drum's as built (52 and 47 cells against 57 and 57) |
| the tour's walk, 85 bodies at `f64`, default ε, first merge | not taken on the landing branch (its build was stopped for memory) | 82 hold, 3 cross (`bench`, `bench60`, `bench90`). Against `embed/remeter` before either merge, no line of the walk differed |
| `editor-core`'s clearance rows (the document's strict same-body question), first merge | not taken | 97 of 97 pass |
| `topo` and `sweep`, every test (8333), first merge | not taken | 8328 pass. Of the 5 red, 4 are on main's own list (`work/green/log.md`, 23 Nov, "nine topo and sweep slow rows red on main"); the fifth, a door census reading two parameter names of this unit as field writes, was this unit's and is fixed |

Before the merge, against `embed/remeter` at `18d52c6e9`: the 43
`sweep::all` tests the census (`c2a0c41f40`) read
`Uncertified/SecondOrder/Edge`, every finish asked through a
log-and-pass gate hook (`embed/verdict-read-measure`), read the same
classes before and after, count for count (234 second-order, 72 + 14
departure, 26 out of depth, 2 crossings). That census was not taken
again on the merged tree.

Two COUNTS moved, verdicts unchanged:
`a_rod_less_a_tilted_slab_holds_across_its_curved_corners` read 14
certificates through a corner at both scalars and reads 16 at both
(on the first merge, 16 and 18); the off-grid drum as built
discharges 52 cells at `f64` and 47 at `Interval` where the landing
branch discharges 57.

**The sector sliver of `embed/strip` holds, with its receipt
unchanged** (`contact9_side_codes::the_sector_slivers_meet_is_certified_by_the_reach_lever_once_its_faces_are_read`).
That row was written to go red here, on the reading that the narrower
face's run is the 20 mm over which the sliver's top and bottom stay
within ε of each other. By the definition above both faces run the
sliver's 10 m from the tip edge's line, the summed sine reads 500ε
there, and the pair is certified: `Embedded`, receipt
`(15, 24, 0, 0, 0, 9, 13)`, as before. Whether the tiers clause means
the in-band stretch is not this unit's to settle; it is with the
orchestrator.

**The spec's pins.**
- *The cone that owns its apex, cut through its axis, holds.* The
  triangle `(0, 0), (1.1, 0), (0, 0.9)` revolved half a turn: tier 3
  classes each ruling a convex crease and keeps one extent (0.70, the
  cut plane's run, against the cone's 1.39). On `embed/remeter` it
  refused at its apex's collapsed side, the exit moving with the pose;
  the landing branch reads that side, and the body holds in all four
  poses at both scalars there and here. **The "cone holds" pin is
  struck from this unit's acceptance** (EMBED orchestrator): the
  holding is the landing branch's, and this unit's row
  (`a_cone_cut_through_its_axis_holds_at_its_apex_in_every_pose`)
  pins that reading the verdict's facts does not lose it.
- *The curling sliver is not buildable through the profile door*
  (`NonSimple`, as `embed/wedge-measure` found), so it and the flat
  twin are unit rows on fixtures whose extent the real door reads.

**Newly refusing valid-looking bodies.** None on the populations
above; the mirrored build among them. One body refuses that looks
valid, on the landing branch as here: the off-grid stepped drum
mirrored across a generic plane, at `Interval`, between the two faces
of one cylinder wall across their seam
(`a-stepped-drum-mirrored-across-a-generic-plane-refuses-departure-at-interval`).
Class (a), truly inside the band at the narrower face's far side: the
two fixtures the ruling was weighed on, and no built body. Class (c),
no bound available: one shape is known and filed, a face bounded by
one closed spline edge
(`a-face-bounded-by-one-closed-spline-edge-has-no-spanned-run`); no
body of the suite or the tour builds it.

**A turn contradicted on two valid bodies, found at the merge and
fixed.** The carrier arm checked the side it proved against the
verdict's turn on every cell near the edge. A cone wedge past a half
turn and a notched rod turned about a generic axis each have a cell of
the other face that meets the edge at a vertex alone (the wedge's
apex) or not at all, on the far side of the carrier from the one the
edge's turn names, and were refused `TurnContradicted`. The check is
made only on a cell whose zero set lies along the edge
(`injective::Zero::along`); both bodies hold, as on the landing
branch.

**Pairs near a vertex with no kept run.** Tier 3 keeps a face's run
from a vertex only where more than three edges meet at it. Counted
over every cell pair the suite's 92 rows and the walk's 85 bodies
read: none is near a vertex alone that a face holds no run from.

**A spline shared edge is read from its own arc** (more than the
definition, which reads the whole carrier). 19 of the suite's rows
and the walk read such loci (924 and 520 readings). Whether any
reading changes a verdict against the whole carrier was not measured:
none of those rows' verdicts moved, which shows only that the
over-read does not refuse. Noted
on `work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`.

**Callers found beyond the spec's list.** The strict same-body sweep
is reachable without `embedded`, through `Sweep::run` and the lane's
`sweep`: the document's clearance door (`editor-core`), and test
helpers in `sweep`, `topo`, `editor-core` and `demos/tour`. A touching
pair whose windows carry no facts refuses typed. The document door
runs tier 3's local battery for the facts
(`ClearanceLaneOf::reading_unfinished`); hand-built construction state
in tests, and a body tier 3's battery refuses (the uneven plate
mirrored across a generic plane), are asked on fixture facts
(`topo::Geometric::fixture`, test-support).

- 2026-11-23 — **Two of PR 7329's rows are this unit's to turn** (both of its reviewers, delta re-reads): `injective::margin_reach_rows::the_arm_counts_a_strips_run_along_its_edge` pins today's over-read (the arm counts a strip's run along its edge), and `the_reach_alone_signs_an_edge_check_4_classes_smooth` pins the question signing an edge check 4 classes smooth. Each goes red when the lever is the narrower face's run across the edge and the class is read from the verdict; this unit re-pins them and lists them among its pins. (EMBED orchestrator)

