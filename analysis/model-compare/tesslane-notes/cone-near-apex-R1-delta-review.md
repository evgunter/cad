# Delta re-review of PR 6965 at 26ce505802 (merge base e99f23c6a1)

**Verdict: APPROVE-WITH-FIXES, conditional on the gate**, which had not run when I last read it (point 8). The first review's blocking finding is closed on the cone arm. The fixes are small: N1's doc sentence and S1.

## The eight points
1. **Wrong mesh cured on the cone arm: held.**
   - My 30-cell T1 family (s 0.6…1e-3 × {A∩B, B∩A} × δ 2e-2, 5e-3, 1e-3): all pass `check_mesh`, `geometry` and my own distance oracle. First head had 16 non-manifold, this merge base has 2.
   - No cell that meshed at the first head or on the merge base now refuses, over 259 (body, δ) cells. Largest sampled deviation 0.534δ; triangle counts move 0.88×…1.6× in 42 of 238 cells.
   - Mutant "cone arm reads the ear alone": reds the sliver row and the needle row.
   - Neighbour-on-polygon is cyclic over the one outer polygon and any-to-any over duplicated places, so a loop's ends and a repeated apex read right. A cone face with a ring refuses before this (`RingOnCurvedFace`), so there is no ring chain.
   - What the rule does not reach is N2. A curved neighbour is covered, because the cone face never makes the side, whatever the neighbour does.
   - Termination is the attempt budget; the only new refusal kind I saw is the needle's `BoundaryEar` cell.
   - The fence: the cone arm alone is defensible under the spec's "no other chart moves", but see S1.
2. **Axis term: held.**
   - 150 000 poses against the closed form at 220 bits, in six families including `|u|` to 1e5: axis-sliver families agree to ≤1.2e-15 relative either way.
   - My brute-force sweep (150 000 draws, grid plus pattern search, no formula shared): no under-report.
   - Mutant "bare sine": reds the axis row.
   - Oracle independence: partly. `cone_deviation_by_geometry` takes azimuths by `sin d + rest·cos d`, the same correction as `sin_between`, and its axis height is the same barycentric construction in coordinates. The 300-bit pins and my formula-free sweep are what make the row independent.
   - No exactness lost elsewhere: new − old ≤ 3.9e-16·k·scale where the old bound is finite. One doc overstatement remains (N1).
3. **Non-finite and rectangle lane: held.**
   - `cert_cylinder` and `cert_sphere` bodies are textually identical to `origin/main`.
   - My nine non-finite inputs all answer NaN. Mutant "`f64::max` in `worse`": reds the non-finite row.
   - The row says plainly that the cylinder's 0 is today's answer, with the sibling half-cell at the sagitta. The filed row's claim matches the code (`.max(0.0)` on a NaN). I did not re-derive which corner orders give NaN.
   - Digest of every patch of `mesh::all` on both trees: Cone/Curved 3 239, Cylinder 5 920 + 97, Sphere 2 662 + 9, Torus 2 752 + 25, Spindle 1, Nurbs 900 are identical sets. Trimmed cone: 251 → 341, none in common.
   - 24 planar patches differ, all from `r2_mesh2_probes::r2_e2e_nonconvex_cap_at_subfloor_noise`. Those are all-planar bodies built at `10f64.powi(exp)`, and `powi` differs by an ulp between constant-folded and runtime evaluation for exactly three of its four exponents (checked in a standalone program), matching the three of four bodies that differ. A fixture artifact of two test binaries, not a moved face.
