---
id: sweep-frame-is-a-minimal-rotation-from-the-start-tangent
kind: issue
title: sweep_places carries each station by the minimal rotation from the START tangent, so the section spins about its tangent near anti-parallel
status: open
opened: 2026-10-06
priority: P1
cost: H
design: true
refs: [a-half-turn-spine-sweeps-only-off-its-exact-tangents]
---


## Finding

`sweep::skin::sweep_places` (`crates/sweep/src/skin.rs`) places station
`i` by `T · Rᵢ · place`, where `Rᵢ` is the MINIMAL rotation carrying
the START tangent `t₀` to `tᵢ`. That is not a rotation-minimizing
(parallel-transport) frame: as `tᵢ` nears anti-parallel to `t₀`, the
minimal rotation's axis swings fast, and the section spins about its
own tangent.

**Measured** on the tour's square coil (`demos/tour/src/projectbox.rs`,
6 turns, 32 stations a turn, on `carve/loft-v-is-the-whole-sets`,
2026-10-06):

- The centre advances 0.02756 at every station.
- At stations 15–18 of every turn, a corner's step goes 0.0214 → 0.0607
  while `tangent·t₀` reaches −0.98.
- Chord-length v absorbs the spin: it deviates from uniform by 6.1e-3,
  more than a station step (5.2e-3).
- Under the path parameter (uniform v), the cubic through the corner
  rows overshoots, and two seam isos' `speed_lower_bound` reach −0.055
  and −0.016. They read +1.7 under chord v. The body then refuses at
  assembly: `Escalated { check: ParamSpan, predicate: "nurbs_span_meter" }`.

This is the same root as
`work/carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents.md`
(the documented C6 anti-parallel knife edge): the frame law measures
everything from `t₀`.

## What waits on it

The sweep's v. Both designers of
`loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`
preferred the path parameter for a sweep (the stations ARE samples of the
path at known parameters, and no spelling or roll enters it). It was built,
and it exposed this spin. Sweeps take the loft's chord-length rule until
the frame is right. Once it is, the sweep's v should move to the path
parameter.

## Shape

An incremental frame (each station carried from the previous one by the
minimal rotation between consecutive tangents, i.e. a discrete
rotation-minimizing frame), or a stated frame law the author can pick. It
is a design question (which frame a sweep follows), so it is
`design: true`.
