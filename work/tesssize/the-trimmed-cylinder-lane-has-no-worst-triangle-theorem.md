---
id: the-trimmed-cylinder-lane-has-no-worst-triangle-theorem
kind: issue
title: the trimmed cylinder lane has no worst-triangle theorem, so it sizes at UNPROVED_HEADROOM = ½ of δ
status: open
priority: P2
cost: M
opened: 2026-10-25
---

Filed by PRED's `mesh-sizes-each-lane-by-its-worst-admissible-triangle`
unit, which built `crates/mesh/README.md` `[mesh.sizing]`: every lane
sizes at the coarsest step its own pinned worst-triangle theorem
grants, and a lane without one carries a named `UNPROVED_HEADROOM` at
its call sites, filed as a debt. This is that debt.

**Where it is.** `mesh::trimmed::UNPROVED_HEADROOM` (0.5), read at two
call sites: the cylinder arm of `tessellate_trimmed`'s grid sizing
(`sagitta_step(δ·UNPROVED_HEADROOM, r)` for the lattice's column step)
and `mesh::chords::edge_target`, which chords every edge of a trimmed
cylinder face at the same share of δ so the trim polygon is the one the
lattice was sized beside. Both read exactly what the retired
`sizing::sizing_target` gave them, so this lane's meshes did not move
when the rule landed.

**What is known.** A lattice cell-half certifies (`cert::cert_cylinder`)
at the sagitta of the column step, so at the headroom it lands at δ/2.
A trim chord is a chord of a curve that lies on the cylinder, so its
midpoint is no farther from the cylinder than from the curve: a trim
chord's own azimuth extent is within `sagitta_step(target, r)`. What is
missing is the triangle BESIDE a trim chord. The lane's CDT is
constrained, so a triangle on a trim chord need not have an empty
circumcircle on the far side of the trim, and its apex can be a lattice
point more than one column from the chord's ends. Nothing bounds how
far, so nothing bounds that triangle's azimuth extent below the chord's
plus some multiple of the column step.

**What would settle it.** A lemma bounding the azimuth extent of a
constrained Delaunay triangle on a trim chord in the lane's lattice
metric (`trimmed::lattice_metric`, square in 3-D), in terms of the
chord's own extent and the column step; then the lattice and chord
steps solved so that triangle lands at δ, pinned beside the step in the
shape of `sizing::tests::torus_grid_steps_meet_the_bound_with_equality`,
and the factor deleted. Measuring the worst certificate of the corpus's
trimmed cylinder faces at headroom 1 would say whether the theorem
would be cheap; it is not a substitute for it.

## Moved to TESSSIZE (the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
