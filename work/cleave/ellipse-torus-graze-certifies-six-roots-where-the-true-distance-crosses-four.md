---
id: ellipse-torus-graze-certifies-six-roots-where-the-true-distance-crosses-four
kind: issue
title: the ellipse x torus arm certifies 6 roots on a graze where the true distance crosses 4 times (fuzz seed 0xb471af1930331750, eps 1e-9, graze 91)
status: closed
opened: 2026-10-08
priority: P1
cost: M
branch: cleave/ellipse-torus-graze
closed: 2026-10-10
pr: 4552
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

## Built (branch `cleave/ellipse-torus-graze`)

**The arm is right; the test's oracle was wrong.** Measured on `main`
at `98a3817a1d`: the replay fails at every `CAD_TOLERANCE_EPS` row
(default, 1e-6, 1e-12; the row walks its own ε ladder), certified 6 of
4. A 50-digit mpmath evaluation of the stored pose finds SIX true
crossings: the four the oracle read and a graze pair at
`10.389850001893556` and `10.390414854663097`, 5.6e-4 rad apart about a
bump 3.5e-8 m high. The turn about the arc's midpoint ends at
`10.390444686707540`, 3e-5 rad past the pair, so the whole pair sat in
the oracle's LAST 8000-sample cell, where its extremum scan
(`for k in 1..n − 1`) never looks: neither end sample has an outer
neighbour. The door's six roots lie within 1e-12 m of arc of the mpmath
six. The repo's mpmath script (`scripts/oracles/ellipse_torus_mpmath.py`)
had the same blind spot and read 4 on this pose too.

- `torus_rows::certified_torus_answers_hold_against_the_true_distance`
  reads the exact oracle (`conic_oracle::exact::truth`, double-double,
  whole turn) in place of its own `f64` sampler; with it every
  certified root's place is checked (the "unplaced" escape is gone).
  Six random seeds at `CAD_FUZZ_EFFORT=10` (about 86,000 poses) pass.
- `conic_oracle::crossings` and `extremes` (the shared sampler) search
  the first and last cells like the rest, pinned in closed form by
  `conic_oracle::the_end_cells_are_searched`.
- The mpmath script searches its end cells too; it reads 6 on this pose.
- `conic_quadric::ellipse_rows::fuzz_rows` reads the exact oracle for
  its count (now equality, was a floor) and its `Miss` arm, which a
  sampler could pass on a graze pair inside one cell.
- Pinned: `torus_rows::a_graze_pair_at_the_seam_is_two_crossings`, the
  drawn pose with its arc and seven others that put the seam before,
  inside and past the pair, at ε 1e-6, 1e-9 and 1e-12, against the
  mpmath roots.

Filed: `work/cleave/ray-torus-probe-oracles-cannot-see-a-graze-pair-inside-one-sample-cell.md`.

## Closed (PR 4552, 2026-10-10)

The certificate was sound: an independent 50-digit evaluation finds 6 true crossings. The test's
sampled oracle never searched its first and last cells, and the graze pair sat in the last one. The
torus and ellipse fuzz rows now read `conic_oracle::exact::truth`, the shared sampler and the mpmath
script search the end cells, and the seam-graze pose is pinned across arcs. The ray × torus probe
oracles' blind spot is filed (P3).
