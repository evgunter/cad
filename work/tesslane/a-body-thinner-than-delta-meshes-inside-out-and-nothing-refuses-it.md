---
id: a-body-thinner-than-delta-meshes-inside-out-and-nothing-refuses-it
kind: issue
title: A body thinner than δ meshes as one sheet counted twice: tessellate returns Ok with a mesh of zero or negative signed volume, check_volume's bound is vacuous there, and the export gate reverses every facet
status: parked
opened: 2026-11-23
priority: P1
cost: M
blocked_on: [the-mesh-cures-a-side-two-faces-can-both-make-three-ways]
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

- 2026-11-23 — **Parked on the shared-side unit** (`the-mesh-cures-a-side-two-faces-can-both-make-three-ways`, decided by Ev on PR 7485 and dispatched). That unit's split is the one cure measured on these cells, and Ev chose it over the alternative that leaves zero volume ("allowing zero volume sounds annoying"). When it lands: re-measure this row's table and the families it did not cover, then decide what holds the sign (a `check_volume` bound that is not vacuous past the wall's thickness, or a read where the patches join).

## What `tessellate` answers, and what closes this row (2026-11-23)

Weighed by three designers apart and reconciled in three rounds, on what the review of PR 7573 found: a thin cap whose two faces triangulate the same rim points with different diagonals comes back as the plane disc twice, where the base returned 0.73 of its volume through a rule that read the neighbour.

- **`tessellate` owes distance and a closed manifold, not volume** (`crates/mesh/README.md`, the promise and `[mesh.winding]`). A mesh of a body thinner than δ that encloses nothing is a correct answer. "Thinner than δ" has a sharp form: δ times the body's area reaches its volume, which is where `validate::check_volume`'s bound stops saying anything.
- **The sign is read per shell, by a check that takes the body, where a solid is needed.** A void's shell is rightly negative, so the read needs the body's shells; the STL writer holds no body, so the export path that holds both calls it and refuses typed, naming δ. Not inside `tessellate`: the viewer tessellates every body at δ = 1e9 for its scale probe (`viewer::scene::SCALE_PROBE_DELTA`), and two faces closer than δ that share no point (a hollow ball's two spheres) give the join nothing to split.
- **What the mesh does to keep thin bodies open is PR 7573's**: a split's point is the foot of the side's straight midpoint on the face's own surface, and the join's census also acts where two faces each have a triangle built only from points they share. That removes the dependence of a thin body's volume on which diagonals two lanes happen to lay, where it can; it promises no volume.

**Owed to close it:** the per-shell check beside `check_volume` (signed volume with the shell's sign, resolved above the rounding of its own sum, the global twin of `resolved_normal`; no ε, no δ); the export path's typed refusal; `validate::signed_volume`'s doc, which calls positive volume "the global orientation invariant" of an outward-wound closed mesh and is false on the base's bored ball; `check_volume`'s doc, whose first-order argument fails exactly where δ reaches the body's thickness; the rows `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` and `reach_probes::geometry_enclosing`, which pass a mesh turned inside out. Then this row's table and PR 7573's class re-measured through the check: how many bodies export refuses at which δ.

**Left open on purpose:** two sheets can cross while the net volume stays positive, so the sign is necessary for a solid's boundary and not sufficient. If the measurement after PR 7573 shows no sign failure at the viewer's probe or down its fit ladder, whether the read could move into `tessellate` is asked again here.

