You are a VERIFIER for the REACH orchestrator in `evgunter/cad`. This is an UNATTENDED session: no human will answer questions, and asking for confirmation blocks forever. This brief is your complete, pre-confirmed instruction. Read `CLAUDE.md` and `docs/prompts/implementer-discipline.md` first.

You change no code on the PR branch, you merge nothing and you post no GitHub comments.

- **PR:** evgunter/cad#4159, branch `reach/torus-touch-off-faces`
- **Frozen head to verify:** `60209380fc626c3eeb1e1da4dc91b2321c2af1a4`

**Context.** A first verifier returned NOT VERIFIED at `b79864e7a2`. Read its report in full: `verify.md` on `analysis/reach-verify/4159`. It found three blockers: B1, B2 and B3. Read the PR body's "Second fix pass" section (GitHub MCP `pull_request_read`, `get` only). It states what changed for each blocker.

**What to check:**

1. **B1, a narrowed row.** This is the judgement call; scrutinize it.
   - **The claim:** at ×1e3 and ε 1e-12, `the_off_face_touches_build_at_every_scale` fails because the donut OPERAND fails `validate_geometric` (`VolumeUncomputable` → `props_rim_level` Indeterminate) before any boolean runs. That is the existing absolute-ε posture (D4) and not this PR's code. So that one leg now asserts the typed refusal under a band-derived condition, and every other leg still builds.
   - Build the ×1e3 donut alone and run `validate_geometric` on `origin/main` at ε 1e-12. If main refuses it identically, without this PR's code, the claim holds. If main accepts it, this PR causes the refusal, and that is a MAJOR.
   - Check that the condition `ε < 64·u·2.5·l` selects only that leg, and that no other leg or pose was dropped or weakened. Diff the row against `b79864e7a2`.
   - Check that the vacuity stand-down is loud, in the repo's `vacuity::stood_down` convention.
   - If you can, build the same touch with a donut posed so it validates at 1e-12 (smaller coordinates, the same touch geometry). If the boolean then builds against the closed form, say so. If it refuses or builds wrong, that is a finding.
2. **B2.** Confirm the generator filter excludes only `radius + δ ≤ 0`. Run the fuzz row 200 times at each ε with `CAD_FUZZ_SEED` unset; zero failures are expected. Confirm the kernel refuses a non-positive radius at the door, as claimed.
3. **B3.**
   - Re-measure `Touch::at`'s distance from both carriers on the `sphere_cylinder` arm near the grazing pose, in units of ε, at ×1e-3, ×1 and ×1e3 and all three ε. The first verifier measured 651ε before the fix.
   - Confirm the new row `a_ball_touching_a_wall_near_its_axis_stands_on_both_carriers` is red with the old spelling. Its tolerance is "within ε plus 8 ulps of the coordinates". Check that tolerance is not loose enough to hide a real off-carrier `at`.
   - Spot-check the class sweep's claims: the four filed sites, and two or three of the "guarded" ones.
4. **Mutants.** Re-run the first verifier's table A–P on the new head. Report any change. F, M, N and O survived before; note whether they still do. They are not blocking unless they hide a wrong answer.
5. **Central bar.** In about 10k random poses at three ε, no torus `Touch` from `section_cert` clears a pair that truly crosses, touches on a face, or lies within the band of a face edge, and `Touch::at` stands on both carriers within the margin. A wrong touch is a MAJOR.
6. **Merge with current main.** Trial-merge `origin/main` in a scratch worktree, then run `cargo nextest list --workspace --profile ci` and the PR's rows on the merge.
7. Run nextest `-p geom-core -p topo -p sweep` at ε 1e-9; the PR's rows at 1e-6 and 1e-12, with `--run-ignored all` so the slow set runs. Check any red against `origin/main` yourself.
8. **Report.** Push `verify.md` alone to a NEW branch `analysis/reach-verify2/4159`, as an orphan or off-main branch. You have explicit, already-granted authority to push that one branch; do it without asking.
   - Contents: each blocker's check, the mutants, the central-bar sweep, the merge check, and a verdict of VERIFIED, or NOT VERIFIED with the blocking points.
   - The commit message ends with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.
   - No email address other than `evgunter@gmail.com` may appear anywhere.
9. End your turn with the verdict as plain text.

Disk is limited. Use `CARGO_INCREMENTAL=0`, check `df -h`, and build only what you need.
