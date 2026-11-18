---
id: the-at-rest-gate-certifies-embedding
kind: issue
title: The at-rest gate certifies that a finished body's boundary is embedded off its recorded contacts, at every scalar with certification rights (Ev, PR 6301)
status: open
opened: 2026-11-02
priority: P0
cost: H
design: true
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
3. **Ask each solid against itself** (fork `embed-records`, below).
   The gate never pairs faces of two solids: a meeting between solids
   is the census's, and an assembly's is its `⊔`'s to record or report.
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

Every body here without recorded contacts holds, in 1.3 ms to 0.16 s:
4–25× its tier 3 at `f64`, 0.9–3.3× at `Interval`. The cost grows with
faces, not with the boolean chain behind them. The plates' holes are
cylinders; every other body here is planar.

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

**26 of the 85 do not certify on main** (23 engine refusals and 3 assemblies; 59 hold), so a gate landed today
refuses them at their doors:
- 15 refuse `Uncertified` with the departure arm at the depth floor:
  `chute`, `diefillet`, the lily's stem, arch, pedicel, lantern and
  three buds, `tube_along_arc`, `hollowelbow`, `teapothandle`, and
  `heatsink5`/`7`/`9`. They take 0.06–2.2 s to refuse.
- 8 run out of pairs, in 2.5–6.3 s: `diepips`, `diecomposed`,
  `diechamfer`, `klein_bulb` and the four snowman bodies.
- the 3 assemblies above, at their recorded contacts.

So the gate cannot land as one switch. The 23 engine refusals are
filed (the Spec's last section), and lane C waits on them.

### 2. The refusal price after NETCACHE

The certificate refuses nothing here and is never slow. The four
bodies that should refuse read `Violated`, at both scalars, in
2–75 ms. The roles are the ones `self_intersection_proves_injectivity.rs`
pins for the same builds.

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
`1 ± w`, and the washer of `revolve_washer.rs`'s m10_9 row, revolved
with its inner radius `r0` over a box.
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
  width), the certificate needs none and costs 4–7% of tier 3 (0.5%
  on the washer). The family's width costs it a few splits up to
  `w = 3e-2` (120–156 pairs), not symbolic work.
- No width was found where tier 3 passes and the certificate refuses.
  The certificate outlasts tier 3 here: at `w = 3e-2` tier 3's side
  reader refuses
  (`work/boxfinish/the-side-reader-cannot-read-a-solids-orientation-over-a-box.md`)
  and the certificate still holds. At `w = 0.1` it runs out of pairs.
  Which reading spends them was not traced; a Zero site reading a
  cell over the family is the likely one (section 4).

### 4. What the three Zero sites in `topo::injective` would read

Decision "no second reading" (PR 6821) holds for all three. Each can
read a fact that topology, the separator's construction or tier 3
already holds, and tier 3 decides it in the same finish, before the
certificate runs.

- **`SELF_INTERSECTION_ON_SHARED`** (`is_zero`, `injective.rs` ~702;
  read by `sign_on` ~4360 at ~4376, ~4422 and ~4432). It asks whether
  a presented side, curve or corner point lies on the separator ψ. What
  it reads depends on ψ:
  - ψ is a face's carrier (`Psi::Carrier`). The side lies on the
    shared edge, and check 2 certifies the shared edge on both faces'
    carriers (each edge re-derived through
    `AtRestPolicy::reattach_edge`, `validate.rs` ~7634). A shared
    vertex (`Zero::Point`) is on both carriers by the same check, at
    the edge's end.
  - ψ is a plane built through the shared edges (`plane_through`,
    `Psi::Linear`). It vanishes on them by construction.
  - ψ is the graph separator (`Psi::Graph`). It is already exempt by
    construction (`checked` in `sign_on`).
  - `Zero::Past`, a side that runs past the shared cell on an
    analytic face, lies off the shared edge, where neither topology
    nor tier 3 states anything. It can read by its carrier pair's
    kinds (a line on a plane, a ruling in a plane through a cylinder's
    axis). Otherwise it stays a band reading, and the code must say so.
- **`SELF_INTERSECTION_APEX`** (`present_chart` ~1675). It asks
  whether a vertex of the piece sits at a cone's apex. Check 6's
  chart-singularity arm (`ValidationError::ChartSingularityNotAVertex`,
  `validate.rs` ~1805, in `tier3_local_checks_marked` ~7654) already
  places every chart singularity in a face's closure on a vertex of
  it. So the apex vertex is the one tier 3 placed there, handed over
  per face.
- **`SELF_INTERSECTION_CUSP`** (`cusp` ~4803, decided at ~4818). It
  asks whether a shared edge's faces have opposed normals, that is,
  whether it is a wedge-0/2π edge. Check 4's material arm decides
  exactly that: `wedge_reading` (`validate.rs` ~7173) maps
  `WedgeVerdict::End` to `ContactMark::Tangent`, and `contact_marks`
  (~7232) hands the per-edge mark out.

How the facts reach the engine: `AtRestBody::validate` (`validate.rs`
~10751) runs `validate_geometric_certificate` (~6440) first. That pass
returns the marks and the singular vertices beside its
`SignCertificate`, and the gate hands them to the sweep. At `Sym`
this is where tier 3's identity proofs reach the engine. The measured
leaves did not need them (table 3), so lane B's pin has to be found
first: a family where tier 3 passes and a Zero site refuses.

## Spec

**Order:** A, then B's separators with the engine rows below, then C
(with B's tier-3 input), then D and E in parallel. The engine rows (the 23 refusals of table 1) must land
before C. Otherwise C's gate refuses 23 tour bodies at their doors,
among them the lily, the teapot's handle, the heatsinks and the
snowman.

### Lane A — the verdict door, one crossing, the engine's own scope (M) — done

**Landed (PR 6885, 2026-11-12).**
- The verdict door is `topo::clearance::embedded`, bounded on
  `Decide + CertifiedBounds`. It asks the strict question of a `Side`
  and answers `Embedding::{Embedded, Crosses(Crossing), Refused}` with
  its receipt. The verifier lives in the door. Lane C's gate calls the
  same door, so it gets the witness verifier with it.
- The shell's `cavity_embedded` maps the door's answer onto
  `CavityCrosses` / `CavityUncertified`. Its `lift` parameter is gone.
- **The f64 switch moved out** to
  `the-f64-cavity-gate-refuses-thirty-two-valid-shells` (P0). Asked at
  f64, the gate refused 32 valid shell rows the engine cannot certify
  yet: 22 out of pairs, 8 on the departure arm, 2 at a window. The
  orchestrator ruled that the spec's ordering covers it, so it waits on
  the engine rows. The shell asks through `CavityGate`, only where the
  measure door answers (`Interval`, `Sym<Interval>`). The coaxial and
  dumbbell f64 pins, and the mutant that skips the shell's gate at f64,
  went with it.
- The verifier reads `Candidate::enclosure`: the window carriers the
  engine already crossed. So neither the door nor the shell lifts a
  chart, and no caller spells `Interval::from_certified`.
- `Sweep::run` and `min_separation` run their whole walk under
  `k_stats::unlogged("F41")`. F28, F29 and F30 are gone. F29's
  dimension disposition moved into F17. The one logged reading, the
  box-dependent cue in `wire_clearance`, is F41's stated exception.
- One deviation in change 1. The engine's own crossings (`window_of`,
  `boundary_of`, `cell_region`) still call `Interval::from_certified`
  in place. They do not go through a function the lane value holds,
  because the internals are generic over `T` and are never handed the
  lane. The rule "no caller crosses" holds; "one function on the lane"
  holds as the lane's bound and doc.
