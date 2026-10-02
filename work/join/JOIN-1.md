---
id: JOIN-1
kind: unit
title: A section germ names the cell it lies in (OnEdge or InFace) per operand, and the join matches on it
status: spec
opened: 2026-10-02
priority: P0
cost: H
branch: join/1-germ-locus
refs: [an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired, blind-d-pocket-subtract-refuses-with-join-internal-words, closed-in-face-section-loop-has-one-site, dumbbell-joint-union-leaves-four-loose-ends]
---


Spec: `docs/JOIN-1-SPEC.md`. Carries
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`,
and the typed-refusal half of `closed-in-face-section-loop-has-one-site`.

## Built (branch `join/1-germ-locus`)

- `HalfGerm` carries `a_locus` / `b_locus` (`boolean::Locus`:
  `InFace(face) | OnEdge(edge)`), derived by ONE function,
  `sectors::germ_locus`, from the attributed sector and its bounds'
  codes as first read; the vertex-on-face runs and the vertex-vertex
  insertion both call it before their surgery.
- `find_match` and `loose_partners` compare loci on both operands;
  an `OnEdge` germ whose edge is not incident to its site refuses
  `JoinDesync`.
- One fold rule, `sectors::fold_on_bound` (mixed → In): vtxfac's
  on-entry resolution, recl's edge-sector attribution (the Out-keyed
  flanker) and edge-edge record choice all read it.
- The boolean lanes' adjacency skip is `SegmentEdge::Is(locus edge)`;
  the split lane keeps `SegmentEdge::InPlane`.
- A record whose two germs share loci along a conic, left unmatched
  at quiescence, refuses `Join(SingleSiteSectionLoop { count })`.

## Measured

- `axis_lap.rs` `an_axis_lap_builds_every_op_as_its_planar_twin_does`:
  rod and diamond, ∪/∖/∩, cutter `y ∈ [0, 1]` and `[−1, 0]`: all
  twelve build, pass tiers 2, 3 and 3′, at the closed form (rod ∖
  `2.748893571891069`, rod ∩ `0.39269908169872414` = πR²/2).
- The closed tube in the box top refuses
  `SingleSiteSectionLoop { count: 2 }` under ∪, ∖, ∩.
- The merged teapot cup passes the join and refuses
  `VolumeUnmeasured` (`NotIsoRectangle { "props_rim_level" }`).
- The blind D top pose still refuses `JoinDesync { "ring-run winding
  is degenerate (zero enclosed area)" }` (JOIN-3's ground).

## Fix pass (dual review, PR 3790)

- **The edge-edge fold is the one fold rule, per solid.** At an
  edge-edge site each solid applies `fold_on_bound` /
  `sectors::crossing_flank` to its OWN flankers' membership keys; the
  germ's record is the pair of the two solids' transition flankers,
  minted when `pair_search` met no such pair (`recl::place_germ`). The
  A-alone / B-alone / orbit-order tiers are gone; a declared-`Tangent`
  flank keeps the flanking record whose other bounds read Out on both
  solids, or refuses.
- **A segment along an edge of both solids** joins on
  `JoinLane::AlongEdge`: each solid's chord is a copy of its own edge
  (`chord_join::along_edge_spec`: lines straight, circles their own
  arc), its frame is the edge's curve, and no section of the germ's
  face pair (possibly one carrier) is read.
- ~~A scaffold at rest is restated~~ — withdrawn in fix pass 2: it
  hid an illegal output (two same-sense coplanar neighbours) behind a
  passing tier 3.
- `locus_at_site` reads the whole null-edge site; `find_match` and
  `loose_partners` share one criterion (`join::partners`); the boolean
  lanes' dead `between_edge_is_section` arms and `JoinLane::Planar`'s
  section plane are gone.

## Measured (fix pass)

Main 0abf909cb → fix-pass head, release, R1's and R2's batteries: no
pose moves from a sound body to anything else and none to a wrong
body. Hexagon ∪/∖/∩ box: 2796 refusals → sound; triangle 1164;
diamond 3840; R2's random z-prism pairs (seeds 1–5, 1000 cases, 3 ops)
342; the declared battery 6268; the reflex probe 128; the tube 102.
The dumbbell builds (`dumbbell-joint-union-leaves-four-loose-ends`).
Frontier refusals that became `JoinDesync`/`SeamOrientation` are filed
(`locus-matching-moves-frontier-refusals-to-join-desync`).

## Fix pass 2 (delta review, PR 3790)

- **No continuation check of JOIN-1's own.** An undeclared same-sense
  continuation is refused at the reduction by REACH's scan (PR 3657,
  `reduce::refuse_undeclared_continuations`), which landed while this
  fix pass was open. A declared one is glued by the output's merge
  stage. The fix pass's own edge-edge check (`recl.rs`, then routed by
  provenance) was a strict subset of that scan, and was dropped at the
  merge.
- **The scaffold restatement is gone**, except between the two faces of
  a recorded curved merge skip.
- **A strut's half faces the germ along its own edge** (`insert` spike
  order, `vtxfac` pierce struts): the lens regression and the ball's
  pole desyncs both came from crossed bindings.
- `place_germ` refuses to overwrite another event's germ;
  `along_edge_spec` runs on the both-`OnEdge` lane only.

Main da396111f → fix-pass-2 head, release, with a legal-operand column
(measured before REACH's merge): hexagon ∪∖∩ box 2508 refusals → sound,
288 → `UndeclaredCoincidence`; R2 seeds 6–15 (30000 ops) 517 → sound,
17 → `UndeclaredCoincidence`; no sound body lost, no wrong body, every
build a legal operand.

Since main carries REACH, JOIN-1 newly builds several declared-
continuation unions main refused. Each row now asserts a sound, legal
operand: the rabbet step
(`zip/a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends`,
closed), mate7a's torus peg-in-socket, and the `curved_mergedoor`
scenes C and D, through the join.

