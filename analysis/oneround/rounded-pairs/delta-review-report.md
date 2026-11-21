# Delta review: PR 7221 (`oneround/rounded-pairs`), `361f2274..1ffa23e9`

Frozen head `1ffa23e917cbecbf9e6fa65a9bf908c09e8cee41`, merge base with
main `fdeff1ff`, hosted CI run 44879760325. I read the unit's own commits
`1ca7b92c 13635506 19ff988a f32a42b2 e2497fc1 e114b869 ac1d4148
1ffa23e9`. The three merges of main (`8579846f 93ce0fff 399124ca`) I read
only for what they brought into the unit's files.

## Verdict: APPROVE-WITH-FIXES

Every fix I was asked to check holds, and nothing regressed. My
re-taken differential, at the frozen head against its own merge base,
reproduces the first review's picture exactly:
- the same moved tests;
- the same counts at both ε;
- M0 in the same slow row;
- a clean base-vs-base control;
- no answer moves.

All three new pins go red on their mutants. What is left is three doc
corrections, none blocking:
- the body's stale sentence on the `93ce0fff` merge;
- a vacuous refusal case in the body's M2 account;
- two doc sites that still claim the local read cures a thin `b`.

There is also one NOTE on the gate: it finds charging methods only by
the `with_` name.

## What I executed, and what I only read

**Executed:**
- **The differential, re-taken at the frozen head.** Merge base
  `fdeff1ff` against `1ffa23e9`, with the same instrument as the first
  review (`probe/verdict-census-instrument.patch`), separate worktrees
  and separate target directories.
  - The `ci` profile over `geom-core geom-brep topo profile sweep` at
    ε 1e-9 and 1e-12, with `CAD_FUZZ_SEED` and `PROPTEST_RNG_SEED`
    pinned.
  - A second base run at 1e-9 as the control.
  - The 87 geom-brep and topo slow rows named for these sites, at 1e-9.
  - Outputs are in `probe/delta-*`.
- **The bare-rounding gate**, in a clean worktree at `1ffa23e9`:
  - as shipped: green;
  - with a `.with_charge(` call planted in `piece.rs`: red;
  - with a charging method named `plus_charge` and a call of it
    planted: green;
  - its selftest: green.
- **Three mutants**, one per new pin, in the head worktree with its own
target directory, then reverted:
  - **m1:** `deflated_reading`'s `F′` bound `2·noise` → `noise`;
  - **m2:** both `intersect` `size` sites `gap.magnitude()` →
    `gap.value.abs()`;
  - **mv:** `validate::lines_meet`'s overlap lower end read at the upper
    end.
- **The CI run, at step level, through the jobs API.**
- **For each of the three merges,** `git diff --stat M^1 M` over the
  unit's files.

**Only read:**
- every hunk of the unit's own commits;
- the new rows and the re-posing row's entry;
- the PR body;
- the three sites the implementer says are not bisections.

## The CI run, step level

These ran green:
- `change filter`;
- `test (kernel)`:
  - default ε: 1 m 33 s;
  - ε 1e-6 and 1e-12: 69 s;
  - the slow-set tests the diff adds or edits: 59 s;
  - doc-tests;
- `test (app)`;
- `lint`:
  - gates: 9 min;
  - fmt;
  - clippy;
  - rustdoc;
- `corrupt input`;
- `mesh budget`;
- `gate ok`.

These were skipped:
- the jobs `interval`, `tools (k-lint, tess-lint, tess-meter)`, `viewer
  app`, `python suite`, `demos`, `step import` and `prime the build
  cache`;
- the steps `the merge this dispatch gates`, `change filter self-test`,
  the "slow set did not run" notice, and the failure summaries.

As before, no hosted job ran the `Interval` backend or k-lint. I met no
red of main's.

## The fixes, confirmed or refuted

