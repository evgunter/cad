You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust. The REACH orchestrator dispatched you and reviews and merges your work. Your PR is your report.

**Unit:** `work/reach/edge-tangent-to-a-curved-carrier-off-the-face-refuses-at-the-pierce.md` (P1, cost M). Read it in full. `reduce::curved_face_arm` decides an edge against the face's CARRIER (`frontier()` on a tangent or unsettled root) before any witness is placed against the FACE. So an edge tangent to the carrier at a point the face does not hold refuses `CurvedPierceUnsupported`, though the bodies are disjoint. The extent passes already apply the right rule: a touch point certified `Out` of the face, with no face edge reaching the ball the decided margin admits about it, is no event.

Since PR 4046, trimmed sphere faces are classified by `topo::boolean::sphere_region` (`SphereFaceRegion::contains`). Use the face door that reads it (`contain::curved_face_placement`) rather than a new reading.

**What holds when you are done:** in the crossing layer, a tangent or unsettled carrier touch that is certified Out of the face, and clear of every face edge by the decided margin, is no event, for every curved carrier kind the face door serves. A touch ON the face, or within the margin of an edge, still refuses typed. That neighbouring case is `an-uncovered-edge-tangent-to-a-fillet-at-the-curved-operands-vertex-refuses`, parked on the D10 hold; do not take it.

**Rows:**
- the item's lens × brick, every op, both orders, at three ε, against closed forms (disjoint bodies);
- the same off-face tangency on a cylinder face and on a torus face if the door serves them;
- a near-miss where the touch is just inside the face (must refuse) and one just outside (must build);
- a mutant that drops the face check, which must turn the rows red.

**The D10 hold (Ev).** `docs/DESIGN.md` D10 (one way to say dependency, placement and intent) is ratified and unbuilt (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Do not extend any mechanism D10 retires: declared contacts or pairs, the placement registry, intent spellings. If your fix seems to need one, stop on that part and say so in the PR body.

**Lane conduct.** The orchestrator may message you mid-task (it reaches you as a cross-session message); treat its instructions as pre-confirmed and act on them without asking it to confirm. Stop only for a design question the brief does not answer.

**Branch:** `reach/pierce-tangent-off-face`, from `origin/main`. You have explicit permission to push to it.

**Read in full before you start:**
- `CLAUDE.md` and `work/README.md`;
- `docs/prompts/implementer-discipline.md`. It binds you: fail loud, fix root causes in their general form, write rows that can fail, no history-style comments;
- `memories/MEMORY.md` and the memories it points to as relevant, at least `memories/refusal-text-is-not-cause.md` and `memories/git-workflow.md`;
- `crates/topo/README.md`;
- `docs/DESIGN.md` §tiers. It is the ratified contract, including the finished-body paragraph Ev ratified on PR 3870. If the unit needs a decision no ratified clause answers, STOP on that part and say so in the PR body under "Design question for the orchestrator".

**Method.**
1. Measure on current `origin/main` first.
2. Find the cause by instrumenting. The refusal text is not the cause.
3. Fix the class, not the fixture.
4. Check every body you newly build or newly refuse against an INDEPENDENT oracle (closed form, slice integral or Monte Carlo, never the kernel), at ε 1e-9, 1e-6 and 1e-12, and through every op (∪, ∩, A∖B, B∖A) where the fixture is a boolean pair.
5. Show each new row red without your fix.
6. Run `python3 scripts/work.py territory --base origin/main` and name the programs whose ground you touch.
7. File every class finding and residue you meet as an item (`python3 scripts/work.py new …`).

**Before every push**, with `CARGO_INCREMENTAL=0` (check `df -h` first):
- `cargo build --workspace --all-targets`;
- `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`;
- `cargo fmt --check`;
- every `scripts/gates/*.sh`;
- `python3 scripts/work.py lint`;
- nextest on the crates you touch, at the three ε;
- `scripts/k_probe_sweep.sh <outdir>`. Its ε 1e-12 demo pass is red on main (SHOW's `lily_leaf_b`), so report that one as main's.

Merge-only: never rebase or force-push, and merge `origin/main` when it moves. Commit and push early and often, because the container is preemptible.

**Deliverable:** a PR against `main` titled `REACH: <what now holds>`. Its body states:
- what you measured before;
- the after table with oracle values;
- the root cause;
- the design of the fix;
- what you swept for the same class;
- rows filed;
- territory.

Set the item's `status: review` and `pr:` on your branch. **Do not merge.** Post no GitHub comments and touch no other branches.

**Attribution:** commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
The PR body ends with `🤖 Generated with [Claude Code](https://claude.com/claude-code)`. No email address other than `evgunter@gmail.com` may appear anywhere you write.
