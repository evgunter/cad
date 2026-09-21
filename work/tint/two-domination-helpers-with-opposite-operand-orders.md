---
id: two-domination-helpers-with-opposite-operand-orders
kind: issue
title: Two domination helpers, opposite operand orders: tightness::Sup::new(claim, bound, truth) vs mesh's Domination (lesser, greater) — and mesh/tests/*.rs cannot reach the second
status: open
opened: 2026-09-18
---

## Finding

After PR 2848 (`tess/domination-assert-messages`) the workspace has two
helpers for "a certified bound dominates a sampled truth", and they take
their operands in OPPOSITE orders:

- `test_utils::tightness::Sup::new(claim, bound, truth)` — **greater
  first**. (`Meter::new(claim, bound, truth)` is a LOWER bound, so there
  the first number is the lesser one: the order is "certified, sampled",
  not "greater, lesser".)
- `mesh`'s `nurbs_cert::tests::Domination`, whose components are
  `(name, lesser, greater)` — **lesser first**, i.e. "sampled, certified".

Both take bare `f64`s, so a transposed call compiles, and on a sound
bound it fails loudly only if the bound is not tight — on a tight one
(`truth/bound` near 1, which is what these rows are built to reach) a
transposition can pass. PR 2848 exists because a reader took
`(sampled) vs (bound)` the wrong way round in a MESSAGE; two helpers with
opposite conventions move that same hazard to the CALL SITE.

**And the second helper is unreachable where it is most wanted.**
`Domination` is `pub(crate)` inside `#[cfg(test)] mod tests` of
`crates/mesh/src/nurbs_cert.rs`, so `crates/mesh/tests/*.rs` structurally
cannot name it: `probe_review.rs`'s
`z1_per_triangle_certificate_falsification` (the `worst_ratio` row hosted
CI runs as its falsifier — `work/chord/S237.md`), `budget_meter.rs` and
`m7_nurbs_trimmed.rs`'s promise rows all spell their own messages, as do
the `crates/geom-brep/tests/` rows in
`work/tint/enclosure-asserts-print-too-few-digits-to-show-a-last-bit-red.md`.
PR 2848's helper is a half-fix for that reason and says so in its body.

## What is asked

One home, in `crates/test-utils` beside `tightness::Sup`, with ONE operand
order across `Sup`, `Meter` and the componentwise form — preferably one the
type system carries (named fields, or a `Sampled(f64)` / `Certified(f64)`
pair) so that a transposition does not compile. `mesh`'s `Domination` then
becomes a caller of it or goes away. What `Domination` has that `Sup` lacks:
several named components in one claim, each escaping one called out with
its excess, `{:.17e}` throughout, a soundness-only use that does not have to
reach a ceiling, and a refusal of an empty component list.
