# Review — PR 7221 (`oneround/rounded-pairs`), frozen head `361f2274bf9658a4bda5814aea7e0c19e164bab9`

Merge base with main: `f43c98df`. Hosted CI: run 44836415726.

## Verdict: APPROVE-WITH-FIXES

No answer moves. On my own differential (both lanes, ε 1e-9 and 1e-12,
base-vs-base control clean, every test passing on both sides), the
default suite moves no acted verdict at any touched row. One slow row
does move acted cell verdicts in `gap`'s sweep at the band's edge (M0).
The sweep reads a few more cells, and everything downstream of it stays
bit-identical. That refutes C1 as worded, but not its substance. The fixes are bounded: one doc comment spliced onto the wrong test,
one unmeasured change to the gap floor's refusal kind, and a new
charge door the bare-rounding gate cannot see. Also two read-claims of the
body: the window jets are cited to an admission rule whose premise the
level jet does not meet, and the unpinned reads have no schedule (one of
them is new, from my mutant). None of these blocks the merge on
correctness.

## What I executed, and what I only read

**Executed**
- **The differential.** I instrumented `k_stats::classify`, the one
  funnel every decide passes through, on two worktrees: the merge base
  `f43c98df` and the head `361f2274`. The patch is
  `probe/verdict-census-instrument.patch`: a per-process census of
  (row, lane, Positive/Negative/Zero/escalation) counts and an
  order-free hash of the margins, written at exit. Each tree had its own
  `CARGO_TARGET_DIR`.
  - I ran `nextest --profile ci` over `geom-core geom-brep topo profile
    sweep` at ε 1e-9 and 1e-12, with `CAD_FUZZ_SEED` and
    `PROPTEST_RNG_SEED` pinned. That is 11,302 tests at head and 11,289
    at base, all passing on both sides at both ε.
  - I ran a second base pass at 1e-9 as the control.
  - I ran the slow set, restricted to the 87 geom-brep and topo slow
    rows whose names reach these sites (torus, tube, pipe, gap, apex,
    snowman, pole, chart_singularity, harmonic, deflated), at both ε on
    both trees. The rest of the slow set (1,624 rows; single rows run
    past 250 s) was not run, and neither was any `sweep` slow row.
  - Outputs: `probe/moves-eps1e-*.tsv` (each test/row whose counts
    moved), `probe/rows-eps1e-*.txt` (per row), and
    `probe/control-base-vs-base-eps1e-9.txt`.
  - My first pass lost dumps to PID reuse (`pid_max` 32768), so I
    rebuilt with collision-free file names and reran everything. All
    numbers here come from the second pass.
- **Bisecting M0.** I ran the moved slow row twice per tree. In the
  review checkout, with the instrument and its own target directory, I
  ran two probes:
  - **A**: `nearest` uncharged;
  - **B**: `D²` without its `u·D²`.
- **Mutants.** Each was run against all of geom-brep and topo at the
  `ci` profile (6,437 tests), then reverted.
  - **m1**: the cone arm's slope bound at the call site, `2·noise` →
    `noise` (`probe/mutant-m1-cone-slope-bound.patch`). It survived.
  - **m2**: the author's `intersect` `size` read as `|gap|`. It
    survived, which confirms the author.
- **The gates.** I ran `scripts/gates/bare-rounding-allowlist.sh` with
  `BARE_ROUNDING_LINES` over the six files whose counts rise or fall.
- **The CI run**, resolved through the jobs API to step level (below).
- **Repository checks**, all with `git`:
  - the pre-move and post-move changed-line sets of `conic_quadric`;
  - `past_noise` across the whole tree at the base;
  - the provenance of the new tests' doc comments;
  - the citations in the five rows.

**Only read**
- every hunk of the diff;
- `running.rs` and `piece.rs` whole, and `gap.rs` almost whole;
- k-lint's rules and the nightly k-lint steps;
- the window's height and level jets.

I did not run the k-telemetry probe sweep, and I built no pose for C5.

