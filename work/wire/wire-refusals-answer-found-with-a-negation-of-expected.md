---
id: wire-refusals-answer-found-with-a-negation-of-expected
kind: issue
title: Two eval/wire.rs refusals answer found: with the negation of expected: (carries kind not a datum frame) where node_value_kind would name the family it does carry
status: closed
opened: 2026-09-11
refs: [2376]
pr: 2480
closed: 2026-09-13
---


## Finding

Found by WIRE's `family-consts` lane (PR 2376), inside this program's
fence but outside that unit; filed here by the WIRE orchestrator rather
than widened into a text-preserving PR. Accurate at `af8bbca`.

`crates/editor-core/src/eval/wire.rs:978-979` and `:4191-4192` fill
`found:` with the NEGATION of `expected:`, so the refusal reads

> input 7 carries kind **not a datum frame**; the operand needs kind
> datum frame

which tells the reader what the input is not, twice, and what it is,
never. Both sites refuse off a **node**, and `eval::node_value_kind`
exists precisely to answer that with a family word — the refusal would
read *"carries kind profile; the operand needs kind datum frame"*,
which is strictly more information at the same length.

`memories/refusal-text-is-not-cause.md` is the neighbouring rule rather
than this one: there the payload is discarded, here it is never asked
for. Same consequence for a reader — the refusal's text is not evidence
about the input.

## Why it is its own unit and not a rider

It changes **user-visible refusal text**, so it owes a look at every
assertion over these two messages, and it needs `node_value_kind`'s
`MissingInput` arm plumbed through two call sites. PR 2376 was
byte-identical by construction and deliberately so; folding this in would
have made that PR's central claim false.

Two neighbours that are NOT this row, recorded so the sweep is not
re-run: `crates/editor-core/src/mate/member.rs:716`
(`expected: "datum axis"`) and `crates/editor-core/src/verbs/split.rs:157`
(`tool_expected: "datum plane"`) are composed phrases with the prose
disposition PR 2376 argued for, and the second is already a single const
pinned by `split.rs:218`.


## Closed 2026-09-13 (PR 2480)

Closed as a **consequence** of the door, not as a text edit: the door
computes `found:` from the value it was handed, so a refusal cannot
answer with the negation of its own `expected:` any more.

**One of the two sites the row named no longer existed.** PR 2435 had
deleted `frame_kind`, and `profile_plane_f64` already answered
`kind_name()` on main, so only `section_of` still carried the negation.
Its refusal is the **one** user-visible message this unit moved, and
that was established with an instrument rather than argued: a
19-miswiring probe rendered every document-reachable `WrongOperand` on
both trees and the diff of the two rendered sets is three lines, all
`section_of`.

The probe was written by the reviewer and is now the suite, with
authorship kept.
