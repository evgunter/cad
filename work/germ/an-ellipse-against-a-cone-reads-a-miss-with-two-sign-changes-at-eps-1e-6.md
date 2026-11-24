---
id: an-ellipse-against-a-cone-reads-a-miss-with-two-sign-changes-at-eps-1e-6
kind: issue
title: The conic × quadric cone door answers a Miss for a small ellipse whose residual changes sign twice on the span at ε 1e-6 (cone fuzz seed 0x1ea1fc22a7df1597)
status: open
opened: 2026-11-23
---


Found by the spiric oval-in-band lane (`germ/spiric-oval-in-band`) in
its `curve_face::` run at `CAD_TOLERANCE_EPS=1e-12`; not that lane's
ground (its diff does not reach the cone door).

## Finding

`curve_face::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form`
(`crates/geom-brep/src/curve_face/conic_quadric/cone_rows.rs`, the
`Ok(CircleRoots::Miss)` arm of its assertion) fails on
`CAD_FUZZ_SEED=0x1ea1fc22a7df1597 CAD_FUZZ_EFFORT=1`, at the row's own
ε 1e-6, case 64:

- an ellipse held at its centre, `semi_u` 1.13e-3, `semi_v` 3.68e-4,
  centre `(−0.852, 1.382, 0.885)`, against a cone with half-angle 0.771,
  over `[2.564, 4.305]`;
- the door answers `CircleRoots::Miss`, while the oracle reads the
  ellipse's least distance from the cone at 8.5e-10 and 2 sign changes
  of the residual on the span.

A dip through the cone shallower than the band answered as a definite
miss: the class the band must refuse or escalate.

## Settles when

The seed's case refuses or answers its two crossings, pinned as a
deterministic row beside the fix.

**A second seed (`boxfinish/wall-roots`, 2026-11-24).** At the default
ε, `CAD_FUZZ_SEED=0xb015d658e225e724 CAD_FUZZ_EFFORT=1` reds the same
row at its ε 1e-6 case 373: an ellipse of semi-axes 1.08 mm × 0.196 mm
against a cone of half-angle 0.397 reads a Miss `7.3e-11` from the cone
with 2 sign changes. Red on main at `1e5b1ce503` and on that branch
alike; the branch's diff does not reach the conic × quadric door.
