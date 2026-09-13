# S-TINT — test-suite integrity (plan)

**STATUS: OPEN (2026-09-11).** Opened on Ev's direction (in-chat,
2026-09-11: *"could you move the fourth bucket to a new track in
work/?"*), out of S-TCOST's re-sort of its board against the repository
going public. Live state is `work/tint/log.md`'s tail and the item files
beside this plan, never this file.

Branch prefix (the #396 convention): **`tint/`** — unit branches
`tint/<unit>-<slug>`, orchestrator branch `tint/orchestrator`. Away-channel
tag `(S-TINT orchestrator)`. A/B ordinal band **S-TINT = 3500–3599**, the
next free band, claimed in `docs/MODEL-AB-LOG.md`'s banding entry in this
program's opening commit.

## Charter

**A row that cannot go red is not a test, and a guard that cannot go red
is not a guard.** This program owns the suite's claim on its own
contents: whether a test asserts what its name says, whether a check can
fail, and whether the prose around both describes the tree as it is. It
does NOT own what the suite costs — that is S-TCOST's, and the fence
between them is a QUESTION, not a path.

Four shapes, all of them turned up by S-TCOST while it was measuring
something else, and none of them a cost finding:

1. **Rows that cannot fail.** A codomain assertion
   (`assert!(sup >= 0.0)` on a fold of nonnegative magnitudes); a
   positivity assert riding `3.1e-16` of cancellation noise on a
   measurement that is structurally zero; a certified bound compared
   against a sample with no ceiling and no anti-vacuity floor; eleven
   `compile_fail` doctests in `tests/`, which rustdoc never collects, so
   each is a negative proof no tier has ever run; thirteen silent
   whole-row stand-downs that report green having asserted nothing.
   `memories/test-suite-cost.md` names the class and says the fix is a
   deletion or a repair, never a re-wording.

2. **Guards that do not guard.** A `Shared` ledger row checked by one
   substring, which a bare `use` satisfies with no call. A `compile_fail`
   row whose `EXXXX` is compared to nothing — measured, one annotated
   `E0277` emits `E0308`. A loud-skip marker whose row list is hand-kept
   in eight files, each copy admitting in its own rustdoc that it goes
   stale silently. A body-hash duplicate census blind to rename-only
   twins. The standing rule: **a census has one executable home and every
   other site points at it** — inherited from INSTR's `baseline_census`
   and it binds here for the same reason.

3. **One claim, N copies.** The dedup rows, where the harm is DRIFT and
   never compute — and this program says so out loud, because S-TCOST
   measured it: homing the blend tree's fixtures changed no execution
   time at all (TCOST-10), and `sweep-boolean-suite-brick-and-prism-copies`
   states *"not a cost finding"* in its own body. A copy that must be
   kept in sync in fourteen places is the shape that drifts, and
   `D386` is the class caught mid-drift: the same composite implemented
   twice, already reading its value out of two different views.

4. **Citations and names that have gone stale.** Sixteen `tests/` → `tests/`
   citations across thirteen sweep suites, eight naming files that no
   longer exist. A header saying a crossing is pinned by no row, beside
   the module that pins it with three. Row names that assert the arm the
   shipped default ε does not take.

## Ratified ground (cited, not re-litigated)

- `memories/test-suite-cost.md` — the three shapes of a test and which
  wants a varying seed; **an assertion-free test never gates**; a
  codomain assertion is a deletion, not a repair, and a sweep for the
  class must key on the ASSERTION and not on the test; silent skips are
  the escape-hatch shape and the tree's named loud-skip idiom is the fix.
- `memories/review-and-dependency-policy.md` — retirement is always
  permitted, and a reviewer suite's independence is worth keeping where
  it pulls its weight; **reviewer tests are ordinary tests** (Ev's
  ruling of 2026-09-04, in `work/tcost/log.md`'s seam).
- `memories/output-stability-as-justification.md` — a test kept only
  because its output has not changed has not been justified.
- The aggregation invariant (`scripts/gates/test-aggregation.sh`, one
  test target per crate) and `autotests = false`: a retired suite file
  leaves `tests/all.rs` in the same commit.
- **S-TCOST's keep-outs bind here unchanged.** No test is deleted for
  being slow; every deletion names the row that now owns the claim; no
  fixed seed is introduced; no `#[ignore]` on a row that gates.

## The fence with S-TCOST

Both programs' `paths` cover `crates/*/tests/*` and
`crates/test-utils/*`, so `work.py territory` will warn on most branches
of either. **That is correct and is not a defect**: territory warns and
does not block (`work/README.md`), and the two programs are separated by
the question they ask about the same files, which no glob can express.

The rule, so a lane never has to guess:

- **A row justified by a second — cpu or wall — is S-TCOST's.** If work
  here finds one, `git mv` it back rather than taking it.
- **A row justified by a claim that cannot fail, a guard that cannot
  fail, or a copy that can drift is this program's**, even where the fix
  happens to make something faster; the saving is a side effect and is
  never the argument.
- Three things stay S-TCOST's whatever they look like: the per-file gate
  mechanism (`gated_to!`, `ci-filter.py --gated-check`, the nightly
  re-take) and its two open defects, the fuzz-gating policy question
  (`proptest-modules-in-src-ungated`, `r1-probe-seeds-are-not-on-the-fuzz-dial`),
  and everything under `scripts/`.

A unit whose diff crosses the fence says so in its PR rather than letting
the warning stand unexplained — the CIW precedent.

## Review posture

Inherited from S-TCOST for the test-only work these rows are, and **open
for Ev to reset**: Opus implementer, **one style review per unit**
against `docs/prompts/reviewer-style-lane.md` by path, plus the unit's
own claims (every retired claim has a named owner, no assertion
weakened, labels unambiguous). **No A/B row and no A/B protocol** — the
band above is claimed for bookkeeping, the CIW/CHROME/INSTR posture.

A unit that moves kernel logic, or that changes what a guard decides in a
way worth a second opinion on correctness, gets one extra reviewer named
in its PR with the reason. That is a per-unit judgement, not a default.

Three rows on this slate are **decisions, not work**, and ride `[ev]` PRs:
`D70` (whether 13 silent stand-downs should be ε-conditional at all),
`D113` (what an intra-doc link in a `tests/` file is), and any change to
a `memories/` clause this program finds wrong.

Hosted CI is the only gate. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path.

## The slate

Thirty rows, moved by `git mv` from `work/tcost/` with ids unchanged and
a `## Moved to S-TINT (2026-09-11)` record in each. Fourteen are the
Track W units (`C18`, `D70`, `D72`, `D113`, `D380`–`D386`, `H12`,
`S216`, `S230`); sixteen are the slugs S-TCOST's lanes filed while
measuring. **No unit order is fixed yet** — the first orchestrator cuts
one, and the two obvious pairings are recorded so they are not lost:

- `D383` and `S230` are the same class under two names (a certified
  bound with no ceiling) and both want the `test_utils::tightness` home;
  they want one lane.
- `H12`, `S216`, `C18` and `D113` are one question from four sides
  (what a doctest in `tests/` is, and whether a `compile_fail` row
  verifies the reason it names). `S216` is explicitly *not takeable as a
  doc edit* — it needs machinery — and is the one row here whose fix
  ADDS compute, which the public repo makes cheaper to justify rather
  than harder.

Every row's own body is the spec; several carry a "this enumeration is a
FLOOR" line, and those are to be re-derived against the tree at dispatch
rather than trusted — the line numbers in the older rows are frozen at a
named SHA.
