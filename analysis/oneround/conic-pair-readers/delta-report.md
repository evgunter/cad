# Delta read: PR 7543, `oneround/conic-pair-readers`, the fix pass K1 to K12, frozen at `cc003c1b3ce23619f61cb52857d13316b2ffaa1c`

Second read by the lane that reviewed `6510faceec` (`review-report.md` beside this file). Everything this read ran
is under `analysis/oneround/conic-pair-readers/review2/` on `oneround/conic-pair-readers-review`. No helper agent was
used.

## Verdict: APPROVE-WITH-FIXES

The re-cut acceptance holds and I could not break it. In the box, with the seam at the anchor, nothing is certified
wrongly: not on the unit's ladder (reproduced byte for byte), not on the suspects' harness, and not on 1,980 readings
per ε of my own taken along the arc out to the box's wall, at every size from 1e2 to 1e9, nor on 781 `on_carrier`
points there. No reading the base had right is wrong on the head anywhere in the box. Both MAJORs of the first read
are disposed of: M1 by the ruling and the join, M2 by the row and the doors' docs. What is left is fixable in place
and none of it is a wrong answer: the rewritten meeting rows cannot go red on a refusal at the tolerances CI runs
(m1), the two centres the rulings sent away came back under new spellings (m2, m3), and two statements about refusals
in the box are true of the ladder's poses only (m4).

## What was executed, and what was only read

EXECUTED, own target directories, on the frozen head and on the base I built from
`git merge-tree --write-tree 0645d2b68 258f0d80b` (clean this time: tree `0add3601f1`):

- The unit's ladder, `oracle.py` and `join.py`, base and head, ε 1e-9 and 1e-12; the suspects' harness with its
  oracle and tables, likewise (with `OnSurface::under` made `pub` for the run, as its README says).
- `review2/ladder_wall.rs`: the unit's ladder with the anchor stood 950 m along the arc's tangent from the box's
  centre and poses at 0.3, 30, 300, 950, 1500 and 1850 m of arc, `a` 1e2 to 1e9, fat and thin, both doors, three
  twins, both lanes: 2,736 readings per ε, 1,980 of them in the box by the harness's own predicate; judged by the
  unit's oracle and join unchanged, base and head.
- `review2/probe_level_wall.rs`: level, clearance, `height_of` and `on_carrier` at 781 in-box points per ε along
  the same arcs, against 60 digits.
- `review2/probe_roots2.rs`: `Height::roots` at 160 exact and near-exact touches (at the anchor, the antipode, a
  quarter turn either way and at θ = 1; both sides; arc-held, re-seamed, centre-held and a circle; an axis-aligned
  and a skew frame; two sizes), `f64` and `Interval`.
- `review2/probe_gauged.rs`: where `arc_held_rows`' "an answer is owed" is in force.
- Fifteen mutants, each alone, at 1e-9 and 1e-12, against 516 tests (the same filter as the first read; baseline
  516 passed); four survivors again against `curve_pair`'s whole slow set (213 rows, 1e-9); one more against
  `arc_held_rows` alone at 1e-6, 1e-9 and 1e-12.
- The differential (`--profile ci`, geom, geom-brep, topo, sweep, step-import), head and base, both ε; the unit's
  eleven `arc_held_rows` copied into the base.
- `cargo fmt --all --check`, `scripts/run-gates.sh` (68 gates, exit 0), `scripts/work.py lint`, on a clean checkout
  of the frozen head.

ONLY READ: the diff of the head against its base under `crates/` (18 files, +1937 −383), `conic.rs`'s changed
regions, `arc_held_rows.rs` whole, the plane door, `level_on`, `one_curve_gap`, the rows filed and changed, the PR
body. NOT DONE: `cargo clippy` (so the `#[expect]`s are checked by the gate script and not by the lint itself), the
unit's rows at 1e-6, the slow set outside `curve_pair`, the `window::oracle` seed at 1e-12 on the base, the ignored
slow row.

Every count below is judged against the unit's 60-digit oracle, or against the base run, as stated.

## Findings

