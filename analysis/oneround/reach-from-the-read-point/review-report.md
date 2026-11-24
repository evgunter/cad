# Review: PR 7661 — point-read data levered at the span's reach from their read point

Unit: PR 7661, branch `oneround/reach-from-the-read-point`, frozen head
`db2c9362e2ea3b0a22042636a951a0de4bf698dc`, merge base `fce2fbd919`
(checked: 18 files, +451 −120, as the brief says). Review branch
`oneround/reach-from-the-read-point-review`. The probes are
`crates/topo/tests/reach_review_probes.rs`, registered in `crates/topo/tests/all.rs`.
The mutant harness is `analysis/oneround/reach-from-the-read-point/mutants.py`, and the
differential instrument, kept off the tree, is
`analysis/oneround/reach-from-the-read-point/differential_probe.py`.

## Verdict: APPROVE-WITH-FIXES

Every conversion moves its lever in the sound direction, and every read
point in the code is the point the datum is evaluated at. Two things stop
it merging as it stands:

- Merged with current main, it does not pass the lint job (MINOR-1).
- The trace conversion, which the body calls unobservable, is observable
  on a plane, and nothing pins it (MINOR-2).

The rest are test-pinning gaps and a few claims that are not true of the
tree. None is a wrong verdict the head introduces.

## What I executed, and what I only read

Executed:
- The unit's rows at the frozen head: 41 tests across geom-brep and topo, all green.
- 22 mutants, each planted on its own, run against its rows' modules and
  my probes, then reverted. See C8.
- 4 probes of my own.
- My own old-vs-new differential instrument over the `ci` profile of the
  whole workspace less the viewer, at the default ε: 16 954 tests, all green.
- The lint steps on the frozen head, one by one:
  - `cargo fmt --all --check`;
  - `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings`;
  - `GATE_BASE=fce2fbd919 scripts/run-gates.sh`;
  - `python3 scripts/work.py lint`;
  - `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`.
- A trial merge of the head with `origin/main` (`a7b44d8eb`), and `cargo clippy -p topo`
  with the conic-centre census flags on it.
- A sample of the slow reds on the merge base (C10).

Not executed:
- the 1e-12 ε run;
- the slow set with the instrument;
- the Interval/Dual lanes beyond what the `ci` set reaches;
- the sweep/editor-core rows under the mutants (the mutants ran against the
  geom-brep and topo modules that hold the rows, plus my probes).

Read only: everything under C1–C6 and C9 that is not marked DEMONSTRATED.

## MAJOR

None.

## MINOR

**MINOR-1. Merged with current main, clippy fails on `trace.rs`: an unfulfilled
`#[expect(clippy::disallowed_methods)]`.** DEMONSTRATED, sure.
- After the merge base, main added the expect over `circle_crossings_to`
  (`crates/topo/src/boolean/trace.rs`, the `Cover::arc` read) and a census pin
  `crates/topo/src/boolean/trace.rs 1` (`scripts/gates/conic-centre-reads.counts`).
- This head removes that `Cover::arc` read; the arc arm now goes through
  `cover::piece_reach_bounds`.
- `git merge-tree` merges the two cleanly and keeps the expect. On the merged
  tree, `cargo clippy -p topo --all-targets --all-features -- -D warnings
  --force-warn clippy::disallowed_methods` fails with `error:
  unfulfilled_lint_expectations` at `crates/topo/src/boolean/trace.rs:376`.
- Once the expect is removed, the census pin for `trace.rs` will differ, 1
  against 0, and `conic_centre_census.py --check` exits 1 on any difference.
- This answers the brief's question: yes, the census fires on this file.
  Removing a read is the right direction, but the merge has to drop the
  expect and lower the pin.
- The body's "clippy clean" was true of main at `fce2fbd91` only.

**MINOR-2. C4 is false: the trace conversion is observable on a plane, and no
row pins it.** DEMONSTRATED, sure.
- The body argues that "on a plane, sense × reach is exactly the far end's height,
  which the meeting pose reads first". The meeting pose does not read that
  height. `curve_face::pose::plane_line` decides parallelism on the drift
  `|d·n̂|·run` (`axis_drift`), the run's whole length, and then roots the
  line with no further check.
