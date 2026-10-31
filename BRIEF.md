# Reviewer brief: SECT PR 6062 (the seam-region door row's cost)

You are the single FULL reviewer (claims to falsify plus the style lane).

**Frozen head.** `be61e2961ae88d943a7e0ff402c7b562ccdb866f` of
https://github.com/evgunter/cad/pull/6062 (branch `sect/seam-region-speed`):
`git fetch origin sect/seam-region-speed && git checkout --detach be61e2961ae88d943a7e0ff402c7b562ccdb866f`.
Read the PR body via the GitHub MCP tools (ToolSearch; no gh). Do not read its
comments or reviews. Post nothing, push nothing.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md`; `CLAUDE.md`;
the row `work/sect/main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord.md`
as on main; C2 in `crates/geom-brep/README.md` (certificates decide; a memo
must not skip a decision); `memories/agent-lane-operations.md` on how CI
silently does not run.

**Builds.** `CARGO_TARGET_DIR` outside the checkout, `CARGO_INCREMENTAL=0`;
time at opt-level 2; foreground; anything past 600 s `setsid`-detached and
polled; nothing left running. **Budget: about 2 hours.**

## Claims to falsify

1. **The span-hull memo (`pcurve_cache::span_memo`) is pure.** Keyed by the
   whole input as bits, it returns exactly what the uncached computation
   would, decides nothing inside, and cannot serve a stale or colliding entry
   (a key that misses a field, NaN/-0.0 bits, interval vs f64, a key across
   threads or bodies). Check its scope and lifetime (no hidden state that
   outlives an operation in a way C2 or `docs/DESIGN.md`'s functional style
   forbids), its hit/miss test, and that no verdict moved anywhere: the PR
   claims identical outputs on main and branch with a fixed seed — reproduce.
2. **The split** of the row into five tests keeps its corpus and assertion.
   Four tests now log `VACUOUS` and assert nothing, since all 20 cylinder ops
   refuse at the result gate. Is that a weakening that hides a regression (a
   refusal kind changing, an op starting to return a wrong body)? Should they
   pin their refusals instead? Check the stops by raising site and payload.
3. **The nightly hold-out removal.** Time the five tests at 1e-12 at opt-level 2
   and confirm each fits; check `.github/workflows/nightly.yml` is otherwise
   unchanged.
4. **The profile's claims** (span hulls recomputed per image while limb 2 reads
   them through a memo; rung-3 repeats 40 calls per op): spot-check them.
5. **The headroom estimate** (the full-suite job no longer fits 180 minutes):
   is the arithmetic in the PR body and in
   `work/green/the-full-suite-jobs-three-eps-rows-outgrow-its-180-minutes.md`
   sound from the data given?
6. **Suites.** `pcurve`-touching crates and `sweep` (`--profile ci`) on head and
   merge-base.

## Report

Final message, ≤120 lines: verdict (APPROVE · APPROVE-WITH-FIXES ·
NOT-MERGEABLE-AS-IS · REJECT); findings MAJOR/MINOR/NOTE with `file:line`,
evidence, executed or by inspection; `## Style`; tests worth keeping as code
(commit nothing).
