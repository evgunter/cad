# A centre-held conic read by chords from its exact seam vertex

Measured 2026-11-21 on `oneround/anchored-conic` at `c3fb679d1` with
temporary switches, reverted (`switch-and-probes.patch`). Facts only.

## The readings

All on the conic **built from a centre** (`Circle::new`, `Ellipse::new`),
at `f64`; the door is `conic_quadric_roots`.

- **centre sum**: as stored and read today, `centre + size·direction`.
- **chord, rounded vertex**: `point_at` and `running_point` read
  `fl(centre + u·a) + u·a·(cos θ − 1) + v·b·sin θ` (`CV_CHORD=f64`; the
  switch of `../chord-at-f64-centre-at-interval/`). The same cells were
  also run on an anchored conic built at `fl(centre + u·a)` ("twin" in
  the raw files): every count is the same.
- **chord, exact vertex** (`CV_CHORD=exact`): `point_at` and
  `running_point` read `hi + (lo + u·a·(cos θ − 1) + v·b·sin θ)`, `(hi,
  lo)` the sum `centre_i + u_i·a` per coordinate, error-free: Dekker's
  product, TwoSum, the two error terms added (one rounding, of a term
  below `ulp(hi)`), renormalized. `running_point` takes `hi` and `lo` as
  exact.
- **arc-held**: a conic built anchored at the point that grazes the wall
  (`Circle::anchored`, `Ellipse::anchored`), read as step A reads it.

## What each column is judged against

`conic_oracle::exact::truth`, in double-double, takes of a conic:

| | centre | directions | sizes |
|---|---|---|---|
| **stored entries** (the oracle as it stands) | the stored centre's three `f64` as reals | the stored `u_ref` and `axis` entries as reals; the second direction `axis × u_ref` of those reals | the stored `f64` as reals |
| **polar factor** (temporary variant, `CV_POLAR`) | the same | the first two columns of the polar factor of the stored `(u_ref, v_ref, axis)`, `M·(I − ½(MᵀM − I))` (first order in `MᵀM − I`) | the same |
| **own** (arc-held column only) | the stored point as reals, less `u·a` of the stored entries | stored entries | stored |

The wall is the unit cylinder; the oracle normalizes its axis.

The centre sum and both chord readings are judged against the stored
centre's curve, under each of the two readings of its directions. The
arc-held column is judged against its own stored point's curve.

A cell is `right / wrong / refused`. **Right**: a definite answer with
the oracle's count of roots in the window, each within `ε + 8u·a` of arc
of the oracle's (`8u·a` is the oracle's own rounding of a root's
parameter: 1.3e-12 m at 1500, 8.9e-12 m at 1e4). **Wrong**: any other
definite answer; "worst" is the farthest such root.

Are the frames axis-aligned? The size table's are not (every pose's axis
and seam are generic). Probe 1's revolve edges are; its sketch-arc and
plane-section edges are not (axis `z`, a generic seam in the plane). So
the two truths differ on every size-table cell and on six of each
distance's eight door edges; no separate tilted case was added.

## The size table (`raw-size.txt`)

The near-gap families: a conic of semi-axis ρ grazing the unit wall at
its seam vertex, ten depths 0.5 to 2 µm, four tilts, five families
(circle; ellipses at ratio 1.25, 0.999, 1.001, 0.8): 200 poses a size.
K = 10. Summed over the five families.

### ε = 1e-9

| ρ (m) | centre sum, stored / polar | chord, rounded vertex, stored / polar | chord, exact vertex, stored / polar | arc-held, own | the two truths' roots apart, at most |
|---|---|---|---|---|---|
| 10 | 191/0/9, the same | 191/0/9, the same | 191/0/9, the same | 191/0/9 | 5.8e-11 m |
| 100 | 192/0/8 | 192/0/8 | 192/0/8 | 192/0/8 | 3.9e-11 |
| 500 | 200/0/0 | 200/0/0 | 200/0/0 | 200/0/0 | 1.8e-10 |
| 900 | 153/0/47 | 200/0/0 | 200/0/0 | 200/0/0 | 3.3e-10 |
| 1000 | 127/0/73 | 200/0/0 | 200/0/0 | 200/0/0 | 3.7e-10 |
| 1500 | 29/0/171 | 200/0/0 | 200/0/0 | 200/0/0 | 5.5e-10 |
| 1e4 | 0/0/200 | **114/86/0** (worst 2.7e-9 m) / **101/99/0** (5.7e-9) | 200/0/0 / **150/50/0** (3.6e-9) | 200/0/0 | 3.6e-9 |

Where one triple is shown the two judgings agree.

- **Centre sum**: no wrong answer at any size, under either truth.
- **Chord from the rounded vertex**: this corrects the size table's
  "reader-chord" column (`../size-table/`), which was judged against the
  curve through the rounded vertex. Against the stored centre's curve it
  is 0 wrong up to 1500 on these 200 poses and 86 wrong of 200 at 1e4.
  (At 1500 the fuzzed row `a_circle_of_radius_1500_…` finds wrong roots
  at shallower depths, 2e-8 to 4e-7 m:
  `../chord-at-f64-centre-at-interval/`.)
- **Chord from the exact vertex**: 0 wrong at every size against the
  stored entries' curve, with the arc-held column's counts. Against the
  polar factor's curve, 50 wrong of 200 at 1e4, roots up to 3.6e-9 m
  off: the two truths' roots are themselves that far apart there.
- **Arc-held**: unchanged from the size table; its stored point is exact,
  so its own curve is the truth it is judged against.

### ε = 1e-12 (depths scaled with ε, 0.5 to 2 nm)

