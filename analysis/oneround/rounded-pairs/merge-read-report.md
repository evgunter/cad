# Merge read: PR 7221 (`oneround/rounded-pairs`), `1ffa23e9..2e1122ba`

- **Frozen head:** `2e1122ba156f122a9ef7eb6805fa2821adba935f`.
- **Hosted CI:** run 45065729479.
- **What I read:**
  - the text commit `0b40b513`;
  - from the merge `2e1122ba`, only what was done by hand
    (`git show --remerge-diff 2e1122ba`). I did not read main's
    commits.

## Verdict: APPROVE-WITH-FIXES

The hand-made conversion in `curve_face::pose` is faithful:
- every value/error pair the unit converted is read through `Rounded`'s
  doors;
- no charge is dropped or doubled;
- the bits are identical where the PR body says they are.

My differential re-taken on this head reproduces exactly the moves
measured before, and nothing else. All three fix-pass mutants and both
new mutants of the moved code go red.

What is left is two MINORs on the bare-rounding list and one bare pair
left in the moved code. None of them blocks the merge.

GitHub reports the PR `mergeable_state: dirty`: it conflicts with main
again, so another merge of main is owed before it can land. That merge
is not covered by this read.

## What I executed, and what I only read

**Executed:**
- **Differential.** The base is `7709b0ea`: main at the merge's second
  parent, which is also the PR's merge base now. I used it rather than
  `1ffa23e9` because only it isolates the unit on the new tree.
  `1ffa23e9` sits on an older main, so it would mix main's germ and
  frame refactor into every move. I compared it against `2e1122ba`:
  - the instrument: the same census of the decide funnel as before
    (`probe/verdict-census-instrument.patch`), with separate worktrees
    and separate target directories;
  - the `ci` profile over `geom-core geom-brep topo profile sweep` at
    ε 1e-9 and 1e-12, with `CAD_FUZZ_SEED` and `PROPTEST_RNG_SEED`
    pinned;
  - a second base run at 1e-9 as the control;
  - the 87 site-named geom-brep and topo slow rows at 1e-9;
  - outputs in `probe/merge-*`.
- **Mutants on this head**, each reverted after its run:
  - **build A:** the three fix-pass mutants:
    - `deflated_reading`'s `F′` bound `2·noise` → `noise`;
    - `intersect`'s `size` → `gap.value.abs()` at both sites;
    - `validate::lines_meet`'s lower end read at the upper end.

    Each ran against its own pin.
  - **build B (R3):** in `curve_face::pose`, each extreme read without
    its bound (`spread_readings(…, value, T::zero(), …)`), run against
    all of geom-brep and topo (6,501 tests).
  - **build C (R4):**
    - the circle–sphere arm's `with_charge(h.frame_error)` dropped at
      both extremes in `curve_face::pose`, run against geom-brep and
      topo;
    - in the same build, a print of
      `a_circles_extremes_on_a_sphere_are_their_enclosure`'s runtime
      `frame_error` and bounds.
- **The bare-rounding gate at `2e1122ba`**, with `BARE_ROUNDING_LINES`
  over `curve_face/pose.rs`, `conic_quadric.rs` and `circle_roots.rs`.
- **The CI run**, at step level through the jobs API.

**Only read:**
- the remerge diff;
- `0b40b513`;
- the PR body;
- the new row;
- main's `through_the_apex` at `7709b0ea`, against the head's.

## Findings

### MINOR

**M1. `curve_face/pose.rs` prod 4: one of the four sites is not a
charge.** DEMONSTRATED by the gate's `LINES` output, sure.
- **The four sites:**
  - the two `with_charge(h.frame_error)` calls;
  - the quadric arm's `extreme` construction;
  - the closure parameter `|Rounded { value, error }: Rounded<T>|`
    (`pose.rs:1344`). This is a destructuring pattern, a read, which the
    gate counts as a construction.
- **Why that matters.** The row's reason `re-pose` is false for the
  fourth. The body's gate paragraph names only three ("the arm's
  `extreme` and the two `with_charge` calls").
- **A blind spot.** The gate counting a pattern as a construction is a
  blind spot of its own. A future site that only destructures will
  raise a count.

**M2. The new prod row for `curve_face/pose.rs` is not on the re-posing
row.** BY-INSPECTION, sure.
- The gate's header says every `prod` row other than `exact-arith` and
  `fixture` is scheduled by
  `work/oneround/site-rounding-charges-owed-a-re-posing.md`, "which
  lists its file".
- That row lists `geom_brep::curve_face::conic_quadric`, but not
  `curve_face::pose`.
- The charges that moved there (the frame defect and the quadric noise)
  are now unscheduled where they live.

**M3. A bare value/noise pair is left in the moved code.** BY-INSPECTION;
existence sure, severity likely.
- **The pair.** Main's new `struct Graze` (`curve_face/pose.rs:365`)
  carries `value: T, noise: T` as two fields. Its reader spells
  `magnitude()` by hand: `let z = width + g.value.abs() + g.noise`
  (`pose.rs:420`).
