---
id: a-small-circle-arcs-span-reach-passes-its-whole-turn-by-an-ulp-off-the-origin
kind: issue
title: A 1 mm circle arc's span reach from its centre, 1 km off the origin, passes its whole turn by an ulp (span_reach_hunt, seed 0xe3d7d198da905f4c)
status: open
opened: 2026-11-03
priority: P2
refs: [a-unit-ellipse-arcs-span-reach-passes-its-whole-turn-by-an-ulp, a-conic-arcs-span-reach-passes-its-whole-turn-lever]
---

Found by RESTREAD's 6422 verifier, red at merge base `82938e9734` too.
No row under `work/` names this seed. The class is the unit ellipse's
ulp case (`a-unit-ellipse-arcs-span-reach-passes-its-whole-turn-by-an-ulp`),
here at a 1 mm radius about a centre a kilometre off.

`crates/geom-brep/tests/span_reach_hunt.rs`
`a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`, its
third assertion (`got <= reach.lever_from(p).length()`,
`span_reach_hunt.rs:133`), reds on:

- carrier `Ellipse { frame: { center: (535.8720700319723, 777.7496034004437, -328.55239912394734), axis: (-0.17860460497984457, 0.7737595028734481, 0.6077800809446029), u_ref: (0.8878945916627065, -0.13941854205704954, 0.43841266430932674) }, semi_u: 0.001, semi_v: 0.001 }`;
- span `[-2.589675712030342, -2.58924611454626]` (about 4.3e-4 wide), pivot the centre;
- `span_reach_from` reads `0.0010000000000000005`, past the whole turn's `0.001`.

Reproduce:

```
CAD_FUZZ_SEED=0xe3d7d198da905f4c CAD_FUZZ_EFFORT=1 \
  cargo test -p geom-brep --all-features --test all -- \
  span_reach_hunt::a_conic_arcs_span_reach_is_never_short_of_its_farthest_point
```

## Done when

The fix for the unit-ellipse row (a clamp to the whole turn's reach, or
the assertion's rounding tolerance) also clears this seed, and the seed
is pinned as a deterministic row beside it.
