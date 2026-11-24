---
id: the-mesh-cures-a-side-two-faces-can-both-make-three-ways
kind: unit
title: A side two faces both make is split where the patches join, once, in every patch that has it inside, at a point of that face's own surface; the three present rules retire
status: spec
opened: 2026-11-22
priority: P1
cost: H
---

Filed at the closing of unit 4c (PR 7096) as a design question, weighed by three designers over four rounds (2026-11-22 and 23), and decided by Ev on PR 7485 (2026-11-23: "allowing zero volume sounds annoying; this plan sounds good"). This file is the unit that answer specifies.

## The defect

Two faces that share two boundary chains can each make, as an inside edge of its own triangulation, the same straight side between a point of one chain and a point of the other. The joined mesh then has an edge of four triangles.

The mesh answers it today in three places, each by predicting what the face across will do:

1. **The triangulating arms of the trimmed lane** (`TrimmedFace::emit`, PR 7198). A kept triangle may not have a side joining two polygon points that are not loop neighbours and that one face across the loop's edges holds both of (`TrimPolygon::faces_holding_side`). Where it has one, a point is inserted and the face rebuilt; past the retry budget it refuses `UnclearedSharedSide`. It reads which faces hold the two points, not whether the other face makes the side.
2. **The walk** (`trimmed::walk_face`, PR 7096). The same test, then a second reading: the side is cut at its chart midpoint only where the face across could reach it (`trimmed::may_make`, by the side's distance from that face's surface). It reads where the other face's surface lies, not how that face is meshed.
3. **The rectangle lane** (`curved.rs`). No rule; the census at the join (`tessellate::joined`, `OverusedSide`) refuses the mesh.

Each lane predicts its neighbour, and each prediction is wrong somewhere: the first inserts a point for a side the other face never makes (997 points in 103 rebuilds over the `mesh` suite where about 502 sides are in fact made by both), the second cuts where the neighbour lays a row between the rims (256 to 440 of about 2,640 cells denser than main), and the third refuses a valid body (a ball of radius 1.05 bored at radius 1, unsplit, on seven cells). And a face's patch depends on which faces lie across its edges, which the patch memo's key has to carry (PR 7241).

## What is built

**No lane reads anything about the faces across its edges.** Whether two faces both made a side is a fact that exists only where the patches join, and that is where it is acted on.

1. **Detected at the join.** `tessellate::joined`'s census finds each straight side that more than one patch has as an inside edge: a side of two of that patch's triangles, not a chord of its boundary.
2. **Split once, in every such patch.** Each patch that has the side inside splits it at the point of ITS OWN surface over the side's midpoint in its chart. Its two triangles on that side become four. Each half is held to that face's certificate and to the winding read (`[mesh.winding]`), as any triangle of the face is; one that fails refuses the face, typed, as it would have from its lane.
3. **Nothing is meshed again.** No lane is re-entered, there are no rounds, no patch is chosen to keep the side, and nothing reads the order faces are stored in.
4. **A boundary does not give way.** Where one face's boundary chord carries the side, that face keeps it, and every patch that has it inside splits.
5. **Two things beside it, needed under any cure:**
   - **A chord floor.** Two edges joining one vertex pair are never chorded to one segment. The seven refusing cells of the unsplit bored ball hold this second defect: the bore's two seam-side edges both chord to the segment between the rims' vertices, which the census passes and `check_mesh` refuses.
   - **A fold's own cure.** A face whose triangulation folds gets a point inserted for the fold, not for a shared side (two needle-cone cells lean on today's shared-side insert for this).
6. **What retires:** PR 7198's rule on the triangulating arms with `faces_holding_side` and `UnclearedSharedSide`; the walk's cut and `may_make`; the memo key's entry for the faces across a face; `OverusedSide` as a refusal (after the split a side used past twice is a defect of the join, reported as `[failure.taxonomy]` reports a kernel defect).

