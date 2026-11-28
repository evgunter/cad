# M16b — what an enclosure that enters the declining atoms would decide

Trees: the tour, the verb-built body and the slab on `boxfinish/probe-switch-off`
(box arm off, today's main, head b7c925b9c0); the probe code is also on
`boxfinish/probe-side-reading`. **Measured** unless *inferred*. Tables: `probe/m16b/`.

## The instrument

`geom_core::sym::m16` reads each undecided ask's form at cumulative steps; the
shipped walk is untouched and no answer changes unless `BOXFINISH_M16_ON=<steps>`.

Steps: **0** the shipped walk's reach (`Sqrt`, `Abs`, `Min`, `Max`, depth 8);
**1** + `Sin`, `Cos`, `Tan`, `Asin`, `Acos`, `Atan`, `Atan2` (argument form enclosed,
then `Interval`'s own function); **3** + `Select` (the certified arm, else the hull
of both); **7** + a hull node (the hull of its children's plain forms); **15** +
depth 32, with a memo per atom; **47** + a frozen node opened, its op over its
children's forms (**added by me**); **111** the labelled alternative, the node's
lane value from a probe side table written at mint (the tier keeps none), sampled.

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

No bracket-less parameter, opaque, free value, literal or atom recorded elsewhere
(0 unmapped). M16's "no frozen node" read the leaf's own `budget_frozen`, empty
here: the freezes are the drive's. At step 15 every remaining decline is one
(Mul 14,514; Powi 14,358; Add 2,692).

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

By predicate at step 47: `probe/m16b/tour-first-drive.txt`.

**Verb-built body** (8 undecided asks): step 0 declines all (`Cos` 6, `Sin` 2);
step 1 decides 6; step 47 all 8. **Slab rows** (172 undecided; here the door
finishes at the door): step 0 94 sign / 78 decline; step 15 136 / 32 straddle / 4;
step 47 136 / 36 straddle.

## 4. Where it still straddles at step 47 while members are sampled

Members = the margin's graph evaluated in `f64` at five points across the leaf.
In every traced representative the intersection **is the value's own enclosure**:
the form, opened through frozen nodes by interval arithmetic, is no tighter than
the value channel.

| predicate (straddling at 47) | sampled | members | reading |
|---|---|---|---|
| `curve_carrier_clear` (7,776) | 240 | all Positive, 3.87e-3 … 3.93e-3 against [0, 1.39e-2] | loose by dependency inside the form |
| `period_near_mark` (4,110) | 240 | 70 evaluable: all one sign, ±0.344 against [−1.33, 0.49] (least band); 170 not evaluable | dependency where evaluable |
| `azimuth_near_cut_side` (3,876), `curve_carrier_gap` (2,560), `curve_pair_span_end_side` (2,400) | 240 each | not evaluable (a hull or free value) | not determined |
| `curve_face_pose_disc` (1,048) | 264 | all Positive, 4.8e-4 … 5.0e-4 against ±1.38e-3 | dependency |
| `side_at_infinity_winding` (230) | 230 | all Positive, 6.55e-3 … 6.61e-3 against [−4.8e-3, 2.1e-2] | dependency |
| `assert_bound` (812) | 241 | 174 one-signed; 67 mixed across the band | dependency in 174, a real crossing in 67 |

Forms: `probe/m16b/straddle-representatives.txt`. *Inferred:* these need a
centred or mean-value evaluation; none was tried.

## 5. Soundness

Step 15: 1,808 asks × 5 members, 0 differ; step 47: 3,400, 0 differ; verb-built
120, slab 136: 0 differ. M16's 17 (`pm_census_ef_residual_side`): `prove` uses
`Band::least()` = (5e-324, 1e-323); the exact forms are 6.6e-19 … 6.9e-17 in size
and the `f64` members read ± 5.6e-17 … 1.7e-16 with mixed signs (rows in M16).

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

Verb-built body, reading on (15 or 47): ± 0.05 **builds** (6 faces, census 0,
battery 0); ± 0.1, ± 0.25 pass the join and are refused on the volume's
`OrientationUndecided`; ± 0.45 still refused. Door slab: refuses at 0.488–0.9
off, only at 0.9 on.

## 7. The second round's additions

- **Invalid margins** (`decide_sign`'s `domain_violation` arm; they never reach
  the tier and are NOT in the 37,218): **35,042** more undecided asks in the
  drive: `curve_carrier_clear` 13,392; `curve_pair_span_end_chord` 8,928;
  `side_at_infinity_winding` 8,258; `curve_carrier_gap` 4,464. None in the
  fixtures, the slab or the verb-built body.
- **The hull indeterminate**: confirmed (`combine`'s `Hull` arm,
  `Poly::indet(indet_hull(kid0, kid1))`, no atom record), but it is 3,596 of the
  15,200; the rest are frozen nodes (`form_in`'s `Poly::indet(id.bits())`). To
  enter either the probe walks the margin's node graph (`Session::node_of`), maps
  each indeterminate back from its children's ids, and builds the children's
  plain forms (session held mutably) before the walk.
- **Hold-back** (P = the graph enclosed op by op in `Interval` at the leaf's
  midpoint, same band; "terminal" = `F ∩ V` wholly within P's width of the band
  edge it was read against):

| population | reading decides | P agrees | P straddles/differs | of those terminal | P declines |
|---|---|---|---|---|---|
| fixtures (topo rows) | 319 | 295 | 24 | 24, in 8 rows | 0 |
| slab + verb-built | 81 | 81 | 0 | 0 | 0 |
| tour, first drive | 1,964 | 1,634 | 0 | 0, in 0 leaves | 330 (a hull or free value) |

  The 24 are exactly `pm_census_ef_residual_side` (reading `[6.58e-19]`, P
  `[−3.6e-16, 4.7e-16]`): the hold-back catches M16's differing asks and nothing
  else. Tour by predicate: `probe/m16b/tour-holdback.txt`.
- **`assert_bound`'s 812**: `editor_core::measure`'s measured-versus-bound
  comparand of a document assertion: a decision (the assertion's verdict), on a
  measured value against a stated bound; in 67 of 241 sampled the members cross.
- **Value enclosure ⊇ member `f64`**: 4,767 value-signed asks (tour) + 302 + 529
  (fixtures, slab/verb), × 5 members, and 5,032 unsigned asks with members:
  **no member outside the leaf's value enclosure**.
- **Document replay at `f64` members of ten newly decided leaves: skipped**, no
  leaf is newly decided (section 6).

## Not measured

Whether the residual folds on the verb-built body over ± 0.05; a centred or
mean-value form; the tour past its first drive; members off the box diagonal.
Step 47's 39 ms/ask includes re-walking per opened node (the probe's cost).
