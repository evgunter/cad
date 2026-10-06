---
id: a-plane-plane-blend-cannot-end-at-an-unrequested-corner
kind: issue
title: blend: a plane–plane chain cannot end at a corner whose other edges are unrequested, nor turn a sharp corner, so no proper subset of a box's edges can be chamfered or filleted
status: open
opened: 2026-10-02
priority: P0
cost: H
pr: 4085
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
one Fable designer (`docs/prompts/designer.md`), then Ev if it is Ev's
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

The other half (`Above`, the two leg tips: two solids of one shell
each) meets the same run-out, since the blend carves each chain inside
its own shell (`a-blend-refuses-a-solid-of-several-shells`): its four
cap chords refuse `UnsupportedRunOut` chamfered and filleted (walls 4
and 5).

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
  the oracle above; walls 4 and 5, the leg tips' chords, panic as
  retired with them.
- A face's whole rim chamfers: wall 3 panics as retired.

## Ruled (Ev, PR 4085, 2026-10-06)

The request decides how a straight band ends at a trivalent vertex of one
convexity between planes: all three edges, the corner patch; one, the
CUT-OFF in the end face's plane section (chord; circle or ellipse); two,
the MITRE along the bands' intersection (line; planar ellipse), with one
more short curve where the trihedron is not isosceles. Chain G1 classifies
plane–plane junctions; the planar band carves locally. Text:
`crates/geom-brep/README.md` C8 and `crates/sweep/README.md` ("Where a
straight band ends, and where it turns"). Names `Mitre { vertex }`,
`TurnFoot { vertex }`; `EndFace`/`CutOffAtEndFace`, `Turn`/`Mitre`. The
isosceles verdict lands under option (b): recorded as a value-decided
coincidence, proven structurally at INTENT's stage 4
(`work/intent/isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box.md`).

Build order (each step widens admission; this row closes with step 4):
1. Battery: chains break at definite sharp turns; the `EndFace`/`Turn`
   tags and the refusals re-worded.
2. The local planar carve: the cut-off at any angle for the chamfer and at
   perpendicular end faces for the fillet, with the corner patch moved onto
   the local carve (a band with a patch at one end and a cut-off at the
   other). Retires the bracket's walls 1 and 2 for the chamfer.
3. The oblique fillet (the ellipse; the cap-clearance region widened).
4. The isosceles mitre, chamfer then fillet. Retires wall 3.
5. The non-isosceles overrun (a numeric probe before its spec), then delete
   the whole-face planar path — split to their own rows when step 4 lands.

## Findings (steps 1 and 2, branch `band/plane-plane-band-cuts-off-at-its-end-face`)

- The tree matched the row's claims before the change: one box edge
  refused `UnsupportedRunOut` (pinned by `verbs_chamfer`,
  `blend6_verb_vocab`, `m6_surgery`, `review_d2_recourse_at_the_site`),
  and a face's rim `ChainNotG1` (`blend6_verb_vocab`,
  `closed_chain_junctions`, `m5_pr12_battery`).
- After: the bracket's four chords chamfer at `ΔV = 4·(d²/2)·√2`
  (`demos/tour/src/bracket.rs`); filleting them refuses the oblique end
  (wall 2), and both section faces' rims refuse as the turn (wall 3) —
  step 4's.
- The whole-face planar path is not kept: the local carve's stations
  are every vertex of a face whose boundary is wholly requested, so
  that case is the local carve's and step 5 has nothing left to delete
  but this note.
