---
id: a-concave-fill-and-a-whole-shell-inside-a-bands-region
kind: issue
title: blend: what the bore ruling's concave half and a whole shell inside a band's region do — a hole whose mouth a fill seals, a protrusion standing in the fill, a void or island wholly inside
status: open
opened: 2026-10-13
priority: P2
cost: H
needs_ev: true
design: true
---


Split from `ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver`
at its step 4 (PR 4957), which built the ruling's convex half for the
planar band and the chamfer and stopped at the concave fill.

## The ruling it applies

Ev, PR 4285: "A feature wholly inside the material a fillet or chamfer
band removes (convex) or fills (concave) goes with that material, as a
feature wholly inside a Boolean's tool goes with the tool; a feature
crossing it still refuses."

On the concave side the band adds material in a region F that is air in
the source. Three shapes meet F, and one shape meets both sides.

## The questions

**S. A hole whose mouth the fill seals.** A cycle on the end face, wholly
inside the concave sliver, is the mouth of a hole running into the
material behind that face: blind, or through to a face the fill does not
cover. None of the hole lies in F; it touches F only at its mouth.
- **S1** refuse it.
- **S2** seal it: the mouth becomes a plug face, a through-hole turns
  blind, a blind hole becomes a closed void.

**P. The blended solid's own protrusions standing in F.** A bar between
the two end walls through the concave corner, a pin out of an end wall,
a boss on the floor beside the wall. Each is wholly inside the material
the fill adds. With its convex mirrors (the through-bore built in step 3,
a blind bore, a dimple in a support's removed strip), these form one
class: a feature of the blended solid lying wholly inside the band's
region and attached only by cycles on the faces bounding that region.
The class varies by side (convex hole, concave protrusion), host face
(cap, support) and cycle count (two: through; one: blind).

**I. A closed shell wholly inside the region.** A void of the blended
solid inside the material a convex band removes, or an island (a second
solid of the same body) inside the material a concave band fills.
- **I1** both go, through one shell-removal door in `topo` beside
  `insert_void`.
- **I2** the void goes; an island refuses.
- **I3** both refuse.

A blend's operand is always one part: `body_operand`
(`crates/editor-core/src/eval/wire.rs`) refuses a value of several parts
as `ProductOperand` before the kernel runs, and every op but the placers
and `Part` counts one part (`eval/mod.rs`, `NodeValue::parts`). So an
island in a blend's operand was put there by that part's own recipe (a
union the author wrote, or a lump a subtract left in a cavity), and
absorbing it is the union the author already wrote.

## Recommended

- **S1**, refused as a decided verdict at the mouth cycle, naming the
  hole's mouth and the face the fill would seal, with the recourse
  "blend before drilling it, or remove or move the hole". Today it
  refuses as a bound at the hole's wall, which tells the user the kernel
  could not measure when it did. S2 would turn a through-hole blind or a
  blind hole into a hidden void with no name vanishing (so neither N5 nor
  the consumption lint sees it), would make the blend a fourth birthplace
  of cavities against `docs/DESIGN.md`'s void-insertion convention, and
  its convex mirror (a boss whose foot the cut-off removes, left
  floating) shows that "close what it touched" is not "goes with the
  material".
- **P built** as the ruling applied, in this order: the concave bar
  (step 3's carve with the material side flipped, every meter already
  built); then the one-cycle members (a blind bore, a pin, a floor boss,
  a dimple), which need a reading that a feature's floor lies inside and
  a three-way reading of a cycle against a support's strip; then I.
- **I1**, with a side rule: the removal takes a void shell under a
  convex band and an outer shell under a concave fill, never the
  reverse. A free solid's outer shell inside the region a convex band
  removes is not the blended solid's material: it survives, floating in
  the cavity the band opens. An absorbed island's own cavities become
  the blended solid's voids; a removed void's islands become solids of
  their own (the piece sort's rule).

Two designers (fork-log row 130) agreed on S from their first reports
and on P and I after two rounds; their first reports crossed on I.