No MAJOR.

### MINOR

**m1. The seven meeting rows cannot go red on a refusal at 1e-9 or 1e-12, and the rewrite removed the `f64` lane's
demand for an answer.** DEMONSTRATED, `sure`. Mutant Q1 makes `line_conic` and `conic_conic` refuse every pose
(`Unsettled(Tangency)` on entry): all eleven `arc_held_rows` pass, at 1e-6, 1e-9 and 1e-12
(`review2/mutants/Q1-…patch`). Two reasons, both in `Case::check` (`arc_held_rows.rs:231-290`):

- "An answer is owed wherever the same locus held at its centre answers" is gated on `held_off ≤ 0.1·ε`. Measured
  for the eight poses (`review2/probe_gauged.rs`): `held_off` is 2.3e-10 to 2.8e-10 m at `a` = 1e6 and 1.3e-7 to
  2.2e-7 m at 1e9. So at ε 1e-9 and 1e-12 **no pose is gauged**, and every refusal is `eprintln!`ed as "excused".
  The body's "(`a` = 1e9 from 1e-9, 1e6 at 1e-12)" is off by one size: 1e6 is ungauged at 1e-9 too.
- Where it is gauged (1e-6, `a` = 1e6) the demand is relative to the centre-held twin read by the same kernel, so a
  change that refuses both passes.

The first head's rows were weaker elsewhere (ten escalations of tolerance, the kernel's own clearance as truth) but
their `f64` lane panicked on any refusal (`None => panic!("{what}: refused at f64")`), and `pair` panicked on a
refused twin. That is gone. K10 asked that "the `Interval` rows assert an answer where one is owed and do not pass on
any refusal"; at the tolerances the suite and the ladders run, neither lane asserts an answer anywhere. The right
answers of this unit are now held by the ladder alone, which no job runs.

**m2. `level_on`'s `x₀` is a centre summed from an evaluated point and its second derivative.** BY-INSPECTION,
`sure`; its being unpinned DEMONSTRATED. `pose.rs:2458-2466`: `x0 = gx·A²/2 + P″(0)·û_b`. `gx·A²/2` is `a(0)`'s
coordinate from `b`'s centre and `P + P″` is `a`'s centre, so `x0` is `a`'s centre in `b`'s centre frame; the comment
beside it says so ("`a`'s centre in `b`'s frame with no centre read"). K7 ruled "no centre summed from evaluated
points anywhere in the unit" and named the antipodal mean; this is the same quantity through `P + P″`, in code the
gate cannot see, and the body's table records it as "no centre summed from points". It feeds a bound and I found no
wrong value from it. And nothing reads it: mutant N5 (the `|x₀|` term of the fourth-derivative bound zeroed) survives
all 516 rows and the 213-row slow set, including the new
`the_windows_jets_are_derivatives_and_their_fourth_bounds_hold`, in whose fixture the two centres stand about half a
metre apart and the term is a few per cent of the bound. The term the fix pass rewrote, and derived `up::<36>` for,
has no row.

**m3. `Height::mean()` went and its value came back at its one call site.** BY-INSPECTION, with a mutant, `sure`.
`curve_face/pose.rs:684-692`: the graze's side is decided on
`(|hi| − |lo|).select_le_zero(hi − r, lo + r)`. `hi − r` and `lo + r` are the same number in ℝ, the centre's height,
which is what `mean()` returned; the selection chooses only which rounding of it is read. Mutant N1 swaps the two
arms and all 516 rows of geom and geom-brep pass at both ε, because the arms are equal (topo's zero-margin row,
which may tell the two roundings apart, was not in that run). K8 ruled the side "read from the extremes (crest
and trough)" and asked to be told if the reading truly needs what only `mean()` gives; the doc comment calls it "the
height's level between them". The side that is wanted is already decided by the sign of `|hi| − |lo|` on the line
above. As before, numerically safe; the new graze row is green and the topo zero-margin row did not move.

**m4. "Right on the base and refused on the head: none in the box" is true of the ladder's poses, not of the box.**
DEMONSTRATED, `sure`.