## The CI run, step level

- **Ran green:**
  - `change filter`;
  - `test (kernel)`:
    - default ε: 1 m 23 s;
    - the ε 1e-6/1e-12 step: 60 s;
    - "the slow-set tests this diff adds or edits": 53 s;
    - doc-tests;
  - `test (app)`;
  - `lint`:
    - gates;
    - rustfmt;
    - clippy;
    - rustdoc;
  - `corrupt input`;
  - `mesh budget`;
  - `gate ok`.
- **Skipped steps:**
  - `the merge this dispatch gates`;
  - `change filter self-test`;
  - "the slow set did not run" (a notice step that fires only when the
    slow set does not run);
  - the failure-summary steps, which run only on a failure.
- **Skipped jobs:**
  - `prime the build cache`;
  - `interval`;
  - `tools (k-lint, tess-lint, tess-meter)`;
  - `viewer app`;
  - `python suite`;
  - `demos`;
  - `step import`.

Two of those skips matter here. No hosted job ran the `Interval`
backend, and no hosted job ran k-lint (see C2).

## Findings

### MINOR

**M0. C1 is false as stated: in one slow row, acted cell verdicts in
`gap`'s sweep move, and the sweep reads more cells.** DEMONSTRATED,
sure.
- **The row:**
  `topo boolean::section_cert::section_cert_rows::every_torus_touch_holds_against_a_sampling_oracle`,
  at ε 1e-9. It is in the slow set, which the author's differential did
  not run.
- **Base against head.** The run is deterministic: two runs per tree are
  identical.

  | Row | Base (p, n, z, e) | Head |
  |---|---|---|
  | `ct_critical_gap` | 1,286,886, 0, 1,152, 174,792 | 1,286,890, 0, 1,152, 174,792 |
  | `_clear` | 61,356, 110,186, 354,682, 393,424 | 61,358, …, 393,430 |
  | `_floor` | escalations 160,768 | 160,770 |
  | `_monotone` | Zeros 40,162 | 40,164 |
  | `_turn` | Zeros 1,032,106 | 1,032,110 |

- **What happened.** A cell's clear or monotone verdict, which the walk
  acts on by not splitting, flipped at the band's edge, and the walk
  split cells it did not split before.
- **The cause is not one change.**
  - Removing the new `u·|D − L|` subtraction charge in `nearest` leaves
    the head's counts exactly.
  - Removing `D²`'s own `u·D²` charge gives a third, different pattern:
    clear +4 Positive and −4 escalation, monotone −4 Positive.
  - So the bit-level moves the body files as "margin bits only, with
    the same counts" (association in the Taylor sums, `d.square()`) do
    flip acted verdicts where a margin sits on the band's edge.
- **No answer moves.** The test passes on both sides, and the 29 other
  rows it logs, everything downstream of the gap, are bit-identical
  between base and head.
- **Why it matters.** Both the body's measurement and its C1 claim
  ("no verdict that is acted on moves") are fixture-bound. The honest
  statement is "no answer moves in the suites run, and a walk's path
  can".

**M1. A new test was spliced between another test's doc comment and its
`fn`.** DEMONSTRATED (read in the tree), sure.
`crates/geom-brep/src/implicit.rs:2340-2359`. The new
`a_circles_extremes_on_a_sphere_are_their_enclosure` was inserted after
the doc comment of `the_frame_charge_bounds_the_literal_circle` and
before its `#[test]`.
- The new test now carries two concatenated doc comments, its own and
  the other test's ("The frame charge bounds the gap … An exactly
  orthonormal frame is charged nothing.").
- `the_frame_charge_bounds_the_literal_circle` (`:2404`) now has none.
- The old doc describes a different test (rational points, dyadic
  arithmetic, two defective frames), so a reader of the new test is
  told something false about it.

