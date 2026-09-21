---
id: band-has-no-door-for-an-explicit-eps-with-the-runs-k
kind: issue
title: Band has no door for an explicit eps with the run's K — four suites open-code Band::new(eps, k*eps)
status: closed
closed: 2026-09-16
branch: props/band-doors
pr: 2729
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


## Answered at the fix (2026-09-15, `props/band-doors`)

`Band::linear_at(tol, eps)` — the named constructor, per the dispatch
ruling; `from_zero_threshold` stays private. Witness-first, matching
`linear(tol)` / `angular_at(tol, lever_arm)`.

The count moved in both directions. `crates/sweep/tests/review_fillet_h6_r2_probes.rs`
is **not** one of the door's sites: its `worst_rim_verdict(eps, k, arm)`
takes `k` as a parameter and its callers pass the literals 10.0 and 1.2
deliberately — the row asserts what the classifier does *at a named K*,
with the run's ε and a K that is not the run's, which is the inverse of
this door's shape. A sweep of every `Band::new` site coupled to a K found
two more that are the shape exactly, both in
`crates/geom-brep/tests/curved_torus_arc_residual.rs`. Five sites
converted, not four.


## At the fix pass (2026-09-16)

The dispatch ruling that `review_fillet_h6_r2_probes.rs` should take the
door was **withdrawn** on the review's evidence: the conversion would
break that row twice, not once. `worst_rim_verdict` builds the band AND
the adversarial vector `worst_admitted_w(eps, k)` from the same `k`, so
converting only the band silently decouples the two — on top of making
the row's "at K = 1.2" claim follow whatever K the run committed.

`Band::from_zero_threshold` is retired with this unit. Once the public
door existed the private one was byte-identical to it — same signature,
same body — so `linear` and `angular_at` now call `Band::linear_at`, and
the pure `Band::from_thresholds(zero, k)` (k as a number, not off a
witness) stays as the scaling policy the lib tests can reach without the
global.

`Band::linear_at` is `#[doc(hidden)]`, on `geom-brep`'s `offset_fit`
precedent for the same `_at` shape: an instrument, not a door. Every
consumer is a suite pinning a scale its row is about.

## Closed

Landed on PR #2729 (run 35072003347 green on the fix head). The door is
`Band::linear_at(tol, eps)` — the witness first, as every other
tolerance-coupled `Band` constructor takes it — and
`from_zero_threshold`, which it made byte-identical, is retired.
`from_thresholds(zero, escalate)` stays: it takes its second operand as
a NUMBER rather than off a witness, which is what lets a lib row
exercise the scaling without touching the global.

The door is `#[doc(hidden)]` on `offset_fit`'s precedent for the `_at`
suffix — an instrument rather than a door, since naming your own ε
beside the run's is exactly what D4 ¶1's witness rule exists to stop in
production. It stays `pub` because the five consumers are suites
outside the crate, and its doc names them and points a production
caller at `Band::linear`.

**Five sites converted, not four, and one refutation.** The sweep found
two the item did not list (`crates/geom-brep/tests/curved_torus_arc_residual.rs`,
twice). And `crates/sweep/tests/review_fillet_h6_r2_probes.rs` does NOT
want this door: `worst_rim_verdict(eps, k, arm)` takes K as a
parameter, and its callers pass named values deliberately because the
row's claim is about the classifier AT that K. The review lane found a
second break the first did not — the same `k` builds the adversarial
vector `worst_admitted_w(eps, k)` as well as the band, so converting
only the band would decouple the vector from the band it is the worst
case for. The orchestrator had ruled that site in; the ruling is
withdrawn on that evidence.
