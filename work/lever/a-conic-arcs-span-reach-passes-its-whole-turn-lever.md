---
id: a-conic-arcs-span-reach-passes-its-whole-turn-lever
kind: issue
title: span_reach_hunt finds a thin ellipse arc of nearly two turns whose span reach passes its whole-turn lever (Reach::span_reach_from > Reach::lever_from)
status: open
opened: 2026-11-03
priority: P3
cost: E
---


Filed by the split-parameter lane (`splice/split-parameter`), from a
full `geom-brep` run. Reproduced on main's own tree at d1ac6881. Not
this lane's ground: the lane touched only `geom_brep::curve_face`.
`scripts/work.py territory` names no owner for
`crates/geom-brep/src/extent.rs`.

## What

The fuzzer `crates/geom-brep/tests/span_reach_hunt.rs`
(`a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`) fails
its third assertion, "never past the whole turn"
(`got <= reach.lever_from(p).length()`, with no rounding allowance):

```
CAD_FUZZ_SEED=0x9a93d993d0da955b CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p geom-brep --features test-support \
  -E 'test(a_conic_arcs_span_reach_is_never_short)'
```

The arc is an ellipse with `semi_u = 1e-3` and `semi_v ≈ 4.36e-4`,
spanning `[-4.068341839926886, -16.471045300156703]`. That window runs
backwards and covers about 1.97 turns. The pivot is about 0.0146 from the
arc, some 15 times the arc's size. The span reach reads 0.014602591642708138
and passes the lever.

## Not yet measured

By how much it passes. The message does not print `lever_from`'s value.
Two readings are open:

- a rounding-level excess: the span reach's composition for a window
  past a turn rounds above the whole-turn lever, and the assertion wants
  `tol`;
- a real defect: the span reach of a window past a turn does not clamp
  to the turn, which is the claim `Reach::lever_from`'s bound rests on.

The first measurement is `got − lever` at this seed.

Moved from `work/issues/` to `work/lever/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

**A second seed** (ONECOPY `onecopy/angle-reads`, 2026-11-03), found by a
local full geom-brep run and reproducible with `CAD_FUZZ_SEED=0xd54786f7cb0d235c
CAD_FUZZ_EFFORT=1` (`--features test-support`): a thin ellipse
(`semi_u` 0.0014142132088478148, `semi_v` 1.0; centre the origin, axis
`(-0.5572965940438884, 0.28438696616547604, 0.7800926609975752)`,
`u_ref` `(0.6166163900257462, -0.48744344408386014, 0.6182096055313937)`)
over `[6.778616771278262, 18.0108665539107]`, from the origin, fails the
same third assertion. Neither side of the comparison reads anything that
branch changed.
