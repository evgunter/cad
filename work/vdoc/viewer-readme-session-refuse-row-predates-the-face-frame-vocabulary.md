---
id: viewer-readme-session-refuse-row-predates-the-face-frame-vocabulary
kind: issue
title: The README's session::refuse row lists three composers and not the face-frame half the module now holds
status: open
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