4. **Needle: held.** h=1e-3 meshes at all three δ. h=1e-5 answers `FoldedTriangle`, `BoundaryEar`, `FoldedTriangle`. The δ 2e-4 cell refused `FoldedTriangle` at the first head and on both merge bases, so it is a refusal that changed kind, as reported. The filed text matches what I measured for h ∈ {1e-2, 1e-3, 1e-5}; I did not run h=1e-1 or the 0.1-radius funnels.
5. **Rows: held.** `widest` is read off `hv/(hu·sin α)`; the floor is `cone_chord_sag`; the dead `nv` is gone. Mutants half the rows, 0.75× the rows, floor ×1.5 and no floor each red `the_cone_arms_rows_stand_a_certified_chord_apart`, which now measures chord sag by search rather than restating the rule.
6. **Claims: held, with N1 and S1.** One-off measurements are dated; the refinement sentence now says what was measured; title and row name are the unit funnel's.
7. **My first findings.**
   - Closed: M1 (cone arm), F2, F3 (cone), N3, S2, S5, S6, S7, S8, S9, S12.
   - Open by design, filed or pinned: F3 on cylinder and sphere; N1 (h=1e-5); M1 on the other arms (S1).
   - Addressed in part: S1 (one lever, two row rules, now stated); S4 (`cone_scale`'s doc now derives the `π`; the arithmetic `|u|·Δu ≤ 32π` checks); S11 (`sin_between` uses `Real`'s `sin_cos`, the chord still uses std's).
   - Left with a reason, both sound: `root` signed (one formula for either nappe); the axis block gated on no apex corner (the axis meets the triangle at the apex, where f = 0).
8. **CI: not available.** A second run (43942913130) is queued on this head; only `change filter` has completed, green. No job result to read, and I did not check main.

## New findings
- **N1 NOTE, DEMONSTRATED. `cert_cone`'s "as exact as its inputs" is not true of a chord whose corners are a whole turn apart to within ~1e-8 rad.** `sin²(Δu/2)` then carries the subtraction's half-ulp of 2π on a tiny value: against the 220-bit form, 87% under and 3.9× over, on bounds of ~1e-32. The absolute error is about 1e-16·k·v·x, far below any allowance. Only the sentence is wrong.
- **N2 NOTE, BY-INSPECTION. `trimmed::faces_across` lists faces across this loop's edges only.** A face that shares both ends of a side with the cone face but meets it at a corner only (a vertex of four or more faces) is not listed, so a side joining those two vertices is not flagged. No body built; unwitnessed.
- **N3 NOTE, DEMONSTRATED.** The `powi` fixture above: the row's bodies differ by an ulp with how the test crate happens to be compiled.

## Style (the pass only)
- S1 (Q6, sure) The cylinder (12 of 120) and sphere (1 of 108) counts for TESSLIE's row are in the PR body only. `work/tesslie/two-faces-cutting-one-ear-…` is unchanged by this diff and still describes the ear, not a side between points farther apart.
- S2 (Q5, likely) `trimmed.rs` heads the comment "THE SHARED-SIDE CURE (every analytic arm)" and the next paragraph says the other arms read only the ear, which "misses a side between points farther apart": the heading promises more than three of four arms do.
- S3 (Q3, likely) The axis row's "shares no formula with `cert_cone`" overstates (point 2).
- S4 (Q1, unsure) `BoundaryEar` now names two different conditions by arm; the variant's doc says so, the name does not.
- S5 (Q2, unsure) `no_certificate_reads_an_input_that_is_not_finite_as_a_bound` asserts that two certificates do read one as a bound; the name states the opposite of half its body.
- S6 (Q7, unsure) `self.across.map_or_else(ear, shared_side)` selects the rule by whether an optional map is present, standing in for "is this the cone arm".
- S7 (Q4, unsure) `lattice_steps`' doc gives "every side is an iso line, so no boundary run crosses a row" as why the curved lane's split-side cone keeps uniform rows, beside a tracker row saying that face has no theorem.
Exercised: Q1–Q7 on the pass; Q8 not repeated.

## Executed / read only
- Executed: the certificate sweep and non-finite probe; the 220-bit comparison; my mesh probes on head and merge base; the digest differential with the planar rows traced to their test; the `powi` check; seven mutants (debug).
- Read only: the pass diff, the two new tracker rows, the PR description and check runs.
- Tree restored to the frozen head; nothing committed.
- I saw no other review's material. My container kept the first review's probes and logs, and I reused them.