- The pose: a ray tilted `c = 7.5·10⁻⁹` off `z = 0`, crossing it at the
  middle of a run of 2, under `(10⁻⁹, 10⁻⁸)`:
  - the drift reads `1.5·10⁻⁸`, definite, and the crossing is rooted;
  - the sense at the reach from the crossing (1) reads `7.5·10⁻⁹`, in band;
  - the new code refuses `Escalated(trace_crossing_sense)`;
  - the old arm (the run) reads `1.5·10⁻⁸` and claims `Leave`, on a run that
    stands within `7.5·10⁻⁹` of the plane over its whole length.
- Probe `reach_review_probes::probe_trace_ray_crossing_mid_run_reads_the_reach_not_the_run`.
  It is red under the old ray arm (planted by hand) and under M4b, the reach
  from the ray's start.
- M4c (the arc's old ball radius) and M4d (the whole circle's old `r`)
  survive every row and every probe, so the arc and circle arms stay unpinned.
- `2r` is the exact reach of a whole circle from any of its points, so the
  circle's doubling is sound.
- `circle_crossings` has no production consumer: it is called only from
  `crates/topo/tests/a_trace_reads_a_piece_against_its_carrier.rs`.

**MINOR-3. The graze's read point is pinned by no row.** DEMONSTRATED, sure.
- Every unit row seeds `analytic_root` at the root itself: the line rows at 0,
  and the ellipse row at 0, which is its root.
- So both of these survive all the unit's rows:
  - M2g: the loop reads the lever from the seed's point instead of the iterate;
  - M2k: line × plane reads at the seed instead of the root.
- This is brief lesson (2). The code is right; nothing would notice it going wrong.
- My probes kill both, through the public `topo::root_along`:
  - `probe_corner_graze_is_read_from_the_iterate_not_the_seed`: a circle tilted
    so that `sin ψ = 1.2·10⁻⁸`, edge `[−π/4, π/4]`, seed `π/4`. The honest code
    serves a graze. Under M2g every iterate reads Positive and the root's read
    escalates.
  - `probe_line_graze_is_read_at_the_root_not_the_seed`.

**MINOR-4. C2(c): "at an iterate every reading short of Positive is a graze" is
not true at the root, and the root's bound is unpinned.** DEMONSTRATED (mutants), sure.
- At the final check, `graze(..)?` serves Zero as a graze, takes Positive, and
  escalates both in band and on a straddle.
- These survive every row and probe:
  - M2i: the final check at the bound from above alone;
  - M2j: the final check at the bound from below alone.
- A wrong bound there goes unseen because the loop's last iterate has already
  required both bounds Positive at almost the same point. The final read is
  then effectively dead under convergence.
- The ellipse row's doc says "a lever read in band or Zero there answers
  alike". That holds of the loop, not of the root.

**MINOR-5. C2(d): the body does not say what now flows where, once `Incident`
and `root_along` hold a region instead of a lever.** BY-INSPECTION, likely.
- `root_along` is `pub`. Step-import now hands topo the file's own carrier
  `spec.carrier` over `(spec.t0, spec.t1)`, and topo reads its cover
  (`cover_over`) there. That is a view of the file's geometry topo did not
  hold before.
- It is also not the carrier the root is solved on (`c.carrier`, the chosen
  intrinsic one): the sine is read on one curve and levered by the reach of
  another (`crates/step-import/src/build.rs:2424` and `:2418`).
- Two consequences, both unsure as to reachability:
  1. In the Newton loop, `!matches!(graze_at(..), Ok(Positive))` serves *any*
     `Err` as a decided graze. That includes a poisoned cover's escalation,
     where a cover-less span reads the poison ball. So an uncoverable edge is
     served as `Graze`: `Rooted::Unfixed`, then `singular_point`. The base's
     `edge_extent_inf` had no poison path.
  2. On the line × plane closed form with a non-exact file edge (a B-spline
     edge whose chosen carrier is a line), a Zero below and an in-band or
     Positive reading above now escalate. Step-import then refuses where the
     base served a graze.
