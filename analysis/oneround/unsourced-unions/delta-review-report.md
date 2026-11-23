# Delta review: PR 7294 (`oneround/unsourced-unions`), `fe5a961f4` → frozen head `2a4d0cb82`

**Verdict: APPROVE-WITH-FIXES.** No MAJOR.

The fix pass does what it says at the sites it names:

- F1: the router reads the spans' reach from the overlap's middle, and the
  ring arc reads its reach from the meeting.
- F2: the cap pair keeps both bounds, now for a stated reason that holds.
- F7: there is one home for the boundary's reach from below.

All 21 of the body's mutants that I re-planted on this head are red on the rows
it names, and the merge drops nothing of main's.

What is owed:

- MINOR 1: the class row omits two in-class crossing levers that sit in the
  same loop as the arc that was fixed.
- MINOR 2: one paragraph of the PR body still describes the pre-fix router.
- F4(c) is unrefuted, not proven, and its stated mechanism does not hold at
  my probe's pose.

## Executed and read

**Executed:**

- **Worktrees.** One at the frozen head (`2a4d0cb82`). One at main as merged
  (`2a4d0cb82^2` = `b3cc4b38`).
- **Mutants.** Re-planted on the head, run against their modules' rows:
  `cargo nextest -p geom-brep --lib` for `curve_pair`, and `-p topo` for the
  rest (`mutants.py`, `delta-mutant-results.txt`). Mine compile only where a
  mutant builds; D8 and G1 were re-run after I fixed two mutant-side type
  errors.
- **Probe.** `review_arc_probe.rs`, run through `spline_crossings` itself.
- **Gates.** `lever-sources`, `lever-doors` and `cover-kind-match` on a clean
  head tree, and `GATE_BASE=origin/main lever-sources`.
- **`payload-rung-sweep.py --check`** on head and main.
- **`cargo fmt --all --check`** on the head.
- **clippy.** `-p geom-brep -p topo --all-features --all-targets -D warnings`:
  clean.
- **The verdict print.** On `census::a_transverse_declared_pair_holds_no_crossing_its_edges_hug`.
- **CI.** Run 45154911253 at step level, with the two failing logs.

**Read:**

- The diff of the unit's own commits.
- `span_reaches`, `route`, `spline_crossings`, `cover::boundary_reach_below`,
  `chart_region`'s fold, and the new `offset_axial` rows.
- The class row and the shell row.
- `Curve3::cover_over`.

**Not re-taken:** the F3 differential (the `ci` and slow sets in both
modes). I checked one of its rows (claim 5) and no others.

## Claims

**C1 (F1). Holds, at the two sites it names.**

- **`span_reaches`** (`curve_pair/pose.rs:964–985`) reads each span's cover
  from `mid`. `mid` is the point on the first line at the projected overlap's
  middle, where the offset is read (`:1552`, `:1617`).
  - **`over` = `min` of the two reaches from above.** A meeting is a point of
    both spans, so the region where they can meet reaches from `mid` no
    farther than either span. The `min` of the upper bounds bounds it from
    above, as a Zero acted on (Level) needs. `sure`.
  - **`lever` = `max` of the two reaches from below.** Both are taken from one
    pivot, so this is the union's reach from below, as a definite crossing
    needs. `sure`.
- **The ring arc** reads `piece_reach_bounds(.., curve.eval(tp)).below()`
  (`ring_path.rs:1876–1885`), the arc's reach from the meeting, where the
  sine is read. `sure`.
- **The pivot.** D6 (each span read from its own middle) is red.
- **No diameter is left at a site F1 names.** At sites it does not name, see
  MINOR 1.

**C2. The class row omits callers.** MINOR 1.

- **What my sweep covered.**
  - Every production caller of `edge_extent_inf`/`edge_extent_sup` (non-test,
    non-rows) is on the row: `certify.rs:5652`, `conic_bend.rs:82`,
    `curve_face/pose.rs:991`, `edge_nurbs.rs:2152`,
    `step-import/build.rs:2402`, `contact_verify.rs:337`, `census.rs:398`,
    `offset_derive.rs:796`, `replace_face.rs:2829/3260/4186`,
    `ring_path.rs:1816`.
  - The other callers are test modules: `recl.rs:3128`, `sectors.rs` ×3,
    `vtxfac.rs:3516`, `bound_side.rs` ×4 (all under `#[cfg(test)]`), and
    `link/cone_fuzz.rs`.
  - The row's line numbers are about 20 lines stale after main's merge (it
    says 5602, 2404, 399, 792, 2849/3280/4206 and 1802). NOTE.
