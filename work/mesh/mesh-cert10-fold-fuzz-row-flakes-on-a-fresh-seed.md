---
id: mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed
kind: issue
title: cert10 and the rational sweep discard 44% and 32% of their own trials: a degree-1 generator bug, and a floor counted against trials never run
status: closed
opened: 2026-09-06
refs: [1907]
closed: 2026-09-11
---


## What

CI run 34082791499 (PR #1907's docs-only state-sync head `59e524d9e`,
job `test (eps = 1e-6, 1/2)`) reddened on
`crates/mesh/src/nurbs_cert_fuzz.rs:247`:

```
the sweep must keep producing STRICT gaps: 59 strict of 300 comparisons
— reproduce with CAD_FUZZ_SEED=0xdcc78227f392d565 CAD_FUZZ_EFFORT=1
```

The PR touches `topo/src/boolean/boxes.rs`, `census.rs` and demo/test
files — nothing under `crates/mesh/`; main's two most recent CI runs
on the same tree family are green. The row draws a random seed per
run, so this is a seed that falsifies the row's "strict gap" count, not
a regression the PR made. Per `memories/test-suite-cost.md`, a seeded
row that gates owes either a fixed seed with the property it pins, or
the property re-stated so a fresh seed cannot red it without a defect.

## Whose

S-MESH (`crates/mesh/*`). The failed jobs were re-run to draw a new
seed; the merge was annotated with this file. Filed by the CURVED
orchestrator.

## Claimed by S-MESH and closed (2026-09-11)

Moved here from `work/issues/` by the lane that fixed it. The body above
said *"Whose: S-MESH"* from the day it was filed and the file sat in the
unowned pile for five days; `work/README.md` says an issue whose owner is
clear goes straight onto that program's slate. **The id keeps the word
"flakes" because ids are stable for life** — the title no longer does,
and this section says why.

**It was never a flake.** Ev, in chat 2026-09-11: *"there shouldn't be
any flakes here at all. any transient red on a test (but not on like,
download or something) is a real bug in either the code or the test."*
Read that way in ten minutes of measurement it stopped being a seed
story and became a generator bug with a false denominator on top.

## The root cause, measured

`mk` drew a **degree-1 direction with interior knots**, which
`patch_bound::check_direction` refuses as `Degree1Crease` — so
`whole_net_bound` returned `None` and the whole trial hit the `continue`.
The generator's own comment said *"a degree-1 direction admits no
interior knot at all"* and the code beside it wrote
`if p >= 2 { 1 + r.below(p - 1) } else { 1 }`: the multiplicity fell to
**1** at `p == 1` where it had to fall to **0**.

Arithmetic: `p` is uniform on {1,2,3} and interior knots exist when
`spans > 1` (3 in 4), so P(a direction is degree-1 and knotted) = 1/4 and
P(either is) = 1 − (3/4)² = **43.75%**. Instrumented over 40 seeds:
**26.8 of 60 trials discarded per run, and every single discard was a
degree-1 direction carrying interior knots** (the `NurbsSurface::new`
arm never fired once).

**The floor then counted those discards as if they had run.** `strict` is
incremented only inside the comparison loop; the floor is
`strict > trials`, where `trials` is the count ATTEMPTED. So it silently
demanded ~1.8 strict comparisons per *measured* trial while reading as
1 per trial, and the reported denominator `trials * 5` named comparisons
that never happened. On the seed this issue was filed for:
**24 of 60 trials compared, 59 strict, reported as "59 strict of 300"** —
there were 120. `strict` tracked `compared` at r = 0.797 across 60 seeds:
the floor was mostly measuring how many trials survived.

That false denominator is why this was diagnosed as a seed twice. It made
a discard problem look like a thin statistical margin, which is
`memories/refusal-text-is-not-cause.md` exactly.

## The sibling, found by the same measurement

`r1_random_rational_soundness_sweep` ten lines up has the same generator
shape and discards **32%** of its trials — measured, every discard a
`pu == 1` or `pv == 1` draw. Its floor is a max (`worst > 0.5`) not a
count, so it cannot fail the same arithmetic way; what it loses is
adversarial breadth, silently. Fixed in the same commit because it is the
same defect in the same file, not a widening.

## What landed

- **Both generators draw a degree-1 direction single-span.** Nothing the
  bounds refuse is drawn any more.
- **Both rows count `compared` and `assert_eq!(compared, trials)`**, so a
  future drift between generator and bound reds loudly instead of
  shrinking the sample in silence.
- **The denominator is spelled from `compared`, never `trials`**, so the
  number in the message is one the run actually performed.
- The "RAISE the trial count" remedy in the old comment is gone: it only
  concentrated the ratio and never touched the discard rate driving the
  tail.

## Measured after

The seed this issue names, `0xdcc78227f392d565`, **passes**: 154 strict
of a true 300. Over 200 fresh seeds, 0 failures, min strict **114**
against a floor of >60 — where before it was mean 111.5 with a **min of
61** against that same floor, a margin of one. Comparisons per run went
from ~165 actual (300 claimed) to 300 actual, so the row now measures
roughly **twice** what it did and says so honestly. 150 seeds across all
three fuzz rows in the file: 0 failures.

The class — a fuzz row whose `continue` shrinks its own sample while its
floor is written against the attempted count — has **twelve candidate
files** tree-wide and is filed for S-TINT as
`fuzz-rows-discard-trials-against-a-floor-that-counts-them`.