- So "STEP import: no body's admission moved" holds of the corpus only.
- C2(a), answered: step-import's Zero does not "only refuse". A graze sets
  `decided`, and the vertex becomes `Rooted::Unfixed`, placed by
  `singular_point` (`build.rs:2199`). Both bounds are owed, as the body says;
  the unit's first message was wrong.

**MINOR-6. The sibling `trace::ray_crossings` carries the same defect, left in
the same file.** DEMONSTRATED, sure.
- It reads the sense at the caller's scene `reach`: the run from the ray's
  start, a bound from above where the bound from below is owed.
- Its production consumer is editor-core's trace marks
  (`crates/editor-core/src/trace.rs:766`).
- On the MINOR-2 pose it claims `Leave`
  (`probe_trace_ray_crossings_sibling_claims_the_passage`).
- Its row, `work/oneround/unsourced-levers-await-a-source.md`, names it only
  as "unsourced", not as a too-long lever of this class.
- The class row does not list it.

**MINOR-7. C3: the iso row is hand-built and pins the arm's length, not the
lean.** BY-INSPECTION, sure.
- `an_iso_pieces_lean_is_levered_at_its_reach_from_its_stop` builds an
  `IsoPiece` directly and asserts `reach()` in `[1.25, 1.3]`. It goes red
  under M3a, M3c and M3d.
- No row reaches `split_ring_path_lean` with the arm deciding. My instrument
  saw the arm in 4 `ci` tests (the rows), and no lean verdict.
- The Interval lane is unreached: the instrument saw only `dual` reads of the arm.
- I did not build the door pose the brief asks for.
- The consumer's direction is right: `chord_join.rs`'s `read` takes Zero as
  `Ok(None)` and a definite sign as the side, so the bound from below is correct.

**MINOR-8. C6: no row where the shorter length and the overlap differ.**
BY-INSPECTION, sure.
- The orchestrator asked for one; there is none, and the code is unchanged.
- The argument holds for the Zero:
  - `collinear_overlap` reads `overlap_gap`, the least distance over the
    projected run;
  - that run is no longer than either span;
  - so the far distance from the gap's foot is at most the shorter length,
    which is a bound from above.
- For the definite sign I am unsure. It routes the pair to `crossing_reading`
  over both spans, so it is sound if the crossing question is over whole spans.
  On a short partial overlap of two near-collinear segments it can classify
  as a crossing what is an overlap.

**MINOR-9. C9: part two is not its own row.** BY-INSPECTION, sure.
- The spline lanes' per-sample levers (`certify.rs`, `edge_nurbs`,
  `boundary_section`, `contact_verify`) are a section of the class row, under
  "Left, in class", not a separate P1 row as the orchestrator asked.
- `spline_implicit` `ray_at` (a definite sign at a bound from above, the
  unsound direction) sits in the same list.

## NOTE

- **C1, a wrong premise in the brief.** The foot cannot fall outside the run.
  `line.param_within(apex, (lo, hi))` clamps it, and so does the quadratic's
  `shift` in `line_cone_quadratic` (`crates/geom/src/curves.rs:817`). So the
  reach is in `[run/2, run]` and the verdict can only move toward Zero or the
  band. The coefficient's centre is that same clamped foot. Mutants M1a, M1b
  and M1c are red as claimed, and M1d is unkillable, since a line is `Exact`.
  BY-INSPECTION, sure.
- **C5, the new door's soundness.** On every non-exact kind its bound from
  below is the samples' far distance, `sampled_reach`, which is a lower bound.
  Its bound from above is the cover's far distance. On a line or circle arc it
  is `Exact(cover far distance)`. A poisoned cover gives `Exact(poison)` on
  exact kinds, so `line_cone_read`'s `unreachable!` cannot fire. Sure.
