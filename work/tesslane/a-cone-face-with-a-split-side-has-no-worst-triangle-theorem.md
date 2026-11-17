---
id: a-cone-face-with-a-split-side-has-no-worst-triangle-theorem
kind: issue
title: a ruled face whose straight side holds a point meshes on the chart lane at the unproved headroom, and no theorem sizes it
status: closed
priority: P3
cost: M
opened: 2026-10-26
refs: [the-trimmed-cylinder-lane-has-no-worst-triangle-theorem]
closed: 2026-11-17
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

## Moved to TESSSIZE (the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

- 2026-11-14 — moved from TESSSIZE to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Re-pointed (2026-11-17, `a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`)

The arm this row describes is deleted. `curved::grid_counts`' cone arm
takes no rows on any face, and the rectangle lane takes a cylinder or
cone face only with bare sides: one whose straight side holds a point
(a ruling split into several edges, or a cone's apex between two
nappes) is routed to the chart lane (`walk::splits_a_straight_side`,
read in `tessellate::lane_of`). A chord point inside a straight side,
which a spline neighbour's lattice could put there and no body does,
refuses on the rectangle lane (`curved::SIDE_CHORD_POINT`). The
fallback lattice made triangles of no area on a cone through its apex
(the apex copy fanned down the column beside the split side, three
points of one ruling) and the cylinder wall's split ruling exceeded its
certificate (`a-cylinder-wall-with-a-split-ruling-exceeds-its-certificate`).

What is left is this row's debt on the chart lane: such a face meshes
there at `trimmed::UNPROVED_HEADROOM`, on a lattice of rows it does not
need, and no theorem sizes it. Not every one meshes: a ruling split a
hair from its end can fold a triangle
(`a-ruling-split-a-hair-from-its-end-folds-a-triangle`), and a split
ruling's re-chorded rim can reach a thin planar neighbour's refusal
(`a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`). The settling shape is the strip unit's:
the face without the split, with no interior rows, plus one triangle
per split vertex.


## Closed (2026-11-17, TESSLANE, unit 4c, `tesslane/ruled-density`)

The chart lane walks such a face (`tessellate::Lane::Walked`, `crate::ruled_walk`):
no lattice and no triangulation step, its triangles written along its two
rows (its rims, or a rim and the apex) in order of angle, a split side's
points fanned from the corner opposite. Each walked triangle spans at most
one rim chord in angle, so the theorems at `sizing::sagitta_step` and
`sizing::cone_grid_step` bound it, and the face sizes at δ. The count is
the face without the split plus one triangle per point its sides gained,
pinned by `split_rulings::a_split_side_adds_one_triangle_per_point`. A
cone face whose sides pass through its apex onto the other nappe is not
walked and keeps the CDT, which refuses it.
