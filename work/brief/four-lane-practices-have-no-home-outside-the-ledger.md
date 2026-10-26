---
id: four-lane-practices-have-no-home-outside-the-ledger
kind: issue
title: Four standing lane practices survive only in DOC-LEDGER sweep notes and belong in docs/prompts/
status: open
opened: 2026-09-22
needs_ev: true
---


Ev ruled on 2026-09-22 that a `docs/DOC-LEDGER.md` entry is only a pointer
saying what was deleted. Compressing the sweep notes to pointers surfaced
four standing practices that were written **only** into DOOR's and FIX's
sweep entries (sweeps 19 and 18) and METER's (sweep 10). Each is a rule
for future lanes and orchestrators, not a record of finished work, so each
belongs in `docs/prompts/` — which is this program's territory and Ev's
call to merge. They are carried here so the ledger can compress without
losing them; **the text below is the proposal, not ratified text.**

Evidence that nothing else carries them, taken 2026-09-22 on
`ledger/pointer-notes`, excluding `docs/doc-ledger/`:

| grep | hits |
| --- | --- |
| `route from the instrument` | 0 |
| `does any existing pin discriminate` | 0 |
| `pin discriminate` | 0 |
| `negative result by deletion` | 0 |
| `worktree per` | 0 |

`grep -rni '<pattern>' --include='*.md' --include='*.rs' --include='*.py' --include='*.sh' --include='*.yml' . --exclude-dir=.git --exclude-dir=doc-ledger`

The near misses, which is why the rules are not simply redundant:
`docs/prompts/implementer-discipline.md` §6 names
`scripts/work.py territory --files -` as the instrument but does not warn
that a `keep_out` read in prose is unreliable; §5 has *"A pattern with no
hits recorded is a claim; a hit list is a receipt"* but nothing about
deletion as the stronger negative; §2 fences `CARGO_TARGET_DIR` per lane
but never says one worktree per lane.

## 1. Route from the instrument, never from a fence read in prose

For `docs/prompts/implementer-discipline.md` §6.

> `scripts/work.py territory` is the answer to who owns a path. A
> `keep_out` clause or a charter sentence is prose that goes stale the day
> a program closes: run the instrument, and do not infer an owner from a
> fence you read.

FIX's walk measured eight recorded corrections where the seat inferred an
owner instead of running it, two of which had been the stated reason a row
was homed on FIX at all (`topo/src/census.rs` recorded as CURVED's,
actually REACH's; `editor-core/src/mc.rs` recorded as unowned, actually
PROPS'). DOOR's own `keep_out` asserted *"`crates/editor-core/*` is
DOCM's"* and #2391 measured it false; three rows named DOCM as owner for a
week after DOCM had left the tracker at sweep 14.

## 2. The pin question

For `docs/prompts/implementer-discipline.md` §2, beside *Write assertions
a bug could break*.

> A unit that changes what a refusal carries, or how it renders, owes an
> answer to: **does any existing pin discriminate the old behaviour from
> the new one?** Where the answer is no, the missing pin is part of the
> defect.

Carried as instruction 3 of every FIX dispatch; it went five for five in
the program's last wave. The sharpest statement is #2946's: nothing was
re-baselined, because nothing had ever pinned that sentence.

## 3. A negative result by deletion beats a negative result by grep

For `docs/prompts/implementer-discipline.md` §5, beside the hit-list rule.

> A negative result by grep is a claim about the pattern. A negative
> result by deletion is a fact about the tree: delete the thing and read
> the compiler.

DOOR's #2989 deleted each sibling member of `VectorSlot` in turn and read
the compiler. Its sibling failure is already a row —
`work/census/all-census-idiom-forces-the-visit-not-the-update`, where a
hand-written census passed green with a variant absent from `ALL`, after
two lanes and an orchestrator had read it and approved it; thirty seconds
of `rustc` falsified it.

## 4. One `git worktree` per concurrent lane, and a path a lane names is not free

For `docs/prompts/implementer-discipline.md` §2, beside the
`CARGO_TARGET_DIR` bullet, or for an orchestrator brief if one exists.

> Every parallel lane gets its own `git worktree`. A shared checkout means
> a shared index and scratchpad. And a path a lane mentions in passing is
> not thereby free to delete.

Two orchestrator errors on 2026-09-12: two lanes dispatched into one
checkout, producing one `git add -A` from an unrecoverable commit under
merge-only rules; and a live lane's worktree deleted because the lane had
mentioned it as a loose end, discarding six uncommitted edits. METER's
walk §6 recorded the first as an operational lesson and Ev's ruling on
PR #2218 deliberately did not carry it into the discipline — so this row
should say what it is re-proposing, and Ev may rule the same way again.

## Disposition

This row does not edit `docs/prompts/`. A META unit takes it to Ev as an
`[ev]` PR with the diff, per the CLAUDE.md rule that text binding future
work waits for Ev. If Ev declines any of the four, close the row citing
the ruling: the practices remain recoverable in this file and in the
ledger's own git history.