- **The bound swap inside `span_reach_bounds` (M5a)** is killed only by the
  router rows and by the ellipse row's own premise assertion
  (`above > 1.0 * below`), not by a verdict: `decide_agreed` is symmetric in its
  two bounds. M5b and M5c are killed the same way. DEMONSTRATED.
- **`edge_extent_inf`'s new doc is contradicted by callers it does not name.**
  It says the door is for "a datum of the whole edge". Point-read callers
  remain: `certify.rs:5654` (per-sample), `conic_bend.rs:82` (at a vertex) and
  `replace_face.rs:4192` `corner_arm` (the spline lane's per-sample read).
  They are known and listed in the body, but the doc states a contract four
  callers break without a word at the door (Q4). Sure.
- **`Reach::lever_from` ignores its pivot for `Measured` and `Face`.** True:
  `extent.rs`, `reach_from`. It is documented there ("whatever the pivot"). It
  is recorded only as a bullet on the class row, with no row or band of its
  own. No converted site reads through it. Sure.
- **`offset_corner_arms_for_tests` (`replace_face.rs:1750`) has a stale doc.**
  It says it reports "the arm `incident_edges` gives the lane";
  `incident_edges` no longer gives an arm. Its one reader
  (`sweep/tests/encl_curved_loft_shell.rs:1061`) asserts spline sections only,
  so it is still right there. Sure.
- **`blend/battery.rs`'s added text.** The band row's proposed fix (lever
  `chain_g1` from above) passes over `cap_transverse` (`battery.rs:3125`),
  which reads `Link::arm_len` on both signs: Zero gives a circle, a definite
  sign an ellipse. So one bound in `arm_len` cannot serve all its readers.
  That supports the unit's "Link would have to hold" conclusion more than its
  text does. The row is P2, which is defensible: a latent wrong Zero with no
  known pose. Likely.
- **C9 sample of the out-of-class sites.** I read 14 production sites:
  - `locus.rs:580` and `:615`;
  - `pair_section/torus.rs:148`;
  - `pair_section/cones.rs:709`;
  - `props/curved.rs:6588`;
  - `intersect.rs:555`;
  - `carrier_eq.rs:841`;
  - `boolean/join.rs:1488`;
  - `section_cert.rs:1255`;
  - `star/cut.rs:84`;
  - `plane_eq.rs:122`;
  - `solid_contain.rs:1420`;
  - `ring_path.rs:1775`;
  - `splitting/join.rs:476`.

  I also read four test-only sites (`fate.rs:2373`, `validate.rs:15023`,
  `reduce.rs:8245`, `rim_wedge.rs:1271`). I found none that disagrees with the
  unit's out-of-class reading. Unsure: `locus.rs:615` reads `at` at `foot1`
  and levers the slack from `foot2`. That is plausibly right, since the second
  wall turns about its own foot.

## C7 — what moved: my instrument

**The instrument.** At each converted site I decided the datum at both the old
and the new lever, under `k_stats::detached`, and logged both levers and both
verdicts, per read, per test and per scalar.
- The old levers are the base's:
  - the cone lead and the corner graze: `edge_extent_inf` of the edge;
  - the ray: the run's segment ball levered from the start;
  - the arc: the arc cover's ball radius;
  - the circle: the circle cover's ball radius.
- The iso piece is logged at its arm only (old against new). Its lean verdict
  is downstream in `chord_join` and was not logged.

**Where it ran:** the `ci` profile of the workspace less the viewer, at the
default ε: 16 954 tests, all green (the unit's 16 950 plus my 4 probes).

| site | lane | reads | tests | lever new < old | equal | new > old | between bounds | verdict moves |
|---|---|---|---|---|---|---|---|---|
| cone lead | f64 | 37 386 | 446 | 36 171 | 1 215 | 0 | — | 1: the unit's row, esc → Zero |
| cone lead | Interval | 169 | 10 | (interval) | | | | 0 |
| corner graze | f64 | 54 352 | 188 | 19 008 | 29 737 | 5 605 | 2 | 3: the unit's line row, and my 2 probes |
| corner graze | Interval | 34 | 1 | (interval) | | | | 0 |
| trace ray | f64 | 110 | 14 | 110 | 0 | 0 | — | 1: my probe, Positive → esc |
| trace ray | Interval / Dual | 2 / 2 | 1 / 1 | | | | | 0 |
| trace arc | f64 | 6 | 1 | 0 | 0 | 6 | — | 0 |
| trace circle | f64 | 1 | 1 | 0 | 0 | 1 | — | 0 |
| iso arm | f64 / Dual | 17 / 8 | 4 / 1 | 1 | 16 | 0 | — | (not read) |

How to read the columns:
- "Lever new < old" etc. is judged against the base's lever at the same read.
- "Verdict moves" is judged against the base's verdict at the same read, the
  sign or the escalation.

What it shows:
- No verdict moved outside the unit's rows and my probes. This matches the
  unit's 2 moves per ε within a few reads; the author's counts sit within 1%
  of mine.
- **Unpinned lanes**, where no `ci` test reaches the site on that lane:
  - the iso lean on Interval;
  - the trace arc and the trace circle on Interval and Dual;
  - in the `ci` set, all 6 of the trace arc's reads have the new arm *longer*
    than the old, so the author's 1.396-against-0.75 case did not recur.
- I did not re-take the 1e-12 run or the slow set.

## C8 — the mutants

Each mutant was run against the unit's rows' modules and my probes. Red/green
is judged against those tests, not the full suites.

| mutant | what | result |
|---|---|---|
| M1a | cone lead at `edge_extent_inf` | red (the cone row) |
| M1b | cone lead's reach from an end | red |
| M1c | cone lead at half the run | red |
| M2a | graze at `edge_extent_inf` | red (line row) |
| M2b | graze's reach from the edge's start | red (line row) |
| M2c | graze at the bound from above alone | red (ellipse row) |
| M2d | graze at the bound from below alone | **survives**, as the unit says |
| M2f (mine) | graze at `edge_extent_sup` | red (both rows) |
| M2g (mine) | loop reads the lever from the seed's point | **survives the unit's rows**; red on my probe |
| M2k (mine) | line × plane reads at the seed's point | **survives the unit's rows**; red on my probe |
| M2i / M2j (mine) | the root's final check at the bound from above / from below alone | **survive** |
| M3a / M3c / M3d | iso arm at the old extent / from the start / at the bound from above | red |
| M4a | ray old arm (by hand) | **survives the unit's rows**; red on my probe |
| M4b | ray's reach from its start | red on my probe only |
| M4c | arc's old ball radius | **survives** |
| M4d | circle's old `r` | **survives** |
| M5a | bounds swapped inside `span_reach_bounds` | red: router rows and the ellipse row's premise |
| M5b / M5c | `span_reach_bounds` exact at its bound from below / from above | red: router rows and the ellipse row's premise |

## C10 — the body, the gates, and the slow reds

- Lint steps on the frozen head, which also carries my probe file:
  - `cargo fmt --all --check`: clean;
  - `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings`: clean;
  - `GATE_BASE=fce2fbd919 scripts/run-gates.sh`: 67 gates OK;
  - `python3 scripts/work.py lint`: ok;
  - `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`: OK.
- On current main, clippy and the census would red (MINOR-1).
- The `edge_extent_inf` and `edge_extent_sup` docs now name no reading taken at
  a point. But not every remaining caller is a whole-span datum (NOTE above).
- **Slow reds on the merge base.** I sampled three of the 25 the unit did not
  re-run on main, in a worktree of `fce2fbd919` with its own target directory,
  release build, sweep:
  - `bisector_nodes::the_near_node_poses_at_the_witness_band`;
  - `self_intersection_proves_injectivity::a_notched_rod_holds_across_its_reflex_corners_in_every_pose`;
  - `section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds`.

  All 3 fail on the base as well, so none of the three is the unit's.
- **The green row `tier_gate` halfcap_eps6.** I did not re-run it on the base:
  that is the step-import slow build. Its claim that the failure is
  bit-identical on main is unchecked by me.

## Style

**Questions exercised:** Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (whole-file read of
`crates/topo/src/boolean/trace.rs`, 554 lines).

- **Q1: two topo-side doors for one quantity, used side by side in this diff.**
  - `geom_brep::span_reach_bounds` is `edge_reach_bounds` over
    `cover_over(span)`.
  - `topo::cover::piece_reach_bounds` is the same, hulled with the span's ends.
  - The unit uses the second in `trace.rs` and `IsoPiece::reach`, passing
    `carrier.eval` of the span's ends, so the hull adds nothing. It uses the
    first in `offset_derive`.
  - Both call `geom_brep::edge_reach_bounds`; neither calls the other. With
    `topo::cover::edge_reach_bounds` that makes the four doors the brief names,
    two of them new readers' choices in this one PR.
  - likely.
- **Q4: a sentence this PR made false.** `cover::arc_cover`'s doc
  (`crates/topo/src/cover.rs`) says it is "The one place topo reads a carrier
  over an edge's span". `offset_derive::analytic_root` now reads exactly that
  through `geom_brep::span_reach_bounds`. sure.
- **The fix mints a hand-written census.**
  - `cover-kind-match` tracks readers of topo's cover doors by token.
    `span_reach_bounds` is a geom-brep door, so its readers (`line_cone_read`,
    `analytic_root`, the router) are invisible to the gate.
  - The unit deleted the router's registered line, which held its direction
    argument, and added one line at `extent.rs` whose reason recites those
    three readers in prose.
  - The `extent.rs` line itself registers only because `edge_reach_bounds`
    shares its token with topo's door of the same name.
  - The next reader of `span_reach_bounds` registers nowhere. likely.
- **Q3: the ellipse row's premise does the killing.** `above > 1.0 * below` is
  what reds M5a, M5b and M5c, not the verdict. The row's own verdict assertion
  cannot tell both bounds from the bound from below. sure.
- **Q2: a justification that postdates the code.**
  `analytic_root`'s new doc says "Both signs are acted on… so it is read at
  both bounds". But the loop it governs still serves every non-Positive
  reading as a graze, including in band, a straddle and poison. So the "both
  bounds" there is the bound from below in effect.
  - In-band served as graze is the base's shape, but the new text makes the
    loop look two-sided.
  - likely.
- **Q7: `IsoPiece::reach` re-evaluates the piece's ends.** It calls
  `self.curve.eval` beside an `ends()` method that already does that, in
  travel order (`spline_wall.rs:250`). Minor; unsure.
- **Q6: the gate decision is filed rather than added.** P3,
  `edge-extent-callers-have-no-ratchet.md`. It is scheduled as a row, so
  nothing further is owed. sure.
- **Q5: the class row's dated section reports "Left, in class" items and an
  unsure `PairSection.lever` on the analytic arms.** The unsure item is not
  re-read and has no schedule of its own beyond the open row. unsure.
- **Q8: `trace.rs` read end to end.**
  - `ray_crossings` keeps the unsourced too-long arm (MINOR-6).
  - `circle_crossings` is test-only.
  - The `crossing()` doc now states the direction, which is correct.
  - sure.

**Ratings, 1 to 5:**
- **Idiom and structure: 4.** The conversions are small and local. Two parallel
  doors are used side by side, and a lever became a region on a public
  signature without saying so.
- **Test quality: 2.** Five surviving mutants on converted code (M2g, M2k,
  M2i/j, M4c, M4d), plus M4a and M4b against the unit's own rows. There is no
  trace row, although a plane pose exists. The iso row pins a number on a
  hand-built piece.
- **Doc and comment honesty: 3.** The per-site docs are accurate. The body's C4
  argument is wrong, as is the first message's "only refuses". `arc_cover`'s
  "one place" is now false, and `edge_extent_inf`'s contract is contradicted by
  callers it does not name.

## Helpers

None. No subagents were used.
