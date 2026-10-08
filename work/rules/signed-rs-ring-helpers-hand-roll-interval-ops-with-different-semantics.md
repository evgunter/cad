---
id: signed-rs-ring-helpers-hand-roll-interval-ops-with-different-semantics
kind: issue
title: sym/signed.rs's ring_sqrt/ring_abs/ring_min/ring_max keep RingInterval vocabulary and hand-roll Interval's ops: ring_sqrt certifies [0, √hi] on a straddling argument where Interval::sqrt decorates Trv
status: open
opened: 2026-10-01
priority: P2
cost: M
refs: [the-decision-read-answers-theorems-the-must-carry-stations-would-prove]
---

## What was found (PR #2468's review, filed by LINALG's merge lane, 2026-10-01)

`geom_core::sym::signed`'s deep enclosure (`enclose_indet`, which the
decision read and rule G's side condition use) encloses the `sqrt`,
`abs`, `min` and `max` atoms through four local helpers: `ring_sqrt`,
`ring_abs`, `ring_min` and `ring_max`. They were written against
`RingInterval`. RING-3 dissolved that type into `Interval`, and the
merge of `main` into `props/sign-hull` ported their types mechanically.
The helpers still carry the ring vocabulary, and they re-implement ops
`Interval` already has, with different semantics:

- `ring_sqrt` on an argument that straddles zero
  (`lo < 0 ≤ hi`) answers `[0, √hi]`, rounded outward, as a CERTIFIED
  bracket. `Interval::sqrt` on the same argument decorates the result
  `Trv`: the domain was not met over the whole box. The read then
  takes a sign off an enclosure the scalar itself would not certify.
  Whether that is sound depends on an argument the code does not
  state: the `sqrt` atom only has a value where its argument is
  non-negative.
- `ring_abs`, `ring_min` and `ring_max` hand-build `from_bounds`
  results and do not carry the decoration forward the way `Interval`'s
  own `abs`, `min` and `max` do.

## What a fix would be

Either call `Interval`'s own ops and let the decoration answer, which
makes a straddling `sqrt` decline the read, or keep the helpers and
state at `enclose_indet` why a straddling `sqrt` argument may be
clipped to `[0, hi]` (the atom's own domain). Then rename the helpers
off the retired type's vocabulary. Re-measure the decision read's
reach on the measured documents after either change; the read's
`sign_gated` counts are pinned in `m10_9_pins_interval` and
`sym_9_retry_interval`.

## Home

SYM: `crates/geom-core/src/sym/signed.rs`.