**M2. `gap`'s floor refusal changed what it returns, after the measured
head, and nothing measured it.** BY-INSPECTION, likely. The body says
`gap`'s cell and floor came to share `Reading::apart` after `2e4c57bb`
and were not re-measured.
- **What changed** (`gap.rs:237`, `:524`).
  - The floor's lower bound was `min|D − L| − (reach + e)`, unclamped.
  - It is now `room(D − L) − reach`, which is at least `−reach`.
  - For the cell's clear test the two are the same predicate.
- **Where the floor differs.** Where `D` lies within its bound of a
  level, the old lower bound read Negative and the new one reads Zero.
  Where the old one sat in the band's gap, the new one reads Zero.
  - Old: `lower` Negative forces `upper` Positive at K = 10, so the old
    floor returned `straddle(Negative, Positive)`. Where the old lower
    bound sat in the gap, it returned the gap row's own escalation.
  - New: it returns `straddle(Zero, Positive)` with a different hull
    margin, or a straddle where the old code returned the escalation.
- **Effect.** The refusal's predicate (`*_critical_gap` against
  `*_critical_gap_straddle`) and its margin move. No answer moves: it is
  a refusal either way.
- **Reach of my differential.** The default suite moves no count at
  `*_critical_gap_floor`, `*_critical_gap_straddle` or `*_critical_gap`
  at either ε. The one floor move in the slow rows is M0's path change,
  and no `*_straddle` count moves anywhere. So this arm is not
  evidenced, and nothing pins it.
- **What the body owes.** It does not mention that the floor's refusal
  kind can change.

**M3. `Rounded::widened` is a charge door the bare-rounding backstop
cannot see.** DEMONSTRATED (gate output), sure.
- **Why the gate misses it.** The gate counts top-level `pub` items of
  `running` and `Rounded` constructions (`bare-rounding-allowlist.sh`
  header). `widened(charge)` is a method, so the gate counts none of its
  uses.
- **What the door does.** Its doc says it adds a charge to a bound
  (`running.rs:180`). It is used to add one at `vertex_section.rs`
  (the skew), at `conic_quadric.rs:287-288` and at
  `implicit.rs:930-931` (the frame defect).
- **The measured effect.** `vertex_section.rs`'s count fell 6 → 5 while
  its skew charge stayed (`BARE_ROUNDING_LINES` output).
- **Why it matters.** The README's backstop sentence ("a tree gate … on
  every use of `running`'s charge doors (its `pub` items) outside this
  crate") is now false of one door, and future charges added through
  `widened` will not raise a count. This is the shape the style brief
  warns of: the fix for hand-kept charges mints an invisible one.

**M4. The window's uncharged jets are cited to the admission rule, whose
premise the level jet does not meet.** BY-INSPECTION, likely.
- **The citation.** `window.rs:155-160` reads the height's jet and the
  level's jet as `Rounded::exact`. It cites `[simple.conditioned]`: "a
  short chain on in-box coordinates, whose rounding the run's admission
  rule fits in the band's headroom".
- **The premise.** The README's admission floor is the rounding of a
  *short, division-free* chain (`geom-core/README.md`, "Float rounding").
- **The height jet** (`window.rs:190`) is such a chain.
- **The level jet** is not. It divides by the squared semi-axes
  (`pose.rs:1844-1848`, `x.powi(2) / square` and so on). It is
  dimensionless, and its rounding read as a length grows with the
  ellipse's aspect.
- **The measurement.** The body's 0.77 and 1.5 zero bands are inside
  the 9-band headroom at K = 10. But they are measured on one fixture,
  in units of ε rather than of the admission floor, and nothing
  re-measures them.
- **Whether it is sound.** "An exact reading named at its call" is sound
  for the height. For the level it is a measured, unguarded bound wearing
  a proof's citation (style Q6).
- **Not attempted.** I did not build a pose that flips a window
  verdict. A thin, far `b` (small semi-minor axis, centre far from the
  origin) is where I would look.

