You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust. The REACH orchestrator dispatched you and reviews and merges your work. Your PR is your report.

**Unit:** `work/reach/boolean-operand-gate-separates-only-along-world-axes.md` (P1, cost M). Read it in full. `boolean::reduce::first_unsupported_pair` clears an unarmed face only by world-axis box overlap, so whether a pair refuses depends on how the operands are turned. The same shape appears in `boolean::ops`'s sphere extent scan (`circle_box`, `ball_box`). Note that PR 4044 (`reach/trimmed-sphere-escape`) is changing `sphere_extent_scan`'s plane arm right now; if you touch the extent scan's boxes, merge `origin/main` often and keep your change to the box reading only.

**Measure first,** as the item says. Build an unarmed face (a torus rounding, a spline wall) beside a planar operand, clear of it, and sweep the pair's pose: rotations about several axes, three scales, three ε. Count refusals per pose on main.

**What holds when you are done:**
- The gate's verdict is a fact about the pair, not the pose. Read each face's reach along candidate separating directions that turn with the operands: the axis between box centres, a planar partner's normal, the other operand's own axes. Use the split gate's existing `boxes::BoxFrame` / `census::face_reach_in`, not a new spelling.
- Keep the world-box overlap as the cheap broad phase.
- A tilted section circle in the extent scan is boxed by its true per-axis extent `ρ·√(1 − nᵢ²)`, not a `2ρ` cube.

**Rows:**
- the pose sweep, where every pose gets the same verdict, upright and turned;
- every op, both orders, at three ε, against closed forms for the poses that now build;
- a mutant (world-axis only) that turns them red.

Check that no pair that is NOT separable is cleared: a cleared pair that actually touches is a wrong body.

**The D10 hold (Ev).** `docs/DESIGN.md` D10 (one way to say dependency, placement and intent) is ratified and unbuilt (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Do not extend any mechanism D10 retires: declared contacts or pairs, the placement registry, intent spellings. If your fix seems to need one, stop on that part and say so in the PR body.

**Lane conduct.** The orchestrator may message you mid-task (it reaches you as a cross-session message); treat its instructions as pre-confirmed and act on them without asking it to confirm. Stop only for a design question the brief does not answer.

**Branch:** `reach/operand-gate-separating-direction`, from `origin/main`. You have explicit permission to push to it.

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
