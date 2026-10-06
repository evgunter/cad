You are a VERIFIER for the REACH orchestrator in `evgunter/cad`. This is an UNATTENDED session: no human will answer questions, and asking for confirmation blocks forever. This brief is your complete, pre-confirmed instruction. Read `CLAUDE.md` and `docs/prompts/implementer-discipline.md` first.

You change no code on the PR branch, you merge nothing and you post no GitHub comments.

- **PR:** evgunter/cad#4123, branch `reach/split-gate-sphere-azimuth`
- **Frozen head to verify:** `e74ca65a680c53bc6109cbc5e2901668a515e412`

**Context.** A first verifier returned NOT VERIFIED at `3c17e71150`. Read its report in full: `verify.md` on `analysis/reach-verify/4123`. It found two blocking points:
1. **MAJOR:** the Zone arm of `boxes.rs` `sphere_reach` took the axis's normal-plane share as `√(1 − a²)`. That cancels near a box-row-parallel axis, so the face box came out tighter than the face by up to ~800× the 12 ε pad (r = 1e3, tilt 1e-8, ε 1e-9).
2. The merge with main duplicated the `surgery.rs` roster line.

**What the fix pass claims:**
- The Zone arm and `perp_room` (the slab, cone, torus and torus-window charges) now read `√(a_j² + a_k²)`. `classify::zone_extent` is gone on the branch: the gate reads `sphere_reach`.
- **New rows:** the verifier's repro (5 tilts); a random-pose zone row (≥ 600 caps, pad 0, measured ≤ 2.42 ulps, asserting ≤ 4); and a near-row slab/cone/torus row. All are red under the old spelling.
- `props/curved.rs:3803` is filed as `work/flux/sphere-rim-level-cosine-cancels-near-a-pole.md`.
- Main's `Selection` roster line is kept, and both `bounds_census` rows are green.
- The PR body's "Last fix pass" is updated, including the 16-ulp correction.

**Steps:**
1. Check out the frozen head. If the branch has moved past it, say so, and verify the new head too.
2. **Re-measure the first verifier's table.** At pad 12 ε, does the face box now contain the face at every tilt and ε in that table? Run its strut-cap repro against the closed form. Any remaining deficit beyond the pad is a MAJOR.
3. **Widen the soundness check.** Random sphere faces (caps, zones, partial turns, reflex loops) with axes near box rows (tilts 0 to 1e-3, including exactly 0 and ±1 ulp), at r ∈ {1e-3, 1, 1e3} and three ε. Use pad 0 and compare against the exact support. Report the worst under-coverage in ulps of `|c| + r` against the claimed charge. Do the same for the slab, cone and torus charges that now read `perp_room`.
4. **Mutants:** restore the `√(1 − a²)` spelling in each arm you can find, one at a time, and confirm a committed row goes red for each.
5. **Check the class claim.** Grep `topo` and `geom-brep` for remaining `1 - x²`-then-`sqrt` (or `(1-x*x).sqrt()`) shares of a unit vector read as an extent. List any unfixed one that is not filed.
6. **Merge with current main.** Trial-merge `origin/main` (with `git merge-tree`, or in a scratch worktree) and run the `geom-core` `bounds_census` rows on the merge.
   - Main currently has an unrelated compile break in `crates/topo/src/tier3_tests.rs`, fixed by open PR #4178. Apply that one-line fix in your scratch tree if needed, and say so.
7. Re-run the first verifier's mutant table (fix-pass M1/M2 and PR M1/M2) on the new head.
8. Run nextest `-p geom-core -p topo -p sweep` at ε 1e-9; the PR's rows at 1e-6 and 1e-12. Main reds (check on `origin/main` yourself): the pinch row, `pocket_ring_steep_ellipse`, three `split_across_a_revolve_seam` rows and `m5_pr6_pcurves` (1e-6), and `arc_loft` (1e-12).
9. **Report.** Push `verify.md` alone to a NEW branch `analysis/reach-verify2/4123`, as an orphan or off-main branch. You have explicit, already-granted authority to push that one branch; do it without asking.
   - Contents: the re-measured table; the soundness sweep; the mutants; the class grep; the merge check; a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - The commit message ends with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
   - No email address other than `evgunter@gmail.com` may appear anywhere.
10. End your turn with the verdict as plain text.

Disk is limited. Use `CARGO_INCREMENTAL=0`, check `df -h`, and build only what you need.