Every reading: 10/0/190 at ρ = 10 and 0/0/200 at every larger size, the
arc-held column too. No wrong answer. The two truths' roots lie 5.9e-10
m apart at ρ = 10, 1.2e-8 m at 1000, 1.1e-7 m at 1e4.

## Probe 1's door-built edges (`raw-doors.txt`)

The eight far-centred circle edges a distance that the sketch arc, the
plane section and the far-axis revolve stored at ε = 1e-9
(`../far-centre-through-real-doors/`), 48 poses an edge, 384 a cell.

| ε, K | D | centre sum, stored / polar | chord, rounded vertex, stored / polar | chord, exact vertex, stored / polar |
|---|---|---|---|---|
| 1e-9, 10 | 3e3 | 96/0/288, the same | 192/0/192, the same | 192/0/192 / **188/4/192** (worst 1.2e-9 m) |
| 1e-9, 10 | 1e4 | 48/0/336 | **93/3/288** (1.9e-9) / **88/8/288** (1.5e-9) | 96/0/288 / **84/12/288** (2.5e-9) |
| 1e-9, 1.002 | 3e3 | 48/0/336 | 96/0/288 | 96/0/288, the same |
| 1e-9, 1.002 | 1e4 | 24/0/360 | **39/9/336** (2.7e-9) / **36/12/336** (2.1e-9) | 48/0/336 / **39/9/336** (3.5e-9) |
| 1e-9, both K | 1e5 | 0/0/384 | 0/0/384 | 0/0/384 |
| 1e-12, 10 and 2.81 | all | 0/0/384 | 0/0/384 | 0/0/384 |

- Every wrong answer is on a sketch-arc or plane-section edge (the
  frames that are not axis-aligned for the polar judging; the same doors
  for the stored judging). None on the revolve's axis-aligned edges.
- The exact-vertex reading: 0 wrong against the stored entries' curve in
  every cell, with twice the centre sum's certified count; 4, 12 and 9
  wrong against the polar factor's.
- The centre sum: 0 wrong under either.

## The arc far round from the seam vertex (`raw-round.txt`)

The circle family (40 poses) with the seam turned so that the grazing
point sits at θ = 0, 0.5 and 2 rad; the door's window is ±1 rad about
it. ε = 1e-9, K = 10. Stored-entries judging / polar where they differ.

| ρ | seam to arc | centre sum | chord, rounded vertex | chord, exact vertex | arc-held |
|---|---|---|---|---|---|
| 1500 | 0 | 7/0/33 | 40/0/0 | 40/0/0 | 40/0/0 |
| 1500 | 0.5 rad | 3/0/37 | 39/0/1 | 38/0/2 | 40/0/0 |
| 1500 | 2 rad | 1/0/39 | 0/0/40 | 0/0/40 | 40/0/0 |
| 1e4 | 0 | 0/0/40 | **22/18/0** (2.2e-9 m) / **20/20/0** (4.6e-9) | 40/0/0 / **30/10/0** (2.9e-9) | 40/0/0 |
| 1e4 | 0.5 rad | 0/0/40 | 0/0/40 | 0/0/40 | 40/0/0 |
| 1e4 | 2 rad | 0/0/40 | 0/0/40 | 0/0/40 | 40/0/0 |

Away from the seam vertex the exact-vertex reading refuses as the centre
sum does (0 of 40 at 2 rad at 1500; at 0.5 rad at 1e4) and gives no
wrong answer against the stored entries' curve. At ε = 1e-12 every cell
of every reading is 0/0/40.

## Conditioning at the three poses taken apart (`conditioning.txt`)

ρ = 1500, the stored centre moved rigidly by 1.46e-11 m; the exact
roots' movement along the arc per metre of shift:

| depth (m) | along the wall's normal at the graze | along `y` | along the wall's axis |
|---|---|---|---|
| −2.2e-8 | 1.5e4 | 3.2 | 0 |
| +4.3e-7 (four roots) | 3.6e5, 4.7e2, 2.0e5, 5.6e5 | 6.2e2, 91, 1.9e3, 1.2e3 | 0 |
| −3.4e-8 | 1.9e4 | 4.8 | 0 |

At 1.5e4 a shift of 6.5e-14 m moves a root by ε = 1e-9; half an ulp of a
coordinate between 1024 and 2048 m is 1.1e-13 m. At 5.6e5 a shift of
1.8e-15 m does.

## The older numbers (phase 1)

- **(a), the near-gap "40 of 40"** on
  `a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex` (and
  the log's line): both columns, "from the seam vertex" and "summed
  exactly", were a switch on the centre-stored conic, judged by
  `exact::truth` on that same centre-stored carrier (`measure-full.patch`
  line 406; no anchored form existed on `b80352cd9`). So they were judged
  against the **stored centre's curve, stored entries**, not a twin's.
  They stand, for what they measured: ρ = 1500, the 40 poses at 0.5 to 2
  µm. Today's table agrees there (200/0/0). They do not show the rounded
  vertex's reading right at other depths or sizes: it is wrong at 1500 at
  depths 2e-8 to 4e-7 m and at 1e4 on those same poses (above).
- **(b), the section-table misses** on
  `the-section-table-admits-a-curve-on-its-resolution`: each column is
  the `f64` point's exact-rational miss off the two surfaces the section
  was cut from. No curve stands in as truth; nothing to correct.
- **The size table's "reader-chord" column** (`../size-table/`) was
  judged against the twin through the rounded vertex. Corrected above.

## Not measured

- The polar judging uses the polar factor to first order (the neglected
  term is of order `u²`); the `Interval` lane's own enclosure of the frame
  was not read.
- `Interval`, the slow set, the suite under the exact-vertex switch.
- An exact-vertex reading's cost, and any reader outside `conic.rs`.
