---
id: a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin
kind: issue
title: tier 3's volume-sign check on a rational swept wall refuses QuadratureBudget or decides by the section's knot structure, the skin's degree and the body's position — not by the shape
status: closed
opened: 2026-10-02
closed: 2026-10-27
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
`a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first`, closed by PR 5699
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

## 2026-10-21 — the sign lane's `rounds: 1` refusals were the budget exit (PR 5568)

Every tier-3 refusal in the first table carried `rounds: 1`. That was
the lanes' after-round-0 budget exit, answering the reporting question
inside the sign walk: `check-7-refuses-the-reporting-budget-on-a-definite-sign`.
PR 5568 lets the sign walk refine past it. The Klein bottle's
`circle` loop (13 stations, v-degree 2, at the scene's position) now
passes tier 3 at ε = 1e-6, 1e-9 and 1e-12, so wall 5 is retired and
the scene's loop is spelled with `circle` (findings entry 5 rewritten).
The tess crease row it waited on had already closed (#5499).

What this does not touch is the REPORTING half, which still stands:
the widths themselves, and their dependence on knot structure, degree
and position; the quartered bodies' bracket-less continuation at 1e-6;
and the helix (`projectbox` wall 1), which refuses at
`mass_properties`, a number caller.

## 2026-10-24 — the helix certifies at the default ε (PR 5629)

`projectbox` wall 1, the six-turn round-wire spring, now decides its
volume at the default ε. Main went red on it at `debb2a604e`: the tour
panicked because the wall "NO LONGER REFUSES". The change is PR 5629
(`67006526`, ISO's `iso/loft-seam-per-row-lane`). A probe on the
scene's body refuses at its first parent `72a76e0a` (`QuadratureBudget
{ width_len: 7.84e-5, target_len: 1.024e-6, rounds: 7 }`) and decides
at `67006526`. PR 5630 (`3b7dd866`, arc-rim pcurves) changes nothing
here: the same number before and after it.

**The number is sound.** An independent oracle gives the body's
volume as 0.010360121170117. It is the divergence integral over the
four wall patches, using its own Cox–de Boor rational evaluation and
Gauss–Legendre quadrature per knot span, plus the two planar caps. It
converges to 1e-16 by 16 points a span, and it reads the same about
two reference points, so the shell closes. The oracle is the same at
both commits: the body did not change, its measurement did.

| ε | kernel | holds the oracle |
|---|---|---|
| 1e-6 | 0.010359821 ± 4.87e-5 | yes (off by 3.0e-7) |
| 1e-9 (default) | 0.010360120 ± 1.90e-7 | yes (off by 1.2e-9) |
| 1e-12 | refuses `QuadratureBudget { width_len: 2.74e-8, target_len: 1.024e-9, rounds: 7 }` | — |

Neither bracket holds `coil_volume(π·WIRE_H²)` = 0.010405854, the
continuum tube's volume. That value is not this body's: the body is a
stack of skinned slabs, 4.40e-3 short of it, as the square wire is
3.6e-3 short. The 1e-6 retirement (PR 5548) asserted that the bracket
holds the closed form. That held only because the 1e-6 pad, 4.87e-5,
happened to exceed the 4.6e-5 slab gap. The retired wall now asserts
the slab gap alone, inside `COIL_GAP`, the way the square wire's does.

Wall 1 is retired, kept live, at the default ε and looser, and stays
pinned finer than the default. The scene still ships the square wire,
because at 1e-12 the round wire still refuses. So this row's helix
witness now stands at ε < 1e-9 only. The rest of the REPORTING half is
untouched: the loft widths' dependence on knot structure and degree,
and the quartered bodies' bracket-less continuation at 1e-6.

## Closed — 2026-10-27: every cell decides, at every ε and both placements

Re-measured on main `aaf9aa88`, after the measurement door's three
units (PRs 5589, 5628, 5741). The probe is
`crates/sweep/tests/quad_frame_remeasure.rs` `row3_swept_loop`, now
through `validate_geometric_certificate`, whose `SignCertificate` is
continued by `measure` at `pinned_resolution` (1024·ε). Its
`ROW3_MEASURE=0` skips the continuation. The grid is the lofted twin
of 2026-10-21: `circle` and `circle_split` 4/8/16, stations
9/13/17/21/33/65, v-degree 2 and 3, at the body's own position and
moved by (−1.2, 0, −1.8). That is 96 cells per ε.

**The sign.** `validate_geometric` decides all 288 cells (96 at each of
1e-6, 1e-9 and 1e-12). Each one settles at round 0. Each cell's
round-0 width is the same at all three ε, and the moved body reads the
same, to every printed digit, as the body at its own position. The
widths below are 0.05–23 % of the 0.59 m³ volume.

**The number at 1e-9.** Each cell's continuation reaches
`Stop::Met` at 1.024e-6. Cells read as round-0 width / continuation
pad (m³), and the moved body is the same:

| section | stations | v-deg 2 | v-deg 3 |
|---|---|---|---|
| `circle` | 9 | 1.20e-01 / 3.7e-06 | 1.38e-01 / 4.2e-06 |
| `circle` | 13 | 1.23e-01 / 3.8e-06 | 1.20e-01 / 3.7e-06 |
| `circle` | 17 | 1.08e-01 / 7.3e-06 | 1.18e-01 / 3.6e-06 |
| `circle` | 21 | 1.05e-01 / 7.0e-06 | 1.06e-01 / 7.1e-06 |
| `circle` | 33 | 9.35e-02 / 6.2e-06 | 9.54e-02 / 6.4e-06 |
| `circle` | 65 | 8.55e-02 / 1.0e-05 | 8.57e-02 / 7.8e-06 |
| `circle_split` 4 | 9 | 2.08e-02 / 1.0e-05 | 2.61e-02 / 7.1e-06 |
| `circle_split` 4 | 13 | 2.17e-02 / 1.1e-05 | 2.10e-02 / 1.0e-05 |
| `circle_split` 4 | 17 | 1.78e-02 / 8.7e-06 | 2.04e-02 / 9.9e-06 |
| `circle_split` 4 | 21 | 1.69e-02 / 8.2e-06 | 1.73e-02 / 8.5e-06 |
| `circle_split` 4 | 33 | 1.40e-02 / 6.8e-06 | 1.44e-02 / 7.1e-06 |
| `circle_split` 4 | 65 | 1.18e-02 / 5.8e-06 | 1.19e-02 / 5.8e-06 |
| `circle_split` 8 | 9 | 3.75e-03 / 7.3e-06 | 5.07e-03 / 7.3e-06 |
| `circle_split` 8 | 13 | 3.95e-03 / 7.7e-06 | 3.83e-03 / 7.5e-06 |
| `circle_split` 8 | 17 | 3.04e-03 / 5.9e-06 | 3.67e-03 / 7.2e-06 |
| `circle_split` 8 | 21 | 2.84e-03 / 5.5e-06 | 2.95e-03 / 5.8e-06 |
| `circle_split` 8 | 33 | 2.14e-03 / 4.2e-06 | 2.28e-03 / 4.4e-06 |
| `circle_split` 8 | 65 | 1.65e-03 / 6.9e-06 | 1.67e-03 / 5.9e-06 |
| `circle_split` 16 | 9 | 7.75e-04 / 6.1e-06 | 1.10e-03 / 7.2e-06 |
| `circle_split` 16 | 13 | 8.23e-04 / 6.4e-06 | 7.96e-04 / 6.2e-06 |
| `circle_split` 16 | 17 | 6.02e-04 / 4.7e-06 | 7.57e-04 / 5.9e-06 |
| `circle_split` 16 | 21 | 5.54e-04 / 4.6e-06 | 5.81e-04 / 4.8e-06 |
| `circle_split` 16 | 33 | 3.87e-04 / 8.3e-06 | 4.19e-04 / 7.7e-06 |
| `circle_split` 16 | 65 | 2.68e-04 / 8.4e-06 | 2.73e-04 / 8.5e-06 |

**The second refusal is gone.** At ε = 1e-6 the continuation reaches
`Met` at 1.024e-3 on every `circle` and `circle_split` 4 cell, 9–65
stations at both degrees. That includes the 17- and 21-station
v-degree-3 quartered bodies that refused with no bracket. A
`Settle::Resolution` decides nothing, so an unmet one is a bracket;
it is not a refusal.

**At 1e-12** the cells the row cites (`circle` 9–65, `circle_split` 4
9–21) reach `Wall` at 1.024e-9 with a bracket, except `circle_split`
4 at 9 stations, v-degree 2, which reaches `Met`. Volume pads (m³), v-deg
2 / 3: `circle` 9: 5.7e-8 / 6.6e-8; 13: 2.4e-7 / 2.3e-7; 17:
2.1e-7 / 2.2e-7; 21: 2.0e-7 / 2.0e-7; 33: 7.1e-7 / 7.3e-7; 65:
2.6e-6 / 2.6e-6. `circle_split` 4 9: 1.0e-8 (`Met`) / 1.3e-8; 13:
4.1e-8 / 4.0e-8; 17: 3.4e-8 / 3.9e-8; 21: 3.2e-8 / 3.3e-8. This is
the ratified door. `[tolerance.measure]` R3: a face that costs more
per round gets fewer rounds and stops at `Wall` with a sound bracket.
How narrow that bracket is still depends on the section's knots and
the station count, but the bracket is the answer the door is ratified
to give at the wall. It is not a defect of this row.

**Mechanism.** The `rounds: 1` widths in the tables above were the
reporting budget's exit answering the sign question (PR 5568's
finding). They were never the sign's own width. Now no budget stands
between the sign walk and its predicate, and round 0 already excludes
zero by a wide margin on every body. The knot, degree and position
effects that are left are in the width of a bracket a caller asks for,
and only finer than the default ε.

**Pinned.** `demos/tour/src/klein.rs` `loop_volume_grid` builds five
cells with the scene's own `lofted_loop`: `circle` at 9 stations,
v-degree 2 and 3; `circle_split` 4 and 8 at 9 stations, v-degree 3;
and `circle` at 17 stations, v-degree 3, moved across the origin by
`transform_rigid`. Each cell's sign must decide, and its continuation
must reach `Met` at the printed precision. Each bracket must hold the
body's own volume from `patch_quadrature`, an independent divergence
integral of its patches that converges to 1e-12 between 16 and 24
Gauss points and closes its shell about two points. A·L is not the
oracle: the lofts differ between cells by up to 1.4 %.

**The Klein scene** ships `circle`, 13 stations, v-degree 2, and is
unchanged. The volume no longer constrains that setting. The mesher
still does
(`work/tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`).
`STATIONS`' comment now says so. **The helix**
(`projectbox` wall 1) was retired at every ε before this re-measure,
and `standing_spring` asserts `Met` against its own oracle.
