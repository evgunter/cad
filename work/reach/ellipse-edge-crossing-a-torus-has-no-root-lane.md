---
id: ellipse-edge-crossing-a-torus-has-no-root-lane
kind: issue
title: An ellipse edge crossing (or near) a torus face refuses at the frontier - its residual is an octic in the half-angle
status: review
pr: 3973
branch: reach/ellipse-torus-roots
opened: 2026-10-02
priority: P1
cost: H
refs: [non-circle-conic-edge-refuses-against-every-curved-face, a-torus-near-a-tilted-cut-stops-at-the-extent-scan, an-edge-crossing-a-cone-face-has-no-root-lane, degree-2-subdivision-doors-carry-no-root-slack-meter]
---

Found by the REACH lane that gave the ELLIPSE carrier its clearance
and root lane (`work/reach/non-circle-conic-edge-refuses-against-every-curved-face.md`).

## Measured (by reading)

An ellipse arc that the conic rung's enclosures cannot clear of a TORUS
face falls through to the root arms, and `reduce::wall_crossing`
answers `SpanVerdict::Unsettled` for the ellipse × torus cell, so every
arm refuses `CurvedPierceUnsupported`. A clear ellipse IS cleared (the
torus arm of `geom_brep::conic_arc_residual_range`, its curvature bound
read at the semi-major axis, pinned by
`geom_brep`'s `the_arc_enclosures_hold_a_dense_ellipse_sampling`); a
near or crossing one is not.

## Why there is no lane

Along an ellipse `C(θ) = C₀ + a·û cos θ + b·v̂ sin θ` the torus's
`F = (S + R² − r²)² − 4R²(S − h²)` has `S = |C − c|²` of trigonometric
degree TWO (on a circle the second harmonic of `S` vanishes because
`|û cos θ + v̂ sin θ| = 1`) and `h` of degree one, so `F` has degree
FOUR: an octic in the half-angle, where the circle's is a quartic
(`circle_roots::half_angle_roots`). No certified octic ladder exists in
the tree.

## What a fix has to supply

A certified real-root count for a degree-4 trigonometric polynomial on
an arc — e.g. a Sturm sequence or Descartes/Bernstein subdivision on the
half-angle octic, with the same pole and conditioning discipline the
quartic ladder uses — or a subdivision of the arc into pieces on which
the sampled enclosure and a monotonicity bound (`|F′|` from below)
certify at most one root each. No real shape reaches it yet that is
known: the tilted cut of a torus-walled body would.

## Outcome (branch `reach/ellipse-torus-roots`, PR 3973)

The torus implicit along a conic has one home,
`geom_brep::ConicTorusHarmonics` (degree four; the circle × torus door
reads it too). `circle_roots::certified_subdivision` takes degree up to
four, and the ellipse door's torus arm answers on it alone. Each root's
slack is charged from the residual's running bound
(`geom_brep::conic_torus_residual`, bit-identical to
`implicit_residual`; `circle_roots::RootSlack`), each of its terms
pinned by a pose (`torus_rows::the_slack_meter_charges_every_term`). A
clear piece is read in metres through the torus's near-surface ceiling,
capped at `r/2`.

**Measured, not run by CI:** the mpmath oracle
(`scripts/oracles/ellipse_torus_mpmath.py` over the `#[ignore]`
`torus_rows::dump_for_the_mpmath_oracle`) found no failure on four seeds
of 3000 poses, run by the lane; the dual reviewers' own oracles found
none either (7,560 and 3,840 poses). CI runs the f64 fuzz
(`certified_torus_answers_hold_against_the_true_distance`: exact count,
arc place where the f64 oracle resolves it) and the pinned rows.

**No body reaches a build.** A torus near a tilted cut's rim meets the
cut plane and the wall obliquely, which no section arm answers; the
reviewers' end-to-end runs (2,538 in all) built none, and every op stops
at the extent scan or the join's germ frame
(`a-torus-near-a-tilted-cut-stops-at-the-extent-scan`).

**One circle × torus verdict moved** on reviewer r1's 3600-pose
differential: ε 1e-12, ×1e3, inner graze, `Certified` → `Uncertain`
(main's roots 157,518 bands off). The circle door keeps main's clear
reading through the floor, filed:
`work/hone/circle-torus-clear-margin-reads-the-floor.md`.

Residue filed: `an-edge-crossing-a-cone-face-has-no-root-lane`,
`work/hone/degree-2-subdivision-doors-carry-no-root-slack-meter.md`,
`work/hone/circle-torus-clear-margin-reads-the-floor.md`.
