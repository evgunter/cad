---
id: a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin
kind: issue
title: tier 3's volume-sign check on a rational swept wall refuses QuadratureBudget or decides by the section's knot structure, the skin's degree and the body's position — not by the shape
status: open
opened: 2026-10-02
priority: P1
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

## A long-turn helix refuses at every ε, quartered or not

SHOW's `long-turn-helix-has-no-demo` (2026-10-03) met the same refusal
on a round-wire spring: `circle_split(.., 4, ..)` of radius 0.025 m
swept by `sweep_body` (32 stations a turn, v-degree 3) along a
degree-3 interpolant of a helix of radius 0.14 m and pitch 0.08 m,
from `path_start_frame`. `topo::mass_properties` refuses
`Face { source: QuadratureBudget { rounds: 1, .. } }` on a lateral wall
— one face spans the whole coil — with widths that grow with the turns
(coil about the world z axis from the origin):

| turns | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| width (m) | 5.21e-6 | 2.56e-5 | 6.22e-5 | 1.13e-4 |

At the scene's placement (axis through (0.625, 0.625), base z ≈ 0.9)
and six turns the width is **1.68e-3 m at ε = 1e-6, 1e-9 and 1e-12
alike**, past even 1e-6's 1.024e-3 target — 7 % of the wire's radius,
on a body whose square-wire twin (same spine, stations and placement)
certifies its volume with a pad of 1e-14. The refusal is also slow:
`mass_properties` takes 10.9 s to refuse (the round coil builds in
2.6 s), so the live wall costs the tour about 13.4 s a run, most of
the 14 s the spring adds (measured by the PR's review). So on a long
spine the
quartered section stops being the remedy, and no ε row decides. Pinned
live as `projectbox` wall 1 (`demos/tour/src/projectbox.rs`,
`standing_spring`); the scene ships the square wire.

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

## Re-measured on main 258dc6c7c2 (QUAD, 2026-10-21)

**Verdict: the position column is gone (#5442, FLUX's
`shell_reference`). The knot-structure and degree findings stand.**

The recipe as written no longer builds. `sweep_body` on this spine
refuses `Skin(PathTangentReversal { station: 8 })`: the half-turn spine
of `work/carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents`,
the tour's wall 9. I measured its twin, the way `demos/tour/src/klein.rs`
builds the loop today (`lofted_loop`). That twin has the same spine
(degree-3 interpolant through the 49 arc points), the same annulus
0.275/0.225, and stations in planes normal to the spine, rolled by
`path_start_frame`. It is lofted at v-degree 2 or 3, and judged by
`topo::validate_geometric` at the default ε.

Probe: `crates/sweep/tests/quad_frame_remeasure.rs` `row3_swept_loop`.
Its env vars `ROW3_SPLITS` / `ROW3_STATIONS` / `ROW3_DEGS` / `ROW3_OFFS`
pick the grid. Every refusal is `QuadratureBudget { rounds: 1 }` on a
wall, and the cell gives its `width_len`.

| section | stations | v-deg 2 | v-deg 3 |
|---|---|---|---|
| `circle` | 9 | 8.97e-6 | 1.74e-5 |
| `circle` | 13 | 8.56e-6 | 1.54e-5 |
| `circle` | 17 | 7.90e-6 | 1.51e-5 |
| `circle` | 21 | 7.57e-6 | 1.44e-5 |
| `circle` | 33 | 6.62e-6 | 1.38e-5 |
| `circle` | 65 | 5.86e-6 | 1.26e-5 |
| `circle_split` 4 | 9 | decides | **refuses**, 6.41e-6 |
| `circle_split` 4 | 13–65 | decides | decides |
| `circle_split` 8 | 9 | decides | **refuses**, 2.14e-6 |
| `circle_split` 8 | 13–65 | decides | decides |

**Position.** Each of the 36 cells reads the same to four digits at all four
placements: the origin; (−1.2, 0, −1.8), which puts the arcs across
the origin; (100, 0, 0) m; and (1000, −700, 400) m.

On main before #5442 (`ec32861499`), the same probe at v-degree 3 still
showed this row's dependence:

| section | stations | origin | (−1.2, 0, −1.8) | (100, 0, 0) |
|---|---|---|---|---|
| `circle` | 9 | 1.44e-5 | 1.05e-5 | 4.09e-5 |
| `circle` | 17 | 1.27e-5 | 8.12e-6 | 4.44e-5 |
| `circle` | 33 | 1.21e-5 | decides | 4.92e-5 |
| `circle_split` 4 | 9 | 6.64e-6 | 5.09e-6 | 2.67e-5 |

**A cost of the fix.** The new widths are the old ones' worst case
near the body, not their best. At the origin, `circle` v-degree 3 went
from 1.44e-5 to 1.74e-5 (9 stations) and from 1.21e-5 to 1.38e-5 (33
stations). The one cell that decided only across the origin (33
stations) now refuses everywhere. The reference is the start of the
shell's first quadrature face, not a point central to the body, so
each wall's lever is up to the loop's size.
`work/flux/a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first.md`
already tracks the choice of reference point.

**ε.** At 1e-12 the widths are the same as at 1e-9: `circle` 9/17/21
stations at v-degree 2 and 3, and `circle_split` 4 at 9/17/21 stations,
including the 9-station v-degree-3 refusal. At 1e-6 every one of those
cells decides. This row's "does not move with ε" stands. I did not
re-take the 1e-6 `SignCertificate::measure` no-bracket refusal or the
helix table.

So the loft's tier-3 sign still decides by the section's knot structure
(two semicircles refuse at every station count) and by the skin's
degree (v-degree 3 doubles the `circle` width and turns on the 9-station
quartered refusal). It no longer decides by the body's position.
