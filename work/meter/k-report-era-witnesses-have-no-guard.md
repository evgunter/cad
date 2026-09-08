---
id: k-report-era-witnesses-have-no-guard
kind: issue
title: k-report: the M7-era claim rests on three witnesses in the committed .gz and nothing asserts them
status: closed
branch: meter/era-witness-guard
opened: 2026-09-08
closed: 2026-09-08
refs: [k-report-baseline-fold-cert1-roster, 651, 2140]
---

Found by unit 7's style review, and it is the review's strongest point:
the unit argued at length that a **name-shaped k-lint exemption** would
be the wrong repair (it would be) and never considered the guard that
would have been right.

## The claim that is unguarded

`docs/K-REPORT.md`'s M11 addendum and `docs/k-report-data/README.md`
both say M7 is still the era the shipped thresholds were cut from, and
`tools/k-lint/tests/threshold_provenance.rs` **computes with that**: it
re-derives `EPS_COUPLED_FLOOR_RATIO`, `AMBIENT_BAND_MIN` and
`PROXIMITY_FACTOR`'s consequence-interval against `M7` on every
`k-lint (gate)` run. If M7 ever stopped being the right era, that test
would go on re-deriving against the wrong one and stay green — which is
the one shape #651 says a measurement-backed claim must not have.

Nothing covers it today. `k-lint` lints a fresh sweep and never
compares it to the committed files (`docs/k-report-data/README.md`
rule 2), and no row asserts that the era's witnesses are still what
K-REPORT says they are.

## The guard, which costs almost nothing

The era decision rests on three `f64`s in the committed `.gz`, and
`threshold_provenance.rs` already opens exactly those files through
`gzip -dc` for a different derivation. Measured at `c39a904e`
(2026-09-08), identical in `m7-eps-*.csv.gz` and in a fresh sweep:

| quantity | witness | value |
| --- | --- | --: |
| definite-side floor, excluding the ε-coupled family | `volume_backstop` | 4.79652e-5 |
| ε-coupled ratio's P0 at the binding row (1e-9) | `props_quad_converged` | 164.674 |
| zero-side ceiling inside the ambient band | `pm_census_ee_span` | 5.32907e-15 |

(`volume_backstop` and `pm_census_ee_span` are row-invariant; the
coupled ratio is 839.524 / 164.674 / 335.953 at 1e-6 / 1e-9 / 1e-12.)

One pass extracts all three:

```sh
gzip -dc docs/k-report-data/m7-eps-1e-9.csv.gz | awk -F, '
  NR>1 { m = ($3+0 < 0 ? -($3+0) : $3+0); bz = $4+0 }
  bz >= 1e-13 && $2 != "props_quad_converged" && ($6=="positive" || $6=="negative") \
    { if (fl == "" || m < fl) { fl = m; fn = $2 } }
  $2 == "props_quad_converged" { r = m/bz; if (qc == "" || r < qc) qc = r }
  bz >= 1e-13 && $6 == "zero" { if (m > ce) { ce = m; cn = $2 } }
  END { printf "%.6g (%s)  %.6g  %.6g (%s)\n", fl, fn, qc, ce, cn }'
```

**`$4+0` is load-bearing.** `mawk` compares the string `5e-324` as
`>= 1e-13` unless it is coerced, which silently pulls the tie-break
family (`canonical_order_x`, `split_join_order_u`, both recording
`band_zero = 5e-324`) into the ambient population and returns
5.55112e-17 as the floor. That is a wrong answer with no error, and it
is exactly the confusion the prose floor argument exists to prevent.

**What the row should assert**, and why it is not a baseline to
preserve: that the three witnesses of **the era `M7` names** are what
`docs/K-REPORT.md` states. It goes red on a re-cut that moved a witness
without updating the report, and on a report edit that moved a number
without a re-cut. When a later era supersedes M7, `M7` moves in that
test and the expected values move with it in the same PR — the same
contract every other constant there carries.

