---
id: a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin
kind: issue
title: a sweep_body loop of circle sections refuses tier 3's volume sign 3 m from the origin and decides it when moved to the origin
status: open
opened: 2026-10-02
---

Found by SHOW's `klein-scene-should-adopt-the-one-body-loop-sweep` unit
(2026-10-02), trying to replace the Klein bottle's two `revolve`
elbows with ONE `sweep_body` of the annular section along the loop's
whole U-turn spine. Pinned live as the bottle's wall 5
(`demos/tour/src/klein.rs`, `wall_probes`, built by `one_body_loop`
with `annulus(false, ..)`).

## What

The body: two `circle` walls (r = 0.275 and 0.225 m) swept by
`sweep::sweep_body` (33 stations, v-degree 3) along a degree-3
interpolant through 49 exact points of two tangent arcs of radius
1.2 m (270° then 90°), in the world xz-plane, starting at (0, 0, 3)
from `path_start_frame`. Tier 1 and the closed census pass.
`topo::validate_geometric` at `Tol::witness()` refuses:

```
[VolumeUncomputable { solid: SolidKey(1v1), source: Face { face: FaceKey(3v3),
  source: QuadratureBudget { width_len: 1.0381448642581538e-5,
                             target_len: 1.024e-6, rounds: 1 } } }]
```

Measured (each at 17, 33 and 65 stations, world-axis placement and
`path_start_frame` placement alike):
- as authored: refuses at every station count, width 1.0e-5 to
  1.3e-5 against the 1.024e-6 target;
- the same spine translated by (−1.2, 0, −1.8), so its two arc
  centres straddle the origin: 33 and 65 stations DECIDE the sign
  (`validate_geometric_certificate` returns, bracket e.g.
  [0.061, 1.129] m³ against the Pappus value 0.592 m³); 17 stations
  still refuses (width 7.7e-6).

The width does not move with ε: at `CAD_TOLERANCE_EPS=1e-12` the same
face refuses with the same `width_len` (1.0381448642581538e-5) against
a 1.024e-9 target, and at 1e-6 (target 1.024e-3) the sign decides. The
wall is therefore pinned at the default ε and finer only.

So at scale 1 and a few metres from the origin, the tier-3 sign
check on a rational swept wall turns on WHERE the body sits. The
`rounds: 1` payload says it stopped after one round
(`quadrature-budget-conflates-its-lanes-and-budgets` names that
payload as the only tell of which lane refused).

## Relatives

- `export/rational-patch-flux-quadrature-budget`, its 2026-09-04
  comment: an arc-profile loft green at the origin and red at (4, −2),
  authored there with no transform — the same position dependence,
  measured on the reporting lane.
- `quadrature-interval-floor-grows-with-the-body-past-the-band`: a
  floor that grows with the body's scale; this one is at scale 1.
- `tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`:
  the same body is ALSO unmeshable (the semicircle walls' C0 crease),
  and its quarter-arc respelling passes tier 3 but meets
  `tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`.

## Done when

The bottle's wall 5 stops refusing: the one-body loop's tier-3 sign
decides at its own position.
