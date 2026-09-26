---
id: CONTACT-1
kind: unit
title: one local material-cone analysis for every touch kind the census reads: vertex-vertex, vertex-on-edge, edge-edge overlap and conformal patch join vertex-on-face and edge-in-face
status: closed
opened: 2026-09-25
priority: P0
cost: H
branch: contact/1-touch-cones
closed: 2026-09-26
pr: 3253
---


Carries `touch-kinds-without-a-local-side-analysis-block-the-material-test`;
`declared-faces-has-no-cross-solid-check` rides. Spec:
`docs/CONTACT-1-SPEC.md`.

Review tier: **dual**. A lenient touch analysis turns a typed refusal
into a clear over overlapping material, and that is a confident wrong
answer on the door every consumer reads as proof.

## Closed

PR 3253. The census backstop's arm-2 clear admits a touch only when the
two materials' local cones at the touch point have disjoint interiors.
A separating-plane search decides this, complete for two convex cones,
with a complement test when one cone is co-convex. Signs are levered by
one rule, `touch_lever`. Refusals are typed: `MixedTouch`,
`TouchInBand`, `TouchUnanalysed` (saddle), `TouchDegenerate`,
`TouchChordScale`, and `CorruptInstance` for a snapshot tier 1 should
have refused. The L-bracket straddle is now refused as a crossing, not
as an unanalysed touch. Resting poses that used to refuse now clear:
against a wall, seated in an inner corner, flush with a top, and on a
corner, whether undeclared or declared.

Review: dual on `e97c2e2` (DR-8 in `docs/DUAL-REVIEW-LOG.md`), then
two single delta reviews of the fix passes.
- The first delta rejected the pass: vertex fans still levered at
  chords.
- The second approved with fixes, and found a fourth instance of the
  lever class: a `sin α` factor at obtuse sectors.

After four rounds the orchestrator ruled that the lever design is the
root cause. It stays here with the gap stated at its site and pinned
by a row. The redesign, deciding a face's side by its vertices' signed
distances, is `touch-cone-readings-are-levered-directions-not-face-distances`.
No end-to-end wrong clear was shown in any round. The two reviewers'
sweep of about 5.7k poses found none beyond the gate shape the
half-overlap row owns.

Ride-along: `declared-faces-has-no-cross-solid-check` was re-homed,
not fixed; its premise is false for curve records.

Filed by the unit and its reviews:
- `a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses` (P0);
- `touch-cone-readings-are-levered-directions-not-face-distances` (P1);
- `census-touch-cones-are-a-third-vertex-sector-builder` (P1);
- `zero-dihedral-conflates-flat-with-slit-in-touch-cones` (P3);
- `a-touch-at-a-saddle-corner-refuses-unanalysed`.
