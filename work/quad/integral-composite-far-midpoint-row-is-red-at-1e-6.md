---
id: integral-composite-far-midpoint-row-is-red-at-1e-6
kind: issue
title: cert5 integral_composite_midpoints_far_from_the_origin_stay_sound is red at 1e-6: the convergence margin lands in the ambiguity band
status: closed
opened: 2026-10-23
priority: P3
cost: E
refs: [5598, the-integral-composite-reads-its-midpoint-rule-at-a-rounded-midpoint]
closed: 2026-10-27
---


## What

`crates/geom-brep/tests/cert5_r1_patch_probes.rs`
`integral_composite_midpoints_far_from_the_origin_stay_sound`, the row
#5598 (`quad/integral-composite-midpoint`) pinned, is red in the
nightly's full suite at `CAD_TOLERANCE_EPS=1e-6` only (main
`be9b907694`, run 39906156937, its first nightly):

    far u true: the bilinear integrand must certify: whether the
    quadrature's enclosure has converged is undecided: margin
    1.0147622233119443e-6 lies inside the ambiguity band
    (1e-6, 9.999999999999999e-6).

The default and 1e-12 rows pass. The per-PR gate drew neither the
1e-6 row nor the slow set for it, so the PR never saw this.

## Owed

Decide whether the row's "must certify" holds at a coarse ε (the
witness's flux is about `2.8e-8` wide, and the convergence margin sits
at ε itself), and either size the witness so the margin clears the
band at every row or state the refusal at 1e-6 as the row's answer
there.

— INTENT nightly triage, 2026-10-23

## Closed

Dissolved by PR 5628 (measure door unit B), checked 2026-10-27 on main
`9267d3ae`. The red was `props_quad_converged`'s margin landing in its
ambiguity band. Since 5628 that predicate is minted nowhere: a resolution
is a plain comparison, and the row's door (`face_posture` in
`crates/geom-brep/tests/shared/patch.rs:402`, through `to_target` at
`:320`) accepts on `mean_boundary_displacement(flux, area) <= 1024·ε`, with
no band to land in. `grep -rn props_quad_converged crates/geom-brep` prints
nothing.

Re-run at all three ε:
`CAD_TOLERANCE_EPS=<ε> cargo nextest run -p geom-brep --test all -E
'test(integral_composite_midpoints_far_from_the_origin_stay_sound)'`
passes at 1e-6, 1e-9 and 1e-12.