## Why it is filed rather than landed

`tools/k-lint/*` was outside unit 7's fence (held by METER unit 3's fix
pass), and `docs/` and `work/` contain no home a test can reach:
nothing outside `tools/k-lint` and `scripts/` reads
`docs/k-report-data/` at all. So the values and the extraction are
written down here, and the row belongs beside
`threshold_provenance.rs`.

## Two stale rows this same class has already produced

Neither is unit 7's to edit; both are recorded here so the sweep sees
them.

- **`tools/k-lint/src/lib.rs:65`** — *"233 committed here, 231 at
  today's main"*. A sweep at `c39a904e` carries **281**, so the second
  number is off by 50 and the phrase "today's main" is the defect
  rather than the digit. Same class as the four sites unit 7 dated in
  `docs/K-REPORT.md`.
- **`work/code-quality/measurements-have-no-mechanical-guard.md:467`** —
  *"the roster has already drifted 233 → 231"*, in the register whose
  whole subject is this class. 281 at `c39a904e`. That file belongs to
  `work/code-quality/`, so it is reported here rather than edited.

Refs: `docs/K-REPORT.md` (M11 addendum, 2026-09-08),
`docs/k-report-data/README.md`,
`tools/k-lint/tests/threshold_provenance.rs`.

## Closed (2026-09-08)

METER unit 11, `meter/era-witness-guard`. The row is
`the_m7_era_still_carries_the_witnesses_the_report_names` in
`tools/k-lint/tests/threshold_provenance.rs`, beside the four
derivations that compute with the era, reading the committed
`m7-eps-*.csv.gz` through the `gzip -dc` reader already there.

**The three witnesses, each its own assertion.** Floor and ceiling are
asserted at all three ε rows (the report prints one number for each
because each IS one number); the headroom is asserted at the binding
row the report names. Each carries the predicate as well as the value,
because a floor that stayed at 4.79652e-5 under a different name is a
moved distribution wearing the old number. Perturbing any one of the
three expected values reds that one and names its own witness; so does
perturbing any one of the three expected NAMES.

**The witness name is a reading, not a restatement.** The headroom is
minimised over the whole ambient definite population rather than over
`EPS_COUPLED_PREDICATES`, so a second ε-proportional family landing
under `props_quad_converged` reds this row where a derivation filtered
by the one-element roster could not see it.

**Why 1e-9 is the binding row**, re-derived rather than asserted: it is
where `props_quad_converged`'s OWN ratio is least — 839.524 / 164.674 /
335.953 at 1e-6 / 1e-9 / 1e-12 — so it is where
`EPS_COUPLED_FLOOR_RATIO` has the least room. It has nothing to do with
`volume_backstop`, whose whole-population headroom is *smaller* still
at 1e-6 (47.965, its fixed absolute margin over the loosest ε) and
would make 1e-6 the more binding row on that metric. No threshold is
cut against that ratio: rule (3) holds `volume_backstop` to an absolute
floor. That the two selections agree at 1e-9 is a fact about that row,
not about the method.

**One reading was already partly covered, and the row says so rather
than claiming all three were bare.** Nothing computed with the floor or
the ceiling. The headroom was different:
`eps_coupled_floor_ratio_is_re_derivable_from_the_m7_population`
already pinned 164.674 to three figures, at the 1e-9 row and at
`demo/tiltedcut`. What the new row adds there is the POPULATION —
whole ambient definite side rather than the one-element roster — and
the witness name that comes back from it.

