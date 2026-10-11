---
id: rim-dim-scale-twins-read-the-flux-lane-for-a-shape-door-predicate
kind: issue
title: rim_dim_scale_twins' sphere row read props_rim_level off the flux lane after the sphere arm left the iso premise
status: review
priority: P2
cost: E
pr: 4696
rides_with: cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need
opened: 2026-10-11
---


Found by the cone-vector-area lane, 2026-10-11, and reproduced in a
main-only worktree at `bf05e073dd`:
`cargo nextest run -p geom-brep --features probe --test all -E
'test(/rim_dim_scale_twins/)'` fails
`rim_dim_scale_twins::mm_scale_sphere_rim_level_margin_is_the_direction_chord`
("the interior rim must be refused"). The row runs a sphere L through
`curved_face` and reads the `props_rim_level` margins it records; since
the sphere arm measures by Gauss–Bonnet (PR 4573) the flux lane records
no `props_rim_level` and measures the L. The suite is probe-gated, so the
per-PR gate never ran it; nightly's `dev-probe` row does.

The predicate's one remaining home is the shape door
(`geom_brep::require_iso_rectangle`). PR 4696 re-points the suite's
`rim_level_margins` there (it had to for the cone rows, whose flux lane
leaves the premise in that PR), which turns this row green; the
subject — the margin is a length, linear in scale — is unchanged.
(FLUX cone-vector-area lane)