- Measured: the f64 "curved window" row was never a crossing. Its
  dumbbell carried its bulges on the blob walls, not the neck
  (evidence added to
  `shell/offset-corner-on-surface-escalates-at-interval-on-an-arc-necked-extrude`).
  The fixture now bows the neck. Its Interval row still refuses in the
  offset door, and at f64 it would refuse `CavityCrosses` (the moved
  row's pin).
- Pins:
  - `the_measure_door_answers_at_the_enclosure_scalars_and_nowhere_else`
    and `the_verdict_door_answers_at_every_certifying_scalar` (a cube at
    f64, Interval, `Sym<f64>`, `Sym<Interval>`, and Probe under the
    feature), plus `embedded`'s `compile_fail` doctest at `Dual64`;
  - `holds` (the tube and the vessel at `Interval`), which asserts the
    shell logs none of the engine's cell readings.
- Mutants, both run:
  - the door answering `Embedded` at f64 with nothing read reds the
    verdict table;
  - `Sweep::run` without its scope reds the tube (1,027 rows logged),
    the vessel (588) and editor-core's notch node (2,140 verdicts and
    297 escalations).

Items 1 and 7, and changes 1 and 4. Rows touched: this one, and
`the-shells-cavity-sweep-logs-its-cell-readings` (closes).
- **The verdict door** is a function bounded on `CertifiedBounds`, not
  an `Option` per scalar. It answers at `f64`, `Probe`, `Interval`,
  `Sym<f64>` and `Sym<Interval>`; a dual cannot name it. The measure
  door stays `AtRestPolicy::clearance_lane` (`props.rs` ~6935; impls
  ~7151 f64, ~7312 Probe, ~7468 Interval, ~7627 Sym, ~7797 Dual), with
  `None` at `f64`, `Probe` and `Sym<f64>`. Its readers are
  `wire_clearance` (`editor-core` `eval/wire.rs` ~2578) and
  `eval/mod.rs` ~4380. At `Sym<f64>` the verdict door answers as at
  `f64`, through the same crossing `Sym<Interval>` uses.
- **The shell's cavity gate** (`cavity_embedded`, `shell.rs` ~4618,
  called ~1858) reads the verdict door, so it asks at `f64` and its
  window closes.
- **Change 1:** `ClearanceLane` (`clearance.rs` ~4513) holds the one
  crossing, `Interval::from_certified`. `cavity_embedded`'s `lift`
  parameter goes, and so does the verifier's own chart lift.
- **Change 4:** `Sweep::run` (~1439) and `min_separation` (~2838)
  take their cell readings in `k_stats::unlogged` themselves. The
  hand-kept scopes go: F28 (`clearance.rs` ~3058, ~3096), F29 (~4204)
  and F30 (`eval/wire.rs` ~2635), with their rows in
  `docs/predicate-dimension-audit.md`.
- **Pins:**
  - `the_door_answers_at_the_enclosure_scalars_and_nowhere_else`
    (`clearance.rs` ~4691) splits into two tables, one per door. A
    `compile_fail` doctest shows `Dual64` cannot name the verdict door.
  - `crates/sweep/tests/shell_cavity_embedded.rs` gains an f64 row
    for its coaxial crossing: it refuses `CavityCrosses`, where today
    it ships.
  - A shell node at `Interval` logs no cell reading.
- **Mutants:** the verdict door answering nothing at f64 (the f64 shell
  row ships its crossing); `Sweep::run` without its scope (the log row
  reds).
- **Seams:** SHELL (`shell.rs`); editor-core's evaluator (F30).
- **Item 7, recommended:** the shell's cavity check retires in lane C.
  C's gate asks the whole shelled body, cavity included, so the shell
  would pay the question twice. The shell maps C's `Embedding` finding
  to `ShellError::CavityCrosses` when both faces are the cavity's.

### Lane B — no second reading (M)

Change 2. The injectivity question takes tier 3's verdict as its
input. The engine never re-decides a fact that tier 3, or its own
construction, already holds (`[topology.tiers]`).

- **The input.** The strict question over one body is the walk
  `Sweep::run` takes when `same_body` holds and the bound is strict.
  It takes a value that only tier 3's local battery constructs:
  - the body;
  - each edge's wedge class (`WedgeVerdict`: a crease, π, or an end);
  - each face's singular vertex, where check 6 placed it
    (`geom_brep::chart_singularity::Lies::AtVertex`; today it is
    dropped after `place_on_boundary`).

  `topo::clearance::embedded` takes this value in place of `Side`, so
  asking injectivity of an unchecked body does not compile.
  `Side { body, faces }` stays for the metric questions (two bodies,
  or a bound that is not strict); those reach no Zero site.
  `AtRestBody` keeps the two per-entity facts beside its `KeptSign`,
  and `moved_by` carries them. They are keyed by entity, so an
  isometry keeps them.
- **The three sites.**
  - *On the shared cell* (`is_zero` / `sign_on`). Each separator
    carries the cells it vanishes on, and `sign_on` asks membership;
    no value is read.
    - A face's carrier vanishes on that face's edges and vertices, by
      check 2.
    - A plane the engine builds (`plane_through`) vanishes on the edges
      and the vertex it was built through. Its `n` and `q` are computed
      at the lane scalar from the edge's own carrier (a line's stored
      direction, a conic's stored axis and centre), never rounded to a
      midpoint.
    - The graph separator vanishes on its one edge, as now.
    - A plane does not vanish at a shared vertex off the line it was
      built through. There it answers nothing.
  - *Past the shared cell* (`Zero::Past`; `Zero::Curve` and
    `Zero::Corner` alike). The side is zero where its chart line is the
    shared edge's own line or circle, since a separator built from the
    edge contains it whole. Otherwise it answers nothing. No band
    reading is left.
  - *Apex* (`present_chart`). The face's singular vertex, read from the
    verdict. At `Sym`, this is where tier 3's identity proofs reach
    the engine.
  - *Cusp* (`cusp`). The edge's wedge class is an end. The cusp is not
    read off `ContactMark::Tangent`: `wedge_reading` also maps a
    determinate wedge-π seam (a smooth G1 join) to `Tangent`, so a
    blend edge would read as a cusp and refuse `SecondOrder` past
    depth 10. The contact mark is derived from the wedge class.

  The funnel sites `self_intersection_on_shared`,
  `self_intersection_apex` and `self_intersection_cusp` retire, with
  their audit rows.
- **The callers.**
  - The gate (lane C) holds the verdict in the function that asks.
  - The shell's cavity check retires into the gate (item 7).
    `the-f64-cavity-gate-refuses-thirty-two-valid-shells` then closes
    into C: its 32 rows join C's engine rows, and its pins move to C's
    mapping of a cavity crossing.
  - editor-core's strict door (`self_intersection`, whose only callers
    are tests) reads the finished leaf (`Held::finished`,
    `topo::take`).
  - `wire_clearance` asks no strict bound, so it is not a caller.
  - Tests ask finished bodies. Construction-state lofts at `Interval`
    wait on `check-7s-quadrature-refuses-interval-nurbs-walls…`.
