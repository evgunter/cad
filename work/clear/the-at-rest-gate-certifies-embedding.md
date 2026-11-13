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

**Order:** A, then B with the engine rows below, then C, then D and E
in parallel. The engine rows (the 23 refusals of table 1) must land
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

Change 2, at the three sites of section 4. Rows touched: this one.
- **Sites:** `is_zero` / `sign_on`, `present_chart`'s apex arm and
  `cusp` (`injective.rs`). The facts come from
  `validate_geometric_certificate`, `contact_marks` and check 6
  (`validate.rs`), through `AtRestBody::validate`.
- **Pins:**
  - a body whose family passes tier 3 while a Zero site refuses. None
    was found here (table 3), so the lane builds one first. The box at
    `w = 0.1` is the place to start: today it runs out of pairs after
    1.5 s, and after the lane it reads `Holds` or refuses naming the
    family's width (change 3);
  - a cone apex over a box (`topo::test_support` `cone_cap`);
  - the declared wedge-0 kiss in `self_intersection_proves_injectivity.rs`
    keeps its typed refusal, now read off the mark.
- **Mutants:** every edge read as `Tangent` (the cone and pyramid
  `Holds` rows refuse as a cusp); an empty apex map (the cone row
  refuses `CollapsedChart`); `is_zero` always true (the carvetail fold
  and the folded cut stop reading `Violated`).
- **Seams:** TOPO's `validate.rs`, for the hand-over of marks and
  singular vertices.

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
- **Item 3:** `Side` (`clearance.rs` ~373) gains a face-pair
  exclusion, built from the body's `ContactRecords` (`records.rs`
  ~316: `face_face`, `edge_face`, `vertex_face` and the rest, each
  cited cell reduced to its faces). The engine never offers an
  excluded pair.
- **The loft:** `LoftError::SelfIntersecting` names the two faces by
  their `Lofted` roles (`loft.rs` ~131: `side_faces[l][j][s]`, `top`,
  `bottom`) and the section range each meets in.
- **Pins:**
  - the 9 and 13 rad arc spines, the closed spine and CARVETAIL's
    loft refuse at their door at `f64` with `BoundaryMeetsItself` and
    the roles of table 2. Today they ship;
  - `bench`, `bench60` and `bench90` finish: their contacts are
    excluded. Today the question reads `Violated` at 0;
  - an engine refusal reads `EmbeddingUncertified`: the box leaf at
    `w = 0.1`, until lane B moves it.
- **Mutants:** no gate call (the spine rows ship); a refusal mapped to
  `Ok` (the uncertified row ships); no exclusion (the bench refuses).
- **Gates:** every suite, the tour walk (85 bodies) and `demos/wild`.
  The price is table 1's: 4–25× tier 3 on planar bodies, and up to
  9.2 s on the Klein loop.

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
