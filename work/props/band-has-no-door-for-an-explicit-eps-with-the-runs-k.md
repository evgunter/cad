---
id: band-has-no-door-for-an-explicit-eps-with-the-runs-k
kind: issue
title: Band has no door for an explicit eps with the run's K — four suites open-code Band::new(eps, k*eps)
status: open
opened: 2026-09-11
---


(FIX orchestrator) From the `literal-k-where-the-runs-k-belongs` lane,
PR 2346. Placed here because `crates/geom-core/src/*` is PROPS' glob;
PROPS has no open PR (checked).

`Band::linear(tol)` takes only a `Tol` witness and derives ε from the
run. `Band::from_zero_threshold` is private. So there is **no door for
"an explicit ε with the run's K"** — a suite that pins its own ε (a
bisection variable, an ε ladder) but wants the run's escalation
behaviour has nothing to call, and open-codes
`Band::new(eps, tol.k() * eps)`.

**Four sites want it**, not the one the parent item names — the count
is the reason this is a row rather than a sentence:

- `crates/sweep/tests/common/approx.rs` (`reattach_certifies_at`) and
  `crates/sweep/tests/sf2b_r1_probes.rs` — converted to the inline
  spelling by PR 2346, because it is the correct behaviour and no door
  exists to say it in one word.
- `crates/geom-brep/tests/pcurve_p1b_r2_probes.rs:477` and
  `crates/sweep/tests/review_fillet_h6_r2_probes.rs:72` — already
  open-coding it before that unit.

The parent item named only `tcost_k1_budget_exit.rs` and described the
gap as structural to that one suite. It is not: it is the shape any
suite hits that pins ε and needs K, and four of them have hit it.

## The decision

Whether the door is `Band::from_zero_threshold` made public, a named
constructor (`Band::linear_at(eps, tol)`), or a deliberate refusal with
the inline spelling documented as the answer. The third is a real
option — the two operands are exactly what `Band::new` takes, and a
door that saves one multiplication may not earn its name. What the four
sites establish is that the question has an audience.

Not urgent and blocking nothing: every site works today and now says
what it means.
