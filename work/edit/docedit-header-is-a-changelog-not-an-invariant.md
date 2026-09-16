---
id: docedit-header-is-a-changelog-not-an-invariant
kind: issue
title: DocEdit's header documents which milestone added which arm — history where the comment rule wants the invariant
status: closed
opened: 2026-09-16
closed: 2026-09-16
---



## Finding

**`DocEdit`'s type-level rustdoc is a changelog.** It reads as which
milestone added which arm — *"extended by M4 PR 4 with the two
explicit-repair edits"*, *"M4 PR 6 landed the reserved `SetTolerance`
arm"* — rather than as what the vocabulary IS
(`crates/editor-core/src/edit.rs`, the doc comment above
`pub enum DocEdit<P>`).

`docs/prompts/implementer-discipline.md` §4 settles the class:
*"Comments state the invariant, not the history. No retired-type
archaeology, no unit tags, no milestone or PR archaeology. An argument
about how the code used to work belongs in the PR description."* Every
sentence quoted above is milestone archaeology, and the reader who
needs to know what `DocEdit` is has to subtract it.

The arms' OWN docs are not the problem — each states what its edit
does and why, in the present tense. It is the header that narrates.

## What is needed

Rewrite the header to what the vocabulary is: a closed set of recorded
intents over a document value, every arm data, `apply` pure, the two
carry-forward doors and the create-or-replace door as the shape of the
parameter family. The milestone citations go; a clause id that names a
ratified DECISION (`spec D6`) stays, because that is a reference and
not a date.

Q8 shape: one paragraph, no behaviour change, no row moves.

## Home

`work/edit/` — `crates/editor-core/src/edit.rs` is EDIT's `paths`.

## Found by

The style review of `edit/doc-param-unit` (PR 2732). Pre-existing and
untouched by that unit, so it was filed rather than fixed there: the
notation arm's own doc is present-tense, and rewriting the header in a
PR about a new door would have hidden a prose decision inside a
feature diff.

## Closed (2026-09-16, EDIT orchestrator) — E-class

The header is rewritten to what the vocabulary is: a closed set of
recorded intents over a document value, every arm data, `apply` pure,
the three shapes named (structural edits; the parameter family's one
create-or-replace door and its carry-forward doors; the explicit
repairs and presentation state). The milestone citations are gone;
the clause ids (D2, D6, D7, N5, W4) stay as references. No behaviour
change, no row moves; merged on green CI and the orchestrator's read.