- **D1 (M1): confirmed.** DEMONSTRATED (read at the head), sure.
  - `the_frame_charge_bounds_the_literal_circle` carries its own doc
    again (`implicit.rs:2404-2416`).
  - `a_circles_extremes_on_a_sphere_are_their_enclosure` carries only
    its own.
  - The new `a_conics_extremes_are_its_forms_within_its_rounding` sits
    cleanly above both.

- **D2 (M3, N9): confirmed, with one residue.** DEMONSTRATED, sure.
  - `with_charge` is counted. A planted `.with_charge(1.0)` in
    `piece.rs` reds the gate `UNLISTED crates/geom-brep/src/piece.rs
    (prod): 1 rounding sites, listed 0`.
  - The selftest's new case passes.
  - The README's backstop sentence names the door.
  - `with_charge` adds `|charge|`, and the negative-charge assertion
    pins that.
  - The restated counts are consistent with the gate being green at the
    head.
  - **Residue (NOTE, DEMONSTRATED, sure):** the gate finds charging
    methods by their name, the `with_` prefix (`doors()`:
    `fn\s+(with_\w+)`). I added a second charging method `plus_charge`
    to `running`'s impl and planted a call of it. The gate stayed green
    at its listed counts. So the backstop is a naming convention: a
    charging method named otherwise is invisible, which is M3's shape
    again, one name removed. The gate's header says "a charging method
    of it (a `pub fn with_…` …)", which is honest about it, but nothing
    enforces that charging methods are so named.
  - **Taste (NOTE, unsure):** `with_charge(−c)` silently widening by
    `c` hides a caller's sign error. A fail-loud codebase might panic or
    poison on a negative charge rather than fold it.

- **D3 (M2): confirmed, with one wrong case in the body.** The pin is
  BY-INSPECTION (arithmetic), sure.
  - **The pin is red on the old bound.**
    `a_floor_gap_within_its_rounding_escalates_as_a_straddle` (`D = 1 +
    6·10⁻⁹ ± 9·10⁻⁹`, level 1, reach 10⁻¹², ε 10⁻⁹):
    - unclamped, the lower bound is −3·10⁻⁹, in the band's gap, so it
      returns the `ct_critical_gap` escalation and the assertion
      fails;
    - clamped, the lower bound is −10⁻¹², which is Zero, and the upper
      bound 1.5·10⁻⁸ is Positive, so it returns the straddle.
  - **The body's reasons are right for three of its four cases.**
    - Negative → Zero is right: a magnitude is never definitely
      negative.
    - gap → Zero, giving a straddle, is right.
    - The upper bound's escalation → Positive is right.
  - **The fourth case is vacuous** (MINOR, doc honesty,
    BY-INSPECTION, sure). The body lists "an old `(Negative, Zero)`
    becomes `(Zero, Zero)`, so a straddle becomes the gap row's gate
    rejection". The old pair cannot occur:
    - the old bounds were `near ∓ (reach + e)` with `near ≥ 0`;
    - a Negative lower bound needs `reach + e > Kε`;
    - so the upper bound is above `Kε`, which is Positive;
    - this holds at any K ≥ 1.

    So no straddle becomes a gate rejection, and the body describes a
    transition that never happens. Nothing in my 1e-12 counts shows one.

- **D4 (M5): confirmed.**
  - The m1 pin `the_deflated_reading_charges_the_slope_twice_the_noise`
    is red on m1 by arithmetic. At θ = π/4, perturbing `c₂` by `noise`
    moves `F′` by `2·noise·|sin 2θ| = 2·noise`, which is past a
    `noise` bound. DEMONSTRATED: red on m1 (`F′ at 0.785…: Rounded { value: -1.29497474…, error: 1e-6 } against -1.29497674…`).
  - The m2 pin `a_plane_through_the_apex_reads_its_reach_on_its_gaps_rounding`:
    red on m2. DEMONSTRATED, sure.
  - The validate pin `collinear_spans_a_rounding_apart_do_not_meet`:
    red on mv. DEMONSTRATED, sure.
  - **The row `two-rounded-reads-have-no-pinning-fixture` is a
    schedule, not a deferral.** It is an open P2 issue under `work/` and
    names the fixture each read needs (skew: one plane built twice at a
    far vertex, its offset near ε/K; `middle`: a frame whose `û` and
    `n̂` are each a few ulps long, with `n̂·û = 0`, against an exact
    dyadic `‖GᵀG − I‖`). Style Q6 calls exactly that a schedule.

