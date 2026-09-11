---
id: code-quality-item-quotes-a-viewer-doc-string-that-was-rewritten
kind: issue
title: work/code-quality's PathVerb item quotes a viewer doc comment verbatim that PR 2143 rewrote, so the quoted string is no longer in the tree
status: open
opened: 2026-09-08
---

Reported under `docs/prompts/implementer-discipline.md` §6 by the style
review of #2143, and filed here because
`work/code-quality/viewer-pathverb-all-hand-written-seventeen.md` is
CODE-QUALITY's slate — a §6 report is not a durable artifact, so the
orchestrator writes the file.

## What happened

That item's `:25` quotes the viewer's doc comment **verbatim**:

> *"a verb with no label is a compile error … what reaches the MENU is
> `PathVerb::ALL`'s to answer"*

The first fragment existed word for word at #2143's merge base, in the
doc above `PathVerb`'s old hand-written `fn label`. **#2143 rewrote
that doc**, because it converted `PathVerb` to a labelled vocabulary
and the accessor moved into the macro's projection. At head the
replacement sentence is *"a variant with no word does not parse"*.

So the quoted string is no longer anywhere in the tree.

**Where the replacement sentence actually lives, re-derived 2026-09-11,
and the citation this row carried for it was itself wrong.** The row as
filed said *"`crates/viewer/src/forms.rs:190` says 'a verb with no word
does not parse'"*, and all three parts of that miss. `forms.rs:190` is
`pub(crate) const ALL;`. The sentence is not authored in `forms.rs` at
all: `PathVerb` is declared through a `vocabulary!` invocation
(`forms.rs`, the fourth of four in the file), and the sentence is part
of the doc that macro GENERATES for every `label`, written once at
`crates/viewer/src/vocab.rs` in the `macro_rules! vocabulary` body. It
reads *variant*, not *verb*, because it is written for every vocabulary
the macro makes and not for this one. So the repair is not a re-quote of
a line in `forms.rs`; it is a citation of the macro that emits the
sentence, and a taker has to decide whether the row should point at the
generator or at the rendered doc on `PathVerb`.

That miss is this row's own subject happening to this row — a citation
written inside the change it describes, never re-derived — and it is the
argument in `S176` for cite-by-name over re-checking. Cited by name the
sentence is *the `vocabulary!` macro's `label` doc in
`crates/viewer/src/vocab.rs`*, which survives both files being
renumbered.

## Why it is worth an announce line rather than nothing

The claim the quotation supports is **still true**, and more strongly
than before: a half-labelled list now fails to *parse*, which the
review demonstrated by deleting one variant's `= "cusp"` and getting
`error: no rules expected ','` … `note: while trying to match '='`. So
this is quotation rot, not a latent defect.

But it is the shape `stale-file-citations-after-the-split` names as its
hardest class — a citation whose SUBJECT is gone rather than moved —
and a quotation is worse than a line number, because a reader who greps
for the quoted string finds nothing and cannot tell whether the claim
was withdrawn or the text was reworded.

The same file's `:29` cites `app.rs:4419`, stale since the 1c split and
already covered by that row.

## Confidence

`sure` about the tree at both revisions. `likely` that the right repair
is re-quoting the new sentence rather than dropping the quotation, but
that is CODE-QUALITY's call, not this file's.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **E** — re-quote one sentence in one tracker
file; replacement text already identified. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.
