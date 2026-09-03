---
id: interval-eps-1e-12-stale-arc-constant
kind: issue
title: main is red at interval / eps=1e-12 - three sweep rows on one stale measured constant (the arc chain moved)
status: open
opened: 2026-08-31
github: 1338
refs: [921, 1329, 1330]
---

## From GitHub issue 1338

opened 2026-08-31, 0 comments.

**Found by SMELL Track T lane T-a (PR #1329) when its CI draw landed on `interval, eps = 1e-12` — a point that branch's earlier runs never drew. Reported, not fixed: these rows are outside that lane's two rows, and the fix is a re-measurement someone has to adjudicate.**

Same class as **#921** (closed, "Two Interval rows are red on main at eps=1e-12"), different rows.

## Reproduction, on `main` itself

`a71eb713` (current tip, including the #1330 fix), no patch applied:

```
CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --features interval
```

```
FAIL sweep::all m5_s12_curved_ops_interval::certified::interval_sphere_subtract_decides_definitely_after_the_recut
FAIL sweep::all review_arceval_r1_probes::certified::e2_recut_escalation_hi_is_pinned_to_the_measured_constant
FAIL sweep::all m5_s13_pips_interval::certified::interval_finding_union_is_bracketed
```

Confirmed identical on the lane branch and on a clean detached `origin/main` checkout, so nothing in #1329 is involved. Hosted evidence: run 3821, both `test (interval, eps = 1e-12, N/2)` shards, one failure each (the shards split these three).

## One root cause, and the rows say what it is

Two of the three are the same number, pinned in two files:

```
m5_s12_curved_ops_interval: the mapped-source enclosure is 1.1362773333939659e-12,
  not its measured value 1.1414768974413613e-12 — the arc chain moved; re-measure and re-state

review_arceval_r1_probes: the escalation hi 1.1362773333939659e-12 is not the measured
  constant 1.1414768974413613e-12 — the arc chain moved and the m5 row's constant is stale
```

The third is the consequence rather than a second cause: with the enclosure now *below* where the constant says it is, the chain no longer escalates on the mapped-source check and the boolean returns `Ok(..)` where the row expects an escalation —

```
m5_s13_pips_interval: below the enclosure width the chain must escalate on the
  mapped-source check, got Ok(Body(..))
```

So: one moved quantity (`1.14147…e-12` → `1.13627…e-12`, about 0.46% narrower), pinned in two places and load-bearing for a refusal in a third.

## For whoever takes it

The rows' own instruction is "re-measure and re-state", and the two constants clearly want one shared home rather than two copies — but **re-measuring is not the same as restoring the number**. Per the standing rule, a moved baseline is evidence about what the kernel now does: the question is whether `1.13627…e-12` is *correct*, and only then does it get written down. In particular the third row is asserting that a refusal happens below the enclosure width, and if the enclosure genuinely narrowed, what changed is which side of the band that fixture sits on — which is a statement about the arc chain, not about the constant.

Worth noting for scheduling: this point is **sampled**, so `main` shows green on most runs and this stays invisible until a PR happens to draw the 1e-12 interval row. That is the same way #921 surfaced.

## Home

A red-on-main measured-constant adjudication over `crates/sweep`'s interval rows: S-QA (gates that lie) is closed, S-TCOST owns suite cost rather than baseline truth, and no open program's territory covers `crates/sweep/tests/m5_*` — so `work/issues/`.
