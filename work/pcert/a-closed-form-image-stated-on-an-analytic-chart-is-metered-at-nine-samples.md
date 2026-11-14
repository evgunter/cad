---
id: a-closed-form-image-stated-on-an-analytic-chart-is-metered-at-nine-samples
kind: issue
title: A closed-form image, or a curve in UV off its ruling, stated on an analytic chart is metered against its carrier at nine samples, so a harmonic image a millimetre off its line gets an edge certificate
status: closed
opened: 2026-11-11
closed: 2026-11-11
priority: P1
cost: M
branch: pcert/closed-form-image-bounded
pr: 6845
---

Filed by `pcert/analytic-image-fidelity`, which closed
`a-stated-image-on-an-analytic-chart-is-metered-against-its-carrier-at-nine-samples`
for the classes a door certifies (`pcurve_cache/stated.rs` `StatedDoor`):
an iso line or curve in UV on a plane or on a cylinder's or cone's
ruling, and a projected image. This is the rest of that class, still
metered at the schedule's nine samples (`certify.rs` `run_checks`,
`StatedChart::Sampled`, checked as `CertCheck::StatedImage`).

## What

Two witnesses, executed on the branch at `f64`, ε 1e-9; each certifies
(`Ok(())`) with a miss of 1 mm and of 1 µm:

- **A harmonic image on a plane.** The line `(0, 0.3, 0) + t·x̂` over
  `t ∈ [0, 8π]` on the plane `z = 0` (`u_ref = x̂`), stated by
  `Pcurve::free_harmonic((0, 0.3), (0, 0), (0, b), (1, 0))`: the line's
  own image with `b·sin t` added to `v`. The schedule's samples are
  `t = kπ`, where `sin t` vanishes, so the image misses its carrier by
  `|b|` between every pair of samples.
- **A curve in UV off its ruling on a cylinder.** The rim circle
  `z = 0.5` of the unit cylinder (its angle the parameter), stated by the
  degree-1 curve in UV `(t, 0.5)` with knots `0, 1/256, 1/128, 3/256, 1`
  and its `1/128` vertex slid `du` along `u`. The image stays on the
  chart and leaves its carrier by `R·du` between the samples.

## Trace

- `pcurve_cache/stated.rs` `StatedDoor::of`: a closed-form image
  (`Harmonic`, `Spiric`, `FocalSection`, `IsoArc`) on an analytic chart,
  and a curve in UV or iso line on a curved analytic chart that does not
  run on one ruling, have no door; `certify.rs` meters them at the
  schedule (`StatedChart::Sampled`).
- Routing a closed form through its own door (`PcurveCache::certify`)
  does not work as it stands. That door's envelope compares the stated
  coefficients with the image re-derived from the carrier, and on the
  product it refuses ~300 correct reads (`EnvelopeTerm(FidelityU)`, a
  phase in another window or a mirrored winding): mirrored tori, spheres
  and cones (`a_mirrored_body_is_its_image`), plane–torus germs, spiric
  ovals on their mirror torus. The census was taken on this branch's
  merge base over the five PCERT crates' default suites.
- `pncad` re-exports `EdgeCurve::certify`.

## Class

A closed form's residual `C(t) − S(P(t))` against a closed-form carrier
lies in a finite trigonometric family: on a plane, `span{1, cos t, sin t,
t}`; on a cylinder, the same; on a cone, products with `t`; on a sphere
or torus, frequencies up to 2. Nine samples bound none of these over a
span whose parameter runs past `8π` (a line, whose parameter is length),
and the harmonic witness is that case. On a circle's span of at most one
turn they may bound the residual within a constant factor, which nobody
has shown. Within this class, a curve in UV on a curved chart off one
ruling has no patch that agrees with the chart along it, so the fitted
door's composite cannot be used without a remainder.

## Fix direction

- Bound a closed form's residual by its coefficients: expand
  `C(t) − S(P(t))` in its family, reading angle channels as angles (so a
  phase a period away and a mirrored winding cost nothing), and bound
  the sum over the span. The closed-form door's own fidelity term is
  this with the derived image in place of the carrier.
- For the curve in UV off a ruling: the tangent patch along the image,
  plus its second-order remainder over the image's `u` extent, or the
  composite against the chart's own rational patch where its parameter
  is the angle.
- Rows: both witnesses red on main and green after; the refusal census
  above rerun on the routing, and kept empty.

## Closed (PR 6845, 2026-11-11)

Every stated image on an analytic chart has a door that bounds its miss
over the span (`pcurve_cache/stated.rs` `StatedDoor::of`, now total), and
the edge certificate's sampled meter for stated images is deleted
(`certify.rs` `StatedChart` has no `Sampled` arm).

- A closed form (harmonic, spiric, focal section) is bounded by the
  coefficients of its residual against the carrier, written exactly as
  a trigonometric polynomial (`pcurve_cache/closed_bound.rs`,
  `pcurve_cache/trig.rs`): angle channels read as angles, so a phase a
  period away and a mirrored winding cost nothing; a spiric's `f` is a
  factor over `ρ`, a focal section's azimuth is cleared by `q²`. Where the
  bound does not clear, a grid read locates the miss
  (`PcurveCertifyError::MissLocated`).
- A curve in UV or iso line on a curved chart off one ruling goes to the
  fitted composite against a patch that encloses the chart, its Taylor
  remainder in its controls (`pcurve_cache/patch.rs`,
  `PatchClass::Enclosed`).

Both witnesses end as the stated image's miss at f64 and Interval, ε 1e-9
and 1e-12 (`pcurve_cache/closed_image_rows.rs`). The class table, the
refusal census and the cost are in the PR body.
