---
id: the-spiric-carrier-step-asks-thirteen-times-its-sampled-curvature
kind: issue
title: sizing::spiric_step chords the sectioned vessel's cavity spiric at 13 times what its sampled curvature asks
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by `one-chart-table-and-the-lane-takes-every-analytic-chart` (branch `tesslane/chart-table`), whose row `mesh::trimmed::tests::the_cone_and_torus_arms_boundary_chords_step_within_their_lattice` prints each boundary edge's counts (`--nocapture`).

**Measured.** On the sectioned vessel's cavity (the tour's bellied meridian revolved a quarter turn and shelled at `1/128`), the torus wall's two spiric edges are chorded 46, 103 and 229 times at δ = 5e-3, 1e-3 and 2e-4. That count is the carrier's own (`chords::compute_chords`' spiric arm, `sizing::spiric_step` at the wall's target `UNPROVED_HEADROOM·δ`). The count `h²·sup|C″|/8 ≤ target` asks at the carrier's sampled second derivative (a 4096-step second difference) is 4, 8 and 17, and the torus row's lattice asks 7, 14 and 31 through the spiric image's speed bound, which is as tight as the sampled speed. On the klein elbow's walls the carrier asks 21, 47, 105 against a sampled 12, 26, 57.

**Where.** `geom::spiric_curvature_sup`, read by `crates/mesh/src/sizing.rs` `spiric_step`. The spiric's chords are its own 3-D chords, so an over-count costs points and never soundness; the factor of thirteen on the vessel (two on the elbow) says the closed-form `sup|C″|` is loose where the cutting plane stands far from the tube's centre circle.
