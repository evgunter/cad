---
id: the-mesh-cures-a-side-two-faces-can-both-make-three-ways
kind: issue
title: A side two faces can both make has three answers in the mesh: insert and rebuild on the triangulating arms, a predictive cut on the walk, and a refusal on the rectangle lane
status: open
opened: 2026-11-22
priority: P1
cost: M
design: true
---

Filed at the closing of unit 4c (PR 7096). A design question (`design: true`): designers weigh it before anything is built, and the options are theirs. This file states what exists and what was measured.

## The state

Two faces that share two boundary chains can each make, as an interior edge of its own triangulation, the same straight side between a point of one chain and a point of the other. The joined mesh then has an edge of four triangles. `tessellate` used to return that as `Ok`.

Today the mesh has three answers to it, in three places:

1. **The triangulating arms of the trimmed lane** (`TrimmedFace::emit`, PR 7198). A kept triangle may not have a side joining two polygon points that are not loop neighbours and that one face across the loop's edges holds both of (`TrimPolygon::faces_holding_side`). Where it has one, a point is inserted and the face rebuilt; past the retry budget it refuses `UnclearedSharedSide`. The test reads which faces hold the two points, not whether the other face makes the side.
2. **The walk** (`trimmed::walk_face`, PR 7096). The same test, then a second reading: the side is cut at its chart midpoint only where the face across could reach it (`trimmed::may_make`: the side's midpoint is not decided farther than `δ·(1 + ALLOWANCE_SHARE)` from that face's surface). It reads where the other face's surface lies, not how that face is meshed.
3. **The rectangle lane** (`curved.rs`). No rule. PR 7198 argues it needs none (one chain's points lie on one iso line), and that argument does not cover two chains held by one neighbour. The census at the join (`tessellate::joined`, `OverusedSide`) refuses the mesh.

Behind all three the census guarantees no such mesh is returned as `Ok`.

## What was measured (PR 7096 and its reviews)

- **The rectangle lane refuses a valid body.** A ball of radius 1.05 bored through at radius 1, unsplit: `OverusedSide` on seven cells (the torus-neighbour sign at δ 3, 1, 0.3; the sphere-neighbour sign at δ 3, 1, 0.3, 0.1); it meshes at finer δ. `a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles`.
- **The walk's rule over-cuts.** With its cut switched off, the census refuses on exactly fourteen bored-ball cells (the same δ as the unsplit body's seven, two split patterns). The rule also cuts where the neighbour lays a row between the rims and makes no such side: on the 1.05 ball at δ 0.1 to 0.04; on a ball of 1.3, a bored torus, a cone-bored ball and a bead, where EVERY cut is unneeded (one review: 256 of about 2,640 cells denser than main, by 2 to 16 triangles; the other: 440, and about 450 at other thicknesses); and on a one-chord band face beside a wider face of the same cylinder (24 cells), where distance cannot exclude a same-surface neighbour. The number of cuts is pinned per cell (`split_rulings::holds_the_relation`).
- **The triangulating arms' rule was not counted** for how often it inserts a point for a side the other face cannot reach (`a-walked-sides-reach-exclusion-is-read-only-by-the-walk`).
- **The walk's exclusion has an unshown premise at the smallest admitted ε** (same row): there the census is what stands.
- **A face's patch already depends on which faces lie across its edges**, which the patch memo's key does not hold (`work/tesslie/the-patch-memo-key-misses-the-faces-across-a-trimmed-face.md`, P1). Any answer that reads more of a neighbour (its surface, its sizing, its mesh) widens that.

## The question

What is the mesh's one answer to a side two faces can both make: which fact about the other face is read (that it holds both points; that it could reach the side; that it does make the side), by which lanes, and with what cure where a lane has no triangulation to rebuild; and what that answer does to the per-face purity the memo rests on. The rectangle lane's seven refusing cells and the walk's over-cut are the two measured costs of the present three answers.

Neighbouring rows: the three above; `work/tesslie/the-shared-side-rule-on-the-spline-arm-has-no-witness.md`; unit 6 (`rectangle-faces-move-to-the-chart-lane`), which would retire one of the three places.
