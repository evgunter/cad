---
id: the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause
kind: issue
title: The injectivity check's sliver reads a cell that reaches past a shared line by rounding as holding none of its face, and certifies the pair with neither face read; it retires once the departure reading is metered at the faces' reach
status: open
opened: 2026-11-13
priority: P1
cost: M
design: true
---



## Finding

From CLEAR's lane clear-vacuous, while it traced
`a-half-sphere-reads-vacuous-in-its-own-window-so-a-departure-pair-certifies-on-a-zero-width-rectangle`
(PR 6901).

`topo::injective`'s module docs list the sliver among the engine's
concessions:
- a presentation rectangle whose width across a shared line decides
  `Zero` at `SELF_INTERSECTION_SLIVER` reads as holding nothing off the
  line;
- the cell's face is then `Presentation::Vacuous`;
- the departure arm certifies a pair outright when either face is
  vacuous.

Three readers apply it: `present_chart`, `present_line_corner`, and,
since PR 6894, `present_plane_corner`. 6894's lane measured that
`chute` needs it.

DESIGN.md's finished-body clause (`[topology.tiers]`) says: "A reading
inside the band refuses as ambiguous, never as Holds or Violated." The
sliver is a reading inside the band whose consequence is a certified
pair, so on its face it contradicts that sentence.

There may be a reading under which both hold. The dropped points lie
within the band of a shared edge, so a crossing they could hide is
within K·ε of the boundary, and the same clause holds that apart only
"by more than the band". Nobody has argued that, or checked it against
the clause's provenance. The clause came from Ev's embedding ruling,
`31091b6747`.

## What is needed

Decide whether the sliver is consistent with the clause.
- **If it is,** argue it in the module docs and cite the clause. This
  is a faithful elaboration.
- **If it is not,** this is a design fork. The cost is the bodies that
  hold only through it: measure them, starting with `chute` (PR 6894),
  `present_line_corner`'s pins, and the tour walk.

Until this is decided, no new reader adopts the sliver.

## Fork `embed-sliver`: the sliver retires

Three designers, three rounds (`docs/DESIGN-FORK-LOG.md`, row 285);
the clause is `docs/DESIGN.md` `[topology.tiers]`.

**The premise above is corrected.** The sliver decides on `Zero`, a
width of at most ε, never on a width inside the band, so it is not the
reading the clause refuses. Measured (below): every one of its 96
`Zero` decisions in a sweep is at rounding scale, on a cell that lies
across the shared line from its face, with the other face unpresented,
so the pair certifies with neither face read.

**What it is evidence of.** `sign_on` decides a departure by the rate
times the presentation rectangle's own span. A thin rectangle cannot
pass whatever the faces do, a narrower cell makes it worse, and a sweep
cell is not a lever the kernel admits. The sliver waves those cells
through. It is also a `Zero` taken from a cell bound, which PR 6821
rules out.

**The final state.** The departure reading is metered at the reach of
the faces it governs. `Presentation::Vacuous` means only that the cell
lies off the face. The sliver's funnel site, its entry in the module's
"ε concessions" and the rows that assert `Vacuous` through it go.

**The order.**
1. The margin is metered at the faces' reach. Measured by proxy (the
   rate's sign decided at the least band, the sliver left on): no
   verdict moves in the tour or the suites, and 40 tour bodies take
   fewer pairs. `a_rod_less_a_tilted_slab_holds_across_its_curved_corners`
   pins a count that moves (14 corner certificates to 18).
2. `present_line_corner`'s and `present_plane_corner`'s slivers go
   with it: no sweep reaches the first, and `chute`, the stepped ring
   and `tiltedcut_above` hold without the second, each pair by the
   carrier arm with the thin rectangle read.
3. `present_chart`'s goes once the filleted drum holds at `Interval`
   without it
   (`the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam`).

Until step 3 no new reader adopts the sliver. The touch concession
(`few_ulps`, in `vacuous` and `present_chart`) is the same compensation
at chart scale and does not retire here: at 0, with the sliver gone,
`hollowring`, `hollowtorus` and `teapotlid` refuse.

## Measured: what holds only through the sliver (EMBED, lane sliver-measure, 2026-11-17)

Main at `362cd72fd`. Nothing here judges the concession: it records
what depends on it. Default ε row only (ε = 1e-9 m, K·ε = 1e-8 m).

