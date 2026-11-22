# Review: PR 7294 (`oneround/unsourced-unions`), frozen head `fe5a961f4`

**Verdict: APPROVE-WITH-FIXES.** No conversion makes a verdict less sound than
the base. Every planted mutant the PR names is killed by the row it names, and
the suites in `geom-brep` and `topo` pass in both trees at 1e-9 and 1e-12. What
needs fixing:

- The "from below" label on the pivot-free diameter levers is false of the
  README's lever (MINOR 1).
- One converted consumer does not act on every sign, though it is said to
  (MINOR 2).
- The PR body misdescribes some of what moved and some of where its maxima
  pivot (MINOR 3).
- Three "unkillable" claims, and one both-bounds path, have no row (MINOR 4).

## What I executed and what I only read

**Executed:**

- **Base tree.** PR 7227's branch as this unit merged it (`2e721a928`), merged
  with main `3900eb46`. Conflicts resolved to main's side, as the head did. Its
  `crates/` differ from `fe5a961f4` only in the unit's 15 files. The literal
  `git merge-base fe5a961f4 origin/oneround/unsourced-bits` is `3900eb46`
  (main), which holds none of 7227. 7227's current head `dc75d8ffe` holds a
  later main that this branch does not.
- **Verdict differential.** A temporary print in `k_stats::classify`
  (`verdict-probe.patch`) wrote (test, predicate, sign or escalation, margin)
  for the touched predicates. Both trees ran `cargo nextest --profile ci -p
  geom-brep -p topo` at `CAD_TOLERANCE_EPS` 1e-9 and 1e-12: 6497 tests on the
  head, 6488 on the base. Every fuzzer that reaches these sites was re-run in
  both trees at both ε with `CAD_FUZZ_SEED=0x5eed` (150 tests). The first,
  unseeded pass showed moves in fuzzers that seed from the clock. Seeded, they
  moved nothing.
- **Mutants.** 24 planted in a third worktree, each run against its module's
  rows (`mutants.py`, `mutant-results.txt`).
- **Probes.** Two, in `crates/geom-brep/src/curve_pair/review_probes.rs`.
- **Gates.** `lever-sources.sh`, `lever-doors.sh` and `cover-kind-match.sh` on
  the frozen head.
- **CI.** Run 45060992195, at step level through the jobs API.

**Only read:**

- Every converted site and its consumer.
- `extent.rs`, `cover.rs`, `validate::decide_levered`, and
  `offset_together::solve_planar_corner`.
- The rows of C4, C5 and C7.

**Not run:**

- `sweep`, `editor-core`, `profile` and `demos`.
- The slow set, beyond the slow rows inside the `ci` filter of the two crates.
- The Interval lane on its own.

## The brief's premises, corrected

- **NOTE (sure).** The review branch `oneround/unsourced-unions-review` was at
  `69f31a4e3` (origin/main) when this lane started, not at the frozen head. My
  first gate run and first build were therefore of main: 63 unsourced sites,
  100 registered reads. I caught this and reset the branch to `fe5a961f4`.
  Every figure below is from the frozen head.
- **NOTE (sure).** "PR 7227 (merged into this branch)" holds only up to
  `2e721a928`. 7227 has since added `fea8d2bd8`, which rewrites `corner_arms`'
  doc, and `dc75d8ffe`. Neither is here, so `corner_arms` will conflict when
  7227 lands.
- **NOTE (sure).** C6 says "the two new cover-door readers registered". Three
  were registered: `overlap::one_sense`, `chart_region::boundary_reach_below`
  and `offset_axial::corner_arms`. The PR body itself says three.

## CI 45060992195, at step level

- **Ran green:**
  - lint/gates (gates 7m43s, rustfmt, clippy, rustdoc).
  - test (kernel) and test (app): build, default eps, eps 1e-6/1e-12 over the
    touched crates, the diff's slow-set tests, doc-tests.
  - corrupt-input (release).
  - mesh budget.
  - gate ok.
- **Skipped:**
  - "prime the build cache".
  - The **interval** job.
  - python, tools, step import, demos and viewer.
  - Inside both test jobs, "the slow set did not run", and the
    failing-tests summary.
  - "change filter self-test" in lint.

Hosted CI therefore ran no Interval-backend job and no demos. The PR's
Interval-lane and tour statements rest on the author's local runs.

## Gates and counts (judged against the frozen head's own scans)

