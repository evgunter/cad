---
id: a-plane-plane-blend-cannot-end-at-an-unrequested-corner
kind: issue
title: blend: a plane–plane chain cannot end at a corner whose other edges are unrequested, so no edge of one face can be broken alone — one cube edge, or a split half's section chords
status: open
opened: 2026-10-02
priority: P3
cost: M
---

Found by SHOW's `split-node-chords-by-name-has-no-demo`, whose scene
(`demos/tour/src/bracket.rs`, `split_and_break`, wall 1) pins it live.

## What

The planar open band (`crates/sweep/src/blend/open/planar.rs`) carves
a plane–plane link only between trivalent corners "whose three edges
are all requested". A chain that ends anywhere else refuses
`BlendError::UnsupportedRunOut` "a chain ends at a trivalent corner
whose three edges are not all requested", and the recourse
(`FILLET3_CORNER_RECOURSE`) is to request every edge of every corner
the chain ends at — which on a polyhedron closes over the whole body.
The ruled band has the planar case's sibling built
(`RunOutPolicy::CutOffAtTransverseCap`, `blend/open/ruled.rs`); the
planar band has none.

So no subset of a box's edges short of all twelve can be chamfered or
filleted. The kernel's own rows pin the one-edge case as the expected
refusal (`crates/sweep/tests/blend6_verb_vocab.rs`,
`a_chamfer_caller_reads_the_chamfer_verb_over_a_shared_run_out`;
`blend_recourse_followability.rs`); this item is the consumer that
needs it built.

## The consumer

The bracket split at `x + y = 2.75` (`Node::Split`, then `Node::Part`
keeping the corner piece) and its four section chords on the caps
chamfered by name (`Node::Chamfer`, setback 0.1). Every corner of a
section face carries a body edge the selection does not name, so every
subset of a split half's section edges ends at an unrequested corner.
Measured, at the default ε:

| selection on the corner piece | chamfer | fillet |
|---|---|---|
| the four cap chords (`[SectionEdge, Fragment(Ends)]`) | `UnsupportedRunOut` | `UnsupportedRunOut` |
| the four section edges on the side walls (`[SectionEdge]`) | `UnsupportedRunOut` | `UnsupportedRunOut` |
| all eight (both section faces' whole boundary) | `ChainNotG1` (margin 0.75) | `ChainNotG1` |

The geometry a chamfer wants at such an end is a cut, not a patch:
the strip runs on to the end face and stops in that face's plane, a
straight chord, for any end-face angle. For the bracket's chords the
end faces are the leg's two parallel side walls a unit apart, so each
chord's prism is exact and the oracle is closed-form:
`ΔV = 4 · (d²/2) · √2` over the four chords (right-angle dihedral
between cap and section; a 45° chord crosses a unit-wide leg in √2).
A fillet's end there is a cylinder band cut by an end face at 45° to
its spine — the oblique end the ruled band refuses today — so the
chamfer's straight cut is the smaller first step.

## Done when

The bracket's wall 1 panics as retired, and the scene's document ends
in the chamfer at the oracle above.