- **Order.**
  - **Lane B (now): the engine's own separators** — done (PR 6905;
    what landed is below the measured block). These need no
    tier 3:
    - `plane_through` is exact per member;
    - each separator carries its cells;
    - the reading goes for planes and graphs.

    This is sound for every caller today.
    - Pins:
      - a family whose shared line edge turns with the parameter (a
        drafted or sheared extrude at `Interval`);
      - a pair sharing a line edge and a separate vertex, where the
        plane answers nothing at the vertex;
      - the box at `1 ± 1e-2`, `Sym<Interval>`.
    - Mutant: every cell in the separator's list.
    - Before any reading goes, run the forced harness on the carvetail
      fold and the folded cut. These are the rows that `is_zero` always
      true turns off `Violated`. Record which `Zero` arm and which
      `Psi` each kill comes through. If one comes through a carrier
      side on a shared edge, then check 2 does not give what section 4
      says: stop and report.
  - **With lane C:** the input type; the facts kept in tier 3 and on
    `AtRestBody`; the carrier, apex and cusp sites; and the callers.
    - Pins:
      - `embedded`'s `compile_fail` doctest at `&Body`;
      - the revolved cone at `Sym<Interval>`, holding with its apex
        read from the verdict. It needs a cone whose apex is a vertex
        there, which waits on `revolve`'s
        `NoGeneralLane { chart: Cone, carrier: Line }`; `cone_cap`
        fails check 6;
      - the declared wedge-0 kiss, refusing `SecondOrder` read off
        `End`;
      - a blend edge marked `Tangent`, not read as a cusp.
    - Mutants:
      - an empty apex map (the cone refuses `CollapsedChart`);
      - the cusp read off `Tangent` (the blend row refuses).
  - The spec's first pin, a family where tier 3 passes while a Zero
    site refuses, is not needed. The change is owed by
    `[one-way.readings]` (one reading per fact), not by a verdict that
    moves (measured below).
- **Seams:** TOPO's `validate.rs`, for the facts tier 3 keeps;
  editor-core's strict door.

**Measured by lane B (clear-zero, 2026-11-13), main at `b95a5e1eb2`.**
Lane B stopped before building: the facts do not reach the sweep in
the shape the spec assumes. The harness was a scratch test with
counters at each Zero site, run unoptimised. It was not committed. The
question is `embedded` over every face, at depth 40 and 65,536 pairs.
"Forced" means each non-`Past` zero side read as zero, and an apex
decision the band leaves open read as the apex. That is what lane B
would read in their place.

| body | tier 3 | certificate | forced | carrier sides read zero / open | linear sides read zero / open |
|---|---|---|---|---:|---:|
| box, depth `1 ± 1e-2`, `Sym<Interval>` | passes | Holds, 120 pairs | Holds, 74 | 36 / 98 | 0 / 32 |
| box, depth `1 ± 3e-2` | refuses | Holds, 156 | Holds, 108 | 36 / 120 | 0 / 52 |
| box, depth `1 ± 0.1` | refuses | `Budget(Pairs)`, 9.0 s | `Budget(Pairs)`, 9.0 s | 276 / 5,462 | none read |
| revolved cone, `h = 1` exact, `Sym<Interval>` | passes | Holds, 182 splits | Holds, 182 splits | 2 / 206 | 206 / 0 |

- **The box at `w = 0.1` is not a Zero-site pin.** Forced, it still runs
  out of pairs. 221,115 of its 227,632 presentations refuse `Local`.
  190,033 of those are `present_plane`'s catch-all, a cell whose local
  boundary holds three or more of the plane face's edges. Every cell's
  ball carries the family's 0.2 m in depth, so no split narrows it.
  The obstruction is the family's width (change 3, lane C), in a
  function the curved-corner lane owns. Tier 3 also refuses at this
  width, so the body cannot be the spec's pin.
- **No family was found where tier 3 passes and a Zero site refuses.**
  On the box the carrier arm's side reading is open on most cells at
  every width. The graph separator, exempt by construction, certifies
  those pairs instead. Reading the fact saves 38–46 pairs, and no
  verdict moves.
- **The cone has no box to vary.** `revolve` at `Sym<Interval>` refuses
  the triangle `(0,0) (1,0) (0,1 ± w)` for any `w > 0`
  (`NoGeneralLane { chart: Cone, carrier: Line }`). `cone_cap` is an
  `f64` fixture whose apex is not a vertex, which check 6 refuses. So
  the spec's "cone apex over a box" pin has no door to build its body.
- **Where the facts are today.** `structural_via` (`validate.rs`) runs
  `tier3_local_checks_marked` and drops the marks. Check 6's
  chart-singularity arm places each apex on a vertex (`Lies::AtVertex`)
  and keeps nothing. `AtRestBody` holds the body and its `KeptSign`
  alone. The engine is entered through
  `Side { body: &Body<T>, faces }`, and no current caller holds a
  tier-3 verdict:
  - `shell::cavity_embedded` asks the moved clone before it is
    finished;
  - editor-core's clearance takes `Held::Leaf`, which at the box
    scalars is tier 2 alone;
  - the sweep tests ask `into_body()` or construction-state lofts.

  The marks (`cusp`) and the apex vertices (`present_chart`) are
  per-edge and per-vertex data, so the engine can read them only if a
  caller hands them in. That is a new public shape on `Side`, or on
  `embedded` and `Sweep::run`, which every caller above has to fill.
  The spec does not name it, or what a caller without a verdict hands.
- **"By construction" holds only member by member where `n` is built
  from the edge.** `plane_through` rounds its normal to a point
  (`lift(unit(vmid(..)))`). Over a family whose shared line edge
  changes direction, a fixed `n` is not normal to every member's
  edge, so `ψ` does not vanish on the shared edge. Reading it as zero
  by construction would then be unsound. The construction is true if
  `n` and `q` are computed in `Interval` from the edge's own carrier.
  The same holds for a conic's plane: use the stored axis and centre.

**Landed by lane B (clear-sep, PR 6905), main at `39a29dd379`.**

- **The forced harness found no kill.** Every `is_zero` read forced
  true, at all five `Zero` arms under all three `Psi`: the carvetail
  fold, the carvetail loft, the folded oblique cut and the folded torus
  cut stay `Violated`. None of them reaches a `Zero` site on the way to
  its witness; each is found by distance first. Across sweep's 145
  clearance, cavity, self-intersection and embedding rows no verdict
  moves, and no certification goes through a forced read. Only the four
  topo rows that test the reading itself go red. So the stop condition
  (a kill through a carrier side) does not arise. The premise that
  these rows are turned off `Violated` by the mutant does not hold on
  this main.
- **The separators** (`topo::injective`):
  - `planes_through` builds `n` and `q` at the lane scalar from the
    shared edge's stored carrier. A line's stored direction has the
    chosen bisector's along-line part taken out, and `q` is a point of
    the line. Two lines take their stored directions' cross product. A
    conic takes its stored axis and centre.
  - Each separator carries the cells it vanishes on (`On`). A plane
    lists its edges and their ends, and a vertex-only plane lists its
    vertex. The graph lists its one edge.
  - Each zero side, point and curve names its cells (`Along`).
    `sign_on` asks membership and reads no value.
  - The carrier arm reads as before.
- **6892's widening became one plane per conic.** A sphere's two seam
  meridians lie in stored planes a half turn's sine apart, so neither
  plane lists the other's edge. Each near conic's own plane is tried in
  turn.
- **Past and curve sides read membership alone, and no verdict moved.**
  - A past side is zero where the plane holds its edge's whole line or
    circle.
  - A curve is zero under a plane listing its conic, and under the graph
    where its parameters lie in the graph's piece.
  - On closed conic edges the graph's piece now also holds the curve's
    parameters past the seam. Without that, the filleted drum refused:
    the curve runs an ulp past the seam through its join.
- **Corner sides kept one band reading; it is gone (seam lane, PR
  6908).** Removing it moved 6892's two-ball union from `Holds` to
  `Uncertified` (Departure, at a meridian, depth 40). At the pole, one
  face's corner is on one meridian and the other's on the other, and
  the two meridians' stored planes differed by `1.2e-16` rad (the
  revolve placed the angle-π copy through its turned placement, whose
  `sin π` rounds). The revolve now stores the angle-π meridian with the
  angle-0 one's centre and its axis turned over. A conic's plane lists
  every near conic whose stored frame is the same value
  (`one_plane`), and `sign_on` reads membership only. At `f64` the
  union holds. At `Interval` the small ball's centre is a bracket,
  which is not one value, so its pole refuses
  (`a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval`).
