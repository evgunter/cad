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

**A witness, a circle (2026-11-03, `onedoor/crossing-doors`'s full `geom-brep` run, a branch that does not touch `extent.rs`):** a circle of radius 0.001 about the origin, axis `(0.95227, 0.30514, −0.00800)`, over the reversed span `[−1.99326, −1.99974]`, read from its centre: `span_reach_from` gives `0.0010000000000000002`, past the whole-turn lever. Reproduce with `CAD_FUZZ_SEED=0x2f5a851b03a20299 CAD_FUZZ_EFFORT=1`. So the shape is not an ellipse's alone: a span shorter than a hundredth of a turn passes it by one ulp.

**A witness, an ellipse of equal semi-axes (ONECOPY frame-readings lane, PR 6463).** Seed `CAD_FUZZ_SEED=0x8b7ff4d7a91c64d1 CAD_FUZZ_EFFORT=1`: an ellipse with equal semi-axes `1000` over `[8.5169, 17.5946]` (more than a turn), read from its own centre. `span_reach_from` gives `1000.0000000000002`, one ulp past `lever_from`'s `1000`. It reproduces on main with the same numbers, so the defect is not a thin ellipse's alone. Over a turn or more the two covers read the whole conic from different window starts, and their rounding differs by an ulp. Once fixed, pin this seed as a deterministic row.

**Folded in from SCALAR's `a-unit-ellipse-arcs-span-reach-passes-its-whole-turn-by-an-ulp` (2026-11-04, `onecopy/foot-certificate`, PR 6483's review).** That row read the same third assertion of the same test, so it is deleted and its evidence lives here:
- Its first witness (ONEDOOR's wedge-fold lane, `cargo nextest run -p geom-brep --all-features`): a unit circle (an ellipse, `semi_u = semi_v = 1`) about the origin, axis `(-0.03969, 0.75295, 0.65688)`, over `[3.660488584391903, 3.6604896604454873]` (about 1e-6 wide), read from its centre. `span_reach_from` gives `1.0000000000000002` against `lever_from`'s `1.0`. Reproduce with `CAD_FUZZ_SEED=0xc543ab8c865d550e CAD_FUZZ_EFFORT=1`. That row noted that the test's other two assertions carry a `16·ε·scale` tolerance and this one carries none.
- A circle of radius 1000 (`semi_u = semi_v = 1000`), over `[-9.77, 0.935]`, more than a turn, read from its centre: `1000.0000000000002`. `CAD_FUZZ_SEED=0x638e2db0bd762c87`.
- An ellipse `semi_u = 1000`, `semi_v = 44.71`, over `[9.6126, 12.3840]`, under half a turn: `1000.0000000000001` passes the whole turn. `CAD_FUZZ_SEED=0x13cb24d6ae40c408`, reproduced here.
- `CAD_FUZZ_SEED=0xb00ab0bb194042ff` (PR 6483's fix-pass `geom-brep` run): `1.0000000000000002` passes the whole turn.
- `CAD_FUZZ_SEED=0x1d0f25bef65ecd3b` (the same run's rerun on `5a524e418`): `1000.0000000000001` passes the whole turn.

**A second seed** (ONECOPY `onecopy/angle-reads`, 2026-11-03), found by a
local full geom-brep run and reproducible with `CAD_FUZZ_SEED=0xd54786f7cb0d235c
CAD_FUZZ_EFFORT=1` (`--features test-support`): a thin ellipse
(`semi_u` 0.0014142132088478148, `semi_v` 1.0; centre the origin, axis
`(-0.5572965940438884, 0.28438696616547604, 0.7800926609975752)`,
`u_ref` `(0.6166163900257462, -0.48744344408386014, 0.6182096055313937)`)
over `[6.778616771278262, 18.0108665539107]`, from the origin, fails the
same third assertion. Neither side of the comparison reads anything that
branch changed.

## Another seed (2026-11-09, `onecopy/typed-reasons`)

Red on `onecopy/typed-reasons` at main `0c540ab8` plus a diff that touches
neither `extent` nor `cover`: `span_reach_hunt`'s
`a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`
(`span_reach_hunt.rs:133`, "1 passes the whole turn"). The unit
circle again (an `Ellipse` with `semi_u = semi_v = 1`), over
`[3.051601505932741, 9.97795873936163]` (past a whole turn), pivot its
centre `(0.8639433538009402, 0.47549612493460675, 0.1658472688812101)`.

```
CAD_FUZZ_SEED=0x2c1c182ae01fe745 CAD_FUZZ_EFFORT=1 \
  cargo test -p geom-brep --all-features --test all -- \
  span_reach_hunt::a_conic_arcs_span_reach_is_never_short_of_its_farthest_point
```
