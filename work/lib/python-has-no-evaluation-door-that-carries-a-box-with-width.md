---
id: python-has-no-evaluation-door-that-carries-a-box-with-width
kind: issue
title: Three EvalOptions fields defer to an evaluation door that carries a box with width, and no row schedules one
status: open
opened: 2026-09-15
priority: P3
cost: D
---

## Disclosed by PORT's `python-cannot-set-options-structs` (PR #2678)

`crates/pncad-py/src/surface_census.rs`'s `EvalOptions` roster records
three fields as deliberately unbound, and all three reasons defer to
the same absent thing:

- **`param_box`** (E6) — a box IS reachable at `f64` and from Python,
  because `editor-core`'s MC lane evaluates under one and
  `monte_carlo` is its door; but only the DEGENERATE form, the point
  sample `AxisScalar for f64` admits. A box with WIDTH needs an
  evaluation at a scalar carrying a bracket.
- **`seed`** (E4) — the tangent seed needs a scalar carrying a tangent,
  and `f64` carries none. The box's twin, with no degenerate form that
  reaches `f64` at all.
- **`profile_lift`** (M10-P) — not answer-preserving in general; what
  makes it unreachable-without-loss at `evaluate` is that this door's
  box is always the nominal one, which is `param_box`'s entry rather
  than a fact about `f64`.

Bind any one of them and the others become answerable in the same
breath, because what all three want is one thing: **a Python
evaluation door that can be handed a parameter box with width**, at a
scalar that can carry it.

**No row on any slate schedules one.** `advisory-monte-carlo-lane-has-no-python-door`
was the MC half and is a different door — `monte_carlo` takes an
`AnalyzedBox` and answers a statistical report; it does not hand the
caller an `Evaluation` under a box.

`work/README.md`'s rule is that disclosing a residue is not scheduling
it. The `declared_contacts` residue of the same PR got its file; these
three did not, and this is that file. The unit that lands it retires
three `NotBound` entries at once, and `the_not_bound_roster_decays` is
what will notice if it retires the spellings and leaves the reasons.

## Home

`work/lib/` — the missing door is `evaluate` in `crates/pncad-py/*`,
LIB's territory. The scalar work behind it is `editor-core`'s
analysis lanes, which already exist; what is missing is the binding.

## Filed by

A PORT lane, at PR #2678, at the moment the three reasons were written.
