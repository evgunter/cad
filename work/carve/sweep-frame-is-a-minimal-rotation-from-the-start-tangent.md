---
id: sweep-frame-is-a-minimal-rotation-from-the-start-tangent
kind: issue
title: sweep_places carries each station by the minimal rotation from the START tangent, so the section spins about its tangent near anti-parallel
status: closed
opened: 2026-10-06
priority: P1
cost: H
refs: [a-half-turn-spine-sweeps-only-off-its-exact-tangents]
closed: 2026-10-31
pr: 6138
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

## The spin's cost in volume (ISO, 2026-10-24)

The coil skins fall short of the ideal tube's Pappus volume `A·L` by
far more than a cubic fit would explain. Each skin's own volume was
integrated over its surfaces (`demos/tour/src/patch_quadrature.rs`). The square wire falls short by 3.6e-3 of `A·L` and
the round by 4.4e-3, at 32 stations a turn. A cubic spline through
stations `Δ = 2π/32` apart misses by about `2·(5/384)·Δ⁴ = 3.9e-5`, so
the coil's deficit is about 100× that. The lily's swept leaves never
turn far from `t₀` (curl 0.35–0.40 rad), and they meet that scale:
their skins are 9.9e-8 and 8.0e-8 off Pappus, against 1.6e-7 and
9.5e-8. `projectbox.rs`'s `COIL_GAP` (5e-3) admits the coil's deficit
as a measured band. It should narrow toward the fit's scale once the
frame law stops spinning the section.

## A P0 waits on this (2026-10-25)

`work/carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents.md`
is now P0, and its first ask is this row's frame. Any planar U-turn spine
refuses `PathTangentReversal` at its half-turn station, so the klein
scene lofts its loop as a workaround (wall 9). Linked on request in
chat. Whichever row lands the frame should close both, and the design
question here is what the P0 waits on.

## Weighed (2026-10-31): a rotation-minimizing frame by double reflection

Two designers (an Opus and a Fable lane) were given the problem and no
options. They converged in round one, so nothing went to Ev. The
blinding is on `analysis/design-fork/carve-sweep-frame`.

- **The law.** The kernel carries the author's start frame along the
  path by parallel transport, the rotation-minimizing frame: zero twist
  about the tangent. It is discretized station to station by double
  reflection (Wang, Jüttler, Zheng and Liu 2008). The rotation method
  (`turned_between` chained) was also weighed and rejected: it is only
  second order, about 10⁵ times worse at the coil's 32 stations a turn
  (B's measurement). It is the one kernel-owned law left: Frenet is
  undefined on straight runs, and a fixed reference direction reads the
  world axes, unless the user declares it, which the D10 hold forbids.
  The author owns only the start placement, as today.
- **Today's law is singular at one direction.** On a planar path it
  already equals the rotation-minimizing frame, except at −t₀. So the
  half-turn P0 (`a-half-turn-spine-sweeps-only-off-its-exact-tangents`,
  moved here to ride with this row) is the law's singular point, and
  the coil's spin is its non-planar error. One cause, so both close
  together.
- **The pivot.** `sweep_places` turns station i about the section
  plane's origin o, not about the path's start point p₀. The two
  placements differ by (I − R)(o − p₀), so the body depends on where the
  sketch origin sits. The fix pivots at p₀, and lands with this row.
- **Refusals.** `PathTangentReversal`'s global "doubles back on the start
  tangent" retires. The local refusal that replaces it is "the path turns
  about a half turn between two stations", whose recourse is to add
  stations. Coincident consecutive stations stay a refusal.
- **v becomes the path parameter**, as PR 4193's designers asked. The
  stations no longer spin, so chord length has nothing to absorb.
- **Scalar lanes.** The carry is a function of the f64 path and the
  station count alone. It is computed once in f64, and every lane
  composes it onto the author's start plane. Station 0's motion is the
  exact identity.
- **Consequences to show Ev.** A helix sweep rolls its section relative
  to the helix axis at 2π·sin α a turn, which is the physics of a rod bent
  without twist: 0.57 rad a turn, about 196° over the tour coil's 6 turns.
  The tour coil will visibly roll its square wire. A spring whose wire
  keeps a face toward its axis wants a frame held to a direction, which
  is placement vocabulary and waits on the D10 hold. On a planar closed
  path the frame closes exactly. A non-planar closed path generally
  does not close, and no closed sweep exists today (refuse, or spread
  the twist: the designers differ, and that waits for the ring-sweep
  work).

## Closed (2026-10-31, PR 6138)

The sweep carries its start frame by the rotation-minimizing frame, built by double reflection (`PathCarry`), and pivots at the path's start point. Its v is the path parameter. The law is written as `crates/sweep/README.md` `[sweep.frame]`. Three rows pin it: Wang agreement on a helix (≤1e-12), the sweep not depending on where the sketch origin sits, and a helix rolling by its torsion.

- **Refusals.** `PathTangentReversal` is gone. The sweep refuses `PathFoldsBack` where the arrival tangent mirrors the departure tangent in the chord's normal plane; its recourse is to smooth the kink or add stations. It refuses `PathStationsCoincide` where two stations coincide. Each has an in-band twin, `PathUndecided`, reachable from a real path.
- **Measured.**
  - End roll on the coil helix: 9.2e-7 / 5.7e-8 / 3.5e-9 rad at 16/32/64 stations a turn, fourth order.
  - The coil's Pappus gap fell from 3.6e-3 to 4.1e-6, so `COIL_GAP` goes from 5e-3 to 6e-6. `m8_14`'s θ_slab²/4 gap was the old frame's spin; it now reads about 3e-6.
- **Cost, stated in the README.** A rational path's parameter is not arc length, so the swept elbow fits about 2.7× worse than under chord-length v. Its STEP fixture, kernel sidecar and two `m7` caps are re-baselined with that cause.
- **Render.** The tour coil rolls its wire by 196°, and the klein loop sweeps its exact spine. `twisted_duct` rolls less, because the old spin is gone.
- **Waits on CLEAR.** `a_long_helical_sweep_waits_on_the_window_doors_halving` pins a refusal that `work/clear/clearance-refines-reads-only-the-lower-halves.md` fixes; that row's experiment certifies the sweep.
- **Also closes** INLANE's `a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps`.

Review: single FULL, verdict NOT-MERGEABLE on one MAJOR. Six slow rows the change reaches were red and unrun, because the PR gate runs only the slow rows whose names the diff touches. The fix pass re-baselined them with their causes, renamed the fold-back refusal, gave the cusp its recourse, and added the reviewer's probes as rows. The orchestrator read the re-baselines: each cap is the measured value plus a tenth, and none was loosened beyond its measurement.