**M5. The unpinned reads have no schedule, and I add one more.**
DEMONSTRATED for two of them (mutants survived), sure.
- **The author's four**, each disclosed with "I did not build them" and
  no row: `intersect`'s `size`, `vertex_section`'s skew, `validate`'s
  ring-meet overlap end, and `implicit`'s `middle`. That is a deviation
  disclosed without a schedule (style Q6).
- **m2 confirms the first.** With `size = |gap|` at both sites,
  geom-brep and topo stay green.
- **A fifth, the cone arm's call site.** The new `cone_rows` pins test
  `side_past_rounding` and `cone_root_slack` as functions. They do not
  test the call that hands them their bounds (`conic_quadric.rs:636-651`,
  `F` within `noise` and `F′` within `2·noise`). m1 halves `F′`'s bound
  there and survives all 6,437 tests.
- **Can each be pinned?**
  - `size`: yes. A plane through a far cone apex, with `apex_gap`
    computing to within its own rounding (exactly `0.0` at best), makes
    `size` 0 against `err` and moves the `pn_apex_*_reach` levers.
  - `ring_outer_meet_overlap`: yes, but the suite reaches it in only two
    test processes.
  - The cone slope bound: by a `through_the_apex` pose whose root slope
    sits between `noise` and `2·noise` of zero.
  - The skew and `middle`: I believe so, but I did not work out the
    poses.
- **Rows built:** none.

### NOTE

**N1. `piece::term`'s factorials end in a catch-all.** BY-INSPECTION,
sure. At `piece.rs:61`, `_ => 120.0` answers every `k ≥ 5`, so a `k = 6`
call would silently use `5!`. It is unreachable today: `Taylor::over`'s
const assertion caps `N` at 5, so `k ≤ 5`. But a fail-loud codebase
would panic on `k > 5`. The `0 | 1 => return if k == 0 …` arm is awkward
control flow.

**N2. `Rounded::exact` is used on a rounded value in `gap.rs:352`.**
BY-INSPECTION, sure. `nearest` reads each level as exact, but the levels
are `ra + rb` and `|ra − rb|` (`gap.rs:568`), computed sums. The base
did not charge them either. This PR now charges the subtraction's own
`u·|D − L|` but not the level's rounding: half a charge, named "exact".

**N3. Some count rises are new charges, not only re-named old ones.**
BY-INSPECTION, sure. Most of each rise is a former bare pair turned into
a `Rounded` construction, as the body says (verified per site:
`intersect` +2, `census` +2, `conic_quadric` +3 for `extreme`, `F` and
`F′`, `window` +1 for the `exact`). But three charges are new:
- `gap`'s `+7` includes `Rounded::exact(level)` and the charged
  subtraction in `nearest`;
- `d.square()` adds `u·D²`;
- `implicit`'s `middle` gains per-operation `u·|result|` terms.

All three are tiny, and the body discloses them as bit moves. But they
add charges in rows the program schedules for re-posing (fewer charges).
The body's "each new count is a charge that already existed under two
bare names" is not quite true. The README allows a count raised beside
a new site, held by review; this note is that review.

**N4. `conic_residual_extremes` is the hand-written sibling of the new
door.** BY-INSPECTION, likely. `implicit.rs:908-909` reads `c₀ ∓ (A₁ +
A₂ + noise)` by hand, where `noise` is a rounding bound. It shares the
`(lo, hi)` contract with the circle path just above it, which now goes
through `widened(...).lower_end()`. The body files it "not this class (a
range, not a value's rounding)"; I read it as half of each. Q1: two
spellings of one contract, a few lines apart.

**N5. `OnOneLine` still exposes its rounding as a bare field.**
BY-INSPECTION, sure. `OnOneLine` keeps `pub noise` (`one_line.rs:64`)
beside the new `overlap()` and `gap()`, which build a `Rounded` twice
from it. Hand reads `extent() ± noise` stay as easy to write as before.
Today they hand the bare noise to doors (`census.rs:6093`; `seg.rs`
tests).

