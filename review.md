# Review of PR #4122, frozen head 2bbba71676

Lane `reach-full4122`, full single review. Wall clock 08:20–09:16 UTC, 2026-10-06. Glimpse: none. I read the PR body via `get` only, and no comments or reviews.
**Verdict: APPROVE-WITH-FIXES.** Counts: MAJOR 0 · MINOR 2 · NOTE 6.

## Claims, falsified by execution
1. **Soundness of `apart`: holds** (sure). I ran the probes in `probes/review4122_probes.rs`, spliced into `sweep::all`, with signed gaps checked against my own closed forms.
   (a) Point and line touches on a cone, a 270° cone, a torus (equator and 45°) and a ball (corner and edge). Each fixture ran at 7 rigid poses × s ∈ {1e-3, 1, 1e3} × δ ∈ {+2e-2s … 0 … −1e-4s, −1e-6, −1e-9, −1e-12}, under 6 op/order combinations, at ε 1e-9, 1e-6 and 1e-12. That is ~22k ops, and **no touching or crossing pair was cleared**.
   (b) Plates set at the *exact* support of the frustum and torus solids (full and 270°) along 16 random oblique directions, at 3 scales. δ ∈ {1e-3s, 1e-6s, 0, −1e-6s, −1e-9, −1e-12}; 3,456 ops in all. The PR builds 438 frustum plate ops at δ > 0 (main builds none) and none at δ ≤ 0.
   (c) Every built result was sampled with `point_in_solid` against analytic membership: annulus (cylinder, armed), frustum and torus, full and 270°, δ down to −2e-2·s. That is 3,600 runs and ~209k points, **with 0 misclassified**.
2. **Doors: hold** (sure, by inspection). The gate (`reduce.rs:348`), the sweep's curved arm (`reduce.rs:1149`), `walk_pairs` (`ops.rs:1270`), `face_boundary_meets` (`ops.rs:1229`) and the Approx arm (`ops.rs:3353`) all call `apart` only after the box test passes or the overlap fails. The gate and the sweep read the same face pair with the same axis set and pad, so the gate's verdict and the sweep's agree.
3. **`circle_box`: holds** (sure; `probes/review4122_circle_box_probe.rs`). 407 normals (exact axes, 1e-8 and 1e-12 tilts, random), ρ from 1e-3 to 1e3, centres out to 1e3. No sample falls outside, and the worst looseness is 1.1e-13·ρ.
4. **Rows: hold as claimed** (sure). All 22 tests matching the rows the PR touches pass at ε 1e-9, 1e-6 and 1e-12. With main's `topo/src` swapped in, the cone, ball and Approx rows go red and the twin stays green. Mutant **M1** (world axes only) turns the same three red and leaves the twin green. **M2** (`apart` always true) turns the twin red; a later door still refuses with `CurvedBooleanUnsupported`.
5. **Moved rows: acceptable** (likely). See S5 and S6.
6. **K telemetry: correct dimensions** (sure, by inspection). `bool_pair_axis` decides a length through `UnitVec3::new`; `bool_pair_normal` decides through `levered` against the reach diagonal; `bool_pair_gap` decides a length. I did not rerun the k-probe sweep. The PR's `rim_dim` catch fits that reading.
7. **Main red: confirmed** (sure). `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates` fails at ε 1e-6 on both head and base `topo/src` (62bdd557). The payload is identical: `Coincidence(Join, Moot)`, margin −5.196e-6.
   **CI on the frozen head:** the `test` job is red (run 37432923247). Its default-ε step passed, as did the ε 1e-12 pass of the ε step (11930 tests). The ε 1e-6 pass is truncated out of the log, so I reran it locally (`-p topo -p sweep`, ci profile): 4481 of 4482 pass, and the one failure is this row.

## Findings
- **MINOR-1: the PR's rows cannot catch a reach that under-covers** (`crates/sweep/tests/operand_gate_pose.rs`, the whole file). DEMONSTRATED BY EXECUTION.
  - Mutant **M3** shrinks every reach in `apart_along` (`separating.rs:142`) by 0.1% of its width. All four rows stay green: every clear pair has a gap of 0.13·s or more, and the twin crosses deep.
  - My plate probe (b) goes red under M3 with *new* crossing pairs: 270° torus dir3 at δ = −1e-9 and −1e-12 ships an `Assembly` and an empty ∩.
  - A row at the exact support, δ ∈ {0, −ε}, is what makes the soundness claim red-able. Probe (b) is that row.