**In short.**
- Three bodies hold only through the sliver, and two unit pins assert
  it: the tour's `chute`, the stepped ring (both scalars), and the
  filleted drum at `Interval` (its `f64` build takes no sliver
  decision).
- Each depends on one reader: `present_plane_corner` (`chute`, the
  ring) or `present_chart` (the drum). **`present_line_corner`'s sliver
  decided `Zero` in no sweep** of the tour or the suites; only its own
  unit pin reaches it.
- **Every `Zero` was taken at rounding scale**: the widest in any sweep
  is 3.8e-15 m, against ε = 1e-9 m. None was taken between 1e-14 m and
  the band.
- In every traced cell the cell lies **across the shared line from the
  face**, and the strip is the rounding (or the interval enclosure) of
  where the line is placed. The face has nothing in it but the line's
  own edge.

### What was run

- **The tour walk**: 85 bodies, `topo::clearance::embedded`, `f64`,
  depth 40, 65,536 pairs, release profile, `RAYON_NUM_THREADS=1`, one
  core per walk (up to four walks at once on a 4-core box). "Pairs" is
  the receipt's `discharged + violated + within + refused`; it
  reproduces the gate row's `die` 6,283, `table` 1,586, `az` 1,631.
- **The suites**: all of `-p topo -p sweep` under nextest's default
  profile once (8,101 tests), with one log line per injectivity sweep.
  59 tests run such a sweep (248 sweeps): 42 in
  `self_intersection_proves_injectivity`, 5 in `shell_cavity_embedded`,
  5 in `a_seamless_pipe_cut_by_an_oblique_plane_measures`, 4 in
  `sf2b_interval_probe`, 2 in `torax_interval`, 1 in
  `clearance::wiring_rows`. The mutants then ran over the 758 tests
  whose names match
  `self_intersection_proves_injectivity|shell|cavity|a_seamless_pipe_cut|sf2b_interval_probe|torax_interval|injective|clearance|recourse`,
  which holds those 59 and every unit test of `injective.rs` and
  `clearance.rs`. The three readers are `pub(crate)` and reached only
  from those two files, so no other topo or sweep test can move.
- **Not run**: `editor-core`'s suites (its `self_intersection` door),
  `demos/tour`'s own tests, the 1e-6 and 1e-12 rows.

### 1. Baseline

- **Tour: 82 of 85 hold**; `bench`, `bench60` and `bench90` read
  `Violated` between faces 1 and 8, as recorded. **`klein_bulb` holds
  on this main** (23,574 pairs, deepest 22, 1.9 s), where the program
  page's count of 81 has it out of pairs. The walk's certificates cost 95 s (98 s on a
  repeat, with identical pairs).