- **Pins:**
  - topo's `a_plane_through_a_turning_line_edge_holds_it_for_every_member`
    (an `Interval` family whose line edge turns; red on a rounded `n`);
  - `a_plane_through_a_line_edge_answers_nothing_at_a_vertex_off_it`;
  - sweep's `a_box_family_holds_at_sym_interval` (Holds, 120 pairs).
  - The body-level turning family has no door. Oblique extrusion is
    deferred, and a profile vertex at `Sym<Interval>` escalates in
    profile validation (`LineSpan` on its adjacent segments).
- **Mutants:**
  - every cell in the list reds three topo rows;
  - the rounded `n` reds the turning-edge row.
- **The tour walk** (85 bodies, the curved ones included, f64): 81
  hold, against main's 80. No body that held moved. `diecomposed` now
  holds, at 46,337 pairs (70.7%; on main it refused on budget). The
  four that do not hold are main's own: `klein_bulb` (budget) and
  the three benches (`Violated` at their recorded contacts, which this
  harness does not exclude).

### Lane C — the gate asks the certificate (H)

**Note from lane A's review (PR 6885).** Until change 3 lands here, a
shell at `Interval` that refuses `CavityCrosses` or `CavityUncertified`
carries no logged reading: the witness confirmation runs inside F41's
scope. Change 3 is what puts that one row in the log.

Items 2 and 3, and change 3. It needs lane A, and the engine rows
filed below. Rows touched: this one, and
`a-recorded-contact-refuses-the-injectivity-question-until-contact-design-hands-it-off`
(the gate's exclusion is that row's "exemption" candidate; announce).
- **The gate:** `AtRestBody::validate` (`validate.rs` ~10751) and
  `AtRestBody::validate_pseudomanifold` (~10852), that is, every
  certifying scalar's `finish` (`props.rs` ~7157, ~7318, ~7474,
  ~7633) and the box scalars' taker (`finish.rs` `take` ~393). After
  tier 3 passes, the gate asks the verdict door of the whole body at
  one pair budget. `DEFAULT_MAX_CELL_PAIRS` (`editor-core`
  `clearance.rs` ~143) and `CAVITY_MAX_CELL_PAIRS` (`shell.rs` ~4591)
  become the gate's one constant in `topo`.
- **Findings,** in `FindingFamily::Embedding` (`finish.rs` ~615):
  - `BoundaryMeetsItself { faces, witness }` for a verified `Violated`;
  - `EmbeddingUncertified { refusal }` for an engine refusal, which
    never reads `Ok`.
  - Change 3: a refusal names the width that blocked it, the family's
    (a narrower parameter box answers it) or the cell's (refinement or
    budget answers it). The witness is one point reading at two
    stations, made at the body's own scalar after the sweep returns.
    It is the one logged row, and at `Interval` its straddle is the
    drive's cue.
- **Item 3:** the gate asks the verdict door once per solid, over that
  solid's faces (`Side::faces` scopes the ask), and never offers the
  engine a pair of faces of two solids. `finish` is handed no records
  for this: none is in play between the faces of one solid, except a
  record whose two cells lie in that solid (below).
- **The loft:** `LoftError::SelfIntersecting` names the two faces by
  their `Lofted` roles (`loft.rs` ~131: `side_faces[l][j][s]`, `top`,
  `bottom`) and the section range each meets in.
- **Pins:**
  - the 9 and 13 rad arc spines, the closed spine and CARVETAIL's
    loft refuse at their door at `f64` with `BoundaryMeetsItself` and
    the roles of table 2. Today they ship;
  - `bench`, `bench60` and `bench90` finish: no pair between two
    copies is asked. Asked of the whole body the question reads
    `Violated` at 0;
  - a product whose copies overlap finishes, with its interference
    finding;
  - an engine refusal reads `EmbeddingUncertified`: the box leaf at
    `w = 0.1`, until lane B moves it.
- **Mutants:** no gate call (the spine rows ship); a refusal mapped to
  `Ok` (the uncertified row ships); the gate pairs faces of two solids
  (a bench and the overlapping product refuse).
- **Gates:** every suite, the tour walk (85 bodies) and `demos/wild`.
  The price is table 1's: 4–25× tier 3 on planar bodies, and up to
  9.2 s on the Klein loop.

### Fork `embed-records`: what the gate asks, and of what

Raised by the gate lane on 2026-11-17: `AtRestBody::validate` is
handed no `ContactRecords`, so an exclusion "built from the body's
records" has nothing to read at `finish`. Two designers converged
after three rounds (`docs/DESIGN-FORK-LOG.md`, row 287); the answer is
in `docs/DESIGN.md` `[topology.tiers]`.

- **The certificate is each solid's.** Lane C's gate asks per solid.
- **The finished body holds its records**, and the gate confirms each
  before anything relies on it. `BooleanBody::contacts`, the
  evaluator's `Carry`, `Product`, the import's copy and the Python
  `Body`'s become reads of the finished body. A door carries its
  operands' records through its lineage before its finish, not after.
  The Boolean's `gate` already holds them (`Held`); `shell`, split and
  blend take a bare body today. This closes into it
  `work/atrest/a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys.md`
  and its two neighbours on the same gap.
- **A solid that holds a recorded contact between two of its own
  cells refuses typed at the gate** (Ev, PR 7068, 2026-11-18:
  "refusing for now sounds good"; the long run is open,
  `[open.self-contact]`). The two candidates: the certificate reads
  around the recorded cells as around a shared vertex or edge, the
  gate first confirming the record with the census's self-touch
  reading (both designers' recommendation, and
  `a-recorded-contact-refuses-the-injectivity-question-until-contact-design-hands-it-off`'s
  third candidate for records inside one solid); or the recorded face
  pair is left out. An area self-touch inside one shell keeps its
  typed refusal (`TouchesItself`). **What it refuses meanwhile:**
  nothing in the demos (tour 0 of 86 stops, `demos/wild` 0 of 8
  cells); in the workspace, the rows that build a self-touching solid
  on purpose and hold its record (the gate lane's count is on this
  row's `## Census`).
- **After an op over an assembly, its pieces stay pieces, and the op
  answers only for what it made** (the fork's third part, rewritten
  after Ev's questions on PR 7068; nothing about an overlap rides the
  finished body, and `[open.lint-output]` is untouched).
  - Cells a door carried unchanged between two solids that descend
    from different pieces of one operand keep the meeting they arrived
    with: a recorded touch keeps its record, a reported overlap stays
    the `⊔`'s finding, reported once. No gate reads that pair again.
  - Cells a door made (minted or moved) are read once against the
    other pieces. A removing door (`−`, `∩`, split, shell) cannot make
    an overlap, but can leave a bare touch (two crossed bars, the
    overlapping centre cut out: four stubs touching along edges the
    cut made), and records it. A wall cut where the pieces already
    overlapped is the old overlap. Old or new is read off the operand.
  - A solid that crosses itself never passes: a `∪` whose added
    material fuses two overlapping pieces refuses on the per-solid
    certificate. (An overlapping assembly used as a cut's tool refuses
    today for the same reason, which is a kernel limit and not a rule:
    Ev, PR 7068, "the same as cutting by one of the interfering parts
    and then the other";
    `work/seal/a-cut-by-a-tool-whose-pieces-overlap-is-cut-as-it-stands.md`.)
  - **A fillet whose added material reaches the neighbouring piece is
    a new interference finding at the fillet, never a refusal** (Ev,
    PR 7068, 2026-11-18). `[operations.operators]` and
    `[topology.records]` say so.
  - Confirming the records a door writes between solids it cuts out of
    one piece is tier 3′, parked on
    `work/reachhold/boolean-door-runs-the-census-over-its-result.md`.
  - **Pins:** an overlapping assembly cut, met, split and shelled
    finishes with one interference finding, at the assembly, and no
    gate read between carried cells of different pieces (mutant: the
    gate reads that pair); the crossed bars give four edge records,
    each citing the cut; `(bolt ⊔ plate) − pin` reports one finding,
    not six; a `∪` fusing both overlapping pieces refuses.

**Order.** The per-solid ask lands with lane C. The records moving
into the finished body, the confirmed meeting place and the census
inside a deciding door's finish are units of their own, cut when lane
C lands.

### Lane D — once per node; the viewer's preview (M)

Items 4 and 5, after C.
- `finished_operand`, which the row names, is gone: ONEHOME's PR 6371
  made every door return its finished body, and `Framed` stores it.
  What still re-runs the gate is the box scalars' taker
  (`topo::take`, `finish.rs` ~393; the switch at `props.rs` ~6960–7015).
  It runs tier 3, and after C the certificate, once per consumer that
  takes a leaf. The product's aggregate (`T::finish(aggregate)`,
  `product.rs` ~1243) is a new body, so it pays once.
