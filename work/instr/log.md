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
