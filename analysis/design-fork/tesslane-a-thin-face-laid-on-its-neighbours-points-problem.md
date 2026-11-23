# A thin face laid on its neighbour's points: the problem

You are a DESIGNER lane on a Rust B-rep CAD kernel (repository evgunter/cad). Read `CLAUDE.md`, then `docs/prompts/designer.md` IN FULL: it binds you, and its report format is your deliverable. You are not an orchestrator; `memories/orchestration-model.md` is not about you.

What follows states a problem. It offers no answer, and the options are yours to find. Treat every sentence of it as a hypothesis to check against the tree.

## Where to look

- The tree to read is a checkout of pull request 7573's frozen head, `24ee9a28fb` (branch `tesslane/join-split`), at `/root/.local/share/cad-work/tesslane-fork5-tree`. It is READ-ONLY for you: do not build, run, edit, commit or push anything there or anywhere. The base the pull request is measured against is its merge-base with main, `dd8e70198d`; read a base file with `git -C /root/.local/share/cad-work/tesslane-fork5-tree show dd8e70198d:<path>` and the unit's diff with `git -C … diff dd8e70198d...24ee9a28fb -- crates/mesh`.
- Do not read any branch under `analysis/`, any pull request's comment thread, or `/root/.local/share/cad-work/` outside that one checkout.
- The unit's file: `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md` (the decision, what is built, its measurements, and "what the measurements did not cover").
- The open row this touches: `work/tesslane/a-body-thinner-than-delta-meshes-inside-out-and-nothing-refuses-it.md` (P1, parked on the unit above).
- The code: `crates/mesh/src/join.rs` whole (`census`, `split`, `held`, `midpoint`), `tessellate.rs`' join, `trimmed.rs` (the chart lane; on the base, its shared-side insert, `faces_holding_side`), `curved.rs` (the rectangle lane), `cert.rs` (the per-triangle certificate), `validate.rs` (`check_mesh`, `signed_volume`, `check_volume`).
- The pages: `docs/DESIGN.md` `[tolerance.chordal]`, `[failure.taxonomy]`, `[loud-failure]`; `crates/mesh/README.md` `[mesh.winding]`, `[mesh.sizing]`.

## Terms

The mesher turns each face of a solid into a **patch** of triangles and **joins** the patches on shared vertex ids. Two faces that share an edge share that edge's **chord** points. δ is the chordal tolerance: every point of the mesh lies within δ of the true surface, which each triangle's **certificate** holds. ε is the kernel's coincidence tolerance; the mesh layer reads no ε in what chooses a mesh. An **inside side** of a patch is a triangle side that is not a chord of the face's boundary.

## What the unit does, and what the owner decided

Two faces that share two boundary chains can each make, as an inside side, the same straight side between a point of one chain and a point of the other; the joined mesh then has an edge of four triangles. On the base three rules answered that, each by predicting the face across (a lane inserted a point on a side its neighbour could make). The unit retires all three: the join's census finds each side more than one patch has inside, and every such patch splits it at a point of its own surface over the side's chart midpoint (on a cone, the midpoint in its development). No lane reads its neighbours.

The owner chose that over a split at the side's straight midpoint, on a measurement: on a shell thinner than δ (a ball of radius 1.05 bored at radius 1) the straight midpoint leaves the two sheets on one set of chords, a mesh of zero volume whose sign is rounding's, and the own-surface point reopens the wall with the right sign. The owner's words on the question: "what bad thing happens when there's zero volume?", and after the answer, "ok yeah allowing zero volume sounds annoying; this plan sounds good". The unit's file says positive volume on a body thinner than δ "is what was measured on these cells, not a theorem, and no page promises it". The orchestrator's standing ruling for the unit: no body that meshes on the base may be lost.

## What the review found

All of it is measured on both trees by a reviewer and again by the implementer, identical at ε 1e-9, 1e-6 and 1e-12. No cell below is flat or negative on the base.

