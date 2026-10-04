You are a VERIFIER for the REACH orchestrator in evgunter/cad, working on the PR named below. Read CLAUDE.md and docs/prompts/implementer-discipline.md first.

The implementation lane has finished its LAST fix pass after review. Your job is to check, independently, that what the lane claims is true. You change no code on the PR branch, you merge nothing and you post no GitHub comments.

Steps:
1. `git fetch origin <branch>`, then check out the frozen head given below. If the branch head has moved past it, say so and verify the new head as well.
2. For each mutant listed, apply it by hand, run the named rows, and record red or green. Then revert it.
   - A mutant the lane says is killed but which stays green is a FINDING.
   - Write each mutant as the smallest source edit that matches the description.
3. Run the PR's new or changed test rows at CAD_EPS (or the repo's ε switch, see `scripts/` and the nightly workflow) 1e-9, 1e-6 and 1e-12.
   - Run the changed crates' nextest suites at 1e-9.
   - For each red, check whether it is also red on `origin/main`. If it is, it is not this PR's.
4. Read the diff of the last fix pass (the commits after the review's frozen head) adversarially against the review findings listed. Check each claim below. Report any claim that is false, overstated or unsupported.
5. Push a short evidence file, `verify.md`, to branch `analysis/reach-verify/<PR>`. Make it an orphan or off-main branch, with only that file.
   - Contents: a table of mutant → row → red/green; the ε results; the claim checks; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - Commits end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
6. End your turn with the same verdict and table as plain text.

Disk is limited. Build only the crates you need, and run `cargo clean -p` between ε runs only if space runs short.

PR: evgunter/cad#3977
Branch: reach/check7-interval
Frozen head to verify: 94d9f8870d30038538812ae9237c3434276ecc0d
Review to read for the findings: delta review 2 on analysis/reach-delta2/3977 (frozen 831dcd7dc). Mount its probes and oracles from probes/delta2-3977/. Also read the last fix brief: `git show FETCH_HEAD:briefs/fix-3977-last.md` on analysis/reach-briefs/2026-10-03.

This PR changes check 7, the tier-3 volume-sign gate. The bar: no inside-out body that main refused may pass, and no valid in-domain body that main accepts may be refused.

Lane claims (last fix pass):
- **R1, quadrature recentring.** quad_lane::cut_face_rounds takes a centre.
  - The cylinder lane uses (origin − c)·A.
  - NURBS and trimmed lanes carry the lifted net by −c.
  - Approx takes the NURBS path.
  - certify_role reads in two stages: the untight interval first, then a re-run about c only where the sign stays unresolved.
  - d2_quadrature_kind: 48/48 upright pass and all 48 inside-out twins refuse NegativeVolume at 1e-9; 36/36 and 36/36 at 1e-12.
  - Re-run that, plus delta 2's disc, revolved and planar-fan families, at 1–20 km.
- **R1, digest line.** One digest line moved (sym_thin_strip: 692 → 700 decisions). The tcost one-read-per-face rows are green.
- **R2.** No shipped kind reaches Unresolved now, so a unit test pins the arm (validate::tests::an_unresolved_sign_refuses_where_an_in_band_one_passes). The `exempt` mutant turns it red.
- **R3.**
  - door_backstop_settled_residue asserts only what the geometry decides.
  - `fansecond` leaves it green.
  - `offgap` (a body built outside the gap) turns it red.
- **R4.** The tier3 half-disc has a far-anchored arm. The faithful `oldenc` (world origin, straddle exempt) turns it red at 1e-9 because the inside-out half-disc passes check 7.
- **Mutants.** Run each yourself and report red or green:
  - faithful `oldenc`
  - `exempt`
  - `fansecond`
  - `fanorigin`
  - `inflane`
  - `noquadre`
  - `offgap`
- **Battery.** topo+sweep+editor-core+pncad-py: 7144/7144 at all three ε.
- **Cost.** Measure validate_geometric against main on a brick, a bored block, a far disc and a ball, release mode, as delta 2 did.
- **Territory.** It touches 24 paths, including TCOST/TINT's mass_props_are_thread_count_invariant digests. Confirm that each moved digest is explained and that behaviour outside check 7 is unchanged.
- **Merge state.** Main has moved about 35 commits past the head's merge. Report whether merging origin/main conflicts. Don't resolve it.