- **The sweep shaped differently: chord and diameter reads through the
  point doors** (`ExtentBall::point(a).lever_from(b)`, `points_diameter`,
  `points_lever_from`). This turns up two in-class crossing levers in the very
  loop F1 fixed:
  - **`ring_path.rs:1826`**, the Segment/Ruling piece:
    `ExtentBall::point(from).lever_from(to)`, the segment's chord (its
    diameter).
  - **`spline_wall.rs:215–224`**, the Iso piece: `IsoPiece::travel().0`, the
    chord between its ends.

  Both lever `split_ring_path_spline_across`, a sine read at the meeting, from
  below. A segment met at its middle reaches `L/2` from there; its lever
  reads `L`. This is the router's review row exactly, with the arc fixed and
  its two siblings in the same `match` left as they were.

  The row's title scopes it to `edge_extent_inf`/`edge_extent_sup`, so these
  are outside its letter and inside its class. The row also says it lists
  every `ring_path` caller "because the review asked", yet it names only the
  no-length piece reading.

  No row pins the segment arm: my mutant G4 (the segment lever halved, which
  is its reach from a middle meeting) survives all 53 `ring_path` rows.
  `sure` (by inspection plus a surviving mutant).

**C3 (F2, F4). Every mutant the body lists is red on this head.**

- **Killed by the rows the body names:**
  - D1, the diameters (3 rows).
  - D2, the min/max swap (`the_level_tilt_takes_the_nearer_span…` among 5+).
  - D3, the bounds swap.
  - D4, `over` from below.
  - D5, the crossing from above.
  - D6, the own-middle pivot.
  - D7, `route`'s L+2e.
  - D8, `line_conic`'s semi-axis.
  - O1, overlap's unit arm.
  - O2, overlap's world pivot.
  - R1/R2, the rim's sum and world pivot.
  - C1, chart's world pivot.
  - A1, corner arc length.
  - A2, corner from the arc's middle.
  - A3/A4, `solve_pairs` below-only and above-only.
  - A5/A6, the cap pair below-only and above-only.
  - A7, the planar door at above.
- **One caveat on D5.** It also reds
  `pose::window::oracle::each_window_reading_holds_on_a_dense_distance`. That
  is a seeded oracle the head's own commit `76299f03e` records as red on main
  on a second seed, so that extra red is not evidence. `likely`.
- **F2's reason holds.** The cap pair's Zero is a typed refusal that claims
  the caps parallel, so it acts on its Zero (from above), and its Positive
  proceeds (from below). Both bounds are right. `sure`.

**C4 (F4(c)). Unrefuted, not proven.** The argument's conclusion survived my
attempt; its stated mechanism did not.

- **The mutants survive.** G1 (the base arc length), G2 (the diameter) and
  G3 (the bound swap) survive all 53 `ring_path` rows, as the body says.
- **The probe** (`review_arc_probe.rs`, run through `spline_crossings`
  itself):
  - A unit semicircle piece over `(0, π)`, met at its middle by a straight
    degree-1 spline tilted out of the arc's plane by `s`.
  - The band is `(1e-9, 1.3e-9)`, and `s ∈ {6e-10, 6.8e-10, 7e-10}`, where
    `√2·s` reads Zero and `2s` and `πs` read definite.
  - Result: `meet` escalates on `curve_pair_meet_stretch` before
    `split_ring_path_spline_across` is read. It finds the meeting at
    `t = π/2` and a second in-band meeting about `3.2e-6` along.
- **The premise is false.** The argument says "the spline stays within about
  the band of the piece over the whole reach". That is not true here. The
  curves separate quadratically (curvature 1 against 0) and are within band
  only over about `±√(2ε)`. What refused is the stretch rule, which finds two
  in-band meetings within the escalation width, not a shared run over the
  reach.
- **What remains open.** I found no pose that reaches the function with the
  mutant deciding differently. But a pose whose curves match curvature well
  enough to pass the stretch rule and still meet at a sine between `ε/√2r` and
  `ε/2r` is not excluded by anything I read.
- **The bound swap** really is unkillable at a circle piece: the cover is
  exact by kind, so `below() == above()`.

**C5 (F3). Partly checked.** I re-took one row and not the table.