- **How the head builds it.** The head builds it by unpacking a
  `Rounded` (`pose.rs:1396-1397`, `value: extreme.value, noise:
  extreme.error`).
- **Why it counts.** This is the class the two closed rows describe: a
  pair carried beside `Rounded` and read by hand. It now sits one
  conversion away from the code this PR rewrote.
- **Status.** `Graze` was not at `1ffa23e9`, so it is not a pair the
  rows listed, and no row names it.

### NOTE

- **N1 (R4). The test's charge is still live, but its assertion is
  weak.** DEMONSTRATED, sure.
  - **Runtime values.** At runtime the test reads:
    - `frame_error = 5.13e-17`;
    - `lo.error = 1.49e-15`;
    - `hi.error = 6.78e-15`.
  - **The equalities still see the charge.** The charge is not
    absorbed into either bound (`lo.error + frame_error != lo.error`,
    and likewise for `hi`). So the two `assert_eq!`s still tell the
    charge apart, and a dropped frame charge in `circle_residual_extremes`
    would go red there.
  - **The `> 0.0` assertion is weak.** It goes red only if
    `frame_error` computes to exactly `0`, which needs a stored frame
    whose unit defects all vanish exactly (an axis-aligned frame). The
    test's leaning axis does not give one. It passes on any nonzero
    charge, however small, so the equalities do the work.
  - **The pose-arm mutant is caught.** The same charge dropped in
    `curve_face::pose`'s circle–sphere arm is red on
    `circle_sphere_rows::each_term_of_the_root_slack_decides_a_pose`.
  - **The other test is main's deletion.** The test that showed the
    charge is attained (`the_frame_charge_bounds_the_literal_circle`) is
    gone on main (`7709b0ea`). The merge took main's deletion.
- **N2 (R3). The removed pin's behaviour is still pinned.**
  DEMONSTRATED, sure. The mutant "each extreme read without its bound"
  in `curve_face::pose` is red on one test:
  `curve_face::conic_quadric::in_band_sweep::the_sweeps_gap_witnesses_are_judged_in_the_residual`,
  a main test present at `7709b0ea` ("12 gap witnesses answered … Miss").
  It is pinned end to end by one row. The unit no longer has a unit pin
  of its own there.
- **N3. Main is red at 1e-12, on base and head alike.**
  `sweep a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`
  fails with the same refusal on base and head: `VolumeUncomputable`,
  `props_chart_junction_closed`, margin 2.815e-12. That is main's red,
  rowed as
  `work/tesslie/main-red-a-cylinder-wall-through-a-prism-corner-union-is-volume-uncomputable-at-1e-12.md`.
  It is not this PR's.
- **N4. The PR is `dirty` on GitHub.** It conflicts with main again (see
  the verdict).

## The claims

- **R1: holds.** BY-INSPECTION for the code, DEMONSTRATED for the bits,
  sure.
  - **The circle–sphere arm:**
    `lo/hi.with_charge(h.frame_error)` = `error + |frame_error|`, which
    equals the old `lo_error + frame_error`, since `frame_error ≥ 0`.
  - **The quadric arm:** `extreme(value)` with `error: noise`, which
    equals the old `lo_noise = hi_noise = noise`.
  - **The other reads:**
    - noise: `lo.error.max(hi.error)`;
    - `c₀` and `a₁` from the values;
    - each extreme through `spread_readings(value, error)`;
    - the graze's `(|v| + n)/floor` as `extreme.magnitude()/floor`, the
      same bits.
  - **The reach** `h.most()/floor` is read only with `cone: Some`
    (`pose.rs:1324`, `let Some(floor) = cone else { return … }`).
    `cone` is `Some` only from `lane` (`pose.rs:1262`), whose two
    extremes share one noise. The circle–sphere arm passes `None`
    (`:1197`). So it is identical: `max(a, b) + n = max(a + n, b + n)`
    under monotone rounding.
  - **The faithful re-making.** `through_the_apex` is re-made
    faithfully over main's restructured loop:
    - `side_past_rounding`;
    - `cone_root_slack`;
    - `deflated_reading`.

    Main's own two-decide `side` and bare `least`/`unbounded` are gone.
  - **No charge dropped or doubled.**
  - **Not this unit's.** The one bare pair left is main's `Graze` (M3).
  - **Gate counts, judged against the gate's `LINES` at `2e1122ba`:**
    - `conic_quadric` 8 → 5. The remaining sites are two
      `rounding_charge`, one `direction_rounding`, and
      `deflated_reading`'s two constructions. True `re-pose`.
    - `curve_face/pose.rs` prod 4: see M1 and M2.
    - `circle_roots` test 3 → 2: two constructions in the kept test.
      True `oracle`.
- **R2: holds.** BY-INSPECTION, sure.
  `deflated_reading(&h, noise, θ)` borrows a by-value `h`, which leaves
  the arithmetic unchanged.
- **R3: holds.** N2.
- **R4: holds**, with N1's caveat on the `> 0` assertion.
- **R5: holds.** BY-INSPECTION, sure. `gap`'s lever test reads
  `frame.origin()` and `frame.axis()`. The sphere literals use `frame:`
  with `Frame::new(…)`.
