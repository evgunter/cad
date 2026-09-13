---
id: a-document-vocabulary-declared-outside-the-macro-is-uncensused
kind: issue
title: A document vocabulary declared with a plain pub enum gets no ALL_NAMES, no census, and nothing detects it
status: open
opened: 2026-09-13
---


Disclosed by WIRE (PR 2501) at the moment it was created, per
`docs/prompts/implementer-discipline.md` §6: the residue gets its own
file rather than a paragraph in a PR body.

## Finding

`crates/editor-core/src/program.rs` declares its three document
vocabularies — `ProgramStep`, `ProgramArcData`, `ProgramTarget` —
through one `document_vocabulary!` invocation, which projects
`ALL_NAMES` per enum and a `DOCUMENT_VOCABULARIES` roster over all of
them. Two failures are closed by that:

- **a variant arrives without a witness** — `ALL_NAMES` is derived from
  the declaring tokens, so `tests/switch_program_vocabulary.rs` reds;
- **a vocabulary arrives without a census** — the roster is projected
  from the same invocation, and the invocation is single by
  construction (the constant is emitted once per invocation, so a
  second does not compile), so the census iterates it rather than
  naming three vocabularies by hand.

**What neither closes is a vocabulary declared with a plain
`pub enum`.** It has no `ALL_NAMES`, it is absent from
`DOCUMENT_VOCABULARIES`, and nothing anywhere observes that it should
have been in either. The census cannot see it, so it cannot red for it.

That matters for the reason the macro exists: a document enum's
construct hop (`res_step` / `res_spec` / `res_target`, or whatever a
fourth one's would be) matches the document form and CONSTRUCTS the
kernel one, so a variant that never reaches a witness is silently
laundered into an existing kernel form, and every kernel-anchored
census stays green.

## Why WIRE did not close it

Closing it means answering *"is this enum a document vocabulary?"* over
the file's declarations, which is a walk over source TEXT. PR 2501
removed exactly such a walk, measured: a variant carrying any attribute
(`#[doc(hidden)]`, `#[cfg]`, `#[serde]`, `#[allow]`) puts the attribute
in front of the name, the walk reads an empty name, and the census
reports agreement over a set missing exactly the variant it exists to
catch — a silent green. Re-introducing a text walk one level up buys
the same failure mode back.

## Shape of a fix, if it is wanted

The honest options are DOCM's call, not WIRE's:

- **A declaration convention with a gate**: every `pub enum` in
  `program.rs` is either declared through the macro or carries a
  one-line "not a vocabulary, because …" — checkable, but by a text
  walk, with the hazard above.
- **Make the macro the only door**: a `#[non_exhaustive]`-style
  discipline, or moving the vocabularies to their own module whose
  `pub enum`s are all macro-declared, so "declared in this module" and
  "declared through the macro" are the same statement.
- **Accept it and say so at the site**, which is what ships today: the
  macro's `DOCUMENT_VOCABULARIES` doc states the gap in its own words
  and cites this file.

## Fence

`crates/editor-core/src/program.rs` is DOCM's. WIRE's crossing there
was the derived constant plus the doc clauses describing it, announced
on PR 2501; the declaration convention above is a design call about
that file and is left where it belongs.