**The `$4+0` trap, reproduced and then exhibited in code.** Dropping
the coercion from the `awk` above returns 5.55112e-17 at
`split_join_order_u` instead of 4.79652e-5 at `volume_backstop`, at
every ε row, exactly as this item states. The Rust reader parses before
it compares and so cannot take that branch, and
`the_ambient_side_is_chosen_by_the_parsed_band_not_its_spelling` SHOWS
that rather than asserting it: it runs the selection both ways over the
committed rows and pins what each returns. **What it pins is the
disagreement, in rows and in both directions** — the EXCESS the
spelling admits and should not, and the DEFICIT it refuses and should
admit. The excess is the same 1 246 tie-break rows at either ε; at
1e-12 the spelling additionally refuses the whole 1 348 461-row ambient
definite side, `1e-12` sorting below `1e-13`, leaving those 1 246 as
the entire population. Naming the excess at both rows is deliberate:
the population TOTAL at 1e-9 is 1 349 707 and its excess is 1 246, and
a row that pinned the total would red on a corpus that merely grew.

**The voice, and the authority.** Plain claim voice, not the harness
voice this file reserves for input it cannot read: the files parse, the
columns agree, the population is there, and what moved is the claim
rather than the reading. The owed-test question is answered across
these two cargo roots by `tools/README.md`'s `CC5`, which is the only
page spanning them and therefore the licence for a `k-lint` row to cite
a `tess-lint` sentence at all. `CC5` FORWARDS that question rather than
answering it, to `tess_lint::Report`, whose own doc forwards again — it
disclaims being the general rule, *"not a rule of its own beside it"* —
to the general form in `tools/tess-lint/src/lib.rs`'s module docs: *"a
finding is where the gate would otherwise assert something false"*. A
moved witness is exactly that, so the check is owed and owed red rather
than noted. It arrives BY ANALOGY and not by its letter: `Report`
splits the findings and notes of one `compare` call, and a `cargo test`
has no notes register to put the other side in.

**What the row cannot see**, stated at the row itself: it reads
committed files, which `docs/k-report-data/README.md` rule 1 freezes,
so it can only say that the values M7 carries are the values the report
names. It cannot say M7 is still the era that SHOULD be shipping — that
is decided by a fresh sweep measured against these files, and none runs
here (rule 2). A distribution that moved under a corpus the row never
opens leaves it green. It reads only the three ratified ε rows `ROWS`
names, so a fourth would leave every assertion green until `ROWS`
grows. Nor does it read `docs/K-REPORT.md`; the report's spelling of
each number is written out as the expectation.

**Where the era decision is normatively stated.** Not here and not in
`docs/K-REPORT.md`: `docs/k-report-data/README.md` rule 1 is the one
home for the era-cut trigger and names these same three quantities. The
report is a reading of that clause and this row is a guard on the same
three, so a re-argument of the witnesses moves rule 1 first. The row's
doc and the clause now say so from both ends.

**Both stale rows this class produced are disposed.**
`tools/k-lint/src/lib.rs`'s "233 committed here, 231 at today's main"
is fixed: 281 is the tip figure and is DATED as one, and the committed
233 is no longer a figure at all — the new row re-counts the era's
distinct predicate names off the frozen files at every ε row, so the
half of that sentence that could be derived is derived.
`work/code-quality/measurements-have-no-mechanical-guard.md:467` is
another program's slate and stays reported.

**`docs/K-REPORT.md`'s "here is the one that should exist" section is
fixed in place**, not filed: `docs/K-REPORT.md` is in METER's own
`paths` (`work/meter/program.md:11`) and its own unit had already
merged, so the fence reason an earlier draft of this item gave was
wrong. The section now names the row that covers the era claim, keeps
the row's own limit rather than claiming more than it, and says plainly
which of its figures still have no guard — the 281-name census and the
corpus growth, un-retaken and dated to `c39a904e`.

Refs: `docs/K-REPORT.md` (M11 addendum, 2026-09-08),
`docs/k-report-data/README.md`,
`tools/k-lint/tests/threshold_provenance.rs`.

## Closed (2026-09-08)

METER unit 11, `meter/era-witness-guard`. The row is
`the_m7_era_still_carries_the_witnesses_the_report_names` in
`tools/k-lint/tests/threshold_provenance.rs`, beside the four
derivations that compute with the era, reading the committed
`m7-eps-*.csv.gz` through the `gzip -dc` reader already there.

