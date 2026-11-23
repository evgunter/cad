# Review of PR 6965 at 6916292d76 (merge base e9fdcd5242)
    
    **Verdict: NOT-MERGEABLE-AS-IS**, on one finding (M1) and a red gate. The certificate and the rows themselves are APPROVE-WITH-FIXES grade.
    
    ## Claims
    1. **Sound: held.** All four derivation steps check. My own sweep: 150 000 triangles, α from 1e-6 to π/2−1e-6, scales 1e-9…1e6, every pose in the brief; oracle is the 3-D distance on a 40-step grid plus pattern search, blind to the formula. No under-report past 1e-9 relative except F2's pose.
    2. **Computed soundly: falsified in the letter** (F2, F3). The chord term loses nothing I could find; the axis term does.
    3. **Exact, never above the old bound: held.** Where the old bound is finite, new − old ≤ 3.9e-16·k·scale. 99.3% of draws have dev/cert &gt; 1−1e-6; the loosest (0.975) is F2's over-report. Rectangle-lane cone faces: 3 239 distinct patches bit-identical to the merge base.
    4. **Rows: held, with notes.** The rule is derived (second-order chord rule), not fitted. On a cone the chord pass reads `(h_u, +∞)` (`FaceLattice`, `AnalyticRow::steps`), so the `[mesh.sizing]` sentence is still what the code does. ≤2× by inspection (measured 1.9× on the needle face through its apex). `UNPROVED_HEADROOM` kept. Frustum off the apex, tiny small rim, negative nappe, near-flat, tilt 0.75: 209 of 214 (body, δ) cells mesh within 0.534δ by my oracle (merge base: 157). The 5 are N1.
    5. **Nothing else moves: held.** A per-patch digest hook in `tessellate`, the whole `mesh::all` suite on both trees: Cylinder (5 920 curved, 97 trimmed), Sphere (2 662, 9), Torus (2 752, 25), Spindle, Nurbs (900) identical sets. Every trimmed cone patch moved (251 → 287).
    6. **Done-when: held at the default ε.** The filed table (5 h × both halves × 5 δ) all mesh; `geometry` and my oracle pass, ≤0.496δ on the two formerly-over cases. Finer-never-refuses: no violation on head across my different δ grid (merge base: 30+). I did not run ε 1e-6 or 1e-12. The 432-run sweep is unpinned (style S7).
    7. **What moved is argued right: falsified in part** (M1). The needle table reproduces to the digit on both trees.
    8. **Mutants** (debug; lib whole plus 23 cone/apex/sizing rows of `all`):
    
    | mutant | red |
    |---|---|
    | chord halved | the five rows the PR lists |
    | axis term dropped | break-it, attained |
    | `cos` for `|cos|` | attained only |
    | rows uniform in `v` | rows row, near-apex mesh row |
    | cert × 0.9 (mine) | break-it, attained, `the_cone_and_torus_certificates…`, `cone_grid_step_lands…`; no mesh row |
    | axis only when strictly inside (mine) | nothing (equivalent: A = D on the boundary) |
    | floor removed (mine) | rows row only |
    
    No mesh-level row holds the axis term or the floor. Not run: floor ×4, no-nappe-guard.
    9. **Refinement: corrected.** The PR's own figures fall 191 → 165 → 106 → 84 → 26 → 17 → 2 and stop at the round budget. That shows one centroid per round under `MAX_GRID_RETRIES` did not finish, not that refinement fails to converge. Not re-run.
    10. **Thin triangles: held.** Readers are the fold census (sign of `normal·outward`; a cross product at 2.5e-10 carries ~1e-6 relative error) and `check_mesh` (index equality only). The ratio falls ~31× per decade of δ, so it passes `MIN_SHAPE` near δ ≈ 3e-8 (extrapolated).
    11. **Reproduced, and this unit widens it** (M1).
    12. **Table true of the code** (emit pass read): cert reads `uv`, never `tri`; fold census reads `tri`; ear test reads polygon places.
    13. **Sweep: held by inspection**, one correction (S6); sphere mirror case not re-run. Deviations are all reported and none owed a stop; the row count at the lattice's target is `face_target`'s existing reading.
    
    ## Findings
    - **M1 MAJOR, DEMONSTRATED. `tessellate` answers `Ok` with a non-manifold mesh on 8 of 12 cone-tip cells; the merge base does on 1.** T1's cone ∩ turned wedge, edge at `(0.5s, h−1.5s, 0)`, δ {2e-2, 5e-3, 1e-3}:
      - merge base: only s=0.03, δ=5e-3;
      - head: s=0.1 @2e-2; s=0.03 @2e-2, 5e-3; s=0.01 @2e-2, 5e-3; s=1e-3 @ all three. Each is `NonManifoldEdge { count: 4 }`; the seven new cells meshed valid on main.
      - Mechanism (mesh dumped, s=0.01, δ=2e-2): the cone face and the planar face share one chorded conic arc. Both triangulations take the same chord 6–11 between non-adjacent arc points as an interior edge: plane `[12,11,6]`, `[11,10,6]`; cone `[11,15,6]`, `[6,14,11]`.
      - Not the development's vertex rule, the apex fan (the face is off the apex), the chord pass or the boolean. Nothing forbids a face-interior edge joining two chord points of one body edge.
      - This unit does not cause or cure it. Moving one lattice row (slant 2.425e-2 → 2.337e-2) flips the cone CDT onto that chord.
      - `B∖A` on the same cells is valid. The "unturned" wedge as I built it refuses in the boolean (`GermSectionOpenConic`), so that half of the report is not reproduced.
      - No tracker row found. The PR's "each is certified … and passes" does not cover these.
    - **F2 MINOR, DEMONSTRATED. `cert::cert_cone` axis term under-reports by up to 5.1e-9 relative, and over-reports by up to 14%.**
      - Under: a triangle about the axis with two corners ~1e-8 rad apart at one radius. α 2.4124187479124574e-4, uv `[(-1.990817571118756, 188258.19995305588), (-1.9908175783592712, 188258.19995305588), (-5.132410229531247, 376139.88350620563)]`: f64 60.53414746948693, 300-bit value 60.5341475385, my geometric oracle 60.5341475378. That is ~1e5 rounding allowances.
      - Over: an ulp-thin sliver, f64 0.03397 against exact 0.02968.
      - Cause: `sin(u_k − u_j)` near π carries an absolute ulp(u) on a value of size x.
      - Reach: such a triangle is within δ only within ~δ of the apex, so ≤ ~5e-9·δ. But "own arithmetic is relative to its bound" (`cone_scale`), "sums of one sign" and "a sign rounded either way … moves the maximum by rounding" (step 4) are false there.
    - **F3 MINOR, DEMONSTRATED. Non-finite inputs certify.** `cert_cone(0.5, …)`: one `u = ±∞` → 5.26e-4 (old: +∞); one `v = +∞` → 5.26e-4; all `v = ∞` → 0.0; `half_angle` NaN or ∞ → 0.0 (old: NaN); `v = 1e200` → 0.0. `f64::max` drops NaN and `cone_chord_sag`'s `span &gt; 0.0 … else 0.0` maps NaN to 0. The doc says "NaN-sticky"; only `uv` NaN is. `verdict` then reads Certified.
    - **N1 NOTE, DEMONSTRATED.** A needle (`revolved[(0,0),(0.02,1),(0,1)]`) cut 1e-3 or 1e-5 above its apex, upper half: `FoldedTriangle` at δ 1e-3, 2e-4 (and 3e-5 at h=1e-5). Same on the merge base; unfiled. "A cone face ending near its apex meshes at every δ" is the filed family's property only.
    - **N2 NOTE. The gate is red at this head**: `test (kernel)` and `gate ok` failed, four rows: `geom r2_turn_probes::turn_defect_speeds…` (fuzz seed), `sweep band_strip_mouth_cups…` and `tilted_sphere_pair…` at ε 1e-6, `sweep full_turn_wall…` at ε 1e-12. None is in mesh, and each message is from a boolean, pcurve or props step. I did not establish whether main has them.
    - **N3 NOTE, BY-INSPECTION.** `trimmed.rs`: `nv = rows.len() + 1` after the floor filter is no longer a row count. It feeds `a.metric(..)`, which the cone discards, and a debug message.
    
    ## Spec deviations
    Reported by the PR: refinement dropped; h=1e-6 measured; row count at the lattice's target; fold census reads 3-D. SILENT: none.
    
    ## Style
    Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 on `cert.rs`'s cone section and tests and `chart_table.rs`'s rows and tests (not either file end to end).
    - S1 (Q1, likely) `chart_table.rs` `lattice_rows` / `lattice_steps`: two homes for a cone face's rows. The curved lane's split-side cone still lays rows uniform in `v` through `lattice_steps` (`curved::grid_counts`); `lattice_rows` takes `(hu, hv)` though `hv` is `ρ_max·hu`, and re-derives `widest`.
    - S2 (Q1/Q3, sure) `0.5 * widest * (1.0 - (0.5 * hu).cos())` is written in `lattice_rows` and again in its test; my floor mutant's pattern matched twice. The rows row re-derives the implementation's own arithmetic, so it reds on drift, not on a wrong rule.
    - S3 (Q2, sure) `cert_cone`'s doc is ~60 lines for ~35 of code and holds F2's false numerical sentences.
    - S4 (Q2/Q4, likely) `cone_scale` (cert.rs:455-466): the formula is unchanged and its doc rewritten around it; it still cites "a span Δu ≤ π", a precondition the certificate no longer has.
    - S5 (Q3, likely) The break-it sweep's oracle is independent, but a 24-step grid and 1e-9 tolerance cannot see an under-report below ~0.1% off the attained row's poses, and one pose holds the axis term's tightness.
    - S6 (Q5, likely) PR sweep: "Curved lane … its triangles are its strips', which its theorem bounds" is not true of the split-side cone arm, whose own comment says the certificate, not a theorem, holds it. Same shape, unswept.
    - S7 (Q6, sure) Two claims rest on one-off measurements with no guard, schedule or reason at the claim site: the 432-run monotonicity sweep and the refinement trial.
    - S8 (Q5, sure) Title and row name say "meshes at every δ" (N1).
    - S9 (Q2, unsure) `lattice_rows`' doc writes `1 + cos(Δu/2)` where the certificate has `|cos|`.
    - S10 (Q7, unsure) `root` is signed for a window that cannot span both nappes; the axis block is gated on `pole == [false; 3]` where a flagged apex already gives zero weights.
    - S11 (Q7, unsure) The floor compares against std `.cos()` while the module's replay note leans on `Real`'s libm; sizing already uses std trig, so likely no new class.
    - S12 (sure, trivial) Over-long doc lines at chart_table.rs:13-15 and 349.
    
    ## Rubric
    - Idiom/structure 4: small change, one new function per concern; S1 and N3 are the rough edges.
    - Test quality 3: the certificate rows are real and went red on every certificate mutant, but the rows row restates its subject, nothing at mesh level holds the axis term or floor, and no row covers F2, F3 or M1.
    - Doc/comment honesty 3: the derivation is right and checkable; the numerical claims, `cone_scale`'s rationale and the sweep sentence on the curved lane are not.
    
    ## Executed / read only
    - Executed: the certificate sweep and non-finite probe (release); a 300-bit comparison of the formula (310 000 and 120 000 directed poses); mesh probes on head and merge base (filed table, shapes, T1 family, needle to δ 1e-7, mesh dump); the digest differential; seven mutants. Probe sources are in my scratchpad; nothing committed, the tree is back at the frozen head.
    - Read only: the PR body and check runs, the spec, `[mesh.sizing]`, `chords.rs` `tighten`/`FaceLattice`, the trimmed emit pass.
    - Rows worth keeping: F3's nine inputs, each asserting NaN or +∞; F2's pose against 60.5341475385; M1's 12 cells asserting `check_mesh`.
    - I saw no other review's material.
</cross-session-message>

0 notifications remain queued.
