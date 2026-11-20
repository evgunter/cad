---
id: the-margin-allowance-parameter-spreads-to-every-funnel
kind: issue
title: Margin's allowance parameter spreads to every funnel signature, and a T-or-f64 gap type bridges a trait
status: open
opened: 2026-11-20
priority: P4
---


Recorded by the review of `a-bound-to-decide-against-never-becomes-a-t` (PR 7185); not reworked in that unit.

## What

- `Margin<T, A = ()>` carries an `f64` bound as `A = Allowance`. What the parameter buys: a margin with a bound has no `value()` exit, by type. What it costs: `A: MarginTerms` on the nine `k_stats` funnels, the seven `geom_brep::dihedral` wrappers, the two `topo::validate` wrappers, `projected.rs`'s `positive_as`/`holds`, `closure::JunctionExcess<T, A>`, and `unit_vec::decide_direction_by`. The alternative is a door that takes the `f64` and returns a sign, which keeps `Margin<T>` single-parameter at the cost of a second family of `decide*`.
- `topo::chain_parity::Gap::{Read, Bound}` and `topo::spline_trim::Floor::{Charged, Uncharged}` are "`T` or `f64`" types beside the margin's own. Each picks which door its trait's two implementors decide through.
- `editor-core::mate::coset::Arm::lever` holds the one `Lever::unsourced` for every mate decision levered at the bundle's arm (`lever-sources.counts`, `coset.rs 1`), so a new `arm.lever()` reader moves no pin, where the gate's header says each unsourced reference is a question for Ev.

## The question

Whether the parameter earns its spread once a second bound-carrying site wants a funnel it does not yet reach, and whether `Arm::lever` readers should be counted per reader.

## From the delta re-review of PR 7185 (2026-11-20)

- The cost is not prospective: about twenty funnel signatures carry the
  parameter today. With the runtime "none" gone the reviewer judged the
  parameter sound and its no-`value()` guarantee real.
- **The either-enum is the pattern to watch, and it already has two
  instances**: `chain_parity::Gap::{Read, Bound}` and
  `spline_trim::Floor::{Charged, Uncharged}`
  (`crates/topo/src/spline_trim.rs:133-160`), each "a `T` or a bound"
  with its own `decide`. The fix pass grew the second. A third mixed
  site is the trigger to give the pair one home.
- `replace_face::bound_within_eps`
  (`crates/topo/src/replace_face.rs:4236-4249`) copies
  `gap_within_eps`'s body and restates `ladder::decided`'s non-finite
  rule by hand, because `decided` takes `Margin<T>` and reads
  `.value()`. One rule in two places; they agree while ε is finite.
- `Margin<f64, Allowance>::at_f64`
  (`crates/geom-core/src/predicate.rs:1004-1008`) is `pub` with only
  test callers.
- The `Arm::lever` gate question (who counts unsourced readers) is a
  separate matter from the parameter and wants its own row when it is
  taken up.