- **MINOR-2: `operand_axes` is recomputed inside hot loops** (`ops.rs:3174` per section circle, `ops.rs:3357` per Approx face). It is also computed eagerly in every `sweep_direction` (`reduce.rs:1084`), all-planar operands included. Each call reaches every face of both bodies. A box gives six normals, which are ± pairs, so half the candidates repeat. By inspection; not timed.
- **NOTE-1: incompleteness, uniform across poses** (by execution).
  - A bar edge 2%·s off the cone wall (gap 0.019·s) refuses at every pose. So does the 270° cone at the cut.
  - Frustum 270° plates refuse 30 of 120 runs at δ = 1e-3·s.
  - The verdict is the pair's, as claimed, but the narrow phase parts only pairs that a planar normal or the anchor axis separates. This is within the filed seam-anchor residue.
- **NOTE-2: pre-existing, not this PR's** (by execution). A plate touching the 270° torus at its cut-cap rim (δ = 0) builds an `Assembly`, and ∩ comes back `Empty`. Main gives exactly the same result. I did not check whether the contact is recorded.
- **NOTE-3: the doc overclaims** (`reduce.rs:271`). "One the gate refuses the sweep would examine" does not follow. The gate reads face × face; the sweep reads edge × face and skips an edge whose own reach is apart, even when the parent pair is not.
- **NOTE-4: the Approx arm now asks about the sphere *face* where the question is the *ball*** (`ops.rs:3353–3357` against the ball-escape comment above it). The two are equivalent only because a sphere face's reach is `WholeBall` (`boxes.rs:1431`). Unsure whether a tighter sphere reach would turn this unsound.
- **NOTE-5: the PR's own test oracle cancels near an axis** (`boxes.rs:3223`: `rho*(1 - ni*ni).sqrt()`). It is correct at the row's (1,2,3) normal and wrong for n ≈ ẑ. I hit exactly this in my own probe.
- **NOTE-6: CI is red on the frozen head.** The ε 1e-6 pass fails only on the main-red row above. Merging means accepting that known red.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 partial)
- S1 (Q1, likely): one circle is spelled twice at the call site, as `Item::Circle` and as `circle_box(foot, u_ref, normal.cross(u_ref), rho, pad)` (`ops.rs:3162–3170`), with the same four values written out twice. The prose sweep (verbatim, mirror, by hand, must match) over the diff came back clean. It is blind to undisclosed copies.
- S2 (Q1, unsure): `separating::anchor` walks a face boundary much as `census::boundary_reach` does, and it is a second boundary walk.
- S3 (Q2, likely): the curved-arm comment (`reduce.rs:1143–1148`) and the gate doc carry the consistency argument in prose. No row asserts gate/sweep agreement.
- S4 (Q3, sure): see MINOR-1. The twin's premise (deep penetration) rules out the failure mode that matters, a graze.
- S5 (Q3/Q6, likely): `n3r1_prune.rs:136` is now an empty exemption list behind live filter code. The defect it exempted is unwitnessed, and its hone item is `status: open`, with no program or schedule.
- S6 (Q4, likely): `germ_interior_oval.rs:861` keeps the name "certified apart" while it now guards that the certificate is *not* asked. W0 keeps its own row (`section_cert_rows.rs:393`, which I confirmed exists).
- S7 (Q5, sure): the `separating.rs` module doc matches the code, including the `None` and in-band conservative paths.
- S8 (Q7, unsure): I would have deduplicated the normals (±) and hoisted the axis set once per operation.
- Q8: I read `separating.rs` end to end. I did **not** read `ops.rs` (5414 lines) end to end, only the touched regions and their callers.

## Probes
- `probes/review4122_probes.rs` is a `sweep` integration suite. To run it, splice it into `crates/sweep/tests/all.rs` with a `#[path]` line; `PROBE_ONLY=<name>` filters fixtures.
- `probes/review4122_circle_box_probe.rs` is a test body for `boolean::boxes::tests`.
- The plate test fails on the pre-existing torus case (NOTE-2) unless that case is excluded.