- **The memo:** keep the taker's finished body per producing node's
  `ContentKey` (`eval/memo.rs` ~34), so k consumers of one leaf pay
  once. It retires with the switch
  (`work/boxfinish/delete-the-box-scalar-sweep-leaf-switch.md`).
  `work/wiretail/a-body-value-consumed-by-k-nodes-is-gated-at-rest-k-times.md`
  is this item; its site list predates PR 6371.
- **Item 5:** `viewer::session::land` (`viewer/src/session.rs` ~1817)
  lands a preview as construction state, badged, and only a commit
  finishes the product. That is the viewer's seam; this lane announces
  it, and the viewer lane builds it.
- **Pins:** a document whose one extrude three Booleans consume, at
  `Interval`, finishes the leaf once (`finishes_on_this_thread`,
  `finish.rs` ~422); a drag preview of a loft that a Boolean consumes
  runs no finish, and its commit runs one. **Mutant:** no memo (three
  finishes).

### Lane E — CARVE's fold retires (M)

Item 6, after C, on LATHE's ground
(`work/lathe/self-overlapping-spines-build-and-validate.md`, which it
unparks).
- **Sites:** `step` (`sweep/src/loft/stacking.rs` ~458). The far decide
  (`FarSectionNotForward` and `FarStackingEscalated`, ~503;
  `LoftError` in `loft.rs` ~415–440) goes. `ReversedStacking` builds,
  with the rings traversed the other way when slab 0's sign is
  negative. `DegenerateStacking`, the Zero arm, stays before the skin.
- **Pins:** `an_embedded_hood_with_a_downturned_top_is_refused_by_the_far_decide`
  and `an_embedded_oblique_arc_sweep_is_refused_by_the_far_decide`
  (`bool6_per_slab_stacking.rs` ~454, ~480) flip to building and
  finishing; a reversed section list builds the same solid (equal
  volume, outward); the downward-top loft refuses at C's gate.
  **Mutant:** inside read from slab 1's sign (the y-prism of the
  lathe row builds inside out, and check 7 refuses it).

### ONECOPY's pins

No door builds either body today, and each needs a `topo`
test-support fixture built by Euler operators and finished through
`topo::finish` before the certificate is asked:
- **The torus waist.** A ring torus trimmed by a plane parallel to its
  axis, just past the inner equator's distance, so that the spiric has
  one waist narrower than the band. The waist's width grows as the
  square root of the plane's offset, and an f64 offset has an ulp
  floor, so the torus has to be small (radii around 1e-3) to bring the
  waist under the band. The Boolean of a torus and a slab is the
  natural door, but its section is near-singular there; that was not
  tried.
- **The hairpin.** A NURBS wall bounded by a planar NURBS edge whose
  tip is a rational semicircle of radius below the band, over one
  short knot span between long legs. The fixture's first assertion is
  that `certify` accepts it: the span meter must not collapse. The
  loft and the sweep cannot build it: sections or stations within the
  band of each other refuse at the stacking door. STEP import is the
  one door that could carry it, and that was not tried.

### Filed with this spec

- `the-certificate-refuses-a-plane-cap-on-a-torus-or-cone-wall-at-their-circle`
  (P0): 7 tour bodies.
- `the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`
  (P0).
- `departure-reads-no-curved-corner-off-its-shared-edge` takes the
  other 8 tour bodies as evidence, since their refusals have its
  shape (not traced to the cell), and is re-banded P0.
- On CLEAVETAIL's slate: `two-tour-wall-probes-moved-on-main`.

## Census (lane C, embed/gate, 2026-11-18, main `362cd72fd2`)

**The gate measured.** `AtRestBody::validate` asks `topo::clearance::embedded`
of every face of the body after tier 3, at `DEFAULT_MAX_CELL_DEPTH` and
65,536 pairs (`GATE_BUDGETS`), with **no exclusion**: `finish(body, tol)`
holds no `ContactRecords` (they reach only `validate_pseudomanifold`, after
the finish), so the exclusion has nothing to read there. That is a fork,
sent to the orchestrator; the census measures what it costs (class 3).
`validate_pseudomanifold` does not ask again: the finish already asked a
superset of its pairs.

**How.** An uncommitted log at the gate wrote one row per ask: the test,
the scalar, the face count, the receipt, the seconds, the verdict, and the
kind and solid of each face the verdict names. Workspace:
`cargo nextest run --workspace --profile ci --ignore-default-filter
--no-fail-fast` in CI's build configuration (libraries opt-1, test crates
opt-0), 4 cores, 15 GB. Every failure was run once more with the
solid-logging build (1,752 of 1,846; the other 94 keep the first run's
reading, without solids). Outside the workspace: the tour walk
(`demo-tour <dir>`, release) and `demos/wild` with the gate logging and
passing, so every body is read; the tour suite (`cargo nextest run
--release`); `tools/tess-meter`. `benches` holds no tests; `tools/k-lint`,
`tools/tess-lint` and `interval-transcendentals` do not depend on `topo`.
The engine lanes that landed after `362cd72fd2` (conic arcs, the cylinder's
vertex corner) are not in this census.

### Workspace: 1,846 of 19,376 tests fail

| class | tests |
|---|---:|
| 1. not embedded: the gate is right, the test pinned a later refusal | ~12 |
| 2. a valid body the engine refuses | 1,056 |
| 3. a contact the exclusion should cover (no records at the finish) | ~752 |
| 4. other | 26 |

- **Class 1** (named, not traced): tests that expect tier 3′'s own wedge
  or crossing refusal of an invalid body and now meet the gate's
  `BoundaryMeetsItself` first: `a_face_inside_an_edges_wedge_refuses_at_tier_3_prime::the_keel_in_the_floor_refuses_at_tier_3_prime`,
  the two `boolean_pinch_copies::pinches_along_an_operand_edge_refuse_their_overlapping_wedges`
  rows, `review_4740_r1_probes`' two `refuses_its_overlapping_wedges` rows,
  `ring_carry_through_by_piece::a_ring_pinched_to_a_rim_vertex_refuses_its_overlapping_wedges`,
  `a_doubled_line_does_not_revert`'s four refusing rows,
  `an_edge_on_a_ruling_lying_in_its_own_face::a_keel_on_a_cylinders_ruling_through_its_face_refuses_at_tier_3_prime`,
  `review_6020_seam_probes::a_counterbore_half_with_two_faces_on_each_other_refuses_at_the_point`.
  Undecided between 1 and 3: `the_fin_crown_winds_as_a_solid::the_fin_crown_is_a_solid_where_the_old_crown_crossed_itself`.
