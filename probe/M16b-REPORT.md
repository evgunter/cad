# M16b — what an enclosure that enters the declining atoms would decide

Trees: the tour, the verb-built body and the slab on `boxfinish/probe-switch-off`
(box arm off, today's main, head b7c925b9c0); the probe code is also on
`boxfinish/probe-side-reading`. **Measured** unless marked *inferred*. Tables:
`probe/m16b/`.

## The instrument

`geom_core::sym::m16` reads each undecided ask's form at cumulative steps; the
shipped walk is untouched and no answer changes unless `BOXFINISH_M16_ON=<steps>`.

| step | adds |
|---|---|
| 0 | the shipped walk's reach: `Sqrt`, `Abs`, `Min`, `Max`, depth 8 |
| 1 | `Sin`, `Cos`, `Tan`, `Asin`, `Acos`, `Atan`, `Atan2`: the argument form enclosed, then `Interval`'s own function |
| 3 | `Select`: the condition enclosed; the certified arm, else the hull of both |
| 7 | a hull node: the hull of its two children's plain forms |
| 15 | depth 8 → 32 (with a memo per atom, so depth is not paid twice) |
| 47 | **added by me**: a frozen node opened, its op applied to its children's plain forms' enclosures |
| 111 | the labelled alternative: the node's own lane value, from a probe side table written at mint (the tier keeps none); sampled, 40 asks per predicate per thread |

Each reading takes the plain form, else the early one, intersected with the value's
enclosure, read at the decision's band. No atom kind was refused as unsound to
enter; what remains uncertified is `Atan2` over an argument box that holds the
cut, and a quotient whose denominator's enclosure holds zero (both decline).

## 1. The 15,200 indeterminates with no bracket and no atom record

They are **frozen ring nodes standing as their own indeterminate, and hulls**:

| kind | asks (step 0) | representative |
|---|---|---|
| a frozen `Powi` node | 6,648 | `curve_face_pose_disc` |
| a hull | 3,596 | `period_near_mark` |
| a frozen `Mul` node | 2,500 | `curve_carrier_gap` |
| a frozen `Add` node | 2,430 | `azimuth_near_cut_side` |

No bracket-less parameter, no opaque or free value, no literal, no atom recorded
elsewhere (0 unmapped). M16's "no frozen node" was the leaf's own `budget_frozen`
set, which is empty here: the freezes come from the drive's table. With the atoms
entered more of them surface behind the atoms: at step 15 every remaining decline
is one (Mul 14,514; Powi 14,358; Add 2,692).

## 3. Cumulative, the tour's first drive (1,023 leaves; 37,218 undecided asks)

| step | sign | zero in band | sliver | straddles | declines | empty | µs/ask |
|---|---|---|---|---|---|---|---|
| 0 | 0 | 0 | 0 | 812 | 36,406 | 0 | 19 |
| 1 trig | 0 | 0 | 0 | 812 | 36,406 | 0 | 7 |
| 3 Select | 0 | 0 | 0 | 812 | 36,406 | 0 | 21 |
| 7 Hull | 0 | 0 | 0 | 812 | 36,406 | 0 | 13 |
| 15 depth 32 | 42 | 0 | 0 | 5,612 | 31,564 | 0 | 46 |
| 47 frozen opened | 1,964 | 0 | 0 | 22,812 | 12,442 | 0 | 39,355 |
| 111 node values (1,465 sampled) | 180 | 0 | 0 | 919 | 366 | 0 | 2,938 |

Declines at 47: depth 6,164; quotient 3,336; `Atan2` 2,942. Today's ladder on
the same asks: 29.3 ms/ask (1,090 s summed over threads); plain build 9 µs/ask.
On the 552,815 asks the ladder proves Zero with a non-zero plain form, the
step-15 reading reads Zero every time. **No empty intersection at any step.**

By predicate at step 47 (sign / straddles / declines): `curve_carrier_clear`
248 / 7,776 / 4,512; `azimuth_near_cut_side` 912 / 3,876 / 664;
`period_near_mark` 306 / 4,110 / 1,032; `curve_carrier_gap` 0 / 2,560 / 2,256;
`curve_pair_span_end_side` 0 / 2,400 / 648; `side_at_infinity_winding`
42 / 230 / 2,682 (`Atan2`); `curve_face_pose_disc` 456 / 1,048 / 0;
`assert_bound` 0 / 812 / 0; `bool_ruled_region_frame` 0 / 0 / 648 (quotient).

**The verb-built body** (8 undecided asks over the four widths, both budgets):
step 0 declines all 8 (`Cos` 6, `Sin` 2); step 1 decides 6, straddles 2; step 47
decides all 8. **The slab rows** (172 undecided asks; on this tree the extrude
door finishes at the door): step 0 94 sign / 78 decline (depth); step 15 136 /
32 straddle / 4 decline; step 47 136 / 36 straddle.

## 4. Where it still straddles at step 47 while members are sampled

Members = the margin's graph evaluated in `f64` at five points across the leaf.
In every traced representative the intersection **is the value's own enclosure**:
the form, opened through frozen nodes by interval arithmetic, is no tighter than
the value channel.

| predicate (straddling at 47) | sampled | members | reading |
|---|---|---|---|
| `curve_carrier_clear` (7,776) | 240 | all Positive, 3.87e-3 … 3.93e-3 against [0, 1.39e-2] | loose by dependency inside the form |
| `period_near_mark` (4,110) | 240 | 70 evaluable: all one sign, ±0.344 against [−1.33, 0.49] (least band); 170 not evaluable | dependency where evaluable |
| `azimuth_near_cut_side` (3,876) | 240 | not evaluable (a hull or free value in the graph) | not determined |
| `curve_carrier_gap` (2,560) | 240 | not evaluable | not determined |
| `curve_pair_span_end_side` (2,400) | 240 | not evaluable | not determined |
| `curve_face_pose_disc` (1,048) | 264 | all Positive, 4.8e-4 … 5.0e-4 against ±1.38e-3 | dependency |
| `side_at_infinity_winding` (230) | 230 | all Positive, 6.55e-3 … 6.61e-3 against [−4.8e-3, 2.1e-2] | dependency |
| `assert_bound` (812) | 241 | 174 one-signed; 67 mixed across the band | dependency in 174, a real crossing in 67 |

Their forms are in `probe/m16b/straddle-representatives.txt`. *Inferred:* a
centred or mean-value evaluation is what these need; none was tried.

## 5. Soundness

At step 15: 1,808 asks × 5 members, 0 differ (100 not evaluable). At step 47:
3,400 asks, 0 differ (430 not evaluable). Verb-built 120, slab 136: 0 differ.

M16's 17 (`pm_census_ef_residual_side`, the tilted fixtures; not re-run here,
M16's rows): `k_stats::prove` decides at `Band::least()` = (5e-324, 1e-323).

| fixture | the form's enclosure | `f64` members seen |
|---|---|---|
| tilt (0.3, 0.2) | [6.5813e-19] | +5.55e-17, −5.55e-17, −1.67e-16 |
| tilt (0.3, 0.2) | [1.3163e-18] | +1.11e-16, −9.71e-17 |
| tilt (−0.45, 0.6) | [−2.6645e-17] | +5.55e-17, 0, −1.11e-16 |
| tilt (−0.45, 0.6) | [−4.2188e-17] | 0, −1.67e-16 |
| tilt (−0.45, 0.6) | [−6.8834e-17] | +1.11e-16, −1.11e-16 |

23 asks, 17 with a member whose sign is not the form's. The form is exact; a
member's `f64` rounding (1e-16) is larger than it, at a band of 5e-324.

## 6. The verdict question

The tour's first drive, each run to its summary line, per-leaf verdicts compared
by box:

| run | certified | refused | splits | leaves | leaf time (sum) | wall | numeric decisions |
|---|---|---|---|---|---|---|---|
| reading off | 211 | 301 | 511 | 1,023 | 1,546 s | 404 s | 2,541,916 |
| on, step 15 | 211 | 301 | 511 | 1,023 | 1,635 s | 429 s | 2,541,958 |
| on, step 47 | 211 | 301 | 511 | 1,023 | 3,106 s | 807 s | 2,562,874 |

**No leaf moves**: the same 1,023 boxes, the same verdict on each, none
certified that was not, none refused that was certified. The 1,964 asks step 47
decides let their leaves ask about 21,000 more decisions and stop at another.

The verb-built body with the reading on (15 or 47): ± 0.05 **builds** (6 faces,
census 0, battery 0; was refused at `bool_join_nearest`); ± 0.1 and ± 0.25 pass
the join and are refused later, `OrientationUndecided` on the volume
([−0.41, 3.49], [−4.27, 8.04]); ± 0.45 still refused (`curve_pair_span_end_chord`
at 15, `sector_arm` [0, 3.0] at 47). The door slab: refuses at 0.488, 0.49, 0.6,
0.9 off; only at 0.9 on.

## Not measured

- Whether the residual folds on the verb-built body over ± 0.05 (it builds only
  with the reading on; its residual forms were not printed in that run).
- A centred or mean-value form evaluation; node values beyond the sample.
- The tour beyond its first drive. Members along the box diagonal only.
- The step-47 time includes re-walking after each frozen node is opened (up to
  400 rounds), so 39 ms/ask is this probe's cost, not a floor.