- **D5: confirmed.**
  - **N1:** `term` has arms 0 to 5 and `_ => unreachable!`.
  - **N8:** the ends' docs now give the `f64` and `Interval` readings
    separately.
  - **N4:** `conic_residual_extremes` reads `extreme(c₀ ∓ swing)` at its
    end, within `h.noise`.
  - **N4's pin can catch the dropped noise only through its spelling**
    (NOTE, Q3, BY-INSPECTION, likely). It goes red on the
    noise-dropped mutant only through its `assert_eq!` against the
    formula `(c₀ − swing) − noise`, a pin on the spelling. Its second
    half, the sampled residual inside `[lo, hi]`, cannot go red when
    the noise is dropped: the sampled exact-on-the-stored-circle
    residual lies inside `c₀ ∓ swing` up to the form's own rounding, at
    those 256 angles. So the contract half is weak, and the spelling
    half does the work. It is still a pin.

- **D6 (the differential still stands for the head?):** **Yes: it stands, re-taken by me at the frozen head.**
DEMONSTRATED, sure.
  - **The runs.** Base `fdeff1ff` against head `1ffa23e9`:
    - the `ci` set: 11,334 tests at head and 11,316 at base, all
      passing;
    - ε 1e-9 and 1e-12;
    - the 87 site-named geom-brep and topo slow rows at 1e-9;
    - the control (a second base run at 1e-9): zero count moves.
  - **Default suite, both ε.** The moved tests and counts equal the
    first review's, test for test:
    - `pole_plane` (12 f64, 1 `Interval`) and `deflated_side` (3): each
      drop is exactly the old always-Positive clear read, and their
      other counts are unchanged;
    - `ct`/`tt_critical_gap_monotone` (`torus_gap::the_tube_circle_…`,
      `section_cert_square_rows::a_pipe_along_…`): Negative → Zero or
      escalation, Positive identical.
  - **Bit-only rows.** The new ones are `bool_conic_curved_clearance`
    (N4's re-association, as the body says) and four rows of
    `gate_kinds_followed::child_gate_follow`. That test also
    differs base against base, so it is noise.
  - **Slow rows at 1e-9.** Identical to the first review's: M0's
    `every_torus_touch…` path move to the digit, and the `torus_gap`
    monotone rows.
  - **Main's merged `curve_pair` code** (PR 7208, through `93ce0fff`)
    adds no moved row.
  - **Not re-run:** the slow rows at 1e-12, where M2 and N13 live. Those
    sites are untouched since `e114b869`, and the body's 1e-12 numbers
    equal mine from the first review.

- **D7 (the PR body against `1ffa23e9`): true, but for three
  statements.**
  - **The `93ce0fff` merge** (MINOR, DEMONSTRATED, sure). The body
    says "89 commits, none touching a file this PR changes". At the
    merge that was true. But `1ffa23e9` then edits
    `crates/geom-brep/src/curve_pair/pose.rs`, and `93ce0fff` brought
    392 changed lines of main's into that file (PR 7208's slanted-pair
    reading, `git diff --stat 93ce0fff^1 93ce0fff`). Other new
    `curve_pair` code came too: `curve_pair.rs`, `meet_rows.rs`,
    `polish.rs`. So at the head the sentence is false of `pose.rs`, and
    new `curve_pair` paths now run through the unit's changed window
    and touch reads unmeasured. My re-taken differential (D6) is the
    measurement the body lacks.
  - **The `8579846f` merge.** The first merge after the first review
    brought main's changes to `crates/topo/src/validate.rs`, the
    bare-rounding list and `k_stats.rs` (`git diff --stat 8579846f^1
    8579846f`). The body's differential was taken after it, at
    `e114b869`, so that merge is measured. Fine.
  - **`(Negative, Zero)`.** See D3.
  - **What checks out.** Everything else I checked holds:
    - the tables;
    - the M0 and M2 numbers, which equal mine;
    - "one caller" for `most()`;
    - N13's numbers;
    - the gate paragraph.

