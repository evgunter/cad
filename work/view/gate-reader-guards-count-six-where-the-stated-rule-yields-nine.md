---
id: gate-reader-guards-count-six-where-the-stated-rule-yields-nine
kind: issue
title: the vocab gate's header says its reader rule yields six, but six is the count of guards and the rule it states counts stages, of which there are nine
status: open
opened: 2026-09-08
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

