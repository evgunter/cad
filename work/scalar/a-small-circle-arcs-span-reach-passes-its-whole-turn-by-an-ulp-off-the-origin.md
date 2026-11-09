---
id: a-small-circle-arcs-span-reach-passes-its-whole-turn-by-an-ulp-off-the-origin
kind: issue
title: A 1 mm circle arc's span reach from its centre, 1 km off the origin, passes its whole turn by an ulp (span_reach_hunt, seed 0xe3d7d198da905f4c)
status: closed
opened: 2026-11-03
closed: 2026-11-09
priority: P2
refs: [a-conic-arcs-span-reach-passes-its-whole-turn-lever]
---

Found by RESTREAD's 6422 verifier, red at merge base `82938e9734` too.
No row under `work/` names this seed. The class is the unit ellipse's
ulp case (folded into `a-conic-arcs-span-reach-passes-its-whole-turn-lever` by PR 6483),
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

## Another seed (SPLICE, 2026-11-04)

`span_reach_hunt::a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`
was red in `splice/teapot-1e6-floor`'s full geom-brep run, and is red at
`CAD_FUZZ_SEED=0x7236d51c917fecd0 CAD_FUZZ_EFFORT=1` with that branch's
only kernel change switched off. The circle has radius 1000 about the
origin, axis `(0.7086, −0.4991, −0.4988)`, over
`[−1.64831, −1.37312]`, read from its centre: its span reach is
`1000.0000000000003` and passes the whole turn. This is the same class
at a 1 km radius with its centre on the origin, so it is not only a far
centre.

## Closed (BOXES, `boxes/nightly-reds`, 2026-11-09)

Caused by BOXES' PR 6399 (`boxes/cover-edge-doors`). It made `Reach::span_reach` read the span's cover (`cover_far`) and dropped the clamp to the whole turn's reach (`.min(|c − p| + r)`) that the circle and ellipse arms had held, and with it the doc's "never past `lever_from`". At `c036d7ae88^1` the hunt is green on 400 of 400 seeds; at `c036d7ae88` it is red on 400 of 400. `span_reach` now takes the lesser of the span's cover and the whole conic's (`reach_from`), both sound upper bounds of the arc's far distance. The hunt is green on 400 of 400 seeds and on every seed recorded in these rows. Two witnesses from the 11-09 nightly are pinned as `span_reach_hunt::an_arc_read_from_its_centre_is_never_past_the_whole_turn`, red without the clamp.
