---
id: gate-reader-guards-count-six-where-the-stated-rule-yields-nine
kind: issue
title: the vocab gate's header says its reader rule yields six, but six is the count of guards and the rule it states counts stages, of which there are nine
status: closed
opened: 2026-09-08
closed: 2026-09-10
pr: 2282
---

Found by the style review of #2172 while re-deriving the reader
population rather than taking the header's word for it. **Pre-existing**
and not that unit's to fix — but #2172's body restates the six, so the
number is spreading.

## The rule, and the number that does not follow from it

`scripts/gates/viewer-vocab-declared-once.sh:386-395` states the
population rule and then a count. The rule is right and hard-won — it
came out of #2106's fix pass, which found a second unguarded reader by
deriving it:

> a reader is any command that reads the gate's subject whose exit
> status the shell DISCARDS — that is, every **STAGE** of every
> pipeline inside a process substitution. **Stages, not pipelines, is
> the load-bearing half.**

The header then says *the rule yields six*. It does not. Six is the
count of **guards** — one `reader_failed` site per function. The rule
as stated counts **stages**, and the review's enumeration finds nine:
`find` and `sort`; `gate_rust_code`'s `awk` and `awk "$ITEM_AWK"`;
`awk "$HIT_AWK"`; the kinds `awk` and its `sed`; the table `awk`; the
rows `sed`. (Ten if `printf` counts, which it should not — it reads its
arguments, not the subject.)

## The consequence, which is the defect this file already argues against

Because each guard is on the pipeline's `pipefail` status rather than
on a stage, a stage that dies is diagnosed as **whichever stage the
guard is named for**:

- a dead `sort` is reported as *"the source enumerator"*;
- a dead kinds `sed` as *"the kinds reader"*;
- a dead rows `sed` as *"the row reader"*.

That is misdiagnosis-by-`pipefail` — exactly what `:451-456` argues
against for `const_hits`, where the fix pass deliberately used a brace
group rather than `|| reader_failed` on the pipeline **because
`pipefail` reports the rightmost non-zero stage and guarding the
pipeline would blame the wrong reader.** The reasoning was applied at
one site and the other three were left, with the header asserting a
number that hides the gap.

## What the fix is, and what it is not

It is **not** "change six to nine". Either the guards go per-stage, the
way `const_hits` already does, and the count becomes nine honestly; or
the header states plainly that six guards cover nine stages and names
which three diagnoses can therefore blame the wrong reader. The first
is the repair; the second is the disclosure the file owes until it is
made.

Each newly split guard owes a `gate_selftest_without_tool`-shaped case
killing that specific stage and expecting its own name in the error.

## Confidence

`sure` on the arithmetic and on the three misdiagnoses. `likely` that
per-stage guards are worth the shell contortion at all four sites.

## Closed (2026-09-10)

**The citations above are true of the tree this was filed against**
(`:386-395`, `:451-456`); the repair moved both. Today's locations are
given below and were re-derived by finding each subject, never by
shifting a number.

**The repair, not the disclosure.** The guards went per-stage and the
count became nine honestly. Every stage is now wrapped in its own
brace group — the shape `const_hits` argued for alone at
`scripts/gates/viewer-vocab-declared-once.sh:530` — so the status a
guard reads is that stage's own: `viewer_sources` (`:514`),
`const_items` (`:525`), `readme_kinds` (`:719`), `table_rows`
(`:778`). The header states the rule at `:409`, produces the nine at
`:423` and argues the shape at `:442`.

**The arithmetic in this file is right and was re-derived, not taken.**
Five process substitutions in `gate()`: `find`+`sort`, kinds
`awk`+`sed`, table `awk`, rows `sed`, and `gate_rust_code`+`ITEM_AWK`+
`HIT_AWK` — nine, the same nine this file names. `printf` is excluded
for the reason given here and for one more now argued at `:455-463`: it is
a bash BUILTIN, so nothing on PATH can shadow it away.

**Two corrections to the consequence half, both reproduced against the
unfixed gate before any edit.**

- *"a dead rows `sed` reported as the row reader"* is **not a
  misdiagnosis**. `table_rows` is `printf | sed` and `printf` is not a
  reader by this file's own rule, so that guard already covered
  exactly one reader stage and *the row reader* IS that `sed`. Two of
  the three named consequences are real; this one is not.
- *"a dead kinds `sed` reported as the kinds reader"* is real but is
  not a WRONG name — it is **one name covering two stages**, so a CI
  log could not say which of the `awk` and the `sed` died. It is now
  *the kinds scanner* and *the kinds bullet extractor*.
- **One this file's enumeration missed**, and it is the same shape as
  its `sort` case: a dead `gate_rust_code` drew **two** diagnoses —
  `lib.sh`'s own correct *the shared Rust reader* AND this file's *the
  const-item reader*, for a stage this file does not own. Splitting
  `const_items` leaves that stage to its own guard.

**Every newly split guard owes a case, and the cases were run both
ways** (rows at `:1613-1627`):

| case | before | after |
|---|---|---|
| `sort` dead | RED — got *the source enumerator over*, wanted *the source sorter over* | GREEN |
| `awk` dead | RED — got *the kinds reader over*, wanted *the kinds scanner over* | GREEN |
| kinds `sed` dead (consumes, then exits) | RED — got *the kinds reader over*, wanted *the kinds bullet extractor over* | GREEN |
| `gate_rust_code`'s `awk` dead | GREEN | GREEN |
| `ITEM_AWK` dead (consumes, then exits) | GREEN | GREEN |

**The last two are green on both sides and are declared as such rather
than counted as evidence**, at `:474-483` as well as here. The
`gate_rust_code` split REMOVED a wrong second name, and
`gate_selftest_case` and its broken-tool twin match a substring with no
way to assert a string is ABSENT — so a removal is invisible to them.
That is the affordance half of
`work/issues/gate-selftest-cannot-observe-the-identity-a-gate-names`,
restated at this gate rather than filed a second time; `lib.sh` is out
of fence and was read, called and not edited. The `ITEM_AWK` case is
new coverage of a stage that previously had none: before this PR the
only case asserting *the const-item reader* actually killed
`gate_rust_code`, which is the defect this item names, sitting inside
the self-test that was supposed to hold it.

**Disclosed rather than fixed:** stages 5 (the table reader) and 6
(the row reader) have no case. Each is the only reader in its
substitution, so its guard was already on its own stage and a case
would be green before and after — coverage, not a control. Named at
`:466-472` so a later reader derives the population from the rule
rather than counting the rows. **Not scheduled and needing no file**:
there is no defect behind it, only an absence of a case that could not
prove anything.
