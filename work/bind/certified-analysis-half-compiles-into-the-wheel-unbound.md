---
id: certified-analysis-half-compiles-into-the-wheel-unbound
kind: issue
title: The certified analysis half compiles into the wheel on every build and no Python door binds it
status: open
opened: 2026-09-24
priority: P3
cost: D
refs: [ring-4-interval-feature-dropped]
---

## What

RING-4 deleted the `interval` cargo feature, so `pncad::analysis`'s
certified half — the E6 subdivision driver and its `ParamBox`, the E4/E5
sensitivity and stackup, the E10 reporting layer, `assertion_at`, about
forty names (`crates/pncad/src/analysis.rs`, the five `pub use` lists after
the scalar-free one) — now compiles into every build of the façade,
including the one `crates/pncad-py` builds the wheel from.

Before RING-4 the binding census dispositioned those names as
`different-shape` on the ground that they were "not in the crate this
binding compiles into" (`crates/pncad-py/tests/test_binding_census.py`,
the `NOT_BOUND` entries under the analysis family). That ground is gone.
The disposition still holds on the second ground the census gave — the
binding evaluates at `f64` alone (`crates/pncad-py/src/py/value.rs`), so no
Python evaluation has a certified leaf to hand these doors — and RING-4
re-worded the census prose, `crates/pncad-py/src/py/analysis.rs`'s module
doc and `docs/GUIDE.md` §3 to say that rather than the old reason.

What is open is the decision the old ground made for free: whether the
wheel should bind the certified half now that it ships in the artifact.
Binding it needs a Python surface that evaluates at the certified scalar
(the census's own "WHAT WOULD MAKE THIS ROW STOP BEING HONEST" sentence),
which is a design question, not a mechanical one.

## Evidence

- `git grep -n 'feature = "interval"' crates/pncad` answers nothing after
  RING-4; before it, `crates/pncad/src/analysis.rs` carried five gates.
- `crates/pncad-py/tests/test_binding_census.py` — the analysis family's
  `NOT_BOUND` block ("THEY ANSWER AT A SCALAR THIS BINDING NEVER EVALUATES
  AT").
