---
id: ellipse-torus-graze-certifies-six-roots-where-the-true-distance-crosses-four
kind: issue
title: the ellipse x torus arm certifies 6 roots on a graze where the true distance crosses 4 times (fuzz seed 0xb471af1930331750, eps 1e-9, graze 91)
status: open
opened: 2026-10-08
priority: P1
cost: M
---


## Found (ENCL, CI on `encl/offset-cert-coefficient-norms` head `ac85a6e694`, 2026-10-08)

`topo`'s fuzz row
`boolean::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`
(`crates/topo/src/boolean/ellipse_torus.rs`, the root-count
`assert_eq!` in that row) fails on one seed. At ε 1e-9, graze case 91,
the arm certified **6** roots where the oracle's true distance crosses
zero **4** times:

```text
Ellipse { center: (0, 0, 0), axis: (-0.9060351451308201, 0.41952342551827987, -0.05568133645291127),
          major: 1.5931244512625413, minor: 0.14337685431398062,
          u_ref: (-0.3270058756869016, -0.777520351402234, -0.5371491975434657) }
against
Torus { center: (1.3091196645498262, 0.38625237213165453, 0.05725099872995548),
        axis: (0.00546116755272265, -0.396058940744004, 0.9182088493941336),
        major_radius: 1.3929963544521364, minor_radius: 0.3378051545072215,
        u_ref: (0.9999823133111052, 0.0, -0.0059475259537680875) }
on [4.282277677218393, 10.2154263890171]
oracle crossings: [5.069940181296204, 7.565772021816139, 8.086537994185644, 9.791087935253973]
```

Deterministic replay, any eps row (the row walks its own ε ladder):

```text
CAD_FUZZ_SEED=0xb471af1930331750 CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo --lib -E 'test(=boolean::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance)'
```

**Reproduces on `main` at `04a7aea1e5`**, so it is not that branch's
change, which does not reach this arm. Not diagnosed.

## What is open

Either the arm certifies a wrong answer (two extra roots, the P1
reading) or the oracle misses a near-tangent pair of crossings on a
graze. Decide which. Per implementer-discipline §8, pin this
counterexample as an ordinary deterministic row (the ellipse, the
torus and the window above) beside whichever fix it gets.
