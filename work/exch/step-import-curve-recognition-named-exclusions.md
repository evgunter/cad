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

Opened 2026-08-11; 1 comment.

Split out of #327, which shipped exactly ONE curve kind: the CLOSED full-period circle. Each exclusion below is named with what promotion would flip.

**1. OPEN (partial) circular arcs — the nearest one.** #327 refuses any carrier whose first and last control points do not coincide within ε_in, because the promoted carrier is trimmed by `geometry::endpoint_params`, and for an open arc that means an ANGULAR interval the locus certificate says nothing about: a carrier that lies on the circle but doubles back adopts as a different arc. Closing it needs an arc-COVERAGE obligation on the traversal, not just the locus. Would flip: the many wild files whose fillet/round rims are stated as trimmed rational quadratics rather than `CIRCLE` + trim.

**2. The surjectivity certificate (also the closed case's one soft limb).** #327's closed-class promotion carries a **turning witness** — five samples at fixed domain fractions, four wrapped azimuth increments required strictly positive — which is a fixed-schedule NECESSARY condition, not a proof of monotone azimuth. The proof wants a derivative composite `(Q × Q′)·â` in ring coefficients; `geom_core::spline::compose` exposes products and linear functionals of the coordinate channels for curves but no derivative channel. Would flip: the witness becomes a certificate, and (1) above becomes tractable by the same machinery.

**3. Ellipse.** ZERO corpus fixtures carry one as a NURBS carrier, so there is nothing to measure against; `Curve3::Ellipse` exists and `endpoint_params` already handles its eccentric anomaly. The certificate shape is known — a plane composite plus a quadric composite in the ellipse's own frame — but `compose::ImplicitSurface` has no general-quadric arm, so it needs one. Would flip: tilted plane×cylinder and equal-radius cylinder×cylinder cut rims stated as splines.

**4. Helix and everything else.** No implicit form in `compose::ImplicitSurface` at all (a helix is not a quadric section), so there is no certificate substrate to build on — this is a `compose` unit before it is a recognition unit. Would flip: swept-thread geometry.

The NEGATIVE control stays green throughout: `TAIL_TURBINE`'s genuine freeform splines must STAY NURBS, and #327 pins that class (`recognize_curve` pin C3, plus the unchanged `wild.rs` refusal row).

## Comments

**2026-08-12** — comment:

(M8 orchestrator) Tightness-scope addendum from #391's final delta check: the merged coverage gate's 150° span cutoff (δ=π/6) is ulp-sensitive to the frame at exactly the boundary — measured spans to ~148° certify, ~152°+ refuse, and a span admitted a hair past 150° is still 30° from the branch cut, so the fuzziness is INCOMPLETENESS for a genuinely-covering coarse carrier, never a false promotion. If this issue's derivative-composite route lands, it retires that boundary fuzz along with the coarseness limit.

## Home

`work/issues/`: `crates/step-import` recognition and `geom_core::spline::compose` are not in any open program's territory, and no open charter names STEP curve recognition.

## Recon addendum (2026-09-04, EXCH orchestrator — pre-spec measurement, against main)

Corrections to this item's premises, measured before any unit is cut:

- **Exclusion 2's witness description is stale**: the five-fixed-samples
  draft was retired by #391's per-knot-span certificate
  (`recognize_curve.rs:123-131` narrates the retirement; the live
  schedule is the per-span cone + wrapped azimuth accumulation at
  `covers_one_full_turn`, :462-557). The 150° limit is the DERIVED
  cone half-angle `(π−δ)/2`, not a literal.
- **The derivative channel is a four-primitive row, not a tweak**:
  `compose` has no derivative anywhere, no degree elevation on
  `BernsteinSpans` (`ch_add`/`ch_sub` poison on unequal degree), and
  no channel×channel dot/cross — the azimuth integrand `(Q × Q′)·â`
  needs all three plus a public door beside `implicit_composite`.
  `hull.rs:49-53` explicitly hands the rational-derivative quotient
  rule to "the consumer that owns the homogeneous form", which is
  `compose`. S-CERT's territory; file the row there with this shape.
- **Open-arc plumbing downstream already works**: `endpoint_params`'
  arc arm handles non-closed intervals (:82-92), `Curve3::Circle` is
  the full carrier with edge-interval trim, and
  `arc_rim_on_wall_boundary` already meters angular containment
  against the rim's own `(t0,t1)` (:902-914). The missing piece is
  ONLY the coverage certificate.
- **Exclusion 1's "would flip many wild files" is unevidenced in the
  committed corpus**: zero open-arc rational carriers exist in any
  fixture — dm1's 14 rational arcs are all closed full circles
  (same vertex both ends), and b123d's 31 rational quadratics sit
  behind its `SURFACE_CURVE` refusal. An open-arc unit needs a new
  fixture (or a re-measured claim) before its acceptance can name a
  flip.
- **Ellipse**: `ImplicitSurface` has five arms (Plane/Sphere/Cylinder/
  Cone/Torus), not three; a general-quadric arm is additive with
  existing primitives (`ch_mul` gives the bilinear terms). Zero
  ELLIPSE entities corpus-wide (56 files swept) — the item's "nothing
  to measure against" confirmed.
