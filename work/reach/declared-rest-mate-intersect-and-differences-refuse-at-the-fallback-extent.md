---
id: declared-rest-mate-intersect-and-differences-refuse-at-the-fallback-extent
kind: issue
title: A declared cylindrical Rest mate's intersect and both differences refuse FallbackExtentUnsupported, arc-split bore and full-turn bore alike
status: review
opened: 2026-10-02
priority: P1
cost: H
branch: reach/rest-mate-intersect-diff
pr: 3980
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

## Measured cause (`reach/rest-mate-intersect-diff`)

Instrumented on the reduction: under `∪` the declared mate leaves
6–8 null pairs, so the union goes through the join to the REST zip;
under `∩` and `∖` the same reduction (the same vertex contacts) leaves
**no** null pair, since neither op's result changes side across the
contact's rim. Both ops therefore take the no-crossings path, whose
answer for a pure contact is the right one (the vertex probe keeps or
drops each shell whole: `∩` empty, each difference its minuend). What
refused was the path's certificate for curved pairs:
`section_extent_pass` classified the declared bore × wall pair as two
coincident cylinders (`section_cylinder_pair_coincident`, R-tan), and
the pass exempted no pair. The torus socket and peg refuse there too,
and a ball filling a spherical cavity refuses one step earlier, at
`sphere_extent_scan`'s sphere pair (`SpheresMeet`, nested margin
zero), its union included.

## Fix

The no-crossings path reads the coincidence ladder's settled pairs
(`BooleanReduction::coincident`): a pair settled one carrier with
opposed senses (a verified `Rest`, or a shared recipe source) only
touches, because each material stands on its own side of the one
carrier, so it hides no overlap from the vertex probe
(`ops.rs` `touches_only`). The section pass skips such a pair, and the
sphere scan's sphere pair skips a carrier whose every face reaching
the partner face is such a pair. A continuation (aligned senses) is
not exempt.

Pinned by `rest_mate_every_op` (both seam layouts, three radii and
lengths, five spans including blind, azimuths 0° and 60°, two poses;
the ball in a cavity), `full_turn_bore_mate::intersect_and_differences_answer_the_closed_form`
and `mate7a_torus_rest::subtract_and_intersect_on_the_torus_rest_fixtures`,
against closed forms at ε 1e-9, 1e-6 and 1e-12.

A shaft 1e-7° off the bore's seam escalates
`Coincidence(Sectors, Moot)` under every op, the union included: the
sector margin, 1.51e-9, lies inside the band `[1e-9, 1e-8]`, which is
D4's in-band escalation and not this row's.
