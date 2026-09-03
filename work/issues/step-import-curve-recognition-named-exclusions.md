---
id: step-import-curve-recognition-named-exclusions
kind: issue
title: step-import stage-1 curve recognition — the NAMED EXCLUSIONS (open arcs, ellipse, helix) and the surjectivity certificate
status: open
opened: 2026-08-11
github: 389
refs: [327, 388, 391]
---

## From GitHub issue 389

opened 2026-08-11, 1 comment.

Split out of #327, which shipped exactly ONE curve kind: the CLOSED full-period circle. Each exclusion below is named with what promotion would flip.

**1. OPEN (partial) circular arcs — the nearest one.** #327 refuses any carrier whose first and last control points do not coincide within ε_in, because the promoted carrier is trimmed by `geometry::endpoint_params`, and for an open arc that means an ANGULAR interval the locus certificate says nothing about: a carrier that lies on the circle but doubles back adopts as a different arc. Closing it needs an arc-COVERAGE obligation on the traversal, not just the locus. Would flip: the many wild files whose fillet/round rims are stated as trimmed rational quadratics rather than `CIRCLE` + trim.

**2. The surjectivity certificate (also the closed case's one soft limb).** #327's closed-class promotion carries a **turning witness** — five samples at fixed domain fractions, four wrapped azimuth increments required strictly positive — which is a fixed-schedule NECESSARY condition, not a proof of monotone azimuth. The proof wants a derivative composite `(Q × Q′)·â` in ring coefficients; `geom_core::spline::compose` exposes products and linear functionals of the coordinate channels for curves but no derivative channel. Would flip: the witness becomes a certificate, and (1) above becomes tractable by the same machinery.

**3. Ellipse.** ZERO corpus fixtures carry one as a NURBS carrier, so there is nothing to measure against; `Curve3::Ellipse` exists and `endpoint_params` already handles its eccentric anomaly. The certificate shape is known — a plane composite plus a quadric composite in the ellipse's own frame — but `compose::ImplicitSurface` has no general-quadric arm, so it needs one. Would flip: tilted plane×cylinder and equal-radius cylinder×cylinder cut rims stated as splines.

**4. Helix and everything else.** No implicit form in `compose::ImplicitSurface` at all (a helix is not a quadric section), so there is no certificate substrate to build on — this is a `compose` unit before it is a recognition unit. Would flip: swept-thread geometry.

The NEGATIVE control stays green throughout: `TAIL_TURBINE`'s genuine freeform splines must STAY NURBS, and #327 pins that class (`recognize_curve` pin C3, plus the unchanged `wild.rs` refusal row).

## Comments

**2026-08-12** — orchestrator:

(M8 orchestrator) Tightness-scope addendum from #391's final delta check: the merged coverage gate's 150° span cutoff (δ=π/6) is ulp-sensitive to the frame at exactly the boundary — measured spans to ~148° certify, ~152°+ refuse, and a span admitted a hair past 150° is still 30° from the branch cut, so the fuzziness is INCOMPLETENESS for a genuinely-covering coarse carrier, never a false promotion. If this issue's derivative-composite route lands, it retires that boundary fuzz along with the coarseness limit.

## Home

`work/issues/`: `crates/step-import` recognition and `geom_core::spline::compose` are not in any open program's territory, and no open charter names STEP curve recognition.
