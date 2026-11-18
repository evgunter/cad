---
id: a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles
kind: issue
title: a ruled face's strip and a neighbour holding both of its rims make one rim-to-rim chord, an edge of four triangles tessellate returns
status: open
opened: 2026-11-17
priority: P1
cost: M
---


Found by the review of PR 7096 (unit 4c, `a-ruled-face-with-a-split-side-meshes-at-the-density-of-the-face-without-it`), on main as well as on that branch.

## Finding

A ball of radius 1.05 bored through along `y` by a coaxial cylinder of
radius 1: the profile `bulge_loop([((1, −h), 0), ((1, h), ±tan(θ/4))])`
revolved a full turn about `y`, `h = √(1.05² − 1)`, `θ = 2·atan2(h, 1)`
(`split_rulings::bored_ball`; one sign makes the face round the bore a
torus face, the other a sphere face). Either neighbour holds both of the
bore's rims. Unsplit, the bore is the rectangle lane's (`mesh::curved`), one
strip of triangles joining its two rims; where the neighbour lays no point
between the rims (δ at or past the shell's thickness, 0.05) its own
triangles join the rims too, and where both faces join the same two rim
points that chord is an edge of four triangles. `tessellate` answers `Ok`,
and `mesh::validate::check_mesh` refuses `NonManifoldEdge { count: 4 }`: the
first such edge is the bore's seam itself, `(0, 1)`, which the bore uses on
both its sides and the neighbour's strip makes as a chord.

Measured, placed obliquely: the sphere neighbour at δ 3, 1 and 0.3, the
torus neighbour at δ 3, 1, 0.3 and 0.1, at the default ε. The review measured
54 such cells over its family, the same on main and on the branch.

## Class

The class of `work/tesslie/two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge.md`:
two faces making one chord between points of their shared boundary,
which no lane's own census sees. That item's chord joins two points of one
chain (an ear); this one's joins points of two chains, the two rims the
neighbour holds. It is not
`work/tesslie/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md`,
whose four uses come from two topological edges between one vertex pair.

## What cures the walked face, and not this one

With a ruling split, the bore is the chart lane's walk, which cuts every
walked side whose two ends one neighbouring face holds at its midpoint
(`trimmed::walk_face`, reading `trimmed::faces_across`, the fact the cone
arm's shared-side rule reads), and those cells mesh manifold
(`split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`).
The unsplit bore is the rectangle lane's, which reads no such fact.

**The candidate cure here** is the walk's rule as it reads now
(`trimmed::may_make`): cut a side whose two ends one neighbouring face
holds unless its midpoint is decided, on the run's band
(`geom_core::k_stats::decide`), to lie farther than
`δ·(1 + cert::ALLOWANCE_SHARE)` from that face's surface
(`geom::Surface::foot`), so that the face's certificate could pass a triangle on it. On
the bored ball that is δ 0.04 and coarser (the shell is 0.05 thick); the
cut adds two triangles per side cut, so the bore's strip of `2n`
triangles becomes `6n` there, and nothing moves at a finer δ. Each cut
point lies between the same two rulings, so the rectangle lane's theorem
(every triangle within one column) still covers the cut strip.
