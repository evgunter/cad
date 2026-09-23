---
id: viewer-readme-session-refuse-row-predates-the-face-frame-vocabulary
kind: issue
title: The README's session::refuse row lists three composers and not the face-frame half the module now holds
status: closed
closed: 2026-09-21
branch: vdoc/readme-attributions
opened: 2026-09-21
---



## Finding

Found beside `viewer-readme-recourse-count-does-not-say-what-it-counts`
while enumerating where recourse text is composed, and it is the same
shape as that row rather than a new one: a sentence certifying a
population it no longer produces.

`crates/viewer/README.md`, `### The session's vocabularies`, the
`session::refuse` row:

> `Refusal` with its `rank`/`preferred` ladder, its `Display`, and the
> recourse composers `affordance`/`exists_wording`/`offer_wording`;
> `NodeKindWanted` and `admits`, since they are a `Refusal` payload and
> its predicate

The module also holds, today, a second refusal vocabulary and its seat:
`FaceFrameFault` and its `Display` (`crates/viewer/src/session/refuse.rs`,
the `impl core::fmt::Display for FaceFrameFault` below the enum),
`face_frame_seat`, and the recourse constant `NO_FACE_PICKED` — which
is not one of the three composers the row names and is spent outside
this module, by `forms`. They arrived with AUTHOR's AUTH-1 (PR 2955).

**Why it is a row rather than a passing edit.** The sentence is a
`Holds` cell and the cells are read as the module's contents; a reader
checking whether a new refusal vocabulary belongs in `session::refuse`
meets a cell that says the module holds one. The repair is not a third
clause bolted on: the row wants the rule that produces its contents
(`Refusal` and its payloads? every refusal vocabulary the session
raises?), so that the NEXT one sorts itself. That is the same repair
the four count rows closed on this branch took, one level out.

`scripts/gates/viewer-module-kinds.sh` does not cover it: it reads the
vocabulary tables for MODULE names, never for what a cell says a module
holds. It is green over this.

## Confidence

`sure` that the cell is short of what the module holds. `likely` that
the repair is a stated rule rather than an enumeration, by the
precedent of the rows closed at `vdoc/readme-counts`.

## Closed — the cell states the rule that produces its contents, and the neighbouring cell was behind too (#vdoc/readme-attributions)

**Old cell.** `### The session's vocabularies`, `session::refuse`:
*"`Refusal` with its `rank`/`preferred` ladder, its `Display`, and the
recourse composers `affordance`/`exists_wording`/`offer_wording`;
`NodeKindWanted` and `admits`, since they are a `Refusal` payload and
its predicate"*.

**The subject, re-derived.** `rg -n '^(pub )?(enum|struct|fn|const|impl) '
crates/viewer/src/session/refuse.rs` gives the module's items, and the
half the cell does not have is: `NO_FACE_PICKED` (`:604`),
`FaceFrameFault` (`:622`) with `impl Display` (`:672`) and
`impl Error` (`:696`), and `face_frame_seat` (`:716`). All arrived with
AUTHOR's AUTH-1 (#2955), as the row said.

**New cell — the rule, not a third clause.** The cell now leads with
*every refusal vocabulary a session door raises, each with its
`Display`, the payloads and predicates that decide it, and the recourse
text it spends*, and then names the two that rule produces today.
`NO_FACE_PICKED` is recorded as spent outside the module, by `forms`,
which is the fact that makes it recourse text rather than a message.

**The neighbouring cells, checked the same way — and one was also
behind.** `session::op`'s cell said the module holds *"`SessionOp` and
`OpOutcome`"*; it also holds `ValueGestureName` (`op.rs:755`),
`FreeMoveName` (`:818`), `GestureName` (`:869`) and `CancelDoor`
(`:1391`), all public. The cell now has them. Its reader list was a
second defect of the same kind — it named seven modules where
`rg -l -e SessionOp -e OpOutcome crates/viewer/src` gives **27** files,
so a list read as a census was false; the cell now carries the count
and the command instead of the seven.

The other four cells were re-derived and are whole: `session::select`'s
five types are exactly `select.rs`'s five public items;
`session::author`'s four are exactly `author.rs`'s; `session::delete`'s
`DeleteAffordance` and `kind_census` are `delete.rs:27` and `:92`; and
`session::probe`'s *"range probe"* covers `probe.rs`'s four private
search functions beside `BoundsReading` and `BoundsTarget`.

**Both gates run over the edited page.**
`scripts/gates/viewer-module-kinds.sh` exit 0 (*"the README's 11
tabulated vocabularies agree with the modules they name"*) and
`scripts/gates/viewer-vocab-declared-once.sh` exit 0. As the row said,
neither reads what a `Holds` cell claims, so neither could have caught
this and neither reds on the repair.

Re-derived on the merged tree at `fb60ba2b7f`.
