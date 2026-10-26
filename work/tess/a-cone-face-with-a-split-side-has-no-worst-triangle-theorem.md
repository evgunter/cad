---
id: a-cone-face-with-a-split-side-has-no-worst-triangle-theorem
kind: issue
title: a cone face whose meridian side carries a point between its rims is outside the two-rim strip argument, and keeps a lattice no theorem sizes
status: open
priority: P3
cost: M
opened: 2026-10-26
refs: [mesh-sizes-each-lane-by-its-worst-admissible-triangle, the-trimmed-cylinder-lane-has-no-worst-triangle-theorem]
---

Found in PR 5717's fix pass (PRED). That pass drops the curved cone's
interior rows, since `cert::cert_cone` never reads v: with both rims
chorded on the column step, the strip argument at
`sizing::sagitta_step` holds every triangle between the two rims to one
column. The argument needs **bare sides**. When a meridian side carries
a point between the rims, a triangle on two side points and a far rim
point spans several columns. Two cases do this: a ruling split into
several edges (`curved::tests::a_multiply_carried_iso_side…`'s `cone`
fixture), and the apex between two nappes (its `mirror nappe`
fixture). Without rows those faces refused `CertificateExceeded` at 2δ
(0.207 against 0.1).

Such a face keeps the lattice it had: rows every `ρ_max·hu` slant
metres, and the pole floor. `curved::grid_counts`' cone arm says so.
The certificate holds it. No theorem sizes its off-lattice side
triangles: the UV lattice's aspect mixes radians with slant metres,
and the side points are off its rows. This is the same class as the
trimmed cylinder's debt.

**What would settle it.** A bound for a constrained Delaunay triangle
on a side point in the cone's lattice, made unit-consistent: run the
CDT, or state the lemma, in `(ρ_max·u, v)`. Then size it, pin it
beside `sizing::cone_grid_step`, and drop the fallback.
