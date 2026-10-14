---
id: no-row-checks-the-ellipse-torus-door-certifies-at-eps-1e-12
kind: issue
title: At eps 1e-12 no row checks that the ellipse x torus door still certifies: the crossing row accepts Uncertain on every pose there
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [3973]
---

Filed by the REACH orchestrator from the independent verifier's report
on PR 3973 (`analysis/reach-verify/3973`, `verify.md`, note 2 and
claim 6), which landed the ellipse × torus door
(`ellipse-edge-crossing-a-torus-has-no-root-lane`). Both points are
measured at that PR's head `f6f69915e4`.

- **At ε 1e-12 the door's liveness is pinned by no row.** The crossing
  row `torus_crossings_match_the_true_distance`
  (`crates/topo/src/boolean/ellipse_torus.rs`) asks for certificates
  only where the band resolves them. At ε 1e-12 every one of its poses
  has `100·noise/slope` ≈ 4.6e-10 m, wider than the band, so it accepts
  `Uncertain` on every pose there. The verifier found that a door that
  always answers `Uncertain` keeps the row green at 1e-12, and turns it
  red at 1e-9 and 1e-6. The fuzz and slack rows accept or require
  `Uncertain` too. So at 1e-12 the rows check safety only. The fix is a
  pose the 1e-12 band does resolve (a steeper crossing, or a smaller
  torus), with the row asking for its certificate.
- **The fuzz pose builds ellipses with minor > major**
  (`ellipse_torus.rs`, `torus_rows::pose`). The fuzz `pose` swaps the semi-axes half
  the time and builds `Curve3::Ellipse` literals that
  `Curve3::ellipse` would reject as `AxesSwapped`. Reviewer r1 raised
  it as a style point and the fix pass left it. No lane claim covers
  it. Build the pose through `Curve3::ellipse`, or order the axes.
