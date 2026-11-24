# M13b: the same-lane reference, the lane log pairs, the nominal escalations (BOXFINISH measurement lane)

Branch `boxfinish/probe-build-record` (never merged), same instrument as `probe/M13-REPORT.md`, now also replaying each witness's nominal over a zero-width box at the leaves' own scalar ("the reference") and comparing every leaf with it. Tables: `probe/build-record/out/reference-*.txt`. **Measured** unless marked; "read" means read in the source at the cited line, not run.

## 1. The reference

| | `editor-core` | tour |
|---|---|---|
| witnesses (drives + range searches) | 338 (304 + 34) | 9 (the certified cells, the chain study, the drive rows) |
| replayed at `Sym<Interval>` / `Interval` | 335 / 3 | 9 / 0 |
| reference fully decided | 338 | 9 |
| whole log equal to the `f64` witness, checks read | 334 | 8 |
| standing differs (the loft: `Skin` fails at the box scalar) | 3 | 0 |
| choices equal, check sequences part (`props_circle_axis_class` \| `positive_volume_exact`) | 1 (the plate) | 1 (the cut plate) |
| a turned choice / a turned check / a parting in choices | 0 / 0 / 0 | 0 / 0 / 0 |

**Drives that certify any mass today while their zero-width replay is (i) undecided: 0; (ii) parts from the witness in choices: 0; (iii) turns a choice against it: 0.** All 310 `editor-core` witnesses with a certified leaf (246.39 certified, summed) and all 8 on the tour (6.62) have a reference that is decided and equal to the witness in the whole log. The two plates certify nothing.

## 2. The re-sort against the reference

Today's decided leaves: 2,033 in `editor-core`, 274 on the tour. (a) by the whole record, checks included, and (b) by choices and standings:

| today | (a) | (b) | leaves |
|---|---|---|---|
| certified | same | same | 1,372 + 274 |
| `FlipCrossing` | a turned choice | a turned choice (the predicate today's evidence names) | 660 |
| `FlipCrossing` naming nothing (the plate) | same | same | 1 |

- Leaves whose class differs between (a) and (b): 0.
- Leaves whose class differs from today's verdict: 1, the plate's. Against the same-lane reference it is the reference's build with the reference's log; today's refusal is the `f64` lane's extra check runs.
- Parted leaves against the reference: 0, so no halves to ask about.
- Every leaf's record equals its reference's record.

## 3. The lane log pairs, traced and labelled

Labels: (a) one lane logs work the other does not run, and it steers nothing built; (b) a decision only the box scalar makes that steers what is built; (c) a parting a narrower box removes. Sites **read**; five of them re-read by me, the rest by a delegated source read. Labels are *inferred* from the read code.

- **`pose_unit_norm` | `session_box_reach` (1,348) and | `mate_chart_slides_span` (625), | end of log (18): label (a).** One gate: `Chart::charted` renormalizes the carried frame under `POSE_UNIT_NORM` unless `identity || T::spins_by_frames()` (`crates/editor-core/src/mate/value.rs:484-494`); `spins_by_frames()` is `false` at `f64` and `true` at `Interval` (`crates/editor-core/src/eval/mod.rs:3749`). `f64` logs three `pose_unit_norm` entries the box lane never makes; the box lane's entry at that index is simply its next one. **`session_box_reach` is not a box-scalar-only decision**: both lanes decide it (`crates/topo/src/session_box.rs:189-197`); only a definite Negative does anything (refuses `OutsideSessionBox`), no geometry reads it. So it is not a (b).
- **`ssi_hull_sup` | `pcurve_projected_lever` (51): label (a), with a different count of rounds per lane.** `limb2` switches on `T::WITNESS` (`crates/geom-brep/src/edge_nurbs.rs:2272-2281`): the exact scalars take `limb2_enclosure_as`, `f64` takes `upper_bound`, and the lever decisions (`crates/geom-brep/src/pcurve_cache/projected.rs:2566-2584`) run per round until a read clears, which outward rounding moves. A failed lever refuses; otherwise it sets a bound.
- **`bool_point_in_solid_advance` | `bool_point_in_solid_denom` (28): no gate found.** Both are decided in `ray_crossings` (`crates/topo/src/solid_contain.rs:3870`, `:3898`) with no scalar test in the file. *Inferred*: an upstream difference (another ray after a graze, or another hit set). Not labelled.
- **`path_leg_length` | `trace_plane_*`/`trace_reaches` (21): label (a) in form, and the one pair whose extra pass builds something.** `ProfileLane::pass_one` returns the first pass's form at `f64` over the nominal and `None` at the box scalar (`crates/editor-core/src/eval/mod.rs:5673-5687`); on `None` the profile resolves again with its traces and logs the trace decisions, and that resolution is the lane's own profile form.
- **`pcurve_chart_placement` turning between lanes (2 builds, the tour's bracket): not (a), (b) or (c).** Site `Placement::turns` (`crates/geom-brep/src/pcurve_cache.rs:10509-10552`): Positive takes one turn off the azimuth read, Negative keeps it. *Inferred*: the two lanes read the azimuth on opposite sides of the chart's cut and the turn undoes it, so both build the same pcurve; the record and extras are equal (measured). **The two margins: not measured.** A verdict holds a predicate, a sign and a role, no margin.
- None of the six is a (c): each is at a zero-width box.

## 4. Turned checks

- Against the same-lane reference, decided leaves with equal choices and a check of the other definite sign: **0** (2,307 leaves).
- Against `f64`: only the plate's leaf (its `props_*` check runs differ in count, no sign turns).
- A check does turn where a choice turns: on all 220 `path_junction_turn` flip leaves, **`dihedral_sign` turns as a check as well** (7 of its 105 or 126 instances go Positive to Negative); my crease document's 12 flip leaves are among them. On the `segment_gap` (300), `extrusion_normal_component` (122) and `carrier_line_circle` (18) leaves no check's population moves.
- No side-reader ray decision turns on any decided leaf.

## 5. The box lane's escalations at the nominal (the earlier follow-up)

One representative build each; margins at `Interval` on the nominal as in M13's addendum (A5). The `f64` margin's value: **not measured** (a decided verdict keeps no margin).

| predicate | at `Interval` | at `Sym<Interval>`, same build | why undecided at a point (*inferred* from the read site) |
|---|---|---|---|
| `curve_pair_pose_parallel` | `Invalid` | undecided, `Invalid` | a rate with a divisor that encloses zero poisons the margin (`crates/geom-brep/src/curve_pair/pose.rs:1454`, `:1571`) |
| `curve_carrier_gap` | [0, 0.114] | undecided, same enclosure | a distance to a foot whose enclosure widened; the norm's lower end clamps at 0 (`crates/geom/src/curves/on_carrier.rs:197`) |
| `chart_singularity_nearer` | [-3.5e-16, 3.5e-16], band zero 2.2e-308 | **decided** (3 builds) | a tie: the difference of two equal distances read as an enclosure of rounding width against a band it cannot fit (`crates/geom-brep/src/chart_singularity.rs:718-727`) |
| `germ_side` | [-6.3e-13, 6.3e-13], `Rejection { sign: Zero, passes: NonZero }` | not measured: the representative row did not finish in 400 s | a definite Zero refused by a non-zero gate (`crates/topo/src/star/order.rs:98-108`) |

## 6. The loft whose `Skin` fails with no escalation

Payload `Skin(Structure(Built(CarrierWeightNotAPoint { index: 1 })))`, at `Interval` and at `Sym<Interval>` alike, 0 escalations. Raised by `weight_channel` (`crates/geom/src/net.rs:229-244`) when a weight has no point value. *Inferred*: the arc's weight `cos(Δ/2)` is computed at the run's scalar, an interval cosine of a point rounds outward, and the test is a structure test, not a funnel decision, so nothing is logged.

## 7. The 27,667 `Extrude` leaves, and two corrections to M13

- **Variant: all 27,667 are `ExtrudeError::DegenerateExtrusion`** (the failed node's payload printed per leaf; 13,908 end `Budget`, 13,759 split). It is the decided `Sign::Zero` arm of `swept::decide_reported("extrusion_normal_component", Margin::of(depth), band)` (`crates/sweep/src/extrude.rs:800-805`, read). The variant carries no margin (`extrude.rs:356`), so the enclosure is the leaf's depth: a representative that ends `Budget` has offsets [-3.086e-8, -3.066e-8] on a nominal depth of 3e-8, i.e. depth in [-8.6e-10, -6.6e-10], wholly inside the zero band (1e-9). Every such leaf is at most 7.8e-10 wide. That the nominal is 3e-8 for that drive is read off its witness's end cap; for the other drives *inferred*.
- **It is not `ExtrusionEscalated`, and no log is dropped**: 0 escalations on any node of these leaves, and none is expected from a decided Zero.
- **No such box has certified mass below it: 0 of 27,667.** **Correction to M13 section 4:** the 13 boxes that split on a failed node and hold certified mass below (2.02) are 9 `ProfileLaneReplay { loop_: 0, step: 2, structure: None }` (1.52; `m10_3`'s undecided-axis row) and 4 `Extrude(Op { DescriptionUncertified { .. AnalyticRung3(CarrierEnclosure..) } })` (0.50; the `notch` drives). So the degenerate extrusion is a definite refusal the drive bisects to the budget (the filed row), and the failures that a narrower box removes are those two.
- **Re-correction to M13's addendum A2 (`extrusion_normal_component`, 122 flip leaves).** The representative leaf's depth is NEGATIVE: offsets [-1e-7, -5e-8] on a nominal 3e-8, depth in [-7e-8, -2e-8]. `wire_extrude` answers the sweep's `NegativeDepth` by sweeping |d| and placing the start cap d along the profile's z (`crates/editor-core/src/eval/wire.rs:1728-1760`, read). The body's own-coordinate vertices show an end cap at z in [2e-8, 7e-8]; the placement is in the body's frame, which neither the record nor my geometry dump holds. So a consumer holding the leaf's body sees the witness's cells on the OTHER side of the sketch plane; my addendum's "it does not change side; it is thicker" read another node's vertices and was wrong. *Inferred* from the read code that the frame carries the offset; the frame itself not dumped.
- So by class: `segment_gap` (300) and `carrier_line_circle` (18) move coordinates only; `path_junction_turn` (220) turns one crease's convexity (and `dihedral_sign` as a check); `extrusion_normal_component` (122) moves the body across its sketch plane through a frame the record does not hold.

## Not measured

- The two `pcurve_chart_placement` margins and every `f64` margin: the logs keep none.
- `germ_side` at `Sym<Interval>`.
- A body's frame: in neither the record nor the dump.
- The escalation table at `Sym<Interval>` beyond one build per predicate.
- Two rows are red in this pass with the probe on (`docm9_range`'s plate row, `m10_9_what_the_registry_buys`), both red with it off too; one tour nightly row (`the_cut_plates_first_subtract_...`) is red under the probe.