- **The row:** `census::a_transverse_declared_pair_holds_no_crossing_its_edges_hug`
  on this head. Both of its moved tilt reads escalate: the first at
  `6.65e-9`, the crossing read at `9.987e-9`, against an escalation width of
  `1e-8` (ε = 1e-9, K = 10).
- **The refusal is right by the README's lever.** The farther span is a line,
  so its reach from the overlap's middle is exact. The tilt over that reach
  lies in the gap, 0.13% under the escalation width, so no definite crossing
  may be claimed. The base's Positive was read over the span's diameter,
  longer than the datum is read across. `likely`: right by the contract, and
  by a margin of 0.13% of the band.
- **The body's own correction** of "32 of 34" now agrees with what I measured
  on the first head (`esc → definite` and `definite → esc` transitions).

**C6 (F5). The code supports the counter-reading.**

- **The geometry** (`offset_axial.rs:3562–3596`, `tangent_foot`): the station
  line `ρ = 11 − δ` and the unit circle about `(10, 0)` lie within ε of each
  other wherever `|h| ≲ √(2(δ + ε)) ≈ 1.41·10⁻³`. That is about 1400ε along
  the station line.
- **What that means for the foot.** Every point of that stretch is within the
  band of both curves, so the corner is determined to no better than ±1400ε
  under the run's tolerance. The two roots are `8ε` apart, inside the
  escalation width (`10ε`), so they are one root to within the band. The foot
  is on the line and `8·10⁻¹²` off the circle, a point within band of both
  moved surfaces, which is what a solved corner vertex owes.