- **Class 3**: every crossing on a valid body. 506 + 56 cross between
  two solids of one body (assemblies, products of touching copies,
  disjoint unions resting on each other: `bench` is this); about 190 cross
  inside one solid at a slit, keel, pinch, edge lying in a face or ring
  on a line (`a_keel_on_the_floor_subtracts_as_a_slit`,
  `union_flush_onto_edge_contact`, `three_solids_on_one_line`,
  `join_pierce_runs_sweep`, `split_pinch_keeps`). Every crossing is at
  distance 0 to 3e-16 m. A further 208 of class 2's refusals are in tests
  named for a pinch, slit, keel, touch, flush face, contact, mate or
  tangency; their pairs may be ones records would exclude. Not traced;
  they move with the fork's answer.
- **Class 4**: 9 tests whose gate refusal is in a child process (the
  offer rows in `topo` and `sweep`, `the_corners_offer_executes`,
  `fillet_h6_cap_rim`'s and `review_blend_k_rk_probes`' three probes,
  `stl`'s `eps_rows_export_identical_bytes`); 6 source censuses red on
  files this diff does not touch (main's own); 2 of
  `mass_props_are_thread_count_invariant`, whose verdict count moved:
  the gate's windows (`Side::windows`) are read outside F41's scope, so
  the gate's readings reach the log (change 3's "one logged row" is
  where this goes); `torus_doors_join::a_rod_across_the_outer_equator_builds_at_its_volume`
  (`JoinPairsTied`) and `square_search` (a seeded fuzz), untraced; and 7
  that pass when run again (one OOM kill, one seeded fuzz, and five
  rows of this branch's own that it then fixed).

**Class 2 by refusal shape** (tests; the first refusal of each test):

| refusal | tests | f64 / Interval |
|---|---:|---|
| `Uncertified`, Departure, at an edge | 512 | 485 / 27 |
| `Uncertified`, Departure, at a vertex | 179 | 172 / 7 |
| `Budget(Pairs)` | 136 | 135 / 1 |
| `Budget(Depth)` | 90 | 86 / 4 |
| `Unsupported`: "a face whose carrier window is not a finite rectangle" | 59 | f64 |
| `Uncertified`, SecondOrder, at an edge | 52 | 51 / 1 |
| `Unsupported`: a spindle sheet's face | 20 | f64 |
| `Unsupported`: an approximated face | 7 | f64 |
| `Sliver` | 1 | f64 |

The departure arm at an edge, by its two faces' kinds: plane/plane 104,
plane/cylinder 100 (and 63 cylinder/plane), sphere/sphere 47,
plane/torus 37, cylinder/cylinder 30, plane/cone 27, cone/cone 22.
Traced to its cell, one plane/plane row (`dsc_checks::a_thin_slab_counts_as_one_component`)
is a slab `(1 + K)·ε` thick: the side's departure at a top edge reads
the bottom face one ε past the band. The other plane/plane rows are not
traced.

**The ten largest groups** (class, shape, faces' kinds; one test):

| tests | group | one test |
|---:|---|---|
| 506 | 3, crossing between solids, plane/plane | `editor-core` `names::emit_topo::a_pairs_flush_line::a_crossing_of_two_flush_edges_running_opposite_ways_reads_each` |
| 163 | 3, crossing in one solid, plane/plane | `a_keel_on_the_floor_names_its_two_edges::a_keel_on_the_floor_names_its_two_edges_apart` |
| 136 | 2, `Budget(Pairs)` | `names::emit_sweep::tests::a_full_revolves_axis_cavities_are_named` |
| 104 | 2, departure at an edge, plane/plane | `dsc_checks::a_thin_slab_counts_as_one_component` |
| 100 | 2, departure at an edge, plane/cylinder | `band_closed_mixed_chain_names::a_rounded_plate_filleted_by_name_names_each_section_once` |
| 90 | 2, `Budget(Depth)` | `names::emit_topo::touch_reread_rows::a_corner_held_as_several_result_vertices_is_a_fragment_at_each` |
| 79 | 2, departure at a vertex, plane/plane | `emit_crossing_sense::a_corner_resting_on_a_face_is_the_corner_in_either_order` |
| 63 | 2, departure at an edge, cylinder/plane | `names::emit_shell::tests::a_joined_edge_over_an_upstream_set_is_one_flat_set` |
| 47 | 2, departure at an edge, sphere/sphere | `mesh` `chords::tests::each_arm_of_the_rule_answers_its_lanes_lattice` |
| 37 | 2, departure at an edge, plane/torus | `band_mixed_corner_names::the_brackets_mixed_corners_name_each_entity_once_and_keep_their_names` |

Next: window not a finite rectangle on a cylinder (37,
`mesh` `lift_probes::the_spline_section_bodies_read_lifts_that_close`),
second order cylinder/cylinder (35,
`a_blend_refusal_names_its_edge::a_knife_edge_refusal_names_the_edge_by_its_stable_name`).

### Outside the workspace

- **The tour walk**: 8 of its 86 stops refuse somewhere in their build
  (against 26 of 85 bodies at the gate row's first measure): `bracket`
  (departure, plane/cylinder edge), `budfillet` (departure, sphere/sphere
  edge), `teapotvessel` (departure, torus/plane edge), `projectbox`
  (`Budget(Pairs)`), and `bench`, `bench60`, `bench90` (crossings
  between solids, class 3). The MC chain's 20,000 samples: 1,026 cross
  between links (10-solid bodies, class 3). The lily, the heatsinks, the
  snowman bodies, `klein_bulb` and `diepips` hold in the walk.
- **The tour suite**: 15 of 106 fail: the three `eps_regression` tour
  rows and `bracket`'s stop (the bracket's edge), `chaintol`'s three rows,
  `certified_cells` and `gallery` (the chain, class 3), `projectbox`'s two
  rows and `review_blend2_r1_probes::p5` (`Budget(Pairs)`), the two
  `snowman` rows with a head moved off the axis (departure, sphere edges),
  `teapot`'s wall walk (torus/plane edge), `verbs_teapot_r2_probes`' two
  rows (window not a finite rectangle, cylinder), `lily::verbs_gate_r1_probes::wall7_…`
  (departure, sphere edge), and `lily::review_probes::the_spine_curl_wall_re_measured`
  (a wall crossing itself: class 1 or 2, not traced).
- **`demos/wild`**: 2 of its 8 cells refuse, both departure at a vertex:
  `805_slide_switch` (plane/plane) and `nist_ftc_09_asme1_rd`
  (cylinder/cone).
- **`tools/tess-meter`**: 2 of 36 fail
  (`a_public_door_mints_the_removed_refusal`: departure, edge;
  `r2_overflow_edge_needle_cone`: `Budget(Pairs)`).

### The price

Summed per ask, in the workspace run: 247,480 asks, 27,344 s of gate
(15,171 s on the 233,719 that held; 12,173 s on the 13,761 that refused or
crossed). The suite took 8,786 s wall on 4 cores. By binary (seconds of
test time, of which the gate):

| binary | tests | test time | gate | gate on refusals |
|---|---:|---:|---:|---:|
| `sweep::all` | 3,718 | 22,758 | 20,255 | 10,672 |
| `editor-core::all` | 3,034 | 5,883 | 4,404 | 260 |
| `viewer::all` | 1,027 | 1,154 | 1,025 | 108 |
| `mesh::all` | 282 | 1,236 | 1,024 | 756 |
| `step-import::all` | 236 | 357 | 276 | 177 |
| `topo::all` | 1,256 | 350 | 126 | 70 |
| `step-export::all` | 60 | 35 | 29 | 0 |

So the gate is about 90% of `sweep::all`'s and 75% of `editor-core::all`'s
test time: every intermediate finish asks, not only the doors' final
bodies. Two `review_d2_adv_probes` rows run 50 minutes each, nearly all of
it the gate (3,467 asks, 635 refusing, many at `Budget(Pairs)` after 10 s
each). The tour walk: 24,258 asks, 706 s of gate in a 2,002 s release walk.

## Lane C stage 2 landed on embed/gate (2026-11-18)

- **One pair budget.** `topo::clearance::GATE_MAX_CELL_PAIRS` (65,536)
  and `GATE_BUDGETS` (with `DEFAULT_MAX_CELL_DEPTH`). The shell's
  `CAVITY_MAX_CELL_PAIRS` and editor-core's `DEFAULT_MAX_CELL_PAIRS` are
  gone; `ClearanceConfig::default()` is `GATE_BUDGETS`, and the three test
  files that restated 65,536 read the constant.
- **Change 3.** `EmbeddingUncertified { refusal, width }`:
  `clearance::blocking_width` reads `Family` where a vertex's coordinate,
  crossed into the engine, is wider than the band's escalation, and
  `Cell` otherwise; the text gives "narrow the parameter box" for the
  first and the kernel-limit recourse for the second. A crossing is read
  again at the body's own scalar (`AtRestBody::embedded`): the two
  carriers at the verifier's stations, decided once at
  `SELF_INTERSECTION_GAP`, the one logged row; apart there it is
  `WitnessUnverified`, undecided it names the width. `embedded` now reads
  the windows inside F41's scope too (the census's `mass_props` goldens
  moved because they did not).
- **Records.** `ContactRecords::face_pairs(body)` replaces the engine-side
  reduction (`[topo.homes]`: the engine names no boolean item). A cell
  reduces to every face holding a vertex of its closure: measured on two
  resting bricks, the face-to-face record alone left the crossing between
  a side face and the other brick's contact face offered.
- **The loft.** `LoftError::SelfIntersecting { meeting, witness, findings }`
  names each face as `LoftedFace::{Top, Bottom, Wall { loop_index,
  segment, slab }}` with the sections it spans and its chart station;
  `BoundaryMeetsItself` carries its two `stations` for it. Python tag
  `self_intersecting`.
- **Pins:** `topo` `the_at_rest_gate_asks_the_certificate` (two resting
  bricks cross with no exclusion and hold off their records' pairs; the
  same bricks refuse the finish `BoundaryMeetsItself`; a slab `(1 + K)·ε`
  thick refuses `EmbeddingUncertified`, width `Cell`); `sweep`
  `the_crossing_lofts_refuse_at_their_door_naming_their_faces` (9 and 13
  rad arc spines: one wall against itself; the closed spine: bottom and
  top caps; CARVETAIL: walls 0 and 3; 0.8 s, in the slow set).
- **Mutants, all run:** no gate call reds the slab, the bricks' finish and
  the lofts; a refusal mapped to `Ok` reds the slab; no exclusion in
  `Sweep::run` reds the bricks' exclusion row.
- **Not pinned:** the `bench` finishes (they wait on the fork: the finish
  holds no records), and the box leaf at `w = 0.1` (the slab stands in as
  the engine-refusal row).

## The gate's price, measured for the price fork (2026-11-18)

The measuring harness is on `embed/gate-measure` (not for merge):
`topo::gate_measure` writes one row per ask to `$CAD_GATE_LOG/<pid>.log`
(test, scalar, faces, receipt, seconds, verdict, the kinds and solids of
the faces a verdict names, and at a Boolean's `gate` the records its door
holds); `CAD_GATE_OFF=1` skips the ask, `CAD_GATE_DECOMPOSE=<min leaves>`
asks each face pair of a holding ask alone, `CAD_GATE_CENSUS=1` times the
tier-3′ census at a Boolean's `gate` with its records. Each item below says
how it was run.

### In-solid crossings and their records (for PR 7068)

Per-solid gate, `embed/gate-measure` at `674724090` (after PRs 7071 and
7089): the 202 workspace tests the census found crossing inside one solid
or with solids unlogged. 161 still fail on a gate finding: 147 on a
crossing, 14 on an engine refusal (departure, before any crossing).

| crossing inside one solid (147 tests) | tests |
|---|---:|
| a Boolean result whose door held a record of two cells of that solid, covering the crossing's face pair | 55 |
| — point records only (vertex–vertex 14, vertex–face 4, vertex–edge 3) | 21 |
| — edge records, with or without points | 32 |
| — an area record (face–face) | 2 |
| a Boolean result whose door held no contact record (11 named for a refusal) | 56 |
| finished elsewhere (split, Euler-built fixtures, loft), records not visible to the harness | 36 |

The harness reads `Held.contacts`, not `Held.records` (operand
declarations), so some of the 56 may hold a declared record.

### Item 5: the refusals' share (census log, whole-body ask, `362cd72fd2`)

| verdict | asks | seconds | mean |
|---|---:|---:|---:|
| holds | 233,719 | 15,171 | 65 ms |
| departure, at an edge | 1,839 | 5,126 | 2.8 s |
| `Budget(Pairs)` | 834 | 5,095 | 6.1 s |
| departure, at a vertex | 1,593 | 1,352 | 0.85 s |
| `Budget(Depth)` | 484 | 468 | 0.97 s |
| crossing | 8,727 | 61 | 7 ms |
| second order | 65 | 59 | 0.9 s |
| sliver | 1 | 12 | 12 s |
| unsupported (window, spindle, approximated) | 218 | 1 | — |

Every `Budget(Pairs)` ask runs to the full budget (65,493 splits on
average). **A refusing ask does not stop at its first refusal.** `Bound`
keeps the first refusal (`lower_refusal.get_or_insert`) and the walk goes
on, looking for a verified crossing, until the frontier empties or the
budget is spent (`walk`, `Walk::split`). The twelve most expensive asks
are all departure refusals at an edge on four- to seven-face torus
bodies (`germ_torus_components`, `germ_sphere_torus_placements`,
`quadric_pair_sections`): 126–498 s each, at 27,000–65,500 splits.

### The distribution (same log)

| faces | asks | holding: asks, s | refusing: asks, s |
|---|---:|---|---|
| 1–6 | 175,019 | 173,422, 5,472 | 1,597, 5,101 |
| 7–12 | 46,132 | 36,472, 3,671 | 9,660, 4,432 |
| 13–24 | 17,626 | 15,679, 1,592 | 1,947, 1,565 |
| 25–48 | 5,197 | 4,752, 2,708 | 445, 937 |
| 49–96 | 2,826 | 2,719, 1,547 | 107, 63 |
| 97+ | 680 | 675, 181 | 5, 75 |

The most expensive holding ask: 40.9 s, 89 faces, 471 candidate pairs,
45,271 splits (`coincidence_door::every_band_a_revolve_mints_is_about_its_axis_output_at_sym`,
at `Sym`).

### Item 4: is the certificate a conjunction of face-pair facts?

Yes, exactly, on every holding ask measured. `embed/gate-measure`,
per-solid gate, `demos/tour` release walk with `CAD_GATE_PASS=1
CAD_GATE_DECOMPOSE=200`: each holding ask with at least 200 cell-pair
leaves (discharged, violated, refused, within) had every face pair asked
alone (`Side::faces = [a, b]`, the two self-pairs excluded; a self-pair as
`[a]`), once per body signature. 250 asks, 2 to 111 faces, among them the
Klein loop, the die chain, lofts, the teapot vessel, revolves and the
impellers.

- The whole ask's candidate pairs are exactly the pairs that are
  candidates alone: 250 of 250.
- The lone asks' leaves, splits and discharges sum to the whole ask's:
  250 of 250 each.
- Every lone verdict holds: 250 of 250.
- So the memos `Sweep::run` shares across pairs (`RegionMemo`,
  `LocalMemo`, `across`, `cornered`, `widths`) change time only: the lone
  asks take 135.0 s against the whole asks' 112.2 s.
- Refusing asks were not decomposed: `walk` refuses a whole level, so a
  pair's spend there is not defined.

**The budget's two numbers.** The largest spend of any single face pair
on a holding body is 12,774 leaves, 19.5% of 65,536 (the Klein loop, a
NURBS wall against a NURBS wall; the whole ask is 51,322, 78%). On the
largest holding bodies the receipt grows linearly with faces: the die
chain at 76 to 111 faces takes 4,092 to 6,283 leaves (about 57 a face,
3.4 candidate pairs a face, largest pair 77 to 88); impeller-like bodies
of 76 to 86 faces take 6,791 to 8,141 (largest pair 91). One 89-face tour
body takes 45,742, spread over many pairs (largest 557).

Repeat: `cd demos/tour && CAD_GATE_LOG=<dir> CAD_GATE_PASS=1
CAD_GATE_DECOMPOSE=200 cargo run --release -- <out>` on
`embed/gate-measure`; the `DECOMP` rows hold each ask's sums.

### Item 2: per-ask price at CI's build against release

The same tour walk at the dev profile with libraries at opt-1 (CI's
`CARGO_PROFILE_DEV_OPT_LEVEL=1`), set ask by ask against the release
walk: 1,542 asks matched by sequence and signature (faces, candidates,
splits, discharges) before a worker restart stopped the dev walk at stop
70 of 85. CI's build takes 601.0 s on them against release's 490.5 s,
1.23×; on the 1,533 that hold, 271 ms against 229 ms an ask. So the build
is not the gap between the tour's 29 ms and the workspace's 65 ms an ask:
that gap is which bodies are asked.

### Item 3, partial: where the asks come from and what a door carries (2026-11-18)

**Partial, and stopped on purpose** (the interim answered what it was
for). `embed/gate-measure`, full suite with `CAD_GATE_CARRY=1
CAD_GATE_CENSUS=1`, stopped after 3,012 of 19,460 tests: 2,521 of
`editor-core::all`, all 422 of `editor-core`'s unit tests, 57 of `bvh`
and 12 of `sweep::all`. So the tables below are editor-core's; `sweep` is
not measured. Five tests were killed for memory under the decomposition
(two `coincidence_door` rows, `corpus_finish_count`, `docm5_subject`,
`germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint`);
their asks up to the kill are counted.

Every finish site is tagged (`gate_measure::site`). Gate seconds by site:

| site | asks | seconds | share | holding: seconds |
|---|---:|---:|---:|---:|
| Boolean `gate` | 46,696 | 3,147 | 57.0% | 2,085 |
| blend | 708 | 1,448 | 26.2% | 1,337 |
| revolve | 1,846 | 285 | 5.2% | 277 |
| extrude | 40,898 | 179 | 3.2% | 179 |
| a moved leaf (`LeafBody::moved_by`) | 18,468 | 177 | 3.2% | 177 |
| the product's aggregate | 5,243 | 153 | 2.8% | 153 |
| `topo::finish` called directly (fixtures) | 1,939 | 65 | 1.2% | 65 |
| the `⊔` graft | 6,635 | 49 | 0.9% | 49 |
| split | 1,355 | 15 | 0.3% | 15 |
| shell | 45 | 6 | 0.1% | 6 |
| loft and sweep | 139 | 2 | 0.0% | 2 |
| `distribute`, `topo::take`, `carved` | 50 | 0 | — | 0 |

The leaf doors (extrude, revolve, loft, a moved leaf) hold about 15% of
the holding seconds; the deciding doors about 80%.

**The carry fraction.** At the four deciding doors every holding ask's
candidate face pairs were asked alone (exact, item 4) and classed by their
faces against the door's operands, matched by a fingerprint of the
`Debug` bits of carrier and sense and of every rim's curve and start
point: *carried* (both faces bit-identical to faces of one operand),
*trimmed* (both on one operand's carriers and senses, at least one with
new loops), *two operands*, *new* (a face on no operand's carrier).
Shares of the lone-ask seconds:

| door | carried | trimmed | two operands | new |
|---|---:|---:|---:|---:|
| Boolean `gate` (1.70 M pairs) | 65% | 16% | 8% | 11% |
| blend | 5% | 18% | — | 77% |
| split | 1% | 73% | — | 27% |
| shell | 8% | 4% | — | 89% |

A Boolean's untouched faces do match their operand's bit for bit: that is
where the 906,507 carried pairs come from.

**Item 6, partial: the census at a Boolean's `gate`.** The tier-3′
census (`AtRestPolicy::census_at_rest`) over each finished Boolean result
with its door's records: 28,033 runs, 30 s in all (the gate there took
3,147 s), 814 of them with records. It refused 17 bodies the gate had
finished: 11 asks of `atrest_operation_rows_quiet::a_rod_on_a_plate_is_quiet_under_its_gap`
(3 records, 7 findings), 5 of `face_face_touch_carry::a_later_op_on_a_rod_resting_on_a_plate_carries_its_cap_records`
(3 records, 3 findings), and 1 of `docm6_seam_declarations::a_boolean_over_a_product_keeps_its_parts`
(no records, 1 finding). Not traced.

Repeat: on `embed/gate-measure`, `CAD_GATE_LOG=<dir> CAD_GATE_CARRY=1
CAD_GATE_CENSUS=1 cargo nextest run …`; the `CARRY` and `CENSUS` rows,
summed by `site`, give these tables.

### Fork `embed-price`: what a finish asks

Raised by the gate lane's census on 2026-11-18: the gate as built is
247,480 asks and 27,344 s over the workspace's suites, about 90% of
`sweep`'s test time, because every finish proves its whole body again.
Two designers hold one design after six rounds and split on the
budget's unit (`docs/DESIGN-FORK-LOG.md`, row 286); the design is in
`docs/DESIGN.md` `[topology.tiers]`.

- **The certificate is one fact per pair of faces**, of the two faces
  and the cells they share. `finish` takes the body with the finished
  operands it was built from, compares faces exactly itself (surface,
  sense, loops, edge geometry, which cells the two share), carries the
  fact of every pair it finds unchanged in one operand, and asks the
  engine about the rest. The door declares nothing.
- **Asked, always:** a pair with a new face; a pair from two operands;
  a pair sharing a cell the door made or changed; every pair with a
  face the door trimmed. Trimmed pieces of operand faces are not
  carried.
- **Regrouping doors and moves ask nothing** (the `⊔` graft, a
  product, `distribute`, `carved`, the graft in `boolean/material.rs`,
  a moved leaf). A leaf door asks its whole body.
- **The same body gets the same verdict by any route.**
- **Pinned before anything reads a carried fact:** a pair asked alone
  gives the verdict it has in the whole ask, and on asks that hold the
  pairs' cell-pair counts sum to the whole receipt.
- **The budget stays a fixed constant on one ask, and nothing is
  stored with a fact** (Ev, PR 7101, 2026-11-18: "a 'don't make the
  program hang forever' backstop rather than a sharp bound"). It
  counts only what that finish asks; a carried fact costs nothing. A
  recipe of n ops spends at most n times the constant. A budget
  refusal therefore follows the route: a chain can hold where the same
  shape built in one op, or imported, runs out. Item 2's "one fixed
  pair budget" stands as written.
- **Measured** (the gate lane, 250 holding asks of the tour, every
  face pair asked alone): the lone candidates are the whole ask's, the
  lone leaves, splits and discharges sum to the whole's, and every
  lone verdict holds, 250 of 250. The pin above is that measurement
  made a test.

