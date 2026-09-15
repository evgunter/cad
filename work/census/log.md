# CENSUS log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/census/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, each with a `## Re-homed` record: two from `work/issues/`,
five from `work/code-quality/`.

The class was legible in the pile — several rows literally say "spelled
N times" in their titles — and had no owner because it belongs to no
crate. It is the smallest track the cut opened, and deliberately: the
rows that share its shape but whose fix is already written went to DOOR,
and the ones whose population is every file in the workspace went to
COMB.

## A row arrives from M10 (2026-09-13)

M10 closed at its exit sweep (`docs/DOC-LEDGER.md` sweep 13) and
`pncad-py-eval-err-variants-outside-the-tag-inventory` came here by
header edit and `git mv` (id unchanged). It is this program's class
stated in one file: `TAG_INVENTORY` lexes `crates/pncad-py/src/tags.rs`
and nothing else, while eight `eval_err` call sites under
`src/py/` mint four refusal words as string literals that reach Python
exactly as a `tags.rs` word does — three covered by accident and
`measure_unavailable` pinned nowhere. `crates/pncad-py/*` is LIB's
territory and this program's `keep_out` already announces its
pncad-py rows there.

## A second row arrives, from DOCM (2026-09-13)

`a-document-vocabulary-declared-outside-the-macro-is-uncensused` came
here at DOCM's exit sweep (`docs/DOC-LEDGER.md` sweep 14) by header edit
and `git mv`, id unchanged. It was not logged at the time and the slate
in `plan.md` did not carry it; both are corrected here rather than
left for a later reader to notice the board and the plan disagree.

Its live instance is already resolved — WIRE's review of PR 2501 found
`LoopProgram` four lines below the macro invocation and moved it inside,
and `ProgramRefusal` / `RecordedProgramError` are dispositioned out at
the site. **What this program inherits is the general case**: nothing
detects the next plain `pub enum` that should have been a vocabulary.
Every door the row names is a walk over source text, and PR 2501 removed
exactly such a walk after measuring it report agreement over a set
missing the variant it existed to catch. That is the charter's trap in
its purest form and the row is ordered fourth with it stated.

## Posture settled (2026-09-15)

Ev, in chat, on the session opening this program's dispatching:

- **No A/B protocol.** The band stays claimed for bookkeeping; nothing
  draws an ordinal. `docs/MODEL-AB-LOG.md` already recorded this program
  as style-reviews-with-a-correctness-arm, so no amendment is owed there.
- **A full correctness review is for the hardest units only** — the two
  **H** rows on today's slate.
- **The scan-set rule moves into the style brief.** `plan.md`'s posture
  section previously bought a correctness arm mechanically for any unit
  changing what an instrument SCANS. Asked which test to run where the
  two diverge, Ev's answer was to take the hardest-units test for the
  full review and "get some of the benefit" of the other "by telling the
  style reviewer to also watch out for specifically silent omission
  errors". So the obligation survives as a named question in the brief
  rather than as a second lane: *what does this instrument no longer
  read after this diff, and what would it report if the population it
  watches went missing entirely?* `plan.md` §Review posture now states
  it and lists the units that carry it.

## Tracker sync (2026-09-15)

`plan.md`'s slate carried the seven rows of the 2026-09-11 cut and
neither row that arrived after it. Both are now in the slate table with
a class estimate (`pncad-py-eval-err-…` **E**, `a-document-vocabulary-…`
**M**) and both are placed in the order, third and fourth.

One obligation is recorded here rather than discharged: **S57's guard
half has no file on GUARD's slate.** `plan.md` says the anti-re-fork
guard in `scripts/gates/*` is filed on GUARD and never landed from here,
and `work/guard/` carries no such row. Filing it now would guess at the
guard's shape, which the call-site fix decides; it is filed at the
moment S57 is specced, and this line is the record that it is owed in
the meantime.
