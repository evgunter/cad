---
id: a-refused-certificate-adds-a-point-and-no-face-sizes-below-delta
kind: unit
title: Every lane sizes at δ from its chart's cell bound, and a triangle whose certificate reads over δ gets a point and the face is triangulated again; UNPROVED_HEADROOM goes
status: dispatched
opened: 2026-11-23
priority: P1
cost: H
branch: tesslane/certificate-decides
---


The unit `crates/mesh/README.md` `[mesh.sizing]` specifies, as that clause reads since the `[ev]` pull request that carries this file (two designers weighed it apart, each on its own measurement of the `mesh` suite, and agreed on their first reports; fork-log row in `docs/DESIGN-FORK-LOG.md`).

## What it is

The chart lane's analytic arms (cylinder, cone, sphere, torus, spindle) size their lattice and chord their edges at `trimmed::UNPROVED_HEADROOM` = ½ of δ, because a triangle beside a trim chord can reach past one lattice cell and nothing bounds by how much. The factor bounds nothing either: faces refuse on the certificate at the headroom today, and on notched cylinder walls the worst certificate at the headroom reads 0.689 δ, already past half. It costs about twice the triangles on every such face, and on every rim such a face shares.

What holds a triangle within δ is its certificate, triangle by triangle, on every lane. The step only chooses density, and a first guess that leaves one triangle over δ is cured by one more point, not by a refusal and not by shrinking every step.

- **The step.** Every chart-lane face sizes its lattice and chords its rims at δ, from its chart's cell bound (`sizing.rs`; proved and pinned for every chart). `UNPROVED_HEADROOM` and `face_target`'s branch on it are deleted.
- **The correction.** A triangle whose certificate reads over δ gets one interior point and the face is triangulated again. The lane does this today for spline rows only (`refine_lane` in `trimmed.rs`); it runs on every row, under its own named round budget (a termination constant beside `MAX_GRID_RETRIES`, not shared with the fold and shared-side retries), and the face refuses `CertificateExceeded` when the budget ends.
- **Every lane that triangulates.** The rectangle lane's constrained triangulation gains the same correction: its proof covers a point on a grid line and not a chord point its grid did not place (`a-filleted-one-segment-hole-rims-band-exceeds-its-mesh-certificate`).
- **No ε is read** by the correction: it reads the certificate's verdict alone (`[tolerance.chordal]`, `[mesh.winding]`).

## What is measured, and what stage 1 owes

Measured by the two designers over the `mesh` suite at one ε, sized at δ with the lattice otherwise as today (about 1,120 meshings of chart-lane analytic faces that mesh today):

- Without the correction, 14 meshings newly refuse (13 on the certificate, one fold).
- With the correction at the centroid, none newly refuses on the certificate; at most 4 rounds, 85% of faces in one; triangles 0.51 to 0.56 of today's (median per chart); five sphere faces that refuse today mesh; time in these faces 25 s to 12 s.
- With the correction at the midpoint of the widest side that is no boundary side, one wide tilted cylinder wall needs more rounds than the shared budget of 6 (it meshes at 40).
- One face folds at δ that does not at δ/2 (a tube near a sphere's pole; one ringed sphere face in the other designer's run). A fold is not a deviation and the correction does not reach it.

