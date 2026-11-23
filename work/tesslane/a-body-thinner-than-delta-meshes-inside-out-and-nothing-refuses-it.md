---
id: a-body-thinner-than-delta-meshes-inside-out-and-nothing-refuses-it
kind: issue
title: A body thinner than δ meshes as one sheet counted twice: tessellate returns Ok with a mesh of zero or negative signed volume, check_volume's bound is vacuous there, and the export gate reverses every facet
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Found by the designers weighing `the-mesh-cures-a-side-two-faces-can-both-make-three-ways` (2026-11-23), measured again by unit 4c's implementer on main and on PR 7096's head. It is main's, and PR 7096 neither makes nor cures it.

## What

A ball of radius 1.05 bored through at radius 1 is a shell 0.05 thick (volume about 0.137). Meshed at a δ larger than the shell is thick, its outer face and its bore lay the same chords between the same rim points, so the mesh is one sheet counted twice. `mesh::tessellate` returns `Ok`; `check_mesh` passes (every edge has two triangles by vertex id); `validate::signed_volume` is far from the body's and on half the cells NEGATIVE:

| seam split at 0.5, default ε | main `7691e4f872`: triangles, signed volume | PR 7096's head: triangles, signed volume |
|---|---|---|
| arc sign −1, δ 3, 1, 0.3 | 62, −0.0975 | 64, −0.0715 |
| arc sign −1, δ 0.1 | 64, −0.0087 | 88, −0.0390 |
| arc sign +1, δ 3, 1 | 62, +0.0975 | 64, +0.0715 |
| arc sign +1, δ 0.3 | 102, +0.0387 | 104, +0.0282 |
| arc sign +1, δ 0.1 | 152, +0.0591 | 220, +0.1076 |

From δ 0.05 down both trees return +0.082 to +0.117. Two reviewers of PR 7096 measured the same cells apart and agree. One of them also found the sign depends on where the body sits: at sign −1 and δ 0.1 the body is negative on both trees unplaced and under the implementer's oblique placement, and positive on both under that reviewer's own oblique rigid map (+0.082 on main, +0.052 on the head). The unsplit body refuses `OverusedSide` at these δ on both trees.

Written as STL, the negative meshes have every facet reversed by admesh, which the nightly export gate (`scripts/check_admesh.sh`) fails.

## Why nothing refuses it

- `[tolerance.chordal]` promises distance: every point of the mesh within δ of the surface. A sheet lying between two faces closer together than δ keeps that promise for both.
- `[mesh.winding]` is read one triangle at a time against its own face.
- `validate::check_volume` allows `δ · area`, which at these δ is larger than the body's whole volume. `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` and `reach_probes::geometry_enclosing` therefore pass a mesh turned inside out.
- `validate::signed_volume`'s doc calls positive volume "the global orientation invariant" of an outward-wound closed mesh. No check holds it.

## What is known about a cure

On these cells, splitting the collapsed side in BOTH faces at a point of each face's own surface returns +0.078 to +0.092 on every cell and passes admesh (three designers, measured apart); splitting at the chord's midpoint leaves zero volume whose sign is rounding's. That split is the unit `the-mesh-cures-a-side-two-faces-can-both-make-three-ways` specifies, before Ev. It was measured on this one family; nothing proves positive volume for every body thinner than δ.

## What closing it takes

Decide what `tessellate` answers for a body thinner than δ: a mesh with the body's sign (and by what cure, on which families), or a typed refusal naming δ against the thickness. Then a check that holds it: `check_volume` with a bound that is not vacuous there, or a sign read where the patches join.
