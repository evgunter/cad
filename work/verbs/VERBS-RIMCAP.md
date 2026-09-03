---
id: VERBS-RIMCAP
kind: unit
title: the rim-construction capability — sphere half
status: spec
opened: 2026-09-03
branch: verbs/rimcap-1
refs: [VERBS-C5ARMS]
---

The partial-revolve rim for circle-profile walls, sphere half only: the
carried-datum corner rule beside the pole arm, and `mint_carrier`'s
off-axis-circle edge arm, so the sphere lune hollows to a derived closed
form. Born from TORAX's (#1494) elbow-split STOP and its fix pass's
corrected capability record — circle-profile walls refuse at two measured
doors (`TogetherAxialCorner` for the elbow, `TogetherEdgeDisagreement`
for the lune) while cylinder-wall rims (the wedge) already work. The
torus half is a spiric quartic with no `Curve3` carrier kind and stays a
design conversation inside the spec (fence it / fund an exact spiric
carrier via an `[ev]` PR / NURBS-fitting rejected in advance); only that
half would unblock C5ARMS' klein-elbow rows 3/4/8. Spec
`docs/VERBS-RIMCAP-SPEC.md` ratified 2026-09-03 on `mngr/kernel-verbs`;
two pre-registered STOPs (machinery-shape; mechanism — the lune
mechanism story is reading-derived and opening-measurement item 4
executes it). Not yet dispatched; queued behind CYLSPH's cycle.
