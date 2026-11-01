---
id: step-import-unwrap-reads-a-whole-period-off-a-value
kind: issue
title: step-import's outerness unwrap reads a whole-period integer off a value by rounding
status: open
opened: 2026-10-17
priority: P4
cost: E
---

Found by PCERT unit B2b's sweep of the places that turn `k` periods into
a length (`work/pcert/windings-are-stored-structure-counted-in-the-channels-period`).
It is outside that sweep's list, so B2b leaves it unchanged.

## What is there

`crates/step-import/src/chart.rs:350`, `fn unwrap`, is called by
`infer_outer` (`:280`, `:283`) on each periodic channel of a ring's
chart samples. It computes:

```rust
let k = ((prev - next) / tau).round();
let lifted = next + k * tau;
```

So the whole-period integer `k` is read off a value by rounding, at
`f64`, through no band. C4 (geom-brep README, ratified) says no
whole-period integer is read off a value. The kernel decides them
through the half-period marks (`geom_brep::Period::decide`) and turns
them into lengths only through `Period::times`.

## What it costs today

Nothing a reader sees. A step whose gap sits on a half-period mark
lifts, either way the rounding falls, to about `π` from `prev`, past
the quarter turn the very next line refuses as `SeamDependent`
(`:353`). So a silent pick on a mark never reaches outerness
inference; every step that does is within a quarter turn, where the
rounding has one answer. What is left is the spelling: the integer is
read off a value, where C4 decides it by the marks.

## The fix

Decide `k` with `Period::new(tau).decide(…)` at a metre lever (the
chart's arm at the sample), and lift with `Period::times(k)`. That
reads an undecided mark as the refusal it is. Alternatively, the
item can record why this f64 import heuristic is exempt.
