---
id: rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes
kind: issue
title: patch_bound's rational arm hulls the f64-refined net: the described patch's true second partial exceeds the certified sup by ULPs, measured exactly
status: closed
opened: 2026-09-18
refs: [nurbs-face-bound-unsound-on-a-random-rational]
priority: P0
cost: H
parent: TESS-2
closed: 2026-09-22
---


Filed by the TESS orchestrator from a diagnostic lane's measurements
(branch `tess/nurbs-bound-diag` at `db4cb45a8`: instrumentation in
`crates/mesh/src/nurbs_cert_fuzz.rs` `mod diag`, exact-rational referees
`diag/exact_check.py` and `diag/ingredient_check.py`, logs under
`diag/logs/`). On PROPS' slate because the wrong step is in
`crates/geom-brep/src/patch_bound.rs`; the row it explains is TESS's
`nurbs-face-bound-unsound-on-a-random-rational`.

## The defect

`patch_bound::rational_cells` (and `patch_cells_refined` beside it)
refines with the plain-`f64` `NurbsSurface::refine_knots_u` /
`refine_knots_v`, then wraps the refined weights and control points as
`RingInterval::point`. Everything after that is outward-rounded;
the insertion is not. So the cells enclose the refined-`f64` patch, and
the DESCRIBED patch escapes them by the insertion rounding, amplified
by the knot differencing (×16 at `RATIONAL_CERT_SPLITS = 16`).

`PatchCell`'s doc ("What the enclosure encloses") already states the
gap and says it "matters at exactly one place: a component whose true
value is structurally zero". **That sentence is falsified**: it matters
wherever the interval recurrence is tight, and on a bilinear rational
patch it is tight to ~1e-15 at a domain corner, where every factor's
extreme coincides.

## Measured (exact rational arithmetic; every f64 input is a rational)

Bilinear rational patch, knots `[0,0,1,1]²`, weights ≈ 0.0103–0.0135,
at `(u,v) = (0,0)`, the cell that attains the global `muu`:

| quantity | value |
|---|---|
| certified `muu` | 2.66033199807363907 |
| TRUE `‖S_uu‖`, described surface | 2.66033199807363989 (over, +3.07e-16 rel) |
| TRUE `‖S_uu‖`, f64-refined surface | 2.66033199807362752 (under, −4.3e-15) |

refined truth < bound < described truth. The sampler's own f64 error is
≤ 6e-17 per channel and is not the cause. A second surface (seed
`0x5ca58da03160d407`, trial 29) measures the same way, +2.44e-16.

Ingredient hulls on that cell against the described surface's exact
truth: **violated** — `w_u` (4.9e-15 rel), `w_v` (3.6e-14), `Ã_u`,
`Ã_v` components (6–8e-16), `w` (1.7e-16); held — `S − c`, `Ã_u[0]`.
The refined cell's four control points and weights carry ≤ 4.7e-16
relative insertion error; the differencing makes the rest.

Transposing the surface moves the excess from `uu` to `vv` with
bit-identical numbers: no index asymmetry. The INTEGRAL arm never
refines and showed no excess on any nonzero component in 18,000 trials.

Frequency in `mesh`'s `r1_random_rational_soundness_sweep`: 0 reds in
90,000 general trials, 2 in 30,000 bilinear-only trials (worst excess
1.35e-15 relative, 7 ULPs; `uv` can red too). Every recorded red,
hosted or local, is `pu = pv = 1`. Not measured: higher split counts
(`patch_cells_refined` as `offset_meters` calls it), where the
amplification should grow.

## Blast radius

`mesh`'s `NurbsFaceBound` (`muu`/`muv`/`mvv` → the Q/4 per-triangle
certificate, `split_steps`, `band_schedule`, `chords::nurbs_tighten`)
and `offset_meters` (`cell_normal`, `patch_regularity`,
`cell_curvature`, `patch_collapse`). The shortfall is ~1e-15 relative,
far under δ and the documented ε slack, so no mesh is wrong by it —
but the certificate's claim is domination, the claim is false, and
`mesh`'s falsifier is a tree-wide flake at roughly 4e-4 per hosted run
until it is true.

## What closes it

The structural close is to make the insertion part of the enclosure:
refine in the ring (the refined net as `RingInterval`s, so
`RingInterval::point` is applied to the DESCRIBED net and the insertion
widens outward like every later step). Then `PatchCell`'s caveat section
and its "structurally zero" carve-out retire, and the quarter cylinder's
`z` enclosure contains its zero. A padding constant on the sups is the
alternative and is a measured-dust fudge with no bound behind it (the
lane could not bound the dust; 1.35e-15 is an empirical maximum).
Related, same function family: `refine-dir-hairline-knot-insertion`.

`crates/mesh/src/nurbs_cert.rs`'s module header says "interval (ring)
arithmetic end to end"; it is TESS's sentence and TESS re-words or
keeps it according to which close lands here.

Signed: (TESS orchestrator)

## Claimed by TESS, 2026-09-20

Filed on PROPS on 2026-09-18, cut to ENCL on 2026-09-20, and claimed
back the same day: PROPS is paused, ENCL has no orchestrator seated,
and Ev said not to wait (in chat, 2026-09-20) — the defect reds
`mesh`'s falsifier tree-wide and TESS holds the diagnosis. TESS-2
carries it as an announced crossing into
`crates/geom-brep/src/patch_bound.rs` (ENCL's path; shared ground is
expected under ENCL's own `keep_out`). The file moves with its id, per
`work/README.md`'s claiming rule.

## The sweep no longer reds on this (2026-09-21)

RING-2 (SCALAR, PR 3032) gave the sweep a 64-ulp sampler allowance,
so `r1_random_rational_soundness_sweep` is green on main while the
certificate is still short by the amount measured above. Filed as
`work/chord/soundness-sweep-allowance-is-fifty-times-the-measured-
sampler-error.md`; TESS-2's Phase 1 rows compare the exact truth BARE
so the defect stays red until fixed.

## 2026-09-22 — the INTEGRAL refined arm was unsound too (reviewer finding)

This row's title says rational and its diagnosis is about
`rational_cells`. A blinded review of TESS-2's head measured the same
defect on the other arm: `patch_cells_refined`'s INTEGRAL branch also
refined with the plain-`f64` `refine_knots_u/v` before assembling, so
its cells enclosed the refined-`f64` patch too. Over 12,615 exact
containment escapes found on the reverted tree, the BULK were that
branch's point hulls excluding the described value at cell corners —
more than the rational arm's, because the integral arm's cell enclosure
IS the coefficient hull with no quotient rule to widen it, so the
insertion rounding has nothing to hide behind.

The close covers both: TESS-2's fix routes the integral refined branch
through the same ring schedule (`integral_cells_refined`), and the same
review counted 704,835 exact containment checks with zero escapes on
the fixed head.

The consumer that reaches the integral refined branch is
`offset_meters`, through `patch_cells_refined` at its own split count —
`patch_cells` never refines an integral face, which is why no shipped
tessellation path showed this and why the rational arm is the one the
falsifier found.