**N6. `FirstHarmonic::most()` has one caller, not two.** BY-INSPECTION,
sure. Its only caller is `conic_quadric.rs:485` (`cone_roots`), whose
noises are equal (`first_harmonic_arm`). The sphere construction
(`conic_quadric.rs:286`) has unequal bounds but never calls `most()`.
- "Identical where `lo_noise = hi_noise`, as in both its callers"
  should read "its one caller".
- The bits are identical there: `max(a, b) + n` equals `max(a + n, b + n)`
  under monotone rounding, and the same holds per end at `Interval`.

**N7. C6's ends are not bit-identical to the old ends.** BY-INSPECTION,
sure. They are `lo − (le + fe)`, where the old ends were `(lo − le) −
fe`. The two can differ by an ulp in either direction, so the new end
neither equals nor always contains the old one. Both are first-order
estimates, so this is harmless; my differential saw only
`conic_quadric_first_noise` bit moves.

**N8. `lower_end` and `upper_end` are documented as bounds without the
lane's caveat.** BY-INSPECTION, sure. `running.rs:162-174` calls them "a
lower/upper bound on the exact value". At `f64` the subtraction rounds
to nearest, so an end can sit `u·|end|` on the wrong side of the true
bound. The bound is also first-order: an estimate in the module's sense,
not an enclosure.
- At `Interval` the end is itself an outward-rounded interval and
  encloses.
- No caller needs more than the `f64` estimate (census, validate, seg,
  `half_angle`'s fall, `implicit`'s extremes).

**N9. `widened` with a negative charge shrinks the bound silently.**
BY-INSPECTION, sure. A negative charge can make `error` negative, so
`lower_end > upper_end` and `room > |value|`. A NaN charge propagates,
because `Real::max` is NaN-propagating (`real.rs:2295`); a decide then
refuses, which is loud. Every current caller passes a non-negative
charge (skew, frame defect). There is no guard.

**N10. k-lint flags in-band outcomes on every row, not only on named
ones.** BY-INSPECTION, unsure. The body says the new escalations land
"at a row no k-lint threshold names". But k-lint's rule (1) flags every
recorded indeterminate, whatever its row (`tools/k-lint/src/lib.rs:15`).
- The nightly *driver* lint gates on rule (1). The large-K lint is
  pinned red and reports counts.
- So the question is whether the probe corpus reaches the torus gap,
  not whether a threshold names it. The committed baselines
  (`docs/k-report-data/m*-eps-*.csv.gz`) hold no `critical_gap` row,
  which suggests it does not.
- I did not run the probe sweep, and CI skipped the tools job.

**N11. One bisection home serves geom-brep only.** BY-INSPECTION, sure.
`piece::sign_change` is `pub(crate)` in geom-brep. The same 64-step
sign-change loop stays in five other places:
- `geom-core/src/spline/compose/patch.rs:635, 699`;
- `topo/src/props.rs:3767`;
- `topo/src/offset_derive.rs:1364`;
- `window.rs:1294`, a test.

`curve_face::composite` keeps its own `BISECTIONS = 64`, a different
loop, as the body says. "One bisection" is one per crate, not one.

**N12. The brief names a file that no longer exists.** sure. It speaks
of `conic_quadric/mod.rs` 3→6. After main's move the file is
`crates/geom-brep/src/curve_face/conic_quadric.rs`, and the row is
listed under that name. The brief's list of pin rises also leaves out
the new `test`-scope rows:
- `circle_roots` 0→3;
- `cone_rows` 2→4;
- `gap` 0→1;
- `piece` 0→3;
- `curved` 0→1;
- `half_angle` 7→8.

All of these are oracles.

## The claims

