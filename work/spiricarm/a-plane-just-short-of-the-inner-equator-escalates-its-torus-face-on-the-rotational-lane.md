---
id: a-plane-just-short-of-the-inner-equator-escalates-its-torus-face-on-the-rotational-lane
kind: issue
title: The plane 1e-3 short of the donut's inner equator: T ∪ P escalates props_chart_area_side on its torus face, the pinched spirics' terms reading ±1e3 wide past round 0
status: open
opened: 2026-11-11
priority: P1
cost: M
---


Found by the rotational lane's stage 3 (`spiricarm/rotational-lane-torus`), PR 6808,
which moved the spiric wall onto the lane: a regression against main, in
the slow set (nightly only).

## Measured

`crates/sweep/tests/germ_plane_torus_spiric.rs`
`a_plane_near_the_inner_equator_builds_its_true_bodies`, pose
`(2, 0.5), x ≥ 1.499`, `T ∪ P` at the default ε:

```
ResultInvalid { NotFinished { VolumeUncomputable { face: FaceKey(5v1), source: Escalated {
  margin: Enclosure { lo: -92.98, hi: 98.08 }, predicate: "props_chart_area_side" } } } }
```

A probe on the lane (since removed) printed the face's loops: two loops of
spiric, meridian, spiric, meridian (`"shsh"`), every lift turn `0`. At
round 0 their `[−∮F du, −∮G du]` read `29.8058 ± 5e-5` and
`16.5721 ± 3e-5`. The next round reads `[−1012, 938]` and `[−1161, 1225]`:
the pinched spirics' own terms (`spiric::wall_terms`), whose
`1/(ρ·f)` jets blow up on a piece whose `f` reaches the pinch. The
area decision at that round straddles and escalates. On main the spiric
lane read the same integrand; that it accepted before that round is the
unconfirmed reading.

## Done when

The row builds again. Either the spiric's rule stays no wider at a finer
round than at a coarser one (it is sound at both, but not monotone
here), or the lane keeps the narrowest round it read, as `Run` keeps the
narrowest bounds, for the ring and area decisions too.