- My wall ladder, in the box, seam at the anchor, `Interval`, ε 1e-12: **22 readings right on the base and refused
  on the head**, all `curve_pair_pose_basin` on the circle door, all the 20ε twin read `apart` by the base, at 1,500
  and 1,850 m of arc from the anchor on fat sections from `a` = 1e3 to 1e9 (`review2/out/join-wall.txt`). At 1e-9
  there are none, and at `f64` none at either ε.
- The suspects' harness, every pose of which is within 3 m of an anchor at (1.3, −0.7, 0.4): **13 (1e-9) and 52
  (1e-12) `f64` readings right on the base and refused on the head**, family F's apart twins on
  `Unsettled(Tangency)` (`a` ≥ 1e8 and ≥ 1e5) and one meeting of family G on basin (1e-12, `a` = 1e5).

The implementer's report says "every right→refused is past the box"; the body says of basin "64 and 177 past the box,
none in it", and the P2 row "every one of them is past the session box". Each holds for the unit's ladder, whose
in-box poses all stand within 3 m of the anchor. The P2 row does carry family F in its second bullet, with the first
head's counts (14 and 45; on this base 13 and 51). None of this touches the acceptance (a refusal is not a wrong
answer), and K6 kept the `holds_only` change knowingly; the statements should say what they were measured on.