- **C1 — holds for answers, not for every acted verdict** (DEMONSTRATED:
  my differential, at the frozen head). In the default suite it holds as
  stated. In one selected slow row, acted cell verdicts in the sweep move
  (M0).
  - **What moves.**
    - At both ε and in both lanes, the only count moves are at
      `chart_singularity_pole_plane` (f64 in 12 tests, `Interval` in 1),
      `conic_quadric_cone_deflated_side` (3 tests),
      `ct_critical_gap_monotone` (2 tests) and `tt_critical_gap_monotone`
      (1 test: `torus_gap::the_tube_circle_tangency_decides_zero`).
    - The rest are the PR's new tests.
    - The base-vs-base control has zero count moves (53 bit-only rows,
      all at rows this PR does not touch).
  - **The pole plane and the deflated side.** In every moved test, the
    head's Negative, Zero and escalation counts equal the base's. The
    head's Positives are exactly the base's less the number of side
    reads, so each dropped decide was a "clear" read that was Positive.
  - **The gap monotone rows.** Positive counts are identical, and only
    Negatives move, into Zero or escalation.
  - **The selected slow rows at 1e-9.** The same pattern holds in
    `torus_gap`'s three slow rows and in `cone_rows::grazes_and_apex_…`,
    except for M0.
  - **Bit-only moves.** The Taylor rows (`*_sub_monotone`,
    `*_root_slack`, `section_ruling_monotone`, `coplanar_sub_*`,
    `pose_touch_spread`), the gap's `clear` and `critical_gap` rows, and
    `first_noise` move bits only, as the body says.
  - **Interval reach.** `*_critical_gap_*`, `ring_outer_meet_overlap` and
    the census `*_charged` rows are barely reached or not reached at
    `Interval` (the `critical_gap` rows never). So "both lanes" is
    vacuous there.
  - **Not covered**: M2's floor arm is not reached.

- **C2 — holds for every consumer I found.** DEMONSTRATED for the
  counts, BY-INSPECTION for the consumers, likely.
  - **The counts.** At 1e-9:
    - `ct`: Negatives fall by 73,713; Zero rises by 58,082 and
      escalation by 15,631.
    - `tt`: Negatives fall by 83,085; Zero rises by 43,672 and
      escalation by 39,413.

    At 1e-12 the counts are similar, and no Positive moves.
  - **The consumers.**
    - `positive_as` acts only on Positive.
    - Escalations land on verdict-log frames; the production code I
      found reads them only in tests and diagnostics.
    - k-lint: see N10.
  - **The clamped reading is the right one.** A lower bound on `|G′|`
    has no business reading definitely negative, and the
    clamp-then-subtract form matches `half_angle`'s least slope (C4).

- **C3 — holds with caveats.** BY-INSPECTION, sure.
  - At `f64` each end is a first-order estimate, its own subtraction
    uncharged (N8). At `Interval` it encloses. No caller needs more.
  - `widened`: N9. `Neg` is exact.

- **C4 — holds.** BY-INSPECTION, sure.
  - **The walk.** `Taylor::over` with `N = 4` reproduces `half_angle`'s
    `fall` bit for bit: `term` computes `(c·wᵏ)/k!` with the old
    association, and the fold sums left to right.
  - **What differs.** `least_slope`, `most_bend` and the root window
    differ only in association (`room − Σ` against a chain of
    subtractions).
  - **The window.** With exact jets, `pose::window` is bit-identical:
    the error terms are zero.
  - **The gap** charges `D²`'s own `u·D²` and clamps the monotone
    `least` (C2).
  - **The bounds are true.** `least` and `least_slope` may go negative,
    and every reader requires Positive.
  - **The bisection.** `sign_change` is the old `half_angle` loop with
    the bound loosened from `Decide` to `Real`. It is `gap`'s inline
    loop with the same 64 steps, the same `at_lo` and the same
    `select_le_zero` pair. Neither loop stopped early, and on no sign
    change both returned the same collapsed bracket.

- **C5 — partly.** "Inside the headroom" is true of the measured numbers
  at K = 10. The citation is unsound for the level jet (M4). No flip was
  attempted.

