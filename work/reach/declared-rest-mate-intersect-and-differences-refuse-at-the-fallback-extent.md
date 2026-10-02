---
id: declared-rest-mate-intersect-and-differences-refuse-at-the-fallback-extent
kind: issue
title: A declared cylindrical Rest mate's intersect and both differences refuse FallbackExtentUnsupported, arc-split bore and full-turn bore alike
status: open
opened: 2026-10-02
priority: P1
cost: H
---


Found by the REACH lane that closed
`work/reach/full-turn-bore-rest-mate-does-not-union.md`, measured on
`reach/fullturn-bore-mate`.

## Measured

A shaft set into a bore, every bore × shaft wall pair declared `Rest`
(`mate2_common::wall_decls`): the union builds, and `∩`, `A ∖ B` and
`B ∖ A` all refuse

    FallbackExtentUnsupported { operand: A, face: <bore>, what: "a curved
      face and a face of the other solid have tangent or near-tangent
      carriers and no crossing layer saw an event; ..." }

on both collars — the arc-split one (`mate2_common::collar()` ×
`peg(0.5, 2.0)` and `peg(1.0, 1.0)`) and the full-turn revolved one
(`full_turn_bore_mate.rs`'s collar and shaft, through and flush spans,
azimuths 0 and 60). `B ∖ A` takes `wall_decls(&p, &c)`, the
declarations in its own operand order.

The closed forms are simple: the interiors are disjoint, so `∩` is
empty and each difference is its minuend whole.
`full_turn_bore_mate::intersect_and_differences_answer_the_closed_form_or_refuse`
holds the ops to that-or-a-typed-refusal; it does not assert they
build.

## Why

The declared-REST zip (`crates/topo/src/boolean/rest.rs`) is a union
lane (`try_rest_union`'s `debug_assert_eq!(red.op, BooleanOp::Union)`),
and `∩`/`∖` route through the no-crossings fallback's extent gate in
`ops.rs`, which reads the coincident pair as a tangency it cannot
count components across. Where the fallback stops for a declared
conformal pair, and whether `∩`/`∖` of a pure-contact mate should be
answered from the declaration (empty / the minuend) or through a zip
of their own, is unmeasured.

Measured by PR 3814's delta review: with the shaft's seam a hair off
the bore's (1e-7°), `∩` and `∖` answer
`Escalated { Coincidence(Sectors, Moot) }` instead of
`FallbackExtentUnsupported`.
