# M16 — what reading a margin's form over the box's brackets would do

Trees: populations 1, 2, 4 on `boxfinish/probe-side-reading` (main merged at
a86f5378bc); population 3 on `boxfinish/probe-switch-off` (the box arm off, merged
up to today's main, with this instrument merged in; pushed, head after c879941745).
Everything is **measured** unless marked *inferred*. Tables: `probe/m16/`.

## The instrument (`geom_core::sym::m16`, hooked in `Sym::decide_sign`)

For every decision whose value does not read a definite sign, with a session whose
brackets include one of non-zero width (`BOXFINISH_M16=<file>`, one line per ask):
the PLAIN form is built and timed **before** the ladder (so the ladder's time below
excludes it, and the answer is still today's); the form is enclosed with a copy of
`signed::enclose_form_deep` / `enclose_indet` that returns its reason for
declining; the result is intersected with the value's enclosure and read by
`Interval::sign_within` at the decision's band (sliver = undecided and inside the
escalate band); where the plain form declines or straddles, the same on the EARLY
form. `BOXFINISH_M16_ON=1` makes the reading the answer where today's is undecided.
All asks are at an exact witness (`Sym<Interval>`).

Three things it does not do: it counts asks that `deferred::defer` took (none
seen); the early form's build time is a memo hit (the ladder built it), so it is
inside the ladder's time; "frozen / opaque / free / hull" are read off the margin's
graph, not off the form.

## Per population

| | 1: M14/M15 bodies, 4 widths | 2: verb-built cut | 3: tour, first drive | 4a: wide door slab | 4b: false premise pins |
|---|---|---|---|---|---|
| asks the value does not sign | 16,628 | 1,844 | 1,873,068 | 6,660 | 143 |
| today: ladder proves Zero | 16,230 | 1,836 | 1,835,774 | 6,582 | 128 |
| today: value Zero in band | 56 | 0 | 0 | 0 | 12 |
| **today: undecided** | **342** | **8** | **37,294** | **78** | **3** |
| … of those, the zero form | 0 | 0 | 0 | 0 | 0 |
| … plain form decides a sign | 153 | 0 | 0 | 20 | 2 |
| … plain form reads Zero in band | 120 | 0 | 0 | 0 | 0 |
| … early form decides (plain did not) | 42 (18 sign, 24 Zero) | 0 | 0 | 19 | 0 |
| … sliver | 0 | 0 | 0 | 0 | 0 |
| … still straddles | 0 | 0 | 812 | 0 | 1 |
| … declines (both forms) | 27 | 8 | 36,482 | 39 | 0 |
| EMPTY intersections | 0 | 0 | 0 | 0 | 0 |
| plain build, undecided asks | 21 ms, 62 µs/ask | 0.1 ms, 14 µs | 445 ms, 12 µs | 39 ms, 504 µs | 0.2 ms, 56 µs |
| ladder after it, same asks | 49 ms, 144 µs/ask | 0.8 ms, 95 µs | 1,057 s, 28.3 ms | 96 ms, 1.2 ms | 0.2 ms, 60 µs |

On the asks the ladder proves Zero, the reading agrees every time (plain or early
form reads Zero), in every population; there is no empty intersection anywhere.

**Population 1** (fixture bricks by `census_and_certify` and
`validate_pseudomanifold_structural`; the door slab by `census_traces` and the same
battery). By predicate, undecided today → reading: `carrier_endpoint_end` 64 Zero;
`pcurve_line_chart_incident` 32 Zero; `curve_carrier_gap` 27 Positive;
`pm_census_ee_span` 25 sign; `pm_census_ef_residual` 24 Zero;
`pm_census_ef_residual_side` 24 sign; `pcurve_point_chart_incident` 24 Zero (early);
`bool_ring_run_winding` 19 Positive; `bool_contact_vertex` 18 **declines**;
`curve_carrier_clear` 14 Positive. Declines: `bool_contact_vertex` 18
(an indet with no bracket, the frozen square of M15); `side_at_infinity_edge` /
`_edge_span` / `_advance` / `_winding` 39 over both slab populations (depth 8
reached, `Select` at depth 5); `pm_census_ee_line_gap` 9 (a poisoned sum).

**Population 2** (a 1×1×3 brick minus a 5×5×4 brick turned 0.25 rad about
(0.6, 0.8, 0) and lifted by `1.5 ± w`, `boolean_op_with(Subtract)`, budgets 64/8 and
4096/128). Over a box the body is **not built at any width**: the Boolean refuses
at `bool_join_nearest` (± 0.05: [−0.062, 0.076]; ± 0.1; ± 0.25) and at
`curve_pair_span_end_chord` (± 0.45), and the reading declines on those asks
(`Cos` / `Sin` atoms of the turn). At width 0 it builds (6 faces, census 0, battery
0), and **all 24 `pm_census_ef_residual` asks are the zero form**, proven at
`Plain`: the residual does fold on the verb-built body, as the designer's algebra
says; the fixture's 6.6e-19 is the fixture's hand-set plane.

**Population 3** (the whole first drive: 1,023 leaves, certified 211, refused 301,
as M10; 1,002 leaves hold an undecided ask). The reading decides **none** of the
37,294 undecided asks. Early-form reasons: an indet with no bracket and no atom
record 15,200; `Select` 7,897; `Sin` 5,078; `Atan` 2,850; `Atan2` 2,829; `Cos`
2,436; straddles 812 (all `assert_bound`); depth 192. The margins' graphs hold no
frozen node, no bracket-less parameter, no opaque or free value; 13,152 hold a
`Hull`. By predicate: `curve_carrier_clear` 12,560; `azimuth_near_cut_side` 5,480;
`period_near_mark` 5,454; `curve_carrier_gap` 4,822; `curve_pair_span_end_side`
3,048; `side_at_infinity_winding` 2,954; `curve_face_pose_disc` 1,516;
`assert_bound` 812; `bool_ruled_region_frame` 648: every one declines or straddles.
(A first run of the same drive counted 39,493 undecided with the same classes;
the difference between the two runs is not traced.)

**Population 4.** (a) `work/tier/real-margin-dependency-widening.md`: its
`slab(1.0, half)` driver fixture was not found by that name; run instead on the
extrude door's slab at ± 0.44, 0.488, 0.49, 0.6, 0.9 (my substitution, said here):
the battery is clean to 0.44 and reads one `OrientationUndecided` from 0.488; of
78 undecided asks the reading decides 39 (`bool_ring_run_winding`,
`side_at_infinity_order`, `positive_volume`, `positive_volume_enclosure`) and
declines 39 (`side_at_infinity_edge*`, depth). (b)
`work/sym/a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box.md`: its
probe file is not on main; run on the row's pins in
`volume_premises_over_a_box.rs` (`a_false_premise_is_refused_at_every_scalar`,
`a_rim_that_fits_only_at_the_nominal_is_not_accepted_over_a_box`): `props_rim_fit`
reads Negative and `props_cylinder_circle_off_axis` Positive by the plain form; the
rim that fits only at the nominal still straddles, as it should.

## Soundness sample

Members by evaluating the margin's expression graph in `f64` at five points across
every bracket at once (0, ¼, ½, ¾, 1), up to 40 asks per predicate per process, on
asks the reading decides; each member's `f64::sign_within` at the same band against
the reading. Sampled: 984 (pop 1), 384 (pop 2), 6,204 (tour; all are asks the
ladder also proves Zero), 49 + 126 (pop 4); not evaluable 40 / 40 / 200 / 8 (a
`Hull`, `Free` or opaque value in the graph).

**17 asks differ, all `pm_census_ef_residual_side`** (`census::undecided_signs`,
a `prove` at the least band) on the two tilted fixtures: the form's enclosure is
the exact constant (`[6.5813e-19, 6.5813e-19]`, Positive) and the `f64` members
read `+5.55e-17, −5.55e-17, +5.55e-17, …`, a different sign at some members. The
form is the exact value of the stored expression; `f64` rounding at a member is
larger than it. No other predicate differs.

## The verdict question (reading switched on)

- Population 1, census alone: every tilted row goes from undecided to **0
  findings** at all four widths (6, 6, 6/9, 14 → 0), and `brick_at` ± 0.45 from 8
  to 0. The battery on the tilted fixtures drops 19 → 16 / 15 (the fixtures do
  not finish for other reasons). Door slab: 0.488, 0.49, 0.6, 0.9 go from one
  `OrientationUndecided` to clean. No row that was clean changes (29 unchanged).
- Population 4b: the pin goes red by design, `Escalated` → `OffSurface` on
  `props_rim_fit` at the symbolic scalar over the box.
- Population 2: still refused at `bool_join_nearest` with the reading on.
- Tour, first drive: **not run switched on**; the reading decides none of the
  drive's undecided asks, so no leaf's answers change (*inferred* from the count,
  not measured by a second drive).

## Not run, and why

- The tour beyond its first drive (killed after the drive's summary line).
- The dependency row's own driver fixture (not found); the nonzero-constant row's
  P4 probe file (on a review branch, not on main). Substitutes are named above.
- The verb-built body over a box (the Boolean refuses); only its width-0 form.
- Members are sampled along the box's diagonal only, not its corners.