**1. A thin face laid flat on its neighbour.** `sweep`'s `snowman::a_cut_ends_at_the_nearest_of_several_boundary_hits` builds a cap of height 0.001 of the unit ball: one sphere face and one plane face on one circle (true volume 3.1405e-6). Per-patch triangles [sphere, plane], positions, signed volume:

    base, lat ±60, δ 0.04 / 0.02 / 0.01:  [18, 6], 14,  +2.2794e-6 (0.73 of the truth)
    head, lat +60, the same δ:            [14, 14], 16, +1.8649e-6 (0.59)
    head, lat −60, the same δ:            [6, 6], 8,    −7.33e-20

At lat −60 each patch is n − 2 triangles on the circle's n chord points and nothing else; the two patches lay DIFFERENT diagonals, so no side is shared, the census finds nothing and nothing is split. The mesh is the plane disc twice. Every edge has two triangles, the certificate holds (the cap is thinner than δ), `check_mesh` passes, and the row that builds the body passes on both trees. At lat +60 the two patches happen to lay the same diagonals, so those sides are split.

On the base, the sphere face's retired rule inserted a point of the sphere on every side between two non-neighbouring boundary points that the plane face's boundary held, whether or not the plane face made that side. The unit calls that a wrong prediction (997 points inserted where about 502 sides were in fact made by both). On this body those points are the only points the sphere patch had off the plane.

**2. The class.** Caps cut off a trimmed sphere face (the ball less a box, as in that row) at height h 1e-3, 1e-2 and 3e-2, twelve latitudes, four azimuths, δ from 10 h to h/10: 435 cells, 106 moved. On the head:

| h | flat (volume at rounding) | refused `FoldedTriangle` where the base meshes | positive, under half the base's volume |
|---|---|---|---|
| 1e-3 | 11 | 0 | 8 |
| 1e-2 | 11 | 2 | 9 |
| 3e-2 | 5 | 27 | 11 |

The flat cells and the cells under half are mostly at δ ≥ h. Lowest positive ratios to the truth: 0.015, 0.057, 0.071, against the base's 0.73 to 0.84.

**3. Bodies lost.** The refusals are not only "thinner than δ": they run down to δ = h/3, mostly on caps that reach to within a degree of the sphere's pole; a cap of height 0.1 of the plain ball at lat −60 refuses at δ 1 and 0.3 too. Traced by the implementer: every one of them is the join's own check of a split's pieces (`join::held`: a piece wound against the sphere), none is the lane's fold cure, and they sit where the chart midpoint of a side lies far from the side (the chart is stretched near the pole).

**4. Where nothing moved.** Caps of a plain ball that the rectangle and polar lanes take, axial lenses of two spheres, a thin cylinder segment, a cone's tip cap: 168 of 180 cells identical to the base. An off-axis lens of two spheres loses 6 to 17% of the base's volume at the two coarsest δ. In the `mesh` suite the unit goes the other way: 23 cells of the bored ball that are NEGATIVE on the base are positive on the head.

**5. What holds none of it.** `[tolerance.chordal]` promises distance, and a sheet between two faces closer than δ keeps it for both. `[mesh.winding]` is read one triangle at a time against its own face. `validate::check_volume` allows δ · area, which is larger than these bodies' whole volume. `validate::signed_volume`'s doc calls positive volume "the global orientation invariant" of an outward-wound closed mesh; no check holds it. The parked row asks what `tessellate` answers for a body thinner than δ and has not been decided.

## What needs deciding

1. What the mesher owes for a body like these, where two faces closer together than δ are bounded by the same points: what a correct answer from `tessellate` is, and what fact of the body, the patches or the mesh decides it. The base met it by accident, through a rule that read the neighbour; the unit's decided design (a split where two patches make one side) does not reach a patch that shares no side with the patch it lies on.
2. Where the point a split adds belongs on a face whose chart is stretched, given that the present one loses bodies the base meshes.
3. What of this the unit must land before it merges, and what belongs to the parked row. The unit cannot merge while it loses bodies; whether it may merge returning a flat mesh where the base returned most of the volume is part of the question.
4. Whether any ratified text has to say something it does not (the pages above), or says something that makes this hard.

If you need a measurement to decide between final states, name it exactly in `For the orchestrator` (the body, the trees, what to read); the implementer has the instrument and can take it.

## Your report

Per `docs/prompts/designer.md` §5: `## For Ev`, then `## For the orchestrator`; at most 150 lines in all; name no model, no lane and no other designer. Deliver it as your final message.