- **R6: holds.** DEMONSTRATED, sure. Every count is judged against base
  `7709b0ea`.
  - **The `ci` set**, 11,397 tests at head and 11,380 at base, at both ε.
    Every test passes, except N3's main red at 1e-12, which fails on
    both sides.
  - **What moves at both ε:**
    - **`chart_singularity_pole_plane`** (12 f64 tests and 1
      `Interval`) and **`conic_quadric_cone_deflated_side`** (4 tests:
      the first review's 3, plus main's
      `curve_face::pose_rows::a_span_through_a_cones_apex_is_through`).
      In every moved test the Positive drop equals the number of side
      reads, and every other count is unchanged: the always-Positive
      clear reads, and nothing else.
    - **`ct`/`tt_critical_gap_monotone`** in
      `torus_gap::the_tube_circle_tangency_decides_zero` and
      `section_cert_square_rows::a_pipe_along_…`: Negative → Zero or
      escalation, Positive identical. The numbers are the first
      review's, digit for digit.
  - **No count moves at any `curve_face_pose_*` row**, which is the
    moved code.
  - **Bit-only moves in the moved code:**
    - `curve_face_pose_noise` (4 tests): the frame defect's last bit,
      the row main renamed from `conic_quadric_first_noise`;
    - `bool_conic_curved_clearance`: N4's re-association.

    `curve_face_pose_axis_offset` and `_belly_graze` also differ base
    against base, so they are noise.
  - **Control:** zero count moves, base against base.
  - **Slow rows at 1e-9.** M0 again: `every_torus_touch…` moves
    Positive +4 at `ct_critical_gap`, clear Positive +2 and escalation
    +6, floor escalation +2, monotone Zero +2 and turn Zero +4. These
    deltas equal the first review's, on main's slightly different base
    counts. Beside them are the `torus_gap` monotone rows and
    `cone_rows::grazes_…`. Nothing else moves.
  - **The three fix-pass mutants are all red on their pins:**
    - `collinear_spans_a_rounding_apart_do_not_meet`;
    - `a_plane_through_the_apex_reads_its_reach_on_its_gaps_rounding`;
    - `the_deflated_reading_charges_the_slope_twice_the_noise`.
  - **Not re-run:** the slow rows at 1e-12.
- **R7: holds.** BY-INSPECTION, sure.
  - **The doc fixes.** `window.rs`'s module doc (ii) and `Level`'s doc
    now say the local form drops the far centre's share and still
    divides by `A²` and `B²`. That is true of the code.
  - **The PR body:**
    - it replaces the `93ce0fff` line with what is true and cites my
      differential;
    - it removes the vacuous "(Negative, Zero)" case;
    - the body's sweep lists its pattern, its hits and dispositions,
      and I checked the two it fixed.
  - **The P1 row's clause** now says what was observed: the same at
    both ε, a band `CAD_TOLERANCE_EPS` does not move.
  - **The new P3 row** says what the gate does and what I planted, and
    carries the N4 pin note accurately.
- **The PR body on the frozen head: true.** Its third-merge paragraph
  matches the remerge diff item by item. It says that merge is not
  re-measured, which was true when it was written, and this read now
  measures it.
- **Slow rows: none touched.** Of the tests the hand merge added, kept
  or removed, none is in the slow set (`.config/nextest.toml` at
  `2e1122ba`): `a_circles_extremes_…`, the removed
  `an_extreme_is_read_with_its_bound`, the gap lever test, and
  `the_most_residual_…`. Nothing was re-pinned or re-baselined that
  hosted CI then skipped.

## The CI run, step level

These ran green:
- `change filter`;
- `test (kernel)`:
  - default ε: 2 m 23 s;
  - ε 1e-6 and 1e-12: 1 m 42 s;
  - the slow-set tests the diff adds or edits: 1 m 40 s;
  - doc-tests;
- `test (app)`;
- `lint`:
  - gates;
  - fmt;
  - clippy;
  - rustdoc;
- `corrupt input`;
- `mesh budget`;
- `gate ok`.

These were skipped:
- the jobs `interval` (as the implementer says), `tools (k-lint, …)`,
  `viewer`, `python`, `demos`, `step import` and `prime the build
  cache`;
- the steps `the merge this dispatch gates`, `change filter
  self-test`, the "slow set did not run" notice, and the failure
  summaries.

Hosted CI ran no 1e-12 row over `sweep`. That is why N3's main red does
not show there.

## Style (merge only)

Questions exercised: Q1, Q3, Q4 and Q6.
- **Q1 (sure):** `Graze`'s bare pair (M3).
- **Q3 (sure):** N1's `> 0` assertion, which passes on any nonzero
  charge. N2's pin is one end-to-end test.
- **Q4/Q6 (sure):** the unscheduled `pose.rs` row and the miscounted
  pattern (M1, M2).

**Ratings for the merge:**
- **Idiom/structure: 4.** The deduction is the `Graze` pair.
- **Test quality: 4.** The deduction is the weak `> 0`.
- **Doc honesty: 4.** The deduction is the gate paragraph's three of
  four.
