---
id: a-plane-plane-blend-cannot-end-at-an-unrequested-corner
kind: issue
title: blend: a plane–plane chain cannot end at a corner whose other edges are unrequested, nor turn a sharp corner, so no proper subset of a box's edges can be chamfered or filleted
status: open
opened: 2026-10-02
priority: P0
cost: H
design: true
---

Found by SHOW's `split-node-chords-by-name-has-no-demo`, whose scene
(`demos/tour/src/bracket.rs`, `split_and_break`, walls 1–3) pins it live.

## Why P0

Chamfering or filleting ONE edge of a box refuses, and so does every
proper subset of its twelve edges; only all twelve build. That is
`work/README.md`'s P0: a normal verb broken on normal geometry.
Measured through the document (the PR 3842 review's probe, `Node::Chamfer`
on an extruded box): one edge refuses `UnsupportedRunOut`, the first four
edges refuse `ChainNotG1`, all twelve build. The kernel's own rows pin the
one-edge case as the expected refusal
(`crates/sweep/tests/blend6_verb_vocab.rs`,
`a_chamfer_caller_reads_the_chamfer_verb_over_a_shared_run_out`;
`blend_recourse_followability.rs`).

## The two doors, both in scope

1. **The run-out.** The planar open band
   (`crates/sweep/src/blend/open/planar.rs`) carves a plane–plane link
   only between trivalent corners "whose three edges are all requested".
   A chain that ends anywhere else refuses `BlendError::UnsupportedRunOut`,
   and the recourse (`FILLET3_CORNER_RECOURSE`) — request every edge of
   every terminating corner — closes over the whole polyhedron. The
   ruled band has a sibling built (`RunOutPolicy::CutOffAtTransverseCap`,
   `blend/open/ruled.rs`); the planar band has none, and its end faces
   are in general OBLIQUE to the edge.
2. **The sharp turn.** A chain of plane–plane links meeting at a
   non-tangent corner (a face's whole rim) refuses `ChainNotG1` at
   `fillet3_chain_g1`, before any run-out is read. Breaking every edge
   of one face — the commonest request after a single edge — needs a
   corner patch where two requested edges and one unrequested meet.

## Why it is a design fork

`crates/sweep/README.md` A3-3 (ratified #992) names the run-out as
named-and-not-implemented, and the ruled sibling's end geometry (FILLET-H7,
the transverse cut-off) took Ev's ruling on PR 1736. What a planar band's
end IS at an oblique end face, and what the patch at a sharp chain turn
is, are the same kind of decision: weigh them first with one Opus and
one Fable designer (`docs/prompts/designer.md`), then Ev if it is his
call.

A chamfer's end at a planar end face is one candidate that needs no new
surface: the strip runs on and stops in the end face's plane, a straight
chord at any angle.

## The scene's measurements

The bracket split at `x + y = 2.75`, setback 0.1 (`Node::Part` keeping
the corner piece):

| selection on the corner piece | chamfer | fillet |
|---|---|---|
| the four cap chords (`[SectionEdge, Fragment(Ends)]`) | `UnsupportedRunOut` | `UnsupportedRunOut` |
| the four section edges on the side walls (`[SectionEdge]`) | `UnsupportedRunOut` | `UnsupportedRunOut` |
| all eight (both section faces' whole rims) | `ChainNotG1` (margin 0.75) | `ChainNotG1` |

Which refusal fires first depends on the setback and the plane (the
review's probe): at `c = 2.75` the chords' chamfer meets
`FaceClearanceUncertified` from `d = 0.3`, at `c = 2.6` from `d = 0.1`,
and at `c = 2.9` it is `UnsupportedRunOut` through `d = 0.3`.

Oracle for the four-chord chamfer once it builds: each chord sits on a
right-angle dihedral and ends on the leg's two parallel side walls a unit
apart, so `ΔV = 4 · (d²/2) · √2`, exact while `d·√2 < c − 2.5` (the strip
short of the fillet's tangent points).

## Done when

- One edge, and any proper subset ending at unrequested corners, of a box
  chamfers (and, if the fork rules it in, fillets): the bracket's walls 1
  and 2 panic as retired and the scene's document ends in the chamfer at
  the oracle above.
- A face's whole rim chamfers: wall 3 panics as retired.