**Ev's condition (PR 7486, 2026-11-23): "if it's not an instance of doing extra work to get a less precise result, it's good".** The headline's meaning, in Ev's words, is in the clause: where refining to exactly δ would be more work than refining slightly past it, the lane refines slightly past it. Over the suite the change was measured at less work in total. Whether it is less work on every kind of face is not measured, and a face that takes several correction rounds to reach a coarser mesh than one finer pass would have given is the instance Ev's sentence excludes. So stage 1 measures WORK as well as counts: per face family, the time and the triangulation passes under sizing at δ with the correction, against today's sizing, and it names every family where the correction costs more. For such a family the answer is the page's own: size it slightly finer up front (a first guess the family's correction rate justifies), not iterate.

**Ev on what the condition measures (PR 7486, 2026-11-23, after stage 1's table):** "this still looks good to merge--the 'do extra work' is more centrally about the work of writing the code rather than of running it". Stage 1 had read "work" as running time and found the change less work on every chart and family, with 53 of 3,238 faces (1.6%) where a correction round costs more time than the old finer mesh did. Under Ev's reading that residue is not what the sentence excludes: the lane is not given extra machinery to land a less precise mesh. The clause carries the sentence now.

Stage 1 measures, then stops and reports: which insertion rule loses no face lane-wide, spline rows and the rectangle lane's triangulation included, and at what budget; folds counted beside certificates; the three ε; the slow set, `sweep`'s meshing modules, `step-import` and the tour; the faces sharing a rim with a chart-lane face. Stage 2 builds it.

## What it closes

`the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`, `the-trimmed-sphere-arm-has-no-worst-triangle-theorem` and `one-containment-lemma-for-an-oblique-boundary` close as not owed: each asks for a bound on what a constrained Delaunay triangulation does beside a trim chord. `the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound` stops waiting on that lemma and becomes a measurement. The fold near a sphere's pole is filed by stage 1 with its cells.

## Unit

**Letter H. Tier: dual review.** It changes the target of every chart-lane face and of every rim such a face shares, so every pinned count and digest on those faces moves, and it rests on a correction whose termination has no proof.

It builds on unit 4b (the cylinder arm's rows) and unit 4c (the walk, which reads no headroom).

## Stage 1, measured (2026-11-23), and the rulings on it

Measured behind switches that did not ship, over the `mesh` suite with its slow set, `sweep`'s 131 modules that name the mesher and `step-import`, at ε `10⁻⁹`, `10⁻⁶` and `10⁻¹²`, with every chart-lane analytic face sized at δ (at `10⁻¹²`, `sweep`'s rule runs took the 50 tests that hold a chart-lane analytic face). Seven statements above it corrected; each correction stands.

**The rule.** Faces lost on the certificate and the most rounds any face took (`mesh` / `sweep` / `step-import`), the same at all three ε:

| point a triangle read over δ adds | faces lost | most rounds |
|---|---|---|
| its centroid | 0 | 3 / 7 / 0 |
| the middle of its longest side that is no constraint | 0 | 3 / 7 / 0 |
| the middle of its side, no constraint, whose own chord stands farthest from the surface | 0 | 3 / 2 / 0 |

The centroid's 7 is one sphere face (`sweep` `snowman::two_cut_ins_on_one_face_build_in_either_order`), the longest side's 7 a bored cylinder wall (`sweep` `bisector_nodes`). With no correction, 10 meshings are lost at δ. No spline face in any suite takes a round. Of the rectangle lane's 24,767 faces one body reads over δ, `loops_the_meridian_guard_admits`' short spur, and no rule cures it within a small budget.

**Work** (release, one thread, nothing else running, two repeats; `mesh` and `sweep`, ε `10⁻⁹`; time against the old sizing at δ/2):

| chart | faces | time at δ | time at 0.9 δ | triangles at δ |
|---|---|---|---|---|
| cone | 452 | 0.52 | 0.56 | 0.50 |
| cylinder | 1,819 | 0.74 | 0.75 | 0.71 |
| sphere | 858 | 0.56 | 0.60 | 0.53 |
| torus | 109 | 0.69 | 0.53 | 0.51 |

80 faces (2.4%) take a round; 53 of them end coarser and cost more time than the old sizing (median 1.19 times, worst 2.6). A 0.9 δ first guess does not remove the corrected faces: 65 take a round, mostly other faces.

**Folds.** Five meshings that mesh at δ/2 fold at δ, every one a tilted circle near a sphere's pole; the correction does not reach a fold (`a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm`).

**Rulings** (the orchestrator, 2026-11-23):

1. The third rule, on every row that triangulates, spline rows included, under its own budget `MAX_CORRECTION_ROUNDS` = 6, apart from `MAX_GRID_RETRIES`.
2. Every chart sizes at δ. No family is sized finer. The torus at 0.9 δ is six faces of two bodies, filed as a measurement owed (`the-torus-arm-at-a-finer-first-guess-is-measured-on-two-bodies`).
3. The 53 faces are within the owner's condition (the first guess is less work on every chart, and no round is run to get a coarser mesh); the question is put to the owner on PR 7486 and does not hold the build. The cheap round is its own unit (`a-correction-round-triangulates-and-certifies-the-whole-face-again`).
4. The rectangle lane takes the same correction, with a row that forces an over-δ first guess on it. The spur cap keeps refusing, and its pin stays.
5. The folds block this unit's landing, not its build: the fold's cure comes from `tesslane/join-split`, and the PR opens only when all five mesh.
6. `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`, `the-trimmed-sphere-arm-has-no-worst-triangle-theorem` and `one-containment-lemma-for-an-oblique-boundary` close with this unit as not owed; `the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound` opens as a measurement.
