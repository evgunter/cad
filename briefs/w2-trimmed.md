You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust. The REACH orchestrator dispatched you and reviews and merges your work. You cannot message it; your PR is your report.

**Unit:** `work/reach/trimmed-sphere-group-escaping-through-a-plane-face-refuses.md` (P1, cost H). Read it in full. The refusal is `boolean::ops::sphere_extent_scan`'s plane arm, "a TRIMMED sphere face group escapes through a plane face" (`FallbackExtentUnsupported`). A closed sphere group is rotated about its own centre so that its seams cross the escape plane. A trimmed group, meaning any boolean result carrying part of a sphere, cannot be rotated that way.

**What holds when you are done:** a section circle lying wholly inside a trimmed sphere face and a plane face, with no edge crossing, reaches the crossing layer as an event and builds. The item names two shapes:
- re-chart the face's own seam;
- cut the circle in directly. This is a pierce ring on both faces. Since PR 3985, a pierce ring's chord takes its arc from the germs, on any tilt.

Choose by measurement, and say why in the PR body. Note that PR 3985 retired `SectionNotPolar`, so the item's second pose (tilted about z) may now reach further. Measure it on current main first.

**Rows:**
- `snowman.rs` `a_tilted_slab_against_the_lens_builds_or_refuses_the_trimmed_escape`. Tighten it to the build, under every op and both orders, against the cap closed form `πh²(3 − h)/3` at `h = 0.015` and the lens's caps and the slab's 36;
- the z-tilted pose;
- one pose with the circle on a trimmed face of a different carve, such as a pole-strut result.

Sweep the other arms of `sphere_extent_scan`, and every closed-group-only re-chart, for the same premise. File what you do not fix.

If the carved-sphere lane (`reach/carved-sphere-classify`, `solid_contain`'s trimmed-sphere reading) lands first, merge it and use it. Do not duplicate it.

**The D10 hold (Ev).** `docs/DESIGN.md` D10 (one way to say dependency, placement and intent) is ratified and unbuilt (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Do not extend any mechanism D10 retires: declared contacts or pairs, the placement registry, or intent spellings. If your fix seems to need one, stop on that part and say so in the PR body.

**Branch:** `reach/trimmed-sphere-escape`, from `origin/main`. You have explicit permission to push to it.

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
- nextest on the crates you touch, at the three ε (`CAD_TOLERANCE_EPS`). Known red on main: four topo torn-body rows under `--features per-op-postcondition`, `pocket_ring_steep_ellipse` at 1e-6 and `arc_loft_natively_computes_its_rational_volume` at 1e-12;
- `scripts/k_probe_sweep.sh <outdir>`. Any red there that is also red on `origin/main`, run there, is main's; say so.

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
