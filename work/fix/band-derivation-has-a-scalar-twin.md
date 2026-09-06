---
id: band-derivation-has-a-scalar-twin
kind: issue
title: the linear band's derivation has a scalar twin: ~15 sites compute (eps, K*eps) as bare f64s and never build a Band
status: open
opened: 2026-09-04
refs: [1732]
---


## The finding

`band-linear-spelling-not-swept` made `Band::linear(tol)` the one
spelling wherever a linear decision band is **constructed**. It did not
and could not reach a second family, found while enumerating that
population: sites that derive the band's two thresholds as bare `f64`s
and never build a `Band` at all.

- `crates/profile/tests/bool12r2_probes.rs:42`,
  `crates/profile/tests/bool12_probes.rs:46`,
  `crates/profile/tests/bool12_r1_probes.rs:24`,
  `crates/profile/tests/bool11_probes.rs:45` — `fn band() -> (f64, f64)`
  returning `(t.eps(), t.k() * t.eps())`: the derivation, as a tuple,
  under the same name the `Band` helpers use.
- `crates/profile/tests/r1_bool11_review_probes.rs:57,85,111`,
  `crates/profile/tests/r2_bool11_review_probes.rs:45,88,179` — the same
  pair bound as `(eps, kin)` / `keps`.
- `crates/topo/tests/m5_pr8_bvh_diff.rs:241` —
  `let (zero, escalate) = (tol.eps, tol.k * tol.eps);` then
  `pad = escalate + 2.0 * zero`.
- `crates/step-import/tests/review_r1_tier_gate_probes.rs:339` — the
  same destructuring.
- `crates/topo/tests/seat3_flush_detector.rs:140`,
  `crates/editor-core/tests/lib_sel2_flush.rs:254,322` — the midpoint
  and the two edges of the band, spelled from the fields.
- `crates/geom-brep/tests/offa_r1_probes.rs:226` — `1.0e3 * t.k * t.eps`.

These carry exactly the drift risk the parent item names: a change to
the canonical derivation, or to `Tolerance`'s fields, has to find them
by grep, and they are invisible to any pattern keyed on `Band::new`.

## Why it was not swept with its parent

`Band::linear` returns a `Band` with private fields. Reaching the pair
through it means `let b = Band::linear(tol)?; (b.zero(), b.escalate())`
— routing a two-number computation through a fallible constructor to
read the accessors straight back out, and pushing a `Result` into rows
that currently have none. That is a door question, not a spelling
sweep: either these sites are fine consulting `Tol` directly (in which
case the class is closed by saying so once, here), or the pair wants a
named door of its own on `Tolerance`. Deciding that was outside the
parent unit's remit.

## Not this class

Sites that scale a threshold by something other than the run's K
(`1.0e3 * t.k * t.eps`, `0.5 * (eps + K*eps)`) are deriving a test
geometry's placement *from* the band, not restating the band. They
belong here only if the pair itself gets a door; they are not twins of
the derivation.

## A second shape, not covered by the argument above

Two sites spell the band's **midpoint**, not its (zero, escalate) pair:

- `crates/topo/tests/seat3_flush_detector.rs:140` —
  `let gap = 0.5 * (raw.eps + raw.k * raw.eps);`
- `crates/editor-core/tests/lib_sel2_flush.rs:254` —
  `let gap = 0.5 * (tol.eps + tol.k * tol.eps);`

The same expression, twice, in two crates. The "routing a pair through a
fallible constructor to read its accessors back out" reason above does
**not** apply to them: nothing here wants a pair, so there is no
`Result` to thread and no awkwardness to avoid. They are a plain
duplicate of one derivation — the midpoint of the ambiguity band, which
is what a test picks when it wants a gap that is decidedly neither
coincident nor separated.

That makes them the easiest half of this item, and possibly its answer:
if the midpoint earns a name, these two collapse onto it and the pair
sites stay as they are.
