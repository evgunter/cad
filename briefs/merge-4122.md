You are an IMPLEMENTER for the REACH program in `evgunter/cad`. The REACH orchestrator dispatched you.

**PR:** evgunter/cad#4122, branch `reach/operand-gate-separating-direction` (head `28bb10d201`). An independent verifier has VERIFIED the last fix pass (`analysis/reach-verify/4122`, `verify.md`; read it). Two things stand before merge:
- the branch conflicts with `origin/main` in `crates/topo/src/boolean/ops.rs`;
- no hosted CI has run on the head.

You have explicit permission to commit and push to that branch.

**Do:**
1. Merge `origin/main` into the branch. Resolve the `ops.rs` conflict keeping both sides' behaviour: this PR's operand-axes plumbing and Approx-arm ball question, and whatever main changed.
   - Main recently moved face-boundary walks to `face_boundary_linked` (`f2c43f5fbb`, `3d3cb33e84`). Use that API for any walk you touch.
   - If main's change and this PR's change are the same logic and picking either loses behaviour, stop and say so.
2. Fix the verifier's test-hygiene note: `the_approx_arm_asks_whether_the_ball_reaches_the_face` (`ops.rs` around :5787) shares one `OperandAxes` cell across both bricks. Give each its own cell.
3. Re-run the verifier's mutant table rows that the merge could move, at least M3 and M4. Both must still go red.

**Lane conduct.** This brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Before pushing**, with `CARGO_INCREMENTAL=0` (check `df -h` first):
- `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p geom-core -p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).

For each red, check whether it is red on `origin/main` too; report main reds and do not fix them.

**Then:**
- push, and wait in the foreground until hosted CI on your head finishes. Poll `https://api.github.com/repos/evgunter/cad/commits/<sha>/check-runs`, and do not arm background waiters. Fix anything red that is this PR's;
- add a line to the PR body's "Last fix pass" section;
- end your turn by stating the head sha, the CI result and the test counts.

Merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