- **C6 — holds with N6 and N7.** BY-INSPECTION, sure.
  - **`middle`.** `(a + b + a * b − c.square()).magnitude()` has the
    value bits of the hand form (same association, `powi(2)`). Its bound
    contains the hand bound plus each operation's `u·|result|`.
  - **The callers.** One caller of `most()`, not two (N6).

- **C7 — holds.** BY-INSPECTION, sure; DEMONSTRATED for the outcomes in
  the suite.
  - **Every case, at the same band:**

    | Case | Old (two decides) | New (one decide on `toward_zero`) |
    |---|---|---|
    | `\|v\| − e` past the band | the clear read Positive, the side read ±, gives `Some(±)` | ±(`\|v\| − e`), gives `Some(±)` |
    | in the gap | clear read `Err`, gives `None` | `Err`, gives `None` |
    | in or below the zero band | `None` | 0 or the zero band, gives `None` |
    | NaN | `None` | `None` |

  - **At `Interval`**, `select_le_zero` hulls a straddling value, and
    every straddle reads `None` in both forms.
  - **The suite.** My counts show each side read's sign unchanged.

- **C8 — mostly holds.** DEMONSTRATED (gate output).
  - **The prod rises:**

    | File | Count | What the new sites are |
    |---|---|---|
    | `window` | 1→2 | the `Rounded::exact` |
    | `intersect` | 6→8 | two constructions replacing tuples |
    | `gap` | 9→16 | six constructions; `exact(level)` and `select_le_zero`, both new (N2, N3); one throwaway gone |
    | `conic_quadric` | 3→6 | `extreme`, `F`, `F′` |
    | `census` | 8→10 | two constructions |

  - **Not new charges.** None of these is a charge moved off a ratified
    pose.
  - **New charges.** Three tiny ones are new (N3).
  - **One charge left the count:** `vertex_section` 6→5 (M3).
  - **Allowed by the README?** The rise is of the allowed kind ("held
    only by review").

- **C9 — see M5.** The `size` read is confirmed unpinned (m2), and the
  cone call site is added (m1). There is no schedule for any of them.

- **C10 — the sweep holds.** Shown for one pass; BY-INSPECTION for the
  dispositions.
  - **My sweep.** I used two shapes, both over the same area:
    - a `.value` and an `.error` on one line;
    - `[-+] x.error`.

    It found nothing the body missed but `conic_residual_extremes` (N4).
  - **Dispositions checked:**
    - `chart_singularity::area_clears`: fine. Row closed by PR 7145, the
      reason recorded on `WalkSum`.
    - `spread_readings` and `spread_signs`: fine. Five callers. The
      `circle_torus` spreads are tilt charges; the `extreme_sign` route
      carries rounding, through the new door.
    - `conic_residual_extremes`: debatable (N4).
    - `line_quadric::over_root_slope`: fine; a forwarded quotient.
    - `circle_sphere_rows::SlackTerms`: fine; a test oracle.
    - `torus_arms` `clears`: filed. The citation is accurate
      (`torus_arms.rs:528`).
  - **`past_noise`.** It had no caller at the base: `git grep` over the
    whole tree covers every cargo root `doc-gate.sh --print-roots`
    lists (`.`, benches, demos/tour, demos/wild,
    interval-transcendentals, tools/k-lint, tools/tess-lint,
    tools/tess-meter). Only two work rows cited it.

- **C11 — holds.** DEMONSTRATED.
  - **The move.** The sorted sets of the PR's changed lines, before the
    move (topo `conic_quadric/mod.rs` and its children, against their
    own merge base) and after it (geom-brep, against `f43c98df`), are
    identical. Nothing was lost and nothing doubled.
  - **The gate fix** (`5f221647`): the factorial table became a match
    on literals. The values are unchanged, and there is the catch-all
    of N1.

- **C12 — holds.** BY-INSPECTION.
  - **The new rows.** Both cite the code as it stands:
    - `spline_incidence.rs:295, 355, 742`, and `spline_trim.rs:348,
      362, 395` (the `Control` pair, `blend`, `point_rounding`'s
      `lead = w − e[last]` with its `INFINITY` guard);
    - `torus_arms.rs:528` (`delta = |d| + slack + band.escalate()`).
  - **The GERM row.** Its re-citation reads clumsily ("`lo` and its
    bound `lo.error`, here `lo_error`"), and its table still heads a
    column `lo_error`. But it is accurate as history.
  - **The two ONECOPY rows** are re-cited correctly.

- **C13 — mostly true.** BY-INSPECTION.
  - **What it says.** The body says its differential was taken at
    `2e4c57bb` against `99e81181`, and lists the four unmeasured later
    changes.
  - **What it leaves out.**
    - The merge of main (the `conic_quadric` move) came after the
      measurement too. The body does not say so beside "every test
      passes on both sides".
    - The change to the floor's refusal (M2).
  - **Are both rows really closed?** Everything they list is done or
    has a row, except the four or five unpinned reads (M5). The
    "throwaway `Rounded`", the factorials, the window's error array, and
    the hand reads in `half_angle`, the cone side, the pole plane,
    `implicit`'s ends, `middle` and `vertex_section` are all gone.
  - **Status.** Both rows still read `status: dispatched`, which is the
    orchestrator's state-sync to change (as for PR 7145).

## Style

**Questions exercised:** Q1, Q2, Q3, Q4, Q5, Q6, Q7 and Q8. For Q8 I
read `running.rs`, `piece.rs` and nearly all of `gap.rs` end to end. I
did not read the largest touched files whole: `census.rs` is about 21k
lines and `validate.rs` about 10k.

- **Q1** (sure). The PR closes a duplication and mints three:
  - an invisible charge door (M3);
  - a `Rounded::exact` on a rounded sum (N2);
  - `pub noise` beside its own `Rounded` view (N5).

  The bisection's "one home" is per crate (N11). The circle and conic
  extremes read one contract two ways (N4). This is a class: look
  wherever `Rounded`'s methods add to `error` without a construction.
- **Q2** (sure). The window's comment carries a citation that the level
  jet does not meet (M4). The test doc comment was spliced (M1).
- **Q3** (sure). The new pins test extracted helpers, not their call
  sites' wiring (m1 survives). `the_enclosure_ends_hold_the_exact_value`
  rests on one fixture, `0.1 + 0.2`. It can fail, but only where the end
  rounds across the exact sum.
- **Q4** (sure). The README's backstop sentence no longer holds of
  `widened` (M3). That is the code drifting from an invariant, not the
  doc rotting.
- **Q5** (likely). `piece`'s module doc calls itself "the one home of
  the Taylor bound the root walks read". Other Taylor ladders remain
  outside the three sites; the conic slack in `conic_quadric.rs:736`
  reads `1 − w²/12`, a different bound, so this is fine as scoped.
- **Q6** (sure). There are two items here:
  - the measurement behind `window`'s exact jets has neither a guard
    nor a reason given at the claim site (M4);
  - the unpinned reads are disclosed without a schedule (M5).
- **Q7** (likely). `term`'s `0 | 1 => return if k == 0 …` (N1).
  `Taylor::over` computes five bounds where most callers read one or
  two, which is fine. `Reading::turning` returning a bare pair is the
  shape this PR exists to remove.

**Ratings**
- **Idiom/structure: 4.** The doors are small and right, and the site
  table is faithful. The deductions are the invisible charge door (M3)
  and the catch-all factorial (N1).
- **Test quality: 3.** Every new pin goes red on its own mutant. But
  call-site wiring is unpinned (m1, m2), and four reads are knowingly
  unpinned.
- **Doc and comment honesty: 3.** The body is careful and mostly true,
  and its measurements reproduce (the 75–85k count, the
  `pole_plane`/`deflated_side` drops). The deductions:
  - the spliced doc comment (M1);
  - the admission-rule citation (M4);
  - "both its callers" (N6);
  - "no k-lint threshold names it" (N10);
  - the unmeasured floor change (M2).
