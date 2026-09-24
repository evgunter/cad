---
id: a-document-vocabulary-declared-outside-the-macro-is-uncensused
kind: issue
title: A document vocabulary declared with a plain pub enum gets no ALL_NAMES, no census, and nothing detects it
status: open
opened: 2026-09-13
priority: P3
cost: D
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
  from the same invocation, so the census iterates it rather than
  naming the vocabularies by hand.

  That invocation is single by construction **within one module**, and
  the qualifier is load-bearing: `E0428` is scoped to a module's value
  namespace, so a second invocation in a CHILD module compiles clean
  and projects a second `DOCUMENT_VOCABULARIES` the census never reads.
  `program.rs` has no child modules, so the list is complete today —
  what makes it complete is that fact, not the macro.

**What neither closes is a vocabulary declared with a plain
`pub enum`.** It has no `ALL_NAMES`, it is absent from
`DOCUMENT_VOCABULARIES`, and nothing anywhere observes that it should
have been in either. The census cannot see it, so it cannot red for it.

That matters for the reason the macro exists: a document enum's
construct hop matches the document form and CONSTRUCTS the kernel one,
so a variant that never reaches a witness is silently laundered into an
existing kernel form, and every kernel-anchored census stays green.

## The live instance, named — and resolved

This row was first filed stating the class hypothetically. The review of
PR 2501 found the instance sitting **four lines below the invocation's
closing brace**: `LoopProgram`, declared with a plain `pub enum`, whose
`resolve` is a **fourth construct hop of exactly this shape** — it
matches the document vocabulary and builds `Step::Circle` /
`Step::CircleSplit`. A carrier form added to `LoopProgram` alone and
resolved into an existing kernel step left every kernel-anchored clause,
`DOCUMENT_VOCABULARIES` and the census green, and `corpus_vocabulary`
explicitly declined to witness it.

**Decided on the evidence rather than the shape**: the construct hop is
real, so `LoopProgram` IS a document vocabulary. It is now declared
through the macro and witnessed from `corpus()`'s own loops — all three
of its variants were already there, so the witness cost nothing but the
decision to make it. That is the test for membership: *does a variant of
it launder into an existing kernel form at a construct hop*, not what
the type is called.

`ProgramRefusal` and `RecordedProgramError` are the file's remaining
plain enums and are deliberately out, now said at the site: they are the
REFUSAL and ERROR vocabularies, produced for a caller to read, with no
construct hop building a kernel form out of them and so no laundering
direction to guard.

**What stays open is the general case** — a future enum declared with a
plain `pub enum` that should have been a vocabulary. Nothing detects it;
the three named above are dispositioned by hand, in prose, at the site.

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

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/census/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): one vocabulary spelled outside the census macro is CENSUS's charter. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.
