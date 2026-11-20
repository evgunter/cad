---
id: a-gates-line-shaped-matcher-loses-a-site-rustfmt-wraps
kind: issue
title: A line-shaped gate matcher loses a registered site when rustfmt wraps its call across lines, and asks for the register to be lowered
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by `charttail/pole-section` (PR 7180), 2026-11-20.

## What

`scripts/gates/escalation-question-marks.sh`'s callee register counts
"a line holding a `?` on a call that reads `band`". The match is on one
source line. When a change lengthens such a call, rustfmt breaks it so
that `band,` and `)?;` land on different lines. The site then drops out
of the count, and the gate asks for the register to be lowered. The `?`
on a band reading is still in the loop, so the gate has lost a live site
and, if the register is lowered, its disposition too.

The witness is `crates/topo/src/pcurves/site.rs`, `site_rows`' walk. Its
`chart_pcurve_over(&carrier, t0, t1, &face.surface, band)?` became, once
rustfmt broke it,

```
let pcurve = geom_brep::chart_pcurve_over_ends(
    &carrier, t0, t1, ends, &face.surface, band,   // one argument a line
)?;
```

and the gate went from `site.rs 1` to `site.rs 0`. Both reviewers of PR
7180 caught it. One of them showed that a one-line rewrite turns the gate
red again, so the site is live. That PR restores the register entry and
writes the call on one line, by a shorter binding. The class is still
open: any registered or unregistered site whose call rustfmt breaks this
way slips past the gate.

## What a fix owes

Read a call as one statement, not one line: join its continuation lines
up to the `?` before matching. The callee gate's siblings need the same
check (`escalation-question-marks.counts`' `decide*(… )?` rule, the rung
register). `lib.sh`'s Rust reader may already offer a statement view to
build on. Add a selftest plant whose `band` argument and `)?` sit on
different lines.
