---
id: a-thin-corner-bars-spiric-cap-exhausts-the-quadrature-budget-at-1e-12
kind: issue
title: The fat torus less a thin corner bar (w = 0.05) refuses VolumeUnmeasured QuadratureBudget at eps 1e-12 (width 2.13e-9 against target 1.02e-9 at round 13): a spiric-bounded face of P minus T
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Found by GERM U-T4's fix pass (`germ/plane-torus-chord-spiric`,
PR 4927), 2026-10-13, measured.

## Measured

The corner bar of half-width `w` on the fat torus `(1, 0.6)` about `y`
(`crates/sweep/tests/germ_plane_torus_frame.rs` `corner_bar`, its eight
corners on the tube's inner face; `germ_plane_torus_spiric`'s B5 is
`w = 0.1`), at `CAD_TOLERANCE_EPS=1e-12`, `P ∖ T` (the bar less the
torus):

| torus | `w = 0.05` | `w = 0.1` |
|---|---|---|
| fat torus | `QuadratureBudget { width_len: 2.131e-9, target_len: 1.024e-9, rounds: 13 }` on face `6v3` | body |
| fat one-face torus, vertex at 0 / at 1 | the same, same width | body |
| fat one-face torus, vertex at π | width `2.128e-9`, face `8v5` | body |
| fat seamed torus | width `2.128e-9`, face `2v3` | body |

surfaced as `BooleanError::VolumeUnmeasured { operand: None, source:
Face { .., QuadratureBudget { .. } } }`. Which face it is was not
read; every face of that body the spiric lane takes is bounded by oval
arcs that run corner to corner, where the corners lie on the tube's
inner face and the end squares sit just short of the inner equator
(`z = 0.3990` against `R − r = 0.4` at `w = 0.05`, `0.3960` at
`w = 0.1`), so the ovals are pinched there. The other five ops stop earlier on doors filed elsewhere
(`RingOnCurvedFace`, `PartialTorusFace`) or build.

The width is twice the target at the last round, and the same on every
spelling, so it reads as the spiric cap's own enclosure near a pinched
corner rather than a frame effect. The first step is to see which of
`props::spiric`'s terms carries the width at round 13 (the cos-moment
`J` of `spiric_vector_area` near `f_min`, or the endpoint-bracket pad
at the corners) and whether grading the pieces toward the pinch
narrows it. The far-translation budget refusal in
`a-spiric-elbow-a-hundred-kilometres-out-exhausts-the-quadrature-budget`
is a neighbour with a different trigger.

GERM's U-T4 PR lists this as a stop past the join; its rows do not
pin it (the lens row runs `w = 0.1` and `0.3`).

## Update 2026-10-24 (QUAD unit B, PR 5628)

The Boolean no longer refuses. Unit B's volume backstop decides its inequalities from each body's round-0 enclosure, refining only while a bound straddles, so no reporting walk runs inside the op. At ε = 1e-12 all six ops of the fat torus `(1, 0.6)` against `corner_bar(1, 0.6, 0.05)` build. On main at `debb2a60`, `T ∩ P`, `P ∩ T` and `P ∖ T` refuse `VolumeUnmeasured { Face { QuadratureBudget { width_len: 1.551e-9, target_len: 1.024e-9, rounds: 7 } } }`, narrower than the 2.13e-9 at round 13 measured above.

The bodies check against the closed forms. V(P) = 7.979321e-3 and V(T) = 7.106115169. The certificate enclosures of P ∩ T and P ∖ T sum to [7.976439e-3, 7.982204e-3], and those of T ∖ P and T ∩ P sum to [7.106112287, 7.106118051].

What this row describes is still open, at the number door. `mass_properties` and `SignCertificate::measure` on `T ∩ P`, `P ∩ T` and `P ∖ T` refuse `QuadratureBudget` with the same 1.551e-9 width after 7 rounds.

## Update 2026-10-27: the refusal this row names is gone

PR 5741 (QUAD's measure door, unit C) removed `PropsError::QuadratureBudget`.
`mass_properties` now takes a required resolution and answers a bracket
whose `stop` is `Floor` or `Wall` when the resolution is not met. So
this body no longer refuses at the number door; what remains to
establish is whether its bracket is wide or stops at `Wall`. **First
step: re-measure on current main**, then either close the row as
overtaken or re-title it as the width it now reads.
