# INSTR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/instr/plan.md`.

## Opened (2026-09-08)

Opened by METER's exit walk, `docs/METER-EXIT-WALK.md` §4, ratified by Ev
on 2026-09-08 at PR #2212 (*"1. open"*; then *"the new 3, instr, and your
plan all sound good!"*). This is step 3 of the walk's four-step close: the
walk merged, the discipline diff rides its own `[ev]` PR, this PR opens
`instr` and moves the residue, and the sweep after it deletes
`work/meter/` and the walk itself and records both in
`docs/DOC-LEDGER.md`.

**Twenty rows moved by `git mv` from `work/meter/`**, ids unchanged, each
carrying a `## Moved to INSTR (2026-09-08)` record in its body. The list
was re-derived from the tree rather than taken from the walk: the open
rows in `work/meter/` at `origin/main` `b36a8d4b8`, less the two §5 rows
that go elsewhere, is exactly the walk's nineteen instrument rows plus
`C15` — twenty, diffed line for line against §3's enumeration with no
difference. METER's slate was 37 items, 15 closed and 22 open, at that
SHA; the sweep that counted them reads the `status:` line of the files in
that directory and cannot see an item on an unmerged branch, an item
METER filed onto another program's slate, or a residue disclosed in a
merged PR body and never given a file.

A/B band **3300–3399** claimed in `docs/MODEL-AB-LOG.md`'s banding entry
in this same commit, per the rule that entry states. No A/B protocol runs
on this program until Ev says one does; the band is bookkeeping, METER's
posture inherited.

`work/meter/` is NOT deleted here and neither is the walk — that is the
sweep, a separate PR. What remains in that directory after this one is
its fifteen closed rows, the row this PR closed by deleting
`.cert1-notes/pr-body.md`, and the three program files.

**One hazard found while executing, and it belongs to the sweep, not
here**: six of the twenty carry `refs:` naming closed METER rows
(`D201`, `cut-prefix-three-unpinned-spellings`,
`k-lint-predicate-roster-unpinned` twice, `k-report-baseline-fold-cert1-roster`,
`tess-lint-twinned-csv-fixture`). `scripts/work.py lint` requires every
`refs:` id to resolve, so deleting `work/meter/` reds `main` for rows this
program owns and METER's sweep PR cannot touch — the cross-program cost
`work/README.md` already names for `parked` rows, in a second shape. It is
filed as an issue on METER's slate so the sweep sees it before it fires
(`work/meter/sweep-deleting-work-meter-dangles-six-refs-on-instr-rows.md`),
and the form is not a new decision: GATES answered the same hazard at its own
sweep the same day (`3a8dd05fe`) by putting the closing PR's number in the
`refs:` list, where lint does not check it, and saying so in a section at the
citing row. The five ids map to PRs 2167, 2151, 2115, 2140 and 2179, derived
twice; the row carries the mapping and the derivation. The sweep makes the
substitutions in its own commit, as GATES' did across three programs' rows.

## Unit 4 closes; the review caught the fix reproducing its own defect twice (2026-09-16)

`baseline-census-partition-assert-cannot-fail` is CLOSED. The
partition assert is DELETED rather than replaced: the item offered
*"assert that `distinct` has no zero entry"*, and the same
`!sized.is_empty()` guard three lines above makes both unfailable, so
the replacement would have been the defect with a new message.

**The style review earned its dispatch twice over, and both times on
the assertion or the prose the unit KEPT rather than on what it wrote.**

First: given `distinct[i] >= 1`, once the `constant` assert passes the
`discriminating` one is forced to be the rest of `IDENTITY_COLUMNS` in
order, so **no re-cut of the baseline can red it alone** — the pair is
one assertion against a change in the DATA, not two. The PR body's
*"say strictly more"* was half-false for exactly the reason the deleted
assert said nothing. It stays (a source edit adding or removing a
column still reds it), and it now carries the disclosure this file
already gives at three other sites.

Second, and this is the one worth remembering: the fix pass's FIRST
reword of the doc comment moved the partition claim from the assert
into prose — *"between them they name the whole of `IDENTITY_COLUMNS`"*
— a static claim about two literals that no code computes, in a file
whose own module docs say *"a number transcribed into prose is a number
nothing can check"*. The lane did not catch that re-reading its own
diff. `docs/REVIEW-STYLE-DISPATCH.md` §2 predicts exactly this and the
reviewer brief says only a non-author has ever caught it; both held.

**Six rows filed, one of them a correction to this orchestrator's
adjudication.** I told the lane to add the `C15.md` evidence to
`tess-budget-doc-identity-column-list` rather than open a row; the lane
flagged that the subjects differ (membership of the identity list
versus the split among sized rows) and asked me to re-check. It was
right, and for a reason neither of us had named at first: **that row is
unit 2's and closes when unit 2 lands**, unit 2 will never touch
`C15.md`, and `work/README.md` holds that a residue inside a closed
item's prose *"reads as a record of work done, not as an open thread …
and dies with the directory"*. Split into
`c15-transcribes-the-sized-row-identity-split`, with a pointer left on
the membership row.

The lane also corrected a count I had passed through from the review
without checking: the scene-set derivation has THREE sites, not five —
`by_totals` derives recoverable-`SceneTotals` scenes and `scenes`
derives every scene, different sets in the same idiom. Filing my
framing verbatim would have put a wrong count in the tracker. The
dispatcher's own exposure, exactly as `docs/REVIEW-STYLE-DISPATCH.md`
§3 states it.