- `lever-sources.sh` OK: **45 sites** (as claimed), 859 fixture levers.
- `lever-fixtures.counts` topo **308** = main's 301 (`3900eb46`) + 7227's 6
  (`2e721a928` reads 306) + this unit's 1 (`fixture_lever(PI)` in the corner
  row). True.
- `lever-doors.sh` OK: 58 pinned sites.
- `cover-kind-match.sh` OK: 103 reads of 26 doors (main has 100, so 3 added).
- `curve_face/pose.rs 2` is main's pin, unchanged. The line-cone site at
  `curve_face/pose.rs:991` keeps main's marker. C6 holds.
- The eight references named in C1 (rim 1, ring_path 1, overlap 1,
  chart_region 2, curve_pair/pose 3) plus `corner_arms`' one are gone. That is
  nine in six files, as the PR body says. Rim was one reference in the base,
  so "one site, not two" is true.

## The differential (C2)

Judged against the merged base above, over `geom-brep` + `topo` `ci`, at both
ε. Counts are tests that reach each predicate. They are lower than the
author's because I ran two crates of their five.

| predicate | reached | verdict moved | bits only |
|---|---|---|---|
| `curve_pair_pose_tilt` | 408 | 13 at 1e-9, 12 at 1e-12 (deterministic) | 338 |
| `bool_overlap_senses` | 65 | 0 | 65 (margin ratio 2.2e-8 to 13) |
| `chart_region_cyl_tilt` / `_axis_sense` | 25 / 17 | 0 | 8 / 3 (equal to the base's to 4 significant figures) |
| `rim_circle_axis_parallel` | 5 | 0 | 1 (ratio exactly 0.5) |
| `offset_axial_corner` / `offset_together_corner` | 2 / 17 | 0 | 0 |

- **The 13 tests that move at 1e-9 are the author's 13 `ci` tests by name:**
  census ×3, meet_rows ×3, `review_census_fixpass_probes`, union_flush ×6.
  Every one passes in both trees.
- **At 1e-12, 12 move.** `fixpass_a_plug_reaching_over_a_prong_by_w` moves
  at 1e-9 only (see MINOR 3).
- **The "toward the truth" case is confirmed against an exact computation.**
  At 50 significant digits the segment passes 9.2884615e-10 m from the arc's
  middle, inside the zero band (1e-9), so a meeting there is right. At the
  second band it passes 1.0615385e-9, inside the gap (1e-9, 1.2e-9), so the
  head's escalation on `curve_carrier_gap` is right too.
- **Direction.** For a reading that acts on its Zero, a longer arm is the
  conservative direction the README requires ("acts on its Zero reads it from
  above"). A Zero that becomes definite over a much longer `over` is that
  direction. With a line in the pair, the longer span's inf is never shorter
  than the shorter span's sup, so the second tilt read (`!definite_as` at
  `arm`) can never act on a Zero that `over` refused. I checked this by
  inspection; the remaining risk is spelled out under MINOR 1.

## MAJOR

None.

## MINOR

**MINOR 1. The "from below" diameters are not bounds from below of the
README's lever.** BY-INSPECTION plus a measured wrong value, `likely`.

- **Contract and sites.** README "The consumed extent": the lever "is the
  extent's far distance from that point" at which the datum is read. Two
  conversions read pivot-free diameters and call them "from below" for a
  definite claim. `curve_pair/pose.rs` `span_extents` (doc: "Each is a
  diameter, read from no pivot"), and `ring_path.rs:1794–1800`'s crossing,
  whose sine is read at the meeting point on the piece.
- **Measured.** A diameter from below is not a bound from below of the reach
  from a point of the edge, which lies in [D/2, D].
  `review_probes::probe_edge_extent_inf_exceeds_the_reach_from_an_arcs_middle`:
  a unit semicircle's `edge_extent_inf` is **2**, its cover's far distance
  from its middle **√2**.
- **Router.** `review_probes::probe_two_segments_within_band_everywhere_are_not_a_crossing`,
  band (1e-9, 1.5e-9): two unit-half-length segments crossing at their
  middles, tilted 0.9e-9. Every point lies within 0.9e-9 of the other line,
  yet the tilt reads **Positive (1.8e-9) at the diameter 2**, and the router
  poses a crossing. Over the reach from the crossing (1) it reads Zero. The
  meet then escalates on `curve_pair_meet_stretch`, so no wrong answer
  escapes.
- **Pre-existing.** The base reads identically: the old `L + 2e` and metered
  arms were no shorter. And `ring_path`'s arc moved from the arc length
  (longer) toward this.
- **What is not true.** The site table's "From below. A crossing is a definite
  claim" is not true of the README's lever.
- **Class.** Every `edge_extent_inf` caller levering a datum read at a point.
  `edge_extent_inf`'s own doc lists "a section's crossing" against the
  diameter, a second contract beside the README's.

**MINOR 2. `cap_pair_corner`'s cap-pair sine refuses on Zero, but is decided at
both bounds as if it acted on every sign.** BY-INSPECTION, `sure`.

- **The premise, and where it fails.** C1(f) and the PR say "the verdict acts
  on every sign: definite serves the branch root, Zero serves the tangent
  foot". That is true of `offset_axial_corner` in `solve_pairs` and
  `cap_pair_corner`'s second loop. It is false of the first loop,
  `offset_axial_cap_pair` (`offset_axial.rs` at `cap_pair_corner`, the
  `decide_levered(("offset_axial_cap_pair", "offset_axial_cap_pair_straddle")`
  call). There `Ok(_)` returns the typed refusal "the two moved meridian caps
  meet in no line …". Only Positive goes on, so the bound owed is from below.
- **Effect.** On an edge with `Between` bounds (an ellipse, spiric or spline
  edge at the corner), a pair that reads Zero below and Positive above now
  escalates on the straddle row instead of refusing typed. A new row name
  exists for a verdict that needs one bound.
- **No row tells the two apart.** Mutant TM11 (decide at `below()` only)
  survives the 71 offset rows.
- This is the sibling units' lesson in reverse: a consumer was read as acting
  on every sign without reading each arm.

**MINOR 3. The PR body misdescribes what moved and where its maxima pivot.**
DEMONSTRATED, `sure`.

- **Not all moves are Zero to definite.** "32 of the 34: the first tilt reads
  Zero in the old mode and definite in the new … roughly 5·10⁷ times longer"
  does not describe all of them. In census
  `an_edge_in_a_face_bounded_on_its_corners_points_is_structural` and five
  `union_flush` rows, the base's first read **escalated** ("EP" becomes "P").
  In census `an_undecided_chord_cuts_where_a_chord_would` and
  `meet_rows::…far_side_in_the_escalation_band…`, a base **Positive becomes an
  escalation** at the first read ("PP" becomes "EP"): there `over` is
  *shorter* than the old `L + 2e`. Final answers are unchanged.
  (`differential-1e-9.txt`.)
- **Not identical at both ε.** "At both ε identically" is false for
  `fixpass_a_plug_reaching_over_a_prong_by_w`. At 1e-9 two reads sit on the
  escalation edge (base 1.0000000139e-8 Positive, head 9.99999994e-9
  escalates). At 1e-12 nothing moves.
- **Not one shared pivot.** "In `rim_wedge`, `overlap` and `chart_region` it
  folds reads taken from one shared pivot … No sum of reads taken from two
  different points has been replaced by a `max`" is false of `chart_region`.
  It folds reads from **two** pivots, `a.origin()` and `b.origin()`
  (`chart_region.rs` `cylinder_pair_overlap`, the `for pivot in [a.origin(),
  b.origin()]` loop). This is sound for the from-above tilt gate. For the
  from-below sense row it is sound only because the axis cosine carries no
  point of its own, and the body does not say so.
- **The counts disagree.** The tangent-foot row says "193 fast-set rows" reach
  `corner_arms`; the PR body says 204/205.

**MINOR 4. Rows owed where "unkillable" or unexercised.** DEMONSTRATED
(surviving mutants and a measured reach), `sure`/`likely`.

- **Overlap.** "A definite sign changes only for a face under K·ε, which no
  valid body builds" is false. `shell_role_reader::a_boolean_leaving_a_turned_needle_builds_it_in_every_pose`
  reaches `one_sense` with a needle face whose boundary reach from `p` is
  **2.236e-8 m**, 2.2× the escalation width at 1e-9. A thinner needle would
  separate the unit arm (TM14 survives) from the reach. `likely`.
- **Which span each lever reads.** Swapping min/max in `span_extents`
  (crossing at the shorter span's inf, level at the longer's sup; GM5)
  survives all 195 `curve_pair` tests. No row pins *which* span each lever
  reads. `sure`.
- **The corner's both-bounds decision.** Deciding the corner at `below()` only
  (TM9) or `above()` only (TM10), and the planar door at `above()` (TM12), all
  survive. No row reaches a corner edge with `Between` bounds, so
  `offset_axial_corner_straddle` has no row. `sure`.
- **Ring arc.** The bound swap (TM15) and the old arc length (TM16) survive the
  `ring_path` rows. The author says only the slow ring suites reach the arc
  arm, which I did not run. `unsure`.

**MINOR 5 (C4). The tangent-foot row's "the lever is not the question" is
doubtful.** BY-INSPECTION, `unsure`.

- **What happens.** `solve_pairs` acts on a Zero of `offset_axial_corner` by
  serving the foot.
- **Why the lever matters.** The row's own estimate of the foot's error is
  `r_profile·ε/reach`. It scales with the ratio of the profile curve's radius
  to the arm, so the Zero is acted on at an arm shorter than the length the
  served answer is displaced over. "A verdict that acts on its Zero reads it
  from above" is met only if the edges' reach bounds that length, and here it
  does not.
- **Reading.** I read it as the both-bounds decision serving a Zero that it
  should refuse where the profile curve is wider than the edges, rather than
  as the foot's own `√(2r·gap)` contract. The row is filed for shell and no
  suite lands there.

**MINOR 6 (C5). The stored-reach row's reason is overstated.** BY-INSPECTION,
`likely`.

- **The claim.** `work/oneround/stored-reach-scalars-a-union-cannot-be-read-from.md`
  says "the union of the two faces cannot be read again from one pivot".
- **Why it can be bounded.** Each `Carrier` stores a centre and a far distance
  (`measure.rs:669–676`), so the union's reach from `pa` is bounded from above
  by `max(ra, |pb − pa| + rb)` (`ExtentBall::lever_from`). That bound is
  tighter than the summed `arm` (`measure.rs:660–664`).
- **The real obstacle.** What blocks the conversion is the door (a stored
  scalar becomes a ball only through `ExtentBall::unsourced`) and the missing
  bound from below, not the data.
- **The profile row is true of the tree.** `Seg::arm` at `seg.rs:1490`,
  `arc_lever = min(radius, chord)` at `seg.rs:1462`, `len` the stored chord at
  `seg.rs:49–53`, `wn` the anchors' separation in `path.rs`. Note that
  `Seg::arm` reads the record's stored `len`, so the "two-point read" kinds are
  also record fields.

## NOTE

- **C1(a).** The rim reads both circles from `ra.center`, one pivot: a union.
  TM1 and TM2 are killed. TM3 (each circle read from its own centre) and TM3b
  (pivot `rb.center`) survive, as the author says, because the centre datum
  has already decided the centres coincide. `sure`.
- **C1(b).** `one_sense`: Positive goes on, Negative refuses typed
  (`OppositeSenses`), Zero refuses. So from below is right. A face's boundary
  reach from `p` is a lower bound on its region's reach. `sure`.
- **C1(c).** Piece length: Zero answers `Some(0)` (it acts on Zero), and sup
  is right. Crossing: anything not Positive gives `Ok(None)`, so from below is
  owed; see MINOR 1 on what kind of bound inf is.
- **C1(d).** Tilt gate: Zero proceeds, from above, and the max over two pivots
  is sound for an upper bound. Sense row: `decide_nonzero`, so from below
  (MINOR 3 on the pivots). TM4 is killed by the far-peg row, which does reach
  the gate at both ε (probe log: `chart_region_cyl_tilt Zero`). TM5, TM6 and
  TM6b survive, as claimed.
- **C1(e).**
  - Route: only Zero of the first read is acted on, as claimed.
  - `line_line`: the level road's answers are `Coincident` (decided by
    `one_line`), a crossing (decided by a definite read at `arm`) or Meter. So
    "the level road's only answer is Coincident" is not quite true.
  - `line_conic`: the level road runs the in-plane analysis and serves
    meetings, not `Coincident`. That premise is false there. It is harmless
    because with a line in the pair the Zero is always also `over`'s.
    `likely`.
  - GM1–GM4 and GM6 are killed as claimed (GM3 also reds
    `pose::window::oracle::each_window_reading_holds_on_a_dense_distance`).
    GM7 (`line_line`'s old metered arm) survives, as claimed.
- **C1(f).** The planar door: Zero moves to the next triple, and none left
  refuses, so from below is right. TM7 and TM8 are killed as claimed.
- **C7.** Each red named has a row: the three rows carry the dated sections.
  `a_corner_reads_only_its_own_sectors` is held by the open
  `scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type`.
  `square_search` is named by several open germ/onedoor rows; I could not tell
  which one holds it (`unsure`). I did not re-run these reds (slow set and
  other crates). In my two-crate `ci` runs, the base and head failed only my
  own probe, identically.

## Style

**Questions exercised:** Q1, Q2, Q3, Q4, Q5, Q6, Q7. **Q8 not exercised**: I
read the touched functions and their neighbours, not a whole file. The largest
touched file is `chart_region.rs`, 6751 lines.

- **Q1.** `overlap.rs` `one_sense`'s `reach_below` closure and
  `chart_region.rs`'s `boundary_reach_below` are two hand-written folds of
  `edge_reach_bounds(..).below()` over a face's boundary with `Lever::max`.
  They refuse a lone vertex in different words (`OverlapUnread::LoneVertex`
  against `ChartRegionError::Corrupt`). `cover.rs` has the from-above face
  read (`face_reach_from`) but no from-below twin. This fix closes unsourced
  levers by minting a new duplicated reader, which is the trap the style lane
  names. Look at `census/curved.rs:203`, `validate.rs:9396` and
  `boolean/fate.rs:505` for more per-edge folds. `likely`.
- **Q1/Q7.** `Rim::reach_from` (`rim_wedge.rs:65–78`) re-mints a `Circle`
  from the rim's stored fields to read its cover, where the rim's own edge
  cover is the region. `unsure`.
- **Q2.** `span_extents`' doc ("Each is a diameter, read from no pivot") and
  the PR's "none: diameters" pivot column justify a departure from the README's
  pivot rule in prose. The departure is not argued in the README. `likely`.
- **Q2/Q7.** `cylinder_pair_overlap` accumulates `above`/`below` in `Option`s
  only to `unreachable!("two faces read from two origins")`. A fold over the
  four (face, pivot) pairs would need neither. `sure`.
- **Q3.** `lever_rows` rows 1 and 4 assert the *first* recorded
  `curve_pair_pose_tilt` verdict. They pin the router's internal read order,
  not its answer, and a reordering that keeps the answer turns them red.
  `likely`.
- **Q3.** `a_far_peg_levers_its_tilt_from_its_own_origins` has a
  stand-down branch (`fixture_minted_outside_inventory` returns early).
  Without the probe log there is no way to see that it ran. It did run at both
  ε here. `likely`.
- **Q4.** README "The consumed extent" says the lever is the far distance from
  the datum's point. `edge_extent_inf`'s doc says the crossing lever is the
  edge's diameter. These are two contracts for one quantity; this unit cites
  one at some sites and the other elsewhere (class in MINOR 1). `likely`.
- **Q5.** `route`'s doc ("zero over the shorter span, or over the longer span,
  level") describes a second path that cannot fire for any pair with a line,
  which is every caller. `likely`.
- **Q6.** The author's measurement claims ("no verdict moved at corner_arms",
  "demos at 1e-9 only") have no guard and no scheduled re-measure. The tangent
  foot has its row. `unsure`.
- **Q7.** `cover-kind-match.readers`' new `one_sense` row is labelled
  `two-sided` while its text says "only Positive goes on". That is the label
  of a from-below reader whose cover is exact by kind. `unsure`.

**Ratings:**

- **Idiom/structure: 3.** Clean conversions, `LeverBounds` reused, the
  per-kind match gone. But a twice-written boundary fold, and the
  `Option`/`unreachable!` accumulation.
- **Test quality: 3.** Every named mutant is killed by its row, and the far-side
  row checks against exact geometry. But GM5, TM9–TM12, TM11 and the overlap
  unit arm all survive, and two rows pin the read order.
- **Doc/comment honesty: 3.** The site table and consumer reads are mostly
  right. But "from below" for diameters, "acts on every sign" for the cap-pair
  row, "one shared pivot" for `chart_region`, and the moved-test narrative
  are each false somewhere.

## Files on this branch

- `analysis/oneround/unsourced-unions/`: this report,
  `verdict-probe.patch`, `mutants.py`, `run.py`, `mutant-results.txt`,
  `cmp.py`, `seq.py`, and `differential-{1e-9,1e-12}.txt`.
- `crates/geom-brep/src/curve_pair/review_probes.rs` (and its `mod` line):
  the two probes. One is red by design (MINOR 1). One prints the router's
  escalation (MINOR 1) and passes.
