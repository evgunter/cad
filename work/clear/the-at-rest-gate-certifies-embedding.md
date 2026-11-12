---
id: the-at-rest-gate-certifies-embedding
kind: issue
title: The at-rest gate certifies that a finished body's boundary is embedded off its recorded contacts, at every scalar with certification rights (Ev, PR 6301)
status: open
opened: 2026-11-02
priority: P0
cost: H
---


Ruled at PR 6301 (`where-the-embedding-certificate-is-paid`), now in
`docs/DESIGN.md` `[topology.tiers]`. A finished body passes tier 3′
*and* its boundary is embedded off its recorded contacts, certified by
the clearance engine's body-level injectivity question at the at-rest
gate. This runs at every scalar with certification rights: f64, Probe,
Interval and Sym. Duals answer `NotRunAtThisScalar`.

What the unit carries, from the three designers' converged reports
(PR 6301's body):

1. **Split the clearance lane** (`AtRestPolicy::clearance_lane`,
   `topo/src/props.rs`). The minimum-clearance measure keeps `None` at
   f64. A yes/no verdict door reads certification rights.
   `topo::shell`'s cavity gate reads the same door, so its f64 window
   ("a curved crossing shells silently wrong") closes.
2. **The gate asks the certificate.** It has one fixed pair budget,
   PR 6102's rule. `Violated` names the two faces and a witness, and an
   engine refusal refuses typed and never reads Ok. For a loft, the
   editor maps the faces through `Lofted`'s face roles to walls and
   section ranges.
3. **Exclude recorded contact pairs.** This is a face-pair exclusion
   on `topo::clearance::Side`, since a tier-3′ body is not injective at
   its records.
4. **Keep the gate's verdict per node by content key**, which is
   PR 3870's "once, at the door that built it". Today
   `finished_operand` and the product gather re-gate on every
   evaluation or landing. Where the memo lives is ONEHOME's at-rest gate
   question (`onehome/the-at-rest-result-gate-and-its-typing-live-in-five-homes`);
   announce the seam.
5. **The viewer's preview landing shows construction state, badged,
   and does not finish the product.** Only a commit finishes it
   (`viewer::session::land`, G1's preview/commit split). This seam is
   the viewer's.
6. **CARVE's fold retires.** The fold-back arm and the far-normal
   interim go, and the hood and oblique-arc rows in
   `bool6_per_slab_stacking.rs` flip to building. The Zero arm stays
   before the skin as its own decide. The reversed list builds, with
   inside taken from slab 0's sign.
7. **The shell's pre-insertion cavity check** becomes a second reading
   of the same question: it retires, or stays as an early typed arm.

**Measure before the spec:**
- the certificate's price on planar and boolean-chain bodies, since
  every finished body now pays it;
- the refusal price after NETCACHE on the 9 rad and 13 rad arcs, the
  closed spine and CARVETAIL's joints;
- what a drive leaf pays at `Sym<Interval>`.

D5 is unchanged. One residual is named: dragging a loft that a boolean
or shell consumes pays the certificate once per new value (up to 10 s
on the Klein loop). If that bites, it is a D5 question of its own.

CARVE's `self-overlapping-spines-build-and-validate` is parked on this
row.

## Found by ONECOPY's branch-band fork (PR 6625), 2026-11-09

The fork on `a-carriers-branches-within-the-band-of-each-other` (closed
into this unit by PR 6618) put this gate as the home of "an edge or face
that comes within the band of itself". Its designers and its lane read
three defects in the engine this gate runs on. All were read from code
and none was run.

- **(a) A trimmed analytic face is never paired against itself for
  distance.** `Sweep::run` discharges a trimmed analytic face against
  itself by `by_kind` (`topo::clearance`, ~1580; `topo::injective`,
  ~518–538). That proves the chart `S` one-to-one. It never pairs the
  face's own non-touching cells for the distance test, which is
  ERROR-DESIGN's condition (2). The chart decides touching
  (`injective::rects_meet`), not the trimmed region in M. So two cells
  either side of a sub-band slit or waist in one face are certified,
  not measured. Exhibit: a plane cutting a ring torus in a spiric with
  a sub-band waist; no door builds it today. Two fix shapes:
  - pair a trimmed face's cells for distance anyway;
  - or read each face's boundary cycles clear of themselves at the band
    (tier 3's "simple and disjoint", at the band).
- **(b) Cross-face touching is decided by edge key over the whole span,
  not by the shared edge's parameter.** `injective::Local::of` keeps an
  edge whenever a cell's ball is not `clear_of_arc` over the edge's
  whole span (`topo::cell_region`). `Near::of` keeps a shared edge
  wherever neither ball clears it, and SLIVER's `near_edges.contains`
  then concedes a sub-band strip beside it. A cell beside one branch of
  a hairpin edge is thereby "near" it through the other branch. This
  flips no verdict found, but it is where one could be conceded. Fix:
  `Near` keeps the parameter run of the edge that the ball is not clear
  of.
- **(c) Residue:** a same-surface spline split refuses `Uncertified`,
  because the spline pcurve is unpresentable.

**Named residual.** A closed-form section read on the wrong branch
mid-op, before its result is gated. No instance is known. If one is
shown, the remedy is a uniqueness certificate at the section's mint.

**Pins for when this unit lands:**
- a hairpin edge with a NURBS wall, on a fold whose speed stays above
  `certify`'s span meter: a semicircle tip of radius below the band,
  over a short knot span between legs on long ones. The cubic hairpin
  of `geom`'s on-carrier rows does not serve here: its tip speed is
  `1.5·h`, and `certify` refuses it `SpanMeterCollapsed`;
- the torus waist of (a).

**Optional ordering:** run embedding before the census's
branch-resting reads (`census::curved::edge_tangent_at` and its
callers).

## Measured (CLEAR, lane clear-gate, 2026-11-12)

Main at `348ec09727`. The question is the one the gate would ask: one
body against itself, every face, `StrictlyPositive`,
`DEFAULT_MAX_CELL_DEPTH` (40) and 65,536 pairs (PR 6102's budget), with
the shell cavity gate's verifier. "Tier 3" is `topo::finish` on the
same body. The harness is a scratch example in `demos/tour`, not
committed; it ran at the tour's release profile (debug assertions and
overflow checks on, as PRICEMEASURE's did), `RAYON_NUM_THREADS=1`,
pinned to one core of an otherwise idle 4-core box. "Pairs" is the
receipt's cell pairs examined, against the budget. Times are one run
each; repeats of the small bodies moved them by up to 25%.

### 1. Planar and boolean-chain bodies

Every planar body holds, in 0.3 ms to 0.16 s, at 4–25× its tier 3.
The cost grows with faces, not with the boolean chain behind them.

| body | faces | tier 3 | certificate | × tier 3 | pairs | % budget | deepest |
|---|---:|---:|---:|---:|---:|---:|---:|
| box, f64 | 6 | 0.3 ms | 1.3 ms | 3.9 | 74 | 0.1 | 3 |
| box, Interval | 6 | 1.8 ms | 1.6 ms | 0.9 | 74 | 0.1 | 3 |
| L-bracket (one union), f64 | 8 | 0.7 ms | 3.8 ms | 5.7 | 196 | 0.3 | 10 |
| L-bracket, Interval | 8 | 1.9 ms | 3.0 ms | 1.5 | 196 | 0.3 | 10 |
| plate, 1 drilled hole, f64 | 8 | 0.8 ms | 8.1 ms | 10 | 224 | 0.3 | 8 |
| plate, 3 drilled holes, f64 | 12 | 1.5 ms | 12.7 ms | 8.3 | 304 | 0.5 | 8 |
| plate, 3 drilled holes, Interval | 12 | 8.1 ms | 13.4 ms | 1.6 | 304 | 0.5 | 8 |
| 5 slot subtracts, f64 | 26 | 1.4 ms | 14.9 ms | 11 | 1,170 | 1.8 | 10 |
| 10 slot subtracts, f64 | 46 | 3.8 ms | 48 ms | 13 | 2,356 | 3.6 | 11 |
| 10 slot subtracts, Interval | 46 | 13.2 ms | 44 ms | 3.3 | 2,356 | 3.6 | 11 |
| tour `table` (4 unions) | 18 | 1.2 ms | 23 ms | 18 | 1,586 | 2.4 | 23 |
| tour `az` | 28 | 2.9 ms | 31 ms | 11 | 1,631 | 2.5 | 23 |
| tour `impeller6` | 56 | 4.7 ms | 90 ms | 19 | 4,149 | 6.3 | 15 |
| tour `impeller12` | 86 | 7.1 ms | 160 ms | 22 | 8,105 | 12.4 | 15 |
| tour `die` (21 pip subtracts) | 111 | 5.8 ms | 110 ms | 19 | 6,283 | 9.6 | 13 |
| tour `bench` (an assembly, 3 contacts) | 24 | 1.3 ms | 0.4 ms | — | 99 | 0.2 | 0 |

The other 11 planar tour bodies (`spacer`, `silhouette` and its four
shadows, the three `crosslap` bodies, `diechamferblank`,
`benchlayout`) hold in 4.7–16 ms. `bench`, `bench60` and `bench90`
read `Violated` at distance 0 between faces 1 and 8: they are the
assemblies with recorded contacts, which item 3 excludes. No planar
price is near the ruling's line.

**The whole tour (85 finished bodies, 20 planar).** The 42 curved
bodies that hold take a median 0.22 s and at most 9.2 s (the Klein loop,
51,322 pairs, 78% of the budget; PRICEMEASURE's 46.8 s before NETCACHE).
The lily leaf `a` takes 2.6 s (27.2 s before). Over the walk the
certificate costs 92 s against tier 3's 31 s.

**24 of the 85 do not certify on main**, so a gate landed today
refuses them at their doors:
- 15 refuse `Uncertified` with the departure arm at the depth floor:
  `chute`, `diefillet`, the lily's stem, arch, pedicel, lantern and
  three buds, `tube_along_arc`, `hollowelbow`, `teapothandle`, and
  `heatsink5`/`7`/`9`. They take 0.06–2.2 s to refuse.
- 8 run out of pairs, in 2.5–6.3 s: `diepips`, `diecomposed`,
  `diechamfer`, `klein_bulb` and the four snowman bodies.
- the 3 assemblies above, at their recorded contacts.

So the gate cannot land as one switch: the 23 engine refusals are
filed (below) and gate lane C of the spec.

### 2. The refusal price after NETCACHE

Nothing refuses and nothing is slow. The four bodies that should
refuse read `Violated`, at both scalars, in 2–75 ms.

| body | scalar | tier 3 | certificate | verdict | pairs | deepest |
|---|---|---:|---:|---|---:|---:|
| arc spine 3.5 rad | f64 | 0.71 s | 0.22 s | Holds | 304 | 7 |
| arc spine 3.5 rad | Interval | refuses, 11 ms | 0.31 s | Holds | 304 | 7 |
| arc spine 6 rad | f64 | 0.59 s | 0.38 s | Holds | 661 | 8 |
| arc spine 9 rad | f64 | 0.53 s | 73 ms | Violated, the wall against itself | 107 | 2 |
| arc spine 9 rad | Interval | refuses, 10 ms | 74 ms | Violated, the same pair | 107 | 2 |
| arc spine 13 rad | f64 | 0.64 s | 74 ms | Violated, the wall against itself | 116 | 2 |
| arc spine 13 rad | Interval | refuses, 9 ms | 68 ms | Violated, the same pair | 116 | 2 |
| closed spine | f64 | 0.23 s | 2.7 ms | Violated, top against bottom | 20 | 0 |
| closed spine | Interval | refuses, 6 ms | 2.7 ms | Violated, the same pair | 20 | 0 |
| CARVETAIL's joints | f64 | 8.9 ms | 1.9 ms | Violated, wall 0 against wall 3 | 29 | 0 |
| CARVETAIL's joints | Interval | refuses, 4 ms | 2.0 ms | Violated, the same pair | 29 | 0 |

At `Interval`, tier 3 refuses every one of these lofts before the
certificate would run: check 7 cannot measure their spline walls
(`work/boxfinish/check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends.md`).
The certificate there was asked of the unfinished body.

### 3. A drive leaf at `Sym<Interval>`

A box, 2 × 1 in plan, extruded to a depth `h` over a parameter box
`1 ± w`, and SHELL-4's washer revolved about its inner radius `r0`.
"Symbolic" counts the decisions the symbolic tier discharged as exact
during each half (`geom_core::sym::session_counts`).

| leaf | w | tier 3 | tier 3's symbolic decisions | certificate | its symbolic decisions | verdict | pairs |
|---|---:|---:|---:|---:|---:|---|---:|
| box, depth `h` | 1e-6 | 55 ms | 348 | 3.0 ms | 0 | Holds | 128 |
| box, depth `h` | 1e-3 | 45 ms | 348 | 3.3 ms | 0 | Holds | 120 |
| box, depth `h` | 1e-2 | 88 ms | 348 | 3.2 ms | 0 | Holds | 120 |
| box, depth `h` | 3e-2 | refuses (the side reader), 77 ms | 348 | 4.4 ms | 0 | Holds | 156 |
| box, depth `h` | 0.1 | refuses (the side reader), 79 ms | 348 | 1.54 s | 26 | `Budget(Pairs)` | 65,536 |
| washer, `r0` | 1.6e-11 | 4.93 s | 228 | 25 ms | 19 | Holds | 24 |

The washer's profile refuses validation (`line_span`) at `w = 1e-3`
and wider, so it has no wider row. Two readings:
- Where tier 3 needs its symbolic proof (348 exact decisions at every
  width), the certificate needs none and costs 4–7% of tier 3. The
  family's width costs it splits (128 pairs at `w = 1e-6`, against 74
  for the f64 box), not symbolic work.
- No width was found where tier 3 passes and the certificate refuses.
  The certificate outlasts tier 3 here: at `w = 3e-2` tier 3's side
  reader refuses
  (`work/boxfinish/the-side-reader-cannot-read-a-solids-orientation-over-a-box.md`)
  and the certificate still holds. At
  `w = 0.1` it runs out of pairs, which is change 2's subject (below).