**The three witnesses, each its own assertion.** Floor and ceiling are
asserted at all three ε rows (the report prints one number for each
because each IS one number); the headroom is asserted at the binding
row the report names. Each carries the predicate as well as the value,
because a floor that stayed at 4.79652e-5 under a different name is a
moved distribution wearing the old number. Perturbing any one of the
three expected values reds that one and names its own witness; so does
perturbing any one of the three expected NAMES.

**The witness name is a reading, not a restatement.** The headroom is
minimised over the whole ambient definite population rather than over
`EPS_COUPLED_PREDICATES`, so a second ε-proportional family landing
under `props_quad_converged` reds this row where a derivation filtered
by the one-element roster could not see it. That the two agree is a
fact about the 1e-9 row and not about the method: at 1e-6 the smallest
ambient headroom is `volume_backstop`'s 47.97, which is why the report
names 1e-9 as binding.

**The `$4+0` trap, reproduced and then exhibited in code.** Dropping
the coercion from the `awk` above returns 5.55112e-17 at
`split_join_order_u` instead of 4.79652e-5 at `volume_backstop`, at
every ε row, exactly as this item states. The Rust reader parses before
it compares and so cannot take that branch, and
`the_ambient_side_is_chosen_by_the_parsed_band_not_its_spelling` SHOWS
that rather than asserting it: it runs the selection both ways over the
committed rows and pins what each returns. The spelling comparison
fails in two directions and both are pinned — at 1e-9 it admits 1 246
tie-break rows it should refuse (1 349 707 against 1 348 461), and at
1e-12 it also refuses every row it should admit, `1e-12` sorting below
`1e-13`, leaving the tie-break family as the whole population.

**The voice, and the authority.** Plain claim voice, not the harness
voice this file reserves for input it cannot read: the files parse, the
columns agree, the population is there, and what moved is the claim
rather than the reading. `tools/README.md`'s `CC5` does NOT decide
this and is not cited as deciding it — its subject is a cross-column
admission at a lint's reading boundary, and on the owed-test question
it forwards rather than answers. The place it forwards to does reach,
and is cited directly: `tess_lint::Report`'s doc in
`tools/tess-lint/src/lib.rs`, *"a finding is where the gate would
otherwise assert something false"*. A moved witness is exactly that —
the four derivations would go on asserting a provenance against the
wrong era — so the check is owed, and owed red rather than noted.

**What the row cannot see**, stated at the row itself since
`docs/K-REPORT.md` was outside this unit's fence: it reads committed
files, which `docs/k-report-data/README.md` rule 1 freezes, so it can
only say that the values M7 carries are the values the report names. It
cannot say M7 is still the era that SHOULD be shipping — that is
decided by a fresh sweep measured against these files, and none runs
here (rule 2). A distribution that moved under a corpus the row never
opens leaves it green. Nor does it read `docs/K-REPORT.md`; the
report's spelling of each number is written out as the expectation.

**One of the two stale rows above is fixed, the other reported.**
`tools/k-lint/src/lib.rs` was inside this unit's fence, so its
"233 committed here, 231 at today's main" is now dated — 233 committed,
281 at the tip the M11 addendum counted, 61 in and 13 out — with the
live-phrasing defect named rather than only the digit corrected. This
is the class's fifth instance and the four in `docs/K-REPORT.md` were
fixed by the unit that filed this item.
`work/code-quality/measurements-have-no-mechanical-guard.md:467` is
another program's slate and stays reported.

**Residue, with its own file**:
`work/meter/k-report-era-guard-section-still-says-nothing-guards-it.md`
— `docs/K-REPORT.md`'s "here is the one that should exist" section
still describes the guard as unwritten and its figures as un-retaken.
`docs/K-REPORT.md` was outside this unit's fence.
