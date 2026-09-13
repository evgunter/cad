---
id: effort-dials-that-do-not-reach-the-search-space
kind: issue
title: Two EFFORT dials buy nothing: a 1681-point enumeration on a multiplier, and a walk depth that never scales
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the orchestrator out of the style review of PR 2433.
Both instances were found *because* that PR put the rows on
`CAD_FUZZ_EFFORT` for the first time — putting a dial on a row is what
makes it visible that the dial buys nothing.

**Why this is S-TCOST's and why it matters later rather than now.** Both
rows are cheap today and neither is a cost finding. They bind when
`fuzz-depth-not-existence-run-everything-at-effort-1` wires the EFFORT
policy: that row's whole argument is *"a broken marker costs the raise,
not the run"*, and a raise that buys no new coverage is the same defect
seen from the other end — a lane spends the dial and gets nothing. This
row rides with that one.

## Instance 1 — a 1 681-point enumeration on a multiplier

`crates/editor-core/tests/r1_dual_probes.rs`,
`r1_no_value_only_key_collision_search`. Its `draw` helper yields exactly
**41 distinct f64 values**, so the (v, t) space it searches is 41 x 41 =
**1 681 points**. The first loop draws 20 000 of them and the second
2 000; coupon-collector saturation over 1 681 points is about 12 500
draws, so the first loop already exhausts the space with margin.

`CAD_FUZZ_EFFORT=100` therefore buys **2 000 000 draws over 1 681
points** and zero new coverage. Measured on one box: 229 ms at EFFORT=1,
483 ms at EFFORT=20 — so the dial costs real time and returns nothing.

`memories/test-suite-cost.md` names this shape exactly: *"a sweep whose
real content is an edge-value table or a product of boundary cases ... is
an ENUMERATION — write it as one and let the filler vary."* The fix is
to make it one: enumerate the 1 681 pairs deterministically and put the
dial on whatever is genuinely sampled, if anything is.

## Instance 2 — the walk breadth scales and the depth does not

`crates/viewer/tests/review_gui0_r1.rs`,
`the_camera_contract_survives_random_operation_walks`. The walk COUNT is
now `fuzz::scaled(48)`; the walk LENGTH is a bare `let steps = 16;` that
no dial reaches.

Depth is what decides how far into camera state space the search gets —
a contract that breaks only after twenty operations is unreachable at any
EFFORT. So raising the dial buys more short walks and never a longer one,
which is the one axis the row's own name ("survives random operation
walks") is about.

Not necessarily a defect: there may be a reason the length is fixed, and
a walk whose length varies makes a failure harder to shrink. But the
reason is not written down, and `memories/test-suite-cost.md` requires a
count that is deliberately fixed to say so in-file.

## What this row asks for

Per instance: decide whether the quantity should ride the dial, and
either put it on `fuzz::scaled` or **say in-file why it is fixed**. For
instance 1 that probably means rewriting the row as the enumeration it
already is; for instance 2 it may be one sentence.

## The class, and where else to look

The pattern is **a randomized row where the dial does not reach the
quantity that bounds the search**. Both instances were found by reading
two files closely, not by a sweep, so the census does not exist. Where to
look: every row converted to `fuzz::scaled` — the conversion touches the
count and leaves every other constant alone, so any second loop bound,
walk length, grid resolution or retry cap beside a `scaled()` call is a
candidate. A grep for `scaled(` and then reading the surrounding function
for bare integer bounds is the cheap version.
