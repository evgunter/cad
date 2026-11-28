---
id: a-rational-walls-plane-three-eps-off-its-edge-is-red-on-main
kind: issue
title: a_rational_walls_plane_three_eps_off_its_edge_certifies_as_mid_wall is red on main: limb 2 bounds the fitted carrier 1.63e-8 m off, past the tolerance
status: open
opened: 2026-11-25
---


(Found by `inside/ray-spline-face-arm`'s fix pass, 2026-11-25, running
`cargo nextest run -p geom-brep` locally; out of that lane's fence.)

## What

`crates/geom-brep/tests/m5_pr7_ssi.rs:3568`,
`a_rational_walls_plane_three_eps_off_its_edge_certifies_as_mid_wall`,
fails on origin/main (aa5bee4e4), in a clean worktree of it, in 41 s:

> weights 2, ½, 1, 4, the plane 3ε off its edge at ε 1e-9: expected a
> branch, got ssi: limb 2 (control-hull sup-norm bound through its
> pcurve) bounds the fitted carrier's distance from the locus it claims
> by 1.634298373860002e-8 m, past the tolerance

The row is the pin `ssi-a-near-side-locus-could-take-the-walls-own-iso-curve`
names (re-measured 2026-10-04: every wall in its table certifies). Not
bisected. It sits in `.config/nextest.toml`'s slow-set filter, so a
PR's CI runs it only where its diff names it.