## The rulings

- **M4 and N2: the claim sites are now true, but two more still say the
  old thing.** For the named sites, BY-INSPECTION, sure:
  - `window.rs:153-162` (`local_taylor`), `pose.rs:2137-2140` (the
    coplanar subdivision's `jet`) and `gap.rs:350-352` (`nearest`) each
    say only what holds.
  - For `nearest`: two-term sums of stored radii are a short
    division-free chain, which the rule covers.

  **Residue (MINOR, a half-fix of the class, BY-INSPECTION, likely).**
  Two other claim sites still say the local read cures the thin case:
  - `window.rs:68-75`, the module doc (ii): "Posed about `b`'s centre
    their harmonics carry rounding of order `((|w| + R)/s)²` … which at
    a far, thin or large pair reaches the band's zero; read about the
    piece, each rounds as its few local terms do".
  - `pose.rs:2158-2165`, `Level`'s doc: "the local form's rounding is
    that of `F`'s own few terms … a bound that needs no precision".

  Both present the local read as curing "a far, thin … pair". For the
  level, thinness (small `s`) is exactly what the local form does not
  cure: it still divides by `s²`. That is the uncovered growth the new
  P1 row names. Read beside the corrected `local_taylor`, they now
  contradict it.

- **The two rows.** Both are actionable. BY-INSPECTION, sure.
  - **`the-windows-level-jet-is-read-exact-where-its-rounding-grows-with-the-aspect`**
    (P1) names:
    - both readers;
    - why the rule does not cover them;
    - what was measured, and where;
    - what is unknown;
    - the probe that would settle it (a thin, far `b` at the finest
      admitted ε against an exact level oracle);
    - the owed re-pose;
    - the condition for dropping to P2.

    A reader could act on it. One clause is unclear: "which suggests
    those fixtures set their own band". The new entry on
    `site-rounding-charges-owed-a-re-posing` states the debt with the
    0.77 and 1.5 bands and points to the row.
  - **`two-rounded-reads-have-no-pinning-fixture`:** see D4.

- **N11: accepted, and my finding was wrong there.** BY-INSPECTION,
  sure.
  - `patch.rs:789` and `:853` are `min_norm_point_iterates` and
    `frank_wolfe_iterates`, optimiser loops that break on optimality.
  - `props.rs:3767` walks `RoundWindow` rounds.
  - `window.rs:1297` steps `l.next_up()`, one ulp at a time.

  None is a sign-change bisection. I matched `for _ in 0..64` without
  reading the bodies. The row's three genuine copies are the right list.

- **N5: the reader is real.** BY-INSPECTION, sure. `census.rs:6093`
  hands `along.noise` to `span_charge`, which decides whether a span's
  ends are charged at all. It could be spelled `along.overlap().error`,
  but a `pub` field read for a decision is a reason to keep it.

## Style (delta only)

Questions exercised: Q1, Q2, Q3, Q4 and Q6.
- **Q1/Q4 (sure):** the gate's naming convention (D2).
- **Q2/Q4 (likely):** the two uncorrected doc sites for the level jet
  (above).
- **Q3 (likely):** N4's pin (D5).
- **Q6 (sure):** the unpinned reads now have a schedule.

**Ratings for the delta:**
- **Idiom/structure: 4.** The fixes are small and in place. The
  deduction is the naming-convention gate.
- **Test quality: 4.** The new pins are aimed at the call sites and at
  the floor arm. The deduction is N4's spelling pin.
- **Doc honesty: 4.** The body is rewritten carefully and its numbers
  reproduce. The deductions are the stale merge sentence, the vacuous
  case, and the two uncorrected level-jet docs.