No clause of `crates/mesh/README.md` or `docs/DESIGN.md` changes. One sentence of `crates/mesh/src/lib.rs` (the per-face purity of a patch, written by PR 7241) is rewritten: a patch is a function of its face and δ, and the join adds the split points.

## Why a point of the face's own surface, and in every patch

The other consistent answer is the side's straight midpoint, split in every patch but one. Its halves are pieces of triangles already certified, so it needs no certificate and cannot refuse, but one patch must keep the side, chosen by storage order. A straight midpoint in every patch is not an answer: the two new points share a position and the same two corners, which is the edge of four triangles again for any reader that joins vertices by position.

The two were measured against each other where they differ, on a body thinner than δ: the ball of radius 1.05 bored at radius 1 (a shell 0.05 thick, volume about 0.137), both arc signs, seam split and unsplit, δ 3 to 0.1. There both faces lay the same chords, so the mesh is one sheet counted twice.

| | signed volume of the returned mesh | written as STL, read by `scripts/check_admesh.sh` |
|---|---|---|
| today (PR 7096's head) | −0.0715, −0.039, +0.028, +0.0715 on the split body; the unsplit body refuses on seven cells | 4 of the 21 meshes returned fail: every facet reversed |
| straight midpoint, all but one patch | zero to rounding on the split body (±10⁻¹⁶); +0.010 to +0.015 unsplit | 4 of 28 fail: the sign of a zero volume is rounding's |
| own-surface point, every patch | +0.078 to +0.092 (58 to 67% of the body's) | 0 of 28 fail |

A straight split leaves the two sheets on one set of chords. A point of each face's own surface moves each sheet toward its own face, so the wall reopens with the right sign. Today's walk cuts one face only, which is why its volume's sign follows which face that is.

Over the `mesh` suite at the default ε, with the three present rules off and the chord floor and fold cure on: 1,416 such splits over 59 meshes, no half folded against its parent, none farther than δ from its surface by sampled distance (worst 0.905 δ, none worse than its parent), no two new points at one position, no side left overused; six rows red, each a pin of today's cut counts or of a refusal's name.

Positive volume on a body thinner than δ is what was measured on these cells, not a theorem, and no page promises it.

## What the measurements did not cover

- The halves were held to sampled true distance, not to each lane's own certificate; the cone's and the torus's are more conservative.
- The probes split at the surface foot of the 3-D midpoint, standing in for the midpoint in the face's chart; on a cone's development the two differ, and at an apex the chart coordinate has to be chosen.
- Two spline faces can both make a side (`faces_that_bound_no_region::a_lens_of_two_short_arcs_meshes_at_a_coarse_delta`, with the present rules off). A spline surface has no closed-form foot, so its split point comes from the lane's polygon in the chart.
- The halves did not meet the winding read; ε 1e-6 and 1e-12, the slow set and the other crates were not run; what the viewer's pick memo does with a patch that carries split points was not read.

## Unit

**Letter H. Tier: dual review.** It moves a rule out of three lanes into the join, changes what a patch is a function of and what the memo key holds, retires another program's rule (TESSLIE's PR 7198 and PR 7241), and the split's safety on every chart rests on measurement the build has to complete.

Stage 1 measures what the list above names, on the lanes' own certificates and with the chart midpoint, over the `mesh` suite at the three ε, and stops if any half refuses on a body that meshes today. Stage 2 builds it.

Neighbouring rows: `a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles` (main's unsplit bore: closed by the chord floor here); `a-walked-sides-reach-exclusion-is-read-only-by-the-walk` (moot once the walk's rule retires); `work/tesslie/the-shared-side-rule-on-the-spline-arm-has-no-witness.md` (the lens above is a candidate witness); `work/ciw/the-admesh-gate-cannot-see-an-edge-of-four-triangles.md`; unit 6 (`rectangle-faces-move-to-the-chart-lane`).
