---
id: a-kind-enum-does-not-convert-its-own-consumers
kind: issue
title: a kind enum landing does not convert its consumers, and nothing sweeps for the ones left behind: PathErrorKind exists and the viewer's preview door still renders to prose
status: open
opened: 2026-09-04
---


Named by FIX's `boolean-error-has-no-fieldless-kind` lane (PR 1806)
while sweeping the shape it was fixing. Filed by the FIX orchestrator;
the instances span CHROME/VIEW, DOCM and this program's own slate, so
it is homed here rather than on any one of them.

## The shape

A fieldless kind enum is minted so a typed refusal stops degrading to
`reason: String` at a consumer door. The unit that mints it converts
**the door it was cut for**. Every other consumer of the same error
keeps rendering to prose — and **nothing sweeps for them**, because the
kind enum's existence is not a compile error anywhere.

So the tree accumulates doors that *could* carry a class and do not,
each one looking deliberate.

## The instance that proves it

PR 1490 minted `profile::PathErrorKind` (28 variants, exhaustive
`kind()`) for exactly this reason. `crates/editor-core/src/program.rs:396`
carries it. **`crates/viewer/src/sketch.rs:633,909` renders the same
`PathError<f64>` to prose** through `PreviewError::Geometry { rendered }`
— a kind that already exists, at a door that does not carry it, months
after the minting.

Nothing is wrong at that site on its own terms. That is the point: it
reads as a deliberate choice, and the only way to tell it from a
leftover is to know a kind enum was minted elsewhere.

## The other instances the same sweep found

Reported by the lane, not filed by it (§6), and listed here so the
class has its census:

- `crates/viewer/src/tree.rs:75` — `RowStatus::Failed` from `NodeError`
  (the `NodeErrorKind` sibling; two fences).
- `crates/editor-core/src/eval/parts.rs:92,110,117` — `PartFault`'s
  three message arms (DOCM).
- `crates/viewer/src/session.rs:1719` — `AtRestBadge::Refused`
  (CHROME/VIEW).
- `crates/editor-core/src/checks.rs` — `ChecksError::Product` carries
  `product::ProductError` as prose **in the same file, one door over**
  from the one PR 1806 just converted, which makes that file
  asymmetric. Filed separately on FIX's slate as
  `checks-product-refusal-degrades-to-string`.

That last one is the sharpest: the same unit, the same file, and the
door next to it was not converted — so the class does not even need two
programs to bite.

## What would close it

Not a rule that every consumer must carry a kind — some genuinely want
prose. What is missing is a **sweep that can find them**: given a kind
enum, the set of sites rendering its source error to a string. That is
mechanical, and it does not exist.

Note the adjacent guard `work/fix/kind-mirrors-have-no-single-declaration.md`
covers a **different** direction (the mirror drifting from its source);
this one is about consumers that never adopted the mirror at all.
Neither subsumes the other.

## Home

`work/issues/` — instances in `viewer` (CHROME/VIEW), `editor-core`
(DOCM), and FIX's own `checks.rs`. Re-home if one program wants the
whole class.
