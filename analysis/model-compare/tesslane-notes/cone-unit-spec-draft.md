---
id: the-cone-arm-certifies-the-triangle-its-development-makes
kind: unit
title: the cone arm certifies a triangle in the geometry it triangulates in, and a face ending near its apex meshes at every δ
status: dispatched
opened: 2026-11-14
priority: P1
cost: H
branch: tesslane/cone-near-apex
---


Cut from the review of PR 6950 (unit 2 of the chart lane), where both reviewers demonstrated it. Closes `a-cone-face-ending-near-its-apex-refuses-at-fine-delta`, which holds the measurement.

## What it is

The trimmed lane triangulates a cone face in the cone's development (`trimmed::developed`), where three points of one ruling cannot be a triangle. Its lattice, its per-triangle certificate (`cert::cert_cone`), its fold census and its ear test read the chart. The certificate charges a triangle `cos α·sin α·v_max·(1 − cos(Δu/2))`: its whole azimuth span at its widest slant. Near a small end the development puts points of very different azimuth close together, the Delaunay triangulation joins them, and the certificate charges a span the surface does not make the triangle pay. A face ending near its apex then refuses `CertificateExceeded` at fine δ and meshes at a coarse one. With the certificate zeroed most of those faces are within δ in truth, and some are over (to 1.63δ): the lattice is uniform in `v` and has no row between the small end and its first row.

So two things are owed, and they are different: a certificate that is true of the triangle the arm makes, and points where a face is truly over.

## Unit (2026-11-14, TESSLANE)

**Letter H. Tier: dual review.** A certificate that under-reports passes a mesh that is over δ, silently; believing a new bound takes a second reading and a sweep that tries to break it.

Branch `tesslane/cone-near-apex`.

**Owed:**

- **Measure before building.** Re-run the filed row's table (`reach_probes::a_cone_face_ending_near_its_apex_refuses_at_fine_delta` pins it). Print the refused triangles in both geometries and verify or correct the mechanism above, which the filed row marks as likely. For every refusing case say whether the mesh is truly over δ (sampled), and by how much. Where this section is wrong the measurement wins and the PR says so.
- **A certificate that is a lemma about the emitted triangle.** A bound on the true distance between a flat triangle with its corners on a cone and the cone, sound for every triangle the development's triangulation can emit: wide in azimuth at a small slant, touching the apex, on either nappe. Written as a derivation beside the function, with what it assumes. It is at least the sampled true deviation on a sweep built to break it (the shapes of the filed table, needles, flat cones, triangles through the apex, a whole-turn face), and the row PR 6950 added that holds the cone certificate to the sampled deviation still holds. A tightness figure names the poses it was measured on.
- **Points where a face is truly over.** Decide by measurement between certificate-driven refinement on this arm (the spline arm's mechanism, read, not copied) and a lattice graded toward the small end, and say why. `crates/mesh/README.md` `[mesh.sizing]` is the ratified rule: a lane has a theorem naming its worst triangle, or it carries the named `UNPROVED_HEADROOM` as a debt. Say which the cone arm has when you are done. If the cure needs that page's rule to change, or a new constant that is not derived, stop and report: that is Ev's.
- **A finer δ never refuses where a coarser one meshes**, on this family. Done when every row of the filed table meshes at every δ it lists, `h = 1e-6` included, and passes `reach_probes::geometry`; the pinning row is turned into that; cuts at `h = 0.02 … 0.3` and every cone row of `reach_probes` still pass.
- **One geometry per decision.** For each of the lattice, the certificate, the fold census and the ear test, say which geometry it reads on the cone arm after this unit and why that is right there. Do not move one that does not need moving.
- **What may move and what may not.** The D9 goldens, `docs/tess-budget-data` and every face on a cylinder, sphere, torus or spline chart do not move by a byte, and neither does a cone face of the rectangle lane. A trimmed cone face (new with PR 6950; the tour's pins of them included) may move: each re-pin says why the new mesh is right, never that it is close to the old one.
- **Not this unit:** the thin triangles beside a ruling (`a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`): say whether your cure changes its numbers, and re-measure the row's table if it does. The lattice point within rounding of a chord point (`a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area`, filed by unit 3).
- **Another lane is in `trimmed.rs` now**: unit 3 (`tesslane/domain-loops`) is changing how the lane builds its polygon and routes faces. Keep to the cone arm's certificate, its candidates and its retry loop; whichever merges second merges main.
- **Run what CI runs before every push that is not marked work-in-progress:** `bash scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; fmt; clippy for mesh, geom-brep and `demos/tour`; `cargo nextest run -p mesh` whole; the rows you add or change at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well; the tour's mesh and pin rows in release; sweep's rows over bodies with a trimmed cone face. A test costing 0.5 s or more goes in the slow set with its measured time.
