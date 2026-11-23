---
id: a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles
kind: issue
title: a ruled face's strip and a neighbour holding both of its rims make one rim-to-rim chord, an edge of four triangles tessellate returns
status: closed
opened: 2026-11-17
priority: P2
cost: M
closed: 2026-11-23
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

Measured, placed obliquely: the torus neighbour (sign +1) at δ 3, 1 and
0.3, the sphere neighbour (sign −1) at δ 3, 1, 0.3 and 0.1, at the default ε. The review measured
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

With a ruling split, the bore is the chart lane's walk. It cuts a walked
side whose two ends one face across holds (`TrimPolygon::faces_holding_side`,
the CDT arms' one test) wherever that face may make it (`trimmed::may_make`:
the side's midpoint within `δ·(1 + cert::ALLOWANCE_SHARE)` of its surface,
plus the certificate's rounding; no ε is read), and those cells mesh
manifold (`split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`).
The unsplit bore is the rectangle lane's, which reads no such fact.

**The candidate cure here** is the walk's rule: on the bored ball it cuts
at δ 0.04 and coarser (the shell is 0.05 thick), 15 to 69 sides a cell,
two triangles each, and nothing at a finer δ. Each cut point lies between
the same two rulings, so the rectangle lane's theorem (every triangle
within one column) still covers the cut strip. It over-cuts: on the
bored ball the census refuses only at sign +1 δ 3 to 0.3 and sign −1 δ 3
to 0.1, and the rule cuts at sign +1 δ 0.1 and 0.04 and sign −1 δ 0.04
too.

## 2026-11-22 — on merged main (`54e6a5c5b8`), unit 4c

PR 7198 (TESSLIE, `tesslie/four-use-edge`) put one shared-side rule on
every trimmed arm and a census where the patches join
(`tessellate::joined`, `overused_side`), and closed
`work/tesslie/a-cone-face-near-its-apex-meshes-an-edge-of-four-triangles.md`
and `work/tesslie/two-faces-cutting-one-ear-of-a-shared-chord-chain-leave-a-four-use-edge.md`.
The unsplit bored ball now refuses typed instead of answering a
non-manifold `Ok`: `TessellateError::OverusedSide { uses: 4 }` at sign +1
δ 3, 1, 0.3 and sign −1 δ 3, 1, 0.3, 0.1 (sign +1 is the torus
neighbour, sign −1 the sphere), placed obliquely at the default ε; it meshes at
the finer δ (sign +1 from 0.1, sign −1 from 0.05). Split, it meshes on
main at every cell (the chart lane's CDT and the rule), and on this
branch's walk.

So the defect is a refusal now, not a wrong mesh, and the finding stays:
the rectangle lane has no rule. PR 7198 says it needs none, because its
iso-rectangle domain puts one chain's points on one iso line, exactly
collinear, so no triangle side joins two non-neighbours of ONE chain.
That argument does not cover TWO chains held by one neighbour: the
bore's two rims, each an iso line, which the bore's strip joins by
construction and the neighbour's triangles join too where it lays no
point between them. The census is what catches it.

## 2026-11-22 — band, and where the cure is decided (TESSLANE orchestrator)

Banded P1 when it was a non-manifold `Ok`. It is now a typed refusal of a valid body, on seven cells, all at a δ several times the shell's thickness (a ball of 1.05 with a bore of 1 at δ 0.1 and coarser; a ball of 1.3 meshes at every δ run), so P2: it neither blocks authoring nor entrenches. The cure is not chosen here: the walk's rule would mesh these cells and over-cut others, and which rule the rectangle lane takes is one question with the other lanes', `the-mesh-cures-a-side-two-faces-can-both-make-three-ways` (design).

## Closed

By `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md` (branch `tesslane/join-split`): a side the strip and its neighbour both make is split in both patches where they join (`mesh::join`), and two edges between one vertex pair are never chorded to one segment (`chords::coincident_end_floors`, the bore's seam beside the torus face's). `split_rulings::a_shell_thinner_than_delta_meshes_split_or_unsplit` holds it: the bored ball of 1.05, both signs, unsplit and seam-split, δ 3 to 0.03, every cell a closed manifold by `check_mesh` with a positive signed volume.