**m5. The pose the meeting row leaves out runs nowhere.** BY-INSPECTION, `sure`.
`two_thin_far_arc_held_ellipses_across_planes_meet` is `#[ignore]`d (`arc_held_rows.rs:497`). The repository's home
for a slow test is the slow set in `.config/nextest.toml`, which the nightly runs with `--ignore-default-filter`; no
job passes `--run-ignored` for geom-brep (the one `--ignored` in `nightly.yml` is editor-core's latency row). The
filed row says "an ignored slow row"; it is a row that cannot go red in any job.

**m6. Three more of the unit's own readings no row holds.** DEMONSTRATED (mutants; 516 rows at both ε and the 213-row
slow set at 1e-9), `sure`: N7, `one_curve_gap` without its `rounding_charge` (the charge is the whole of K7's "left,
on the centre, charged as before"); N8, `Slant::read`'s floor reading `H`'s wrong extreme at its turn; and S2 as
before. N7 and N8 are in the direction that certifies more.

### NOTE

**N1. Past the box, one reading on my wall ladder is a meeting reported for curves that are apart**: ε 1e-12, thin
`a` = 1e9, the line at 5°, θ = 0.0255, true gap 1.11e-11 (1.1 Kε), no twin shift; the base read it apart. It is past
the box and carries no claim; I record it because the first read found none in that direction and the design row's
fourteen have none.

**N2. "The box's own predicate"** is a predicate the harness wrote: no coordinate past `SESSION_BOX_EXTENT` at 4,097
evaluated points of each span (`ladder.rs`, `span_in_box`). The kernel's own reads a region's cover
(`geom_core::predicate`, `topo::session_box`). For spans a few metres long the two agree; the README and the body
should not call it the box's own.

**N3. "No in-box seam-off poses" is the ladder's choice**, as the body now says plainly; the seamed ladder's 7 and 28
in-box wrong readings stand, on the first head, under K4. I did not re-run it on this head.

**N4. The body's past-box `f64` count at 1e-9 is 7 and its own list sums to 8** (5 + 1 + 1 + 1); my join gives 8.
Every other count in the body I checked against my runs matches: the acceptance table, the in-box refusals by row,
the truth-outside-the-gap counts (420 of 465, 300 of 338, 44 of 52, 2 of 9, 1 of 3), the suspects' totals, the
fourteen and their base join.

**N5. K11's fix moves bits beyond the row it fixes.** `height_along`'s constant is now `plane_offset` (Dot2), and
`extent_along` is `height_along(..).extent(..)`, so every cover and box of a circle or an ellipse
(`geom/src/curves/boxes.rs`, `cover.rs`) reads a constant formed otherwise than on the base, which used the plain
dot. More exact, and the differential shows no row moving; "no centre-held row moved" is true, "no centre-held bit"
would not be.

**N6. `Ellipse::unit_in_plane`'s arc arm** is, by S3's own argument, reached by no live caller. R2 and R3 were
removed on that ground and this was not.

**N7. The truth in `arc_held_rows` is independent of the meeting and clearance readings, not of the evaluator**:
every truth point is `e.point_at(θ)`, directly or under a Newton iteration on it. Beside the anchor that is sound;
"a point computed independently" is more than it is.

## The rulings, K1 to K12

| | carried out as reported | fresh instance | left open |
|---|---|---|---|
| K1 | yes: the ladder flags box and seam, `join.py` gives the tables, no meter added, the row is the design question (P2, `design: true`) | no | N2 |
| K2 | yes: three causes in the row and body; the square frame stated at the module bullet and on `height_of`, `level_at`, `height_along`; the pose on the onecopy row | no | none |
| K3 | yes: the tie reads the nearer extreme by the extremes' values; the new row is red on the old tie (1 row) and on the swap (2) | no: 160 touches probed, none wrong | none |
| K4 | yes | no | N3 |
| K5 | yes, but for one count (N4) and the scope of "in the box" (m4) | no | m4 |
| K6 | yes: row corrected and banded P2 | no | m4 |
| K7 | `one_curve_gap`: yes, on the gated door, counted, listed. `level_on`: **no** | **yes** (m2) | m2, m6 |
| K8 | `mean()` removed, `Debug` opaque, the arc-held graze row added: yes. "Read from the extremes": in letter | **yes** (m3) | m3 |
| K9 | yes (both seeds present) | no | none |
| K10 | R1, R4, R9 killed: yes. Truth points and tolerance: yes (N7). "An answer where one is owed": **not at 1e-9 or 1e-12** | the rows lost the demand they had | m1, m5 |
| K11 | yes: one home (`over_anchor`), the gated doors carry their `#[expect]`s, the gate passes; the regression's fix restores the base's formula for the constant | no | N5 |
| K12 | yes: `crest_roots` and `arc_past` have one home in geom, `MAP_RESIDUAL` and `all_round` gone, `up::<36>` derived, `band_arc`'s `unreachable!` gone, header refreshed | no | none |

A ruling I would revisit: none is wrong. K1(b)'s re-cut is the right reading of the README. I note only that the
acceptance now rests on a measurement nothing re-runs, and m1 removes the one guard in the suite that stood in for
it.

## The claims

**D1.** Holds. Unit's ladder, in the box with the seam at the anchor, 432 per lane per ε: head wrong 0, right→wrong 0,
right→refused 0; base wrong 29, 36, 107, 72; my `join.txt` equals `ladder/join-base-head.txt` byte for byte and all
four tables likewise. Past the box 5 and 9 `f64` wrong; of the fourteen the base read 4 rightly, refused 3, read 7
wrongly: confirmed. My poses: 990 per lane per ε in the box, out to 1,850 m of arc, sizes 1e2 to 1e9: head wrong 0,
right→wrong 0 on both lanes at both ε (the base: 51 and 72 wrong at 1e-9, 222 and 139 at 1e-12). `on_carrier` at 781
in-box points per ε: no wrong answer and no definite answer inside the gap. Suspects: head 0 and 0, base 169 + 6 and
570 + 24, no apart twin read as a meeting. The predicate: N2. Seam-off: N3.

**D2.** True of the ladder; not of the box: m4. Refusal names complete and counted as claimed (N4 apart).

**D3.** `one_curve_gap` as stated. `mean()` is removed. But two centres stand in the diff under other names: m2
(`P + P″`) and m3 (`hi − R`, `lo + R`). I found no other (swept the added lines for means, antipodes, midpoints,
`P + P″`, extremes' sums, `center()`, `frame()`, `origin()`).

**D4.** Holds: m1 of the first read is fixed, the row is red on both mutants, and the siblings are sound (K3 row
above).

**D5.** Truth points: N7. Tolerance: the band's escalation, yes. Fat and thin, 0.3 m and 5 m: yes. "Owed": m1. R1,
R4, R9: killed (R1 by six centre-held circle rows, none of them arc-held; R4 and R9 by the new jets row). R2 and R3:
the code is gone. Red on the base: all eleven `arc_held_rows`, copied in, fail there at 1e-9 and at 1e-12.

**D6.** The fix restores what the base did: its plane door read its constant through `plane_offset`, and the first
head replaced that with a plain dot. `ruled_region_rows` is untouched against the base (25); `KEPT_TORUS` is still
re-baselined 33 → 25 and the plane-crossing fixture still moved, as on the first head. N5 for the bits.

**D7.** Re-planted: S1, S2, S3 (survive, as claimed; S2 also survives the slow set), R1, R4, R9, the old tie, the
swap, `height_of` from the derived centre (8 rows). Mine: N1 (equivalent, which is m3), N2 the arc-held crest halved
(killed by 4 rows, three of them in `arc_held_rows`: better than the first head, where its twin died on one door
assertion), N5, N7, N8 (survive, m2 and m6), Q1 (m1). S1's and S2's arguments hold. S3's holds on the callers that
exist (N6).

**D8.** The design row: true of the tree, its fourteen match my join, P2 with `design: true` as ruled. The `Interval`
row: P2 as ruled; its "past the session box" is m4. The 70 s row: filed, P3, defensible as a cost; its row runs
nowhere (m5). The general-anchor evidence and the seed: present. `work.py lint` passes. The body: true of the frozen
head but for m1's size, m4's scope, N2 and N4. The differential: head 10,777 tests, all pass at 1e-9, and at 1e-12 all but
`pcurve_cache::ruling_reading_rows::a_true_ruling_keeps_the_closed_form_on_both_nappes` and sweep's
`a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`; base 10,761 tests (without the unit's rows), all
pass at 1e-9 and the same two fail at 1e-12. As the body says; nothing else differs.

## Style

Questions exercised: Q1 (the sweep of D3), Q2, Q3 (mutants), Q5, Q6, Q7. Q8 not repeated: `conic.rs` was read whole
on the first pass and its changed regions now.

- **A comment that argues the code is not what it is.** `pose.rs:2454-2457` ("with no centre read") and
  `curve_face/pose.rs:684-688` ("the height's level between them"). Each names the centre and says it was not read.
  `sure`.
- **The fix-mints-the-defect trap, a second time on the same two sites** (m2, m3). Class: any centre a ruling sends
  away from a named door; the sweep for it is by value (what equals the centre in ℝ), not by name. `sure`.
- **A guard measured against the thing it guards.** `Case::check`'s owed answer is relative to the same kernel's
  centre-held reading. `sure`.
- **`eprintln!` as a verdict.** "Excused" lines are the rows' main output at the default tolerance and nothing reads
  them. `sure`.
- **`OnSurface::plane` reads the gated centre frame of an ellipse to throw its origin away**
  (`on_surface.rs`, `self.carrier.frame()?.with_origin(at)`), which keeps an `#[expect]` on the function for a read
  it does not need. `likely`.
- **The harness predicate named as the kernel's** (N2). `sure`.
- **`Over`'s level is now pinned only by circle rows** (R1's six). The arc-held placement it was written for does not
  need it beside the anchor; fine, but the row list in the body reads as if the arc-held rows held it. `likely`.

## Ratings, for the unit as it now stands

- **Idiom and structure: 4.** One home for the root rule and for the extremes, `Height` opaque in earnest, `Window`
  holds a `Height`, the dead arms are gone; held back by the two respelled centres.
- **Test quality: 3.** The door rows and the jets row now kill what survived before (R1, R4, R9, the tie, the crest),
  and the truth points are no longer the kernel's own readings; but the meeting rows pass with every pose refused,
  the slow pose runs nowhere, and three unsound mutants in the unit's own code survive everything.
- **Doc and comment honesty: 4.** The body and rows now say what the unit claims and what it does not, with joins;
  held back by "the box's own predicate", "none in the box", and two comments that deny the centre they form.
