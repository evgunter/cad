---
id: the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause
kind: issue
title: The injectivity check's sliver concession reads a strip inside the band as holding nothing, where DESIGN's finished-body clause says a reading inside the band refuses as ambiguous
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