- **So no answer false of the geometry at ε is served.** Being `4ε` from
  each root is a statement about a meeting the band does not resolve. A
  definite claim would be false only if something downstream read the foot as
  a transversal corner (it does not: the foot is the tangent arm's answer), or
  if a consumer needed the vertex within less than ε of a particular root,
  which no clause I read asks. `likely`.
- **Reach.** A door-built body needs the moved station plane within `δ ≪ ε`
  of tangency to a moved profile circle at a corner whose edges reach less
  than the circle's radius. Nothing excludes that: it is a near-tangent offset
  of an axial body. I did not build one, and no suite reaches it (the body
  says no verdict moved at `corner_arms`; I did not re-take that). `unsure`.

**C7 (F7). Holds.**

- **One home.** `cover::boundary_reach_below` (`cover.rs:401–424`) is the
  only fold of `LeverBounds::below` in `topo` outside rows. `overlap` and
  `chart_region` read it, and `chart_region`'s `Option`/`unreachable!` is
  gone.
- **The `unreachable!`** (`pose.rs:1455–1466`). Both callers pass a line as
  the first span. `edge_reach_bounds` on a line is `Exact(cover far
  distance)`, so `lever = max(r, b) ≥ r ≥ min(r, a) = over`. The margin is the
  tilt times the arm, so "not Zero at `over`" gives "not Zero at `lever`", and
  an in-band read at `lever` escalates through `?` rather than reaching the
  `unreachable!`.
- **Poison.** A conic cover that fails makes `over` poison (Err, not Zero).
  `Curve3::cover_over` fails for a circle or an ellipse only on non-finite
  data, which makes the samples behind `lever`'s bound from below non-finite
  too, so `lever` escalates as well.
- **The site's comment** gives the line-exactness reason and not the poison
  one. `likely` (the poison path by inspection).

**C8. The merge holds.**

- **Nothing of main's dropped.** `git diff 2a4d0cb82^2 2a4d0cb82` touches only
  the unit's 28 files, and no line of main's is removed in `work/`.
- **Gates on a clean head tree.** `lever-sources` OK (37 sites, 875 fixture
  levers, and OK against `GATE_BASE=origin/main`), `lever-doors` OK (46),
  `cover-kind-match` OK (106 reads of 27 doors).
- **`lever-fixtures.counts`:** topo 317 = main's 311 + 6. True.
- **`lever-sources.counts`** only falls: pose 3, overlap 1, chart 2,
  offset_axial 1 and rim 1 are dropped, and ring_path goes from 2 to 1.

**The hosted run 45154911253.** Both reds are main's. `sure`.

- **lint, `gates (scripts/gates)`.** `payload-rung-sweep` reports a NEW
  payload rung, `NodeErrorClass` at `editor-core/src/eval/class.rs:41`. The
  same message prints on main `b3cc4b38`, and the file is not in this diff.
  Because this step failed, **rustfmt, clippy and rustdoc were skipped on this
  head**. Locally, `cargo fmt --all --check` and clippy (`geom-brep`, `topo`) pass on the head;
  rustdoc was not run.
- **test (app), "the slow set names only tests that exist".** The dead entry
  is `editor-core::all: e6_drive_over_a_loft_at_interval::an_arc_loft_bisects_to_its_budget_under_a_box_elsewhere`,
  present in main's `.config/nextest.toml:455`. `.config/` is not in the diff.
- **Green:** test (kernel) (default ε, 1e-6/1e-12, the diff's slow tests,
  doc-tests), test (app)'s test steps, corrupt input, mesh budget.
- **Skipped:** the interval, demos, step import, python, tools and viewer
  jobs.

## MINOR

**MINOR 1. Two crossing levers in the loop F1 fixed are still chords of a
datum read at a point, and the class row omits them.**
`ring_path.rs:1826` (Segment and Ruling: the chord `from → to`) and
`spline_wall.rs:215–224` (Iso: the chord between its ends), both read by
`split_ring_path_spline_across` at the meeting, from below.

- A piece met at its middle reaches half its chord from there.
- No row pins either arm: G4 (the segment lever halved) survives the 53
  `ring_path` rows.
- This is the half-fix shape the style lane names. The arc arm of a three-arm
  `match` is fixed and the other two are not; the row that holds the class
  excludes them by its title (`edge_extent_*`) while claiming the class.

`sure`.

**MINOR 2. The PR body still describes the router as it was before the fix.**
Under "What each site's verdict does on its Zero": "The crossing lever. A
definite reading is a crossing. A non-definite one goes the level way, where
the only answer served, `Coincident`, is decided by `one_line`'s own identity
reads."

On this head a non-definite read at `lever` either escalates or is the
`unreachable!`. It never goes the level way. And `line_conic`'s level road
serves in-plane meetings, not `Coincident`. `sure`.

## NOTE

- **The class row's line references** are about 20 lines stale after the
  merge of main (C2). `sure`.
- **F4(c)** is unrefuted, not proven. The argument should name the stretch
  rule as what refuses (C4). `likely`.
- **`route`'s `unreachable!` comment** gives the line-exactness reason only.
  The poison case escalates for a reason the comment does not state (C7).
  `unsure`.
- **No hosted lint ran on this head**, because main's gate red skipped
  fmt, clippy and rustdoc. Locally fmt and clippy (two crates) pass; rustdoc was not run.

## Style

**Questions exercised:** Q1 (the one-home fold, the chord arms beside the
fixed arc), Q2, Q3 (G4's survival), Q4 (who cites the class), Q5 (the body's
stale paragraph), Q6. Q7 and Q8 were not exercised on the delta.

- **Q1/Q4.** `spline_crossings` now builds its arm three ways in one `match`:
  `None` then `piece_reach_bounds` at the meeting for an arc, a chord for a
  segment, and a chord for an Iso piece. A comment ("an arc's is read at each
  meeting, `None` here") reconciles them instead of one rule.
  `ring_path.rs:1810–1846, 1874–1885`. `likely`.
- **Q2.** `span_reaches` and `route`'s doc now carry the nearer/farther
  argument at length, in two places (`pose.rs:964–970`, `:1426–1431`); the
  same `max(r, b) ≥ r ≥ min(r, a)` appears in the doc and in the
  `unreachable!` comment. `unsure`.
- **Q3.** The Between rows hand `LeverBounds::Between` straight to
  `solve_pairs`, `cap_pair_corner` and `solve_planar_corner`. The body says so
  and gives its reason: no fixture builds a non-exact corner edge. Those rows
  pin the consumers, not `corner_arms` producing `Between`. A disclosed
  narrowing with no scheduled follow-up. `likely`.

**Ratings:**

- **Idiom/structure: 4.** One home for the boundary fold, `span_reaches`
  clean, the dead second Zero path collapsed. The chord arms beside the fixed
  arc cost a point.
- **Test quality: 4.** All 21 listed mutants red on their rows. The ring
  arms (arc and segment) and `corner_arms`' production of `Between` are
  unpinned.
- **Doc/comment honesty: 3.** The new table and corrections are accurate
  where I checked. One stale paragraph describes the pre-fix router, and the
  class row's scope is narrower than its claim.

## Files on this branch

- `analysis/oneround/unsourced-unions/delta-review-report.md` (this report).
- `delta-mutants.py`, `delta-mutant-results.txt` and `review_arc_probe.rs`
  (the probe, to drop into `crates/topo/src/ring_path/` with a `mod` line).
