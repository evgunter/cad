---
id: a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin
kind: issue
title: tier 3's volume-sign check on a rational swept wall refuses QuadratureBudget or decides by the section's knot structure, the skin's degree and the body's position — not by the shape
status: open
opened: 2026-10-02
priority: P2
cost: H
---

Found by SHOW's `klein-scene-should-adopt-the-one-body-loop-sweep` unit
(2026-10-02) and widened by its review. The Klein bottle's loop now
ships as one `sweep_body` at a setting where tier 3 decides
(`demos/tour/src/klein.rs`, `annulus` / `STATIONS` / `V_DEGREE`); this
row is what decides it.

## The body

An annulus `R ± WALL/2` = 0.275 / 0.225 m swept by `sweep::sweep_body`
along a degree-3 interpolant through 49 exact points of two tangent
arcs of radius 1.2 m (270° then 90°, world xz-plane), from
`path_start_frame` at (0, 0, 3) — a metre-scale tube a few metres from
the origin. `topo::validate_geometric` at `Tol::witness()` (ε = 1e-9,
target 1.024e-6):

| section | stations | v-deg 2 | v-deg 3 |
|---|---|---|---|
| `circle` (2 arcs/wall) | 9 | width 7.48e-6 | width 1.30e-5 |
| `circle` | 17 | 6.54e-6 | 1.12e-5 |
| `circle` | 33 | 5.47e-6 | 1.04e-5 |
| `circle` | 65 | 4.83e-6 | 9.98e-6 |
| `circle_split` 4 | 9 | decides | **refuses**, width 5.61e-6 |
| `circle_split` 4 | 13–65 | decides | decides |
| `circle_split` 8, 16 | 9–65 | decides | decides |

Every refusal is
`VolumeUncomputable { source: Face { source: QuadratureBudget { width_len, target_len: 1.024e-6, rounds: 1 } } }`
on a lateral wall — after ONE round (`quadrature-budget-conflates-its-lanes-and-budgets`
names that payload as the only tell of which lane refused). The width
does not move with ε (the same 1.0381e-5 at 1e-12 against a 1.024e-9
target); at ε = 1e-6 every cell decides.

So the same tube's sign decides or not by three things that are not
its shape:
- **the section's knot structure** — two semicircles per wall refuse
  everywhere, four quarter arcs decide almost everywhere;
- **the skin's degree** — v-degree 3 roughly doubles the `circle`
  width, and turns the one quartered refusal on at 9 stations;
- **the body's position** — the `circle` loop translated by
  (−1.2, 0, −1.8), so its arcs straddle the origin, decides at 33 and
  65 stations (v-degree 3), and still refuses at 17 (width 7.7e-6).
  This is the position dependence the 2026-09-04 comment on
  `export/rational-patch-flux-quadrature-budget` measured on the
  reporting lane, here on the sign lane at scale 1.

## A second refusal on the same bodies, at the reporting level

The quartered loop at 17 and 21 stations, v-degree 3, passes tier 3 at
ε = 1e-6 and then its `SignCertificate::measure` continuation refuses
with NO bracket (`TargetUnreached { bracket: None, .. }`) — a refusal
that is not the budget one, so the tour's harness has no bracket to
report and panics. At 1e-9 and 1e-12 the same bodies hand back a
bracket. Untraced.

## Relatives

- `quadrature-interval-floor-grows-with-the-body-past-the-band`: a floor
  that grows with the body's scale; this is at scale 1.
- `tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`
  and `tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`:
  the mesher's half of the same bodies.

## Done when

The `circle` rows decide at ε = 1e-9 at their own position (the loop
could then be spelled with `circle` once the tess crease row closes),
and the quartered 17/21-station v-degree-3 bodies hand back a bracket
at 1e-6 — or each remaining refusal is shown to be the correct answer
for that wall, with its mechanism named.
