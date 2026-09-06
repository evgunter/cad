---
id: pair-subject-witness-strings-unswept
kind: issue
title: the pair-subject sweep could not read witness STRINGS: UndeclaredContact and ContactContradicted carry their subject in format! text, unexamined
status: open
opened: 2026-09-04
---


## The blind spot, stated as its own item

PR 1750's Half A swept the shape *a refusal whose subject is a set,
carrying one member the arena's ordering picked*, anchored on
`EntityId::Face(...)` and on `face:`/`entity:` struct-field payloads.
That pattern is blind to a subject that reaches a reader inside a
**string**, and `crates/topo/src/census.rs` has fourteen `witness:`
sites. Eleven render a POINT through `witness()` — a coordinate, which
is the right thing and is what makes those findings actionable. Three
render arena keys instead:

- `census.rs:1635` — `format!("{fa:?}~{fb:?}")`, the conformal sweep's
  undeclared-contact finding;
- `census.rs:2728` — `format!("{:?}~{:?}", c.face_a, c.face_b)`, the
  patch confirm pass's contradiction;
- `census.rs:2673` — `format!("{:?}", c.witness)`, the curve pass's
  witness edge.

## What is NOT this item

The `{:?}`-where-`Display`-belongs half is already owned by
`tier-3-prime-findings-render-through-debug`, which measured it at
LIB-B-VALIDATE4 and names `witness()` and two `validate.rs` arms. This
item does not re-file that.

Nor is any of the three the S190 defect: each carries its pair or its
edge in a TYPED field beside the string, so a consumer resolves from
the field and never has to read the prose. Nothing here is a
correctness gap.

## What IS this item

**Order.** PR 1750 settled that a census face pair is UNORDERED as a
subject, and wrote that into the type: `CensusSubject`'s `PartialEq`
compares `(a, b)` against `(b, a)`. The two pair strings above were not
revisited, and they still print one order — the arm's, which is the
arena's. So a reader is shown `f~g` for a subject the kernel now
holds to be the same as `g~f`, and two runs that differ only in arena
order produce two different messages for one finding.

The work is to read the three sites and decide, per site, whether the
rendering says what its arm means: normalise the order, say in the
message that the pair is unordered, or record why the arm's own order
is the right thing to show. It may well close with no code change and
a sentence — that is a legitimate outcome, and better than a blind spot
nobody wrote down.

## Provenance

This is a SWEEP gap, not a measured defect: PR 1750's own "what the
pattern could not match" sentence, given a file so it survives FIX's
directory being deleted at close.

## Home

`crates/topo/src/census.rs`.