- **Suites: the 758 pass.** Of the 8,101, 8 fail on the instrumented
  build with no mutant armed. None of them runs an injectivity sweep or
  reaches a reader, so the patch does not act in them; whether they are
  red on a clean main was not checked:
  `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim`,
  `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`,
  `germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint`,
  `mass_props_are_thread_count_invariant` (both rows),
  `torus_doors_join::a_rod_across_the_outer_equator_builds_at_its_volume`,
  `section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
  and `link::cone_fuzz::the_consumers_paths_never_contradict_the_exact_oracle`
  (the last two are seeded fuzzers).

### 2 and 3. What moves, and on which reader

"R" is refused, "H" holds as on the baseline. Every refusal is
`Uncertified`, arm `Departure`, first raised at depth 40 (the floor).

| body | all three refused | `present_chart` alone | `present_line_corner` alone | `present_plane_corner` alone |
|---|---|---|---|---|
| tour `chute` | R | H | H | R |
| stepped ring, `f64` (`a_stepped_rings_floor_holds_against_its_end_caps`) | R | H | H | R |
| stepped ring, `Interval` (same test) | R | H | H | R |
| filleted drum, `Interval` (`a_filleted_drum_certifies_across_its_fillet`) | R | R | H | H |
| filleted drum, `f64` (same test) | H | H | H | H |
| unit pin `presentation_rows::a_refused_lever_measures_no_sliver` | fails | fails | passes | passes |
| unit pin `presentation_rows::a_line_corner_cell_reaching_the_line_from_outside_reads_vacuous` | fails | passes | fails | passes |

| body | refusal (the pair, the shared cell) | pairs, baseline → refused | deepest | time, baseline → refused |
|---|---|---:|---:|---:|
| `chute` | faces 1 and 6, edge 20 | 1,794 → 2,131 | 19 → 40 | 0.15 s → 0.24–0.27 s |
| stepped ring, `f64` | faces 1 and 4, edge 14 | 386 → 747 (10 leaves refused) | 12 → 40 | the test, both scalars: 0.14 s → 0.66 s |
| stepped ring, `Interval` | faces 1 and 4, edge 14 | 386 → 1,646 | 12 → 40 | (same test) |
| filleted drum, `Interval` | faces 1 and 7, edge 12 | 567 → 55,123 (9,660 leaves refused) | 15 → 40 | the test: 0.44 s → 26–29 s |

- **Nothing else moves.** The other 81 tour bodies that hold, and the
  other 57 sweep-running tests, give the same verdict under every
  mutant.
- `tiltedcut_above` takes two sliver decisions (`present_plane_corner`)
  and does not depend on them: it holds with them refused, at 1,837
  pairs for 1,831.
- The two unit pins call the readers directly. The chart pin's cell is
  a synthetic 1e-13 m strip on a cylinder; the line pin's cell reaches
  its line from outside, with a strip of width 0.
- **A second spelling of the mutant** (the reader goes on to present
  the thin rectangle instead of refusing the cell) moves the same four
  tests and `chute`, with the same refusals and the same pair counts.

### 4. How often, and how wide

Sliver decisions asked and decided `Zero`, baseline. Widths are the
upper end of the decided margin, in metres.

| scope | reader | asked | decided `Zero` | widest `Zero` | in units of ε (1e-9 m) |
|---|---|---:|---:|---:|---:|
| tour, 85 bodies | `present_chart` | 183,268 (65 bodies) | 0 | none | none |
| tour, 85 bodies | `present_line_corner` | 267 (4 bodies) | 0 | none | none |
| tour, 85 bodies | `present_plane_corner` | 71 (5 bodies) | 22 (2 bodies) | 7.0e-16 | 7.0e-7 |
| suites, 248 sweeps | `present_chart` | 30,616 | 16 (1 sweep) | 3.8e-15 | 3.8e-6 |
| suites, 248 sweeps | `present_line_corner` | 1,279 | 0 | none | none |
| suites, 248 sweeps | `present_plane_corner` | 150 | 58 (2 sweeps) | 1.3e-15 | 1.3e-6 |

Per body, the `Zero` decisions by width:

| body | reader | `Zero` | exactly 0 | [1e-17, 1e-16) | [1e-16, 1e-15) | [1e-15, 1e-14) | widest |
|---|---|---:|---:|---:|---:|---:|---:|
| `chute` | plane corner | 20 | 9 | 0 | 11 | 0 | 7.0e-16 |
| `tiltedcut_above` | plane corner | 2 | 0 | 2 | 0 | 0 | 9.5e-17 |
| stepped ring (each scalar) | plane corner | 29 | 16 | 0 | 10 | 3 | 1.3e-15 |
| filleted drum, `Interval` | chart | 16 | 0 | 0 | 0 | 16 | 3.8e-15 |

- 20 tour bodies ask no sliver decision; 63 ask and never decide
  `Zero`.
- With the readers refused the walk meets the same strips at every
  depth (`chute` 339 `Zero` decisions, the drum 43,940), and the widest
  stays the same: 7.0e-16, 1.3e-15 and 3.8e-15 m.
- The widest `Zero` anywhere is the chart unit pin's synthetic cell,
  1.0e-13 m.

### 5. Three cells traced

Face and edge ids are the arena's, read off a dump of the body
(`chute`) or off the revolve's face order, which `chute`'s dump shows
(the ring).

**`chute`** (a C-channel revolved 270° about `y`, from the `+x` axis to
the `+z` axis; the missing quarter is `x > 0, z > 0`).
- **Face**: face 6, the channel's floor, the plane `y = 0.1875`
  between radii 1.1875 and 1.5625.
- **Line**: edge 20, the floor's radial line on the end cap (face 1,
  the plane `x = 0`), from `(0, 0.1875, 1.1875)` to
  `(0, 0.1875, 1.5625)`. The reader is `present_plane_corner`, at the
  line's corner with the floor's inner arc (edge 12, vertex 12).
- **Cell**: `x ∈ (0, 0.0015)`, `z ∈ (1.1871, 1.1887)`: in the missing
  quarter, touching the line from outside. In the reader's affine
  chart (the line `y = 0`, the face `y > 0`) the cell's hull is
  `y ∈ (−0.0041, 1.2e-15)`.
- **Width**: 4.4e-16 m (the hull's 1.2e-15 past the line, times the
  chart's speed 0.375).
- **The strip in the model**: the floor has no point in the cell off
  edge 20. The 1.2e-15 is the rounding of the cell's hull against a
  line whose end vertices are stored at `x = −2.2e-16` and
  `x = −2.9e-16`. In 9 of the 20 decisions the hull ends on the line
  exactly and the width is 0.
- **Refused**: the pair (face 1, face 6) at this corner never
  presents face 6's cell, splits to depth 40 and refuses there, the
  cell above being the refusing one.

**The stepped ring** (an L section revolved 270° about `y`).
- **Face**: face 8, the bottom annulus, the plane `y = 0` between
  radii 1 and 2.
- **Line**: edge 18, its radial line on the end cap (face 1), read at
  its corner with a rim arc (edge 7, vertex 7).
- **Cell**: `x ∈ (0, 0.5)`, `z ∈ (0, 1)`: in the missing quarter. Its
  hull in the affine chart is `y ∈ (−0.5, 1.3e-15)`.
- **Width**: 1.3e-15 m, the widest of the ring's 29. 16 of the 29 are
  exactly 0.
- **The strip in the model**: as on `chute`, the annulus has no point
  in the cell off the line.
- **Refused**: at faces 1 and 4 (the step's floor, `y = 0.5`), edge
  14, in a cell at `(0, 0.5, 1.5)`, where the floor's line meets the
  step's wall: the corner the test's own docs call reflex on the cap.

**The filleted drum at `Interval`** (the unit disc extruded to 2, its
top rim filleted at 0.3).
- **Face**: face 7, the torus band.
- **Line**: edge 12, the circle the band shares with the top plane
  (face 1), a coordinate line of the band's chart at minor angle
  `π/2`. At `Interval` the line is placed at
  `[1.5707963267948861, 1.5707963267949072]`, 2.1e-14 rad wide. The
  reader is `present_chart`.
- **Cell**: one octant of the major angle, minor angle `(π/2, π)`:
  past the band's extent. Clipped to the line's upper bound it is
  `[1.5707963267948968, 1.5707963267949072]`, 1.0e-14 rad.
- **Width**: 3.8e-15 m (1.0e-14 rad times the chart's speed bound,
  0.367 m/rad). All 16 decisions are this strip, two per octant.
- **The strip in the model**: the band ends on the circle, so the
  strip holds only the circle's own edge: it is the enclosure of where
  the line is. At `f64` the line is one value, the same cells are
  read without a sliver decision, and the body holds under every
  mutant.
- **Refused**: at faces 1 and 7, edge 12, depth 40, after 55,123
  pairs.

### How to repeat

The patch is not committed. It was kept, for this session only, at
`~/.local/share/cad-work/sliver-measure/sliver-measure-harness.patch`.

- **A probe module**, `crates/topo/src/sliver_probe.rs`, `pub mod` in
  `topo`'s `lib.rs`: per-reader atomic counters (asked, decided `Zero`,
  the widest `Zero`, a decade histogram), the first refusal's depth,
  the deepest pair, and three environment switches read once:
  - `SLIVER_REFUSE` = any of `chart`, `line`, `plane`, or `all`;
  - `SLIVER_MODE=present` for the second spelling;
  - `SLIVER_TRACE=<body>` (or `*`), printing each `Zero` decision's
    face, cell, rectangle, line and width, each vacuous certificate's
    pair, and the first refusal's cells;
  - `SLIVER_LOG=<file>`, appending one line per injectivity sweep:
    the thread's name (the test, under nextest), the verdict, the
    receipt and the counters.
- **The mutant**: in each of the three readers, where the sliver's
  `decide` returns `Ok(Sign::Zero)`, the reader returns
  `Err(Unpresented::Local)` when its switch is on, instead of
  `Ok(Presentation::Vacuous)`. Under `SLIVER_MODE=present` it falls
  through to the reader's `Presentation::Of`.
- **Counters' hooks in `clearance.rs`**: the deepest pair after `walk`
  in the sweep; the pair's depth at `Outcome::Refuse` and where a
  split stops; the log line where the sweep builds its `BoundReport`,
  when the question is the injectivity one.
- **The tour walk**: `demos/tour/examples/embedwalk.rs`, with
  `walk_tour` and `TourStop::name` made `pub`. For each body of each
  stop it resets the counters, calls `embedded` on every face at
  `Band::linear(tol)` with `DEFAULT_MAX_CELL_DEPTH` and 65,536 pairs,
  times it, and writes one TSV row. Built with
  `cargo build --release --example embedwalk` in `demos/tour`; run as
  `SLIVER_REFUSE=<r> RAYON_NUM_THREADS=1 taskset -c <core> embedwalk out.tsv`.
- **The suites**: built and run with ci.yml's dev-profile settings
  (`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`, debug line tables,
  `CARGO_INCREMENTAL=0`):
  `SLIVER_REFUSE=<r> SLIVER_LOG=<file> cargo nextest run -p topo -p sweep --no-fail-fast -E 'test(/<the names above>/)'`.
  The traces are the two sweep tests alone, `--no-capture`, with
  `SLIVER_TRACE='*'`.
- Times are one run each, with other walks or tests on the box's
  other cores.

### 6. The threshold, and each `Zero` decision's far side and partner

**The threshold is ε, not K·ε.**
`decide(SELF_INTERSECTION_SLIVER, Margin::of(width), band)` reaches
`Interval::sign_within`, which reads `Zero` exactly when the width's
enclosure lies in `[−band.zero(), band.zero()]`, and `Band::linear`
sets `zero` to ε and `escalate` to K·ε. An enclosure reaching past ε
is not `Zero`.

- The largest width that decided `Zero` in any sweep is 3.8e-15 m:
  3.8e-6 of ε and 3.8e-7 of K·ε.
- Of the 96 `Zero` decisions in sweeps, **all 96 are at rounding scale
  and none is at the order of ε**: 41 are exactly 0, 2 lie in
  [1e-17, 1e-16), 31 in [1e-16, 1e-15) and 22 in [1e-15, 1e-14).

**(a) The strip's far side** (the bound opposite the shared line).

| body | reader | `Zero` | far side |
|---|---|---:|---|
| `chute` | plane corner | 20 | 11 the cell's own hull; 9 the line itself (the hull ends at or below the line, width 0) |
| `tiltedcut_above` | plane corner | 2 | 2 the cell's own hull |
| stepped ring, each scalar | plane corner | 29 | 13 the cell's own hull; 16 the line itself |
| filleted drum, `Interval` | chart | 16 | 16 the cell's own side, which lies inside the line's enclosure |

- **No strip's far side is one of the face's own clips.**
- The plane-corner reader's rectangle is the cell's affine hull, so
  its far side is the sweep's cut, read through that hull.
- The drum's row is read off section 5's trace (the rectangle's low
  bound is the cell's own bound, `1.5707963267948968`, inside the
  line's `[…861, …072]`). The probe's own test of the opposite side
  was miswritten for this reader in the run; it is corrected in the
  kept patch.

**(b) The partner face in that cell.**

| body | `Zero` | (i) vacuous by another path | (ii) a sliver on the same line | (iii) presented with the line a zero side | (iv) anything else |
|---|---:|---:|---:|---:|---:|
| `chute` | 20 | 0 | 0 | 0 | 20 |
| `tiltedcut_above` | 2 | 0 | 0 | 0 | 2 |
| stepped ring, each scalar | 29 | 0 | 0 | 0 | 29 |
| filleted drum, `Interval` | 16 | 0 | 0 | 0 | 16 |

- **All 96 are (iv), and all 96 the same way: the partner is not
  presented, `Err(Unpresented::Local)`.** The pair certifies at
  `departure`'s vacuous return, before any separator is built: no arm
  reads either face.
- The (iv) pairs:
  - `chute`: the annuli (faces 4, 6, 8, 10) against the two caps
    (faces 1 and 2), on the eight radial lines (edges 2, 4, 6, 8, 18,
    20, 22, 24), depths 5 to 14. The first: faces 2 and 10, edge 8,
    the annulus's cell `u ∈ (−0.875, −0.4375), v ∈ (−0.875, 0)`,
    depth 5.
  - `tiltedcut_above`: face 5 (the sliver's) against faces 8 and 10,
    on edges 16 and 10, both at depth 7; face 5's cell
    `u ∈ (−0.3232, −0.2790)`, `v ∈ (−0.1768, −0.0884)` and its mirror.
  - the stepped ring: the annuli (faces 4, 6, 8) against the caps
    (faces 1 and 2), on edges 2, 4, 6, 14, 16, 18, depths 6 to 10.
  - the drum at `Interval`: the band (face 7) against the top plane
    (face 1), edge 12, the eight octants, twice each, all at depth 7.

### 7. Is the sliver standing in for a reading the departure arm could make?

**The proxy.** In `sign_on`, the departure's margin is the separator's
rate along the chart axis times the rectangle's span, decided against
the band. The proxy drops the span and decides the rate at
`Band::least()` (an exact sign: only an enclosure straddling zero is
undecided). It is applied to `departs` (a zero side, a side past the
shared cell, a curve, a corner point) and to the two-sided corner's
mixed derivative. A separator read by its value (`Zero::None`) is
unchanged. "Sliver refused" below is the second spelling of section
2: the reader goes on to present the thin rectangle.

**First, what stops the pairs with the margin as it is** (sliver
refused, margin scaled). The partner is unpresented, the thin
rectangle is presented with the line as its zero side, and the carrier
arm is tried with the partner's carrier as the separator:

| body | pair visits | how they end |
|---|---:|---|
| `chute` | 339 | 303 split: **the rate is signed, the margin is in the band**; 36 certify by the carrier arm (the partner presented too) |
| stepped ring, `f64` | 396 | 341 split, rate signed and margin in the band; 35 carrier arm; 20 the partner vacuous by another path |
| filleted drum, `Interval` | 43,940 | 43,880 split with the top plane presented by its curve and **a rate enclosure that straddles zero** on one or both faces; 60 split with the top plane unpresented |

No pair is stopped by `zero_of` or `sides_are_the_faces`: the readers
reach their `Presentation::Of`.

**Sliver refused, margin un-scaled.**

| body | verdict | pairs | how the baseline's sliver pairs end |
|---|---|---:|---|
| `chute` | **Holds** | 1,790 (1,794 with the sliver) | all 20 certify by the carrier arm: the cap's carrier separates, the annulus's thin rectangle read, rate signed |
| `tiltedcut_above` | Holds | 1,761 | both certify by the carrier arm |
| stepped ring, `f64` and `Interval` | **Holds** | 383 (386) | all 29 certify by the carrier arm, rate signed |
| filleted drum, `Interval` | **Refused**, `Uncertified`, faces 1 and 7, edge 12, depth 40 | 15,200 (55,123 scaled) | 321 visits certify by the graph separator; 11,512 split on a rate enclosure that straddles zero (11,275 of them on both faces); 60 split with the top plane unpresented and the band's rate straddling zero |

- **What the reader makes of a cell across the line**: the
  plane-corner reader presents the rectangle `y ∈ [0, y₁]` with `y₁`
  0 or rounding, its zero side the line. Its rate against the cap's
  plane is signed, so un-scaled it certifies; scaled, the margin is
  that rate times a span of at most 2.2e-15 and reads in the band.
- The drum's band meets the top plane tangentially on edge 12, and
  there the rate's enclosure straddles zero at every depth.
- The two unit pins fail as before (they assert `Vacuous`).

**Sliver on, margin un-scaled, everything else.**
- **Tour: no verdict moves** (82 hold, the benches as recorded). 40
  bodies take fewer pairs and none more: 479,869 pairs in all become
  428,818 (`diefillet` 3,833 → 1,035, `budfillet` 3,423 → 1,299,
  `sheave` 2,314 → 2,002).
- **Suites: no sweep's verdict moves.** 55 of 248 sweeps change their
  pair count (66,161 → 64,413 in all). One test fails on a count it
  pins, not on a verdict:
  `a_rod_less_a_tilted_slab_holds_across_its_curved_corners` reads 18
  corner certificates where it pins 14.

### 8. The same with the touch's `few_ulps` at 0

**The change.** The four `few_ulps` terms of the touch concession read
0: the two in `vacuous` and the two in `present_chart`'s "past the
line on its face side" test. The sheet's overlap, the cover and the
join keep theirs.

| run | tour | suites (sweep verdicts) |
|---|---|---|
| touch at 0 alone (sliver on, margin scaled) | **5 move**: `sheave` out of pairs; `budfillet`, `teapotlid`, `fivewall`, `fivewallcup` `Uncertified` at depth 40 | filleted drum `f64` `Uncertified` (faces 1 and 7, edge 12); full torus `f64` out of pairs |
| touch at 0, margin un-scaled, sliver on | **none moves** | full torus `f64` out of pairs |
| touch at 0, margin un-scaled, sliver refused | **3 move**: `hollowring` and `hollowtorus` out of pairs, `teapotlid` `Uncertified` (faces 6 and 11, edge 26) | filleted drum `Interval` `Uncertified` (as in section 7); full torus out of pairs at `f64` and at its depth floor at `Interval` |

- **With the touch at 0 the sliver takes decisions it did not take
  before.** Tour `Zero` decisions, chart reader: 0 on the baseline,
  948 with the touch at 0 alone (7 bodies: `sheave` 48, `diecomposed`
  176, `budfillet` 24, `klein_bulb` 8, `hollowring` 304,
  `hollowtorus` 274, `teapotlid` 114). All are still at rounding
  scale: the widest is 4.4e-16 m.
- With the margin un-scaled and the sliver on, those bodies hold and
  the chart reader still decides `Zero` 864 times across 6 of them.
  With the sliver then refused, `hollowring`, `hollowtorus` and
  `teapotlid` do not hold: their thin rectangles are presented, and
  the pairs split on a rate enclosure that straddles zero (17,082 of
  `teapotlid`'s 17,778 visits) or on a partner that is not presented
  (5,684 of `hollowring`'s 6,952).
- The filleted drum at `f64` under "touch at 0, un-scaled, sliver
  refused" holds: its 16 thin rectangles certify by a plane through
  a shared circle. That circle is not the tangent seam of section 7:
  section 9 says which it is and what was decided.
- The line and plane-corner readers' counts do not change in any of
  these runs.

### How to repeat sections 6 to 8

The kept patch adds to the probe of "How to repeat":
- `SLIVER_UNSCALED=1`: section 7's proxy, in `sign_on`.
- `SLIVER_ULPS0=1`: section 8's change, through a `touch_ulps` helper
  at the four sites.
- **Tallies**, taken with the counters: per `Zero` decision the far
  side; per pair in `departure` in which a sliver decided `Zero`, the
  partner's presentation (vacuous, its `Zero` variant and whether its
  edges hold the sliver's line, or its `Unpresented` kind), how the
  pair ended, the arm that certified it, and for each `departs` read
  whether it decided and whether the rate's own enclosure straddles
  zero. Each (iv) partner is noted with its pair, cells and depth
  (the sweep sets the pair's depth before `certify_pair`).
- The runs, tour and the 758 tests each:
  `SLIVER_REFUSE=all SLIVER_MODE=present` (scaled);
  the same with `SLIVER_UNSCALED=1`; `SLIVER_UNSCALED=1` alone;
  `SLIVER_ULPS0=1` alone; `SLIVER_UNSCALED=1 SLIVER_ULPS0=1` with and
  without `SLIVER_REFUSE=all SLIVER_MODE=present`.
- "Pair visits" count every pair the walk classifies in which a
  sliver decided `Zero`, at every depth, so a pair that splits is
  counted again in its children.

### 9. The drum at `Interval`, three variants

The body is `a_filleted_drum_certifies_across_its_fillet` at
`Interval`, faces 1 (the top plane) and 7 (the torus band) across edge
12. Each variant is run alone on top of section 7's configuration
(sliver refused and the thin rectangle presented, margin un-scaled),
where the drum refuses at 15,200 pairs.

| variant | verdict | pairs | sliver `Zero` decisions | pair visits in which a sliver decided `Zero` |
|---|---|---:|---:|---|
| section 7's configuration | Refused, `Uncertified`, faces 1 and 7, edge 12, depth 40 | 15,200 | 11,893 | 321 certify by the graph separator; 11,512 split (below); 60 split with the top plane unpresented |
| 1. the line placed as a point | **Holds** | 559 | **0** | none: the far-side strip does not arise |
| 2. the heading from the chart's derivative | **Holds** | 683 | 124 | 28 certify by the graph separator; 36 split (32 with the rate straddling zero on both faces, 4 on the band alone); 60 split with the top plane unpresented |
| 3. the seam cylinder | **Holds** | 665 | 110 | 34 certify by the seam cylinder, 2 by the graph separator; 14 split (10 with the top plane's rate straddling zero or uncertified under every separator, 4 with each face decided under one separator and straddling under another); 60 split with the top plane unpresented |

The `f64` sweep of the same test holds at 150 pairs in every row, and
takes no sliver decision. With the sliver as it is on main the drum at
`Interval` holds at 567 pairs.

**What each variant changed.**
1. In `placed`, a line on a torus's minor-angle axis whose enclosure's
   middle is within 1e-9 of `π/2` is returned as the point at that
   middle. At `Interval` the line is
   `[1.5707963267948861, 1.5707963267949072]`; its middle is
   `1.5707963267948966`, and the `f64` sweep's line, read in the same
   run, is that same value. Nothing else is changed: the edge's other
   readers of its stored line keep the enclosure. The cell's own low
   side is `1.5707963267948968`, one step above the point, so the
   touch reads the cell outside the face, as it does at `f64`.
2. In `heading`, for a presentation whose zero is a side (or a side
   past the shared cell), the direction is the chart's derivative
   across that side, read on the side and pointed into the rectangle,
   in place of the direction from the edge's point to the rectangle's
   middle. Presentations by a curve, a corner or a point keep the
   middle.
3. In `departure`, after the planes through the shared edges and
   before the graph separator: where the pair shares one edge and it
   is a circle, the cylinder through that circle about the circle's
   own axis is tried as a separator vanishing on the edge's whole
   circle, each face's sign read by `sign_on` as for the other
   separators.

**What distinguishes the 321 visits the graph separator certifies
from the 11,512 that split** (section 7's configuration, read per
visit).
- **Not the cells.** Depths run 8 to 40 with median 36 in both groups.
  The band's cell is 0.0031 rad along the seam at the median in both,
  the top plane's cell 0.0055 wide in both, and in both about 97% of
  the top plane's cells straddle the circle. The band's rectangle is
  1.04e-14 rad wide in both.
- **Not the circle's plane.** In every visit of both groups the plane
  through the circle reads a rate of exactly `[0, 0]` on the top plane
  (it is the top plane) and `1.6e-15 ± 1.8e-15` on the band, and
  decides neither.
- **The graph separator's rate.** In the 321 it decides on both faces:
  the rate's enclosure has its middle at 0.026 (top plane) and 0.012
  (band) at the median, 2.4 and 2.5 half-widths from zero. In the
  splits the enclosures are as wide (half-widths 0.0080 and 0.0032)
  but centred at 0.0006 and 0.0003, under a tenth of a half-width from
  zero.
- Which face straddles, in the 11,512: both in 11,230; the top plane
  alone in 153; the band alone in 84; in 45 no graph separator was
  built.
- The trace does not record the heading or the separator's direction
  itself, so it says that the direction differs between the groups,
  not why.

**What the plane through the circle decided on the drum at `f64`**
(section 8's last run: touch at 0, un-scaled, sliver refused).
- **Those 16 pairs are not at the tangent seam with the top plane.**
  They are the band (face 7) against the wall's two halves (faces 3
  and 4), across edge 13, the circle the band shares with the wall,
  at the band's other line (minor angle `2π`). With the touch at 0
  the sliver fires there: a strip 8.9e-16 rad wide (2.7e-16 m).
- The plane through that circle is normal to the drum's axis, and
  both faces cross it: the rate is exactly `−1` on the wall and
  `[−0.3, −0.29999999999999977]` on the band's thin rectangle. Both
  are decided as signs, opposite once oriented by each face's side.
  The margins are 1 and 0.3, not rounding.
- The carrier arm is tried first there and decides nothing: the
  wall's carrier reads `[−6.9e-16, 4.4e-16]` on the band's rectangle,
  a rate of rounding size that straddles zero.
- At the tangent seam with the top plane (edge 12) the `f64` sweep
  takes no sliver decision in that run.

**How to repeat.** Three more switches in the kept patch, each read
once: `SLIVER_POINTLINE=1`, `SLIVER_HEADING=1`, `SLIVER_SEAMCYL=1`.
Each row is the one test, run from `crates/sweep` on the sweep test
binary with `--exact … --nocapture` under
`SLIVER_REFUSE=all SLIVER_MODE=present SLIVER_UNSCALED=1` plus one
switch, with `SLIVER_LOG` for the receipt and tallies. The per-visit
split is from the same run with `SLIVER_TRACE='*'`, which now prints,
for each `departs` read in a flagged pair, the separator's kind, the
face's kind, the zero's kind, the rate's enclosure, the margin, the
rectangle and the decision, and counts each line `placed` returns on
a torus's minor axis near `π/2`.
