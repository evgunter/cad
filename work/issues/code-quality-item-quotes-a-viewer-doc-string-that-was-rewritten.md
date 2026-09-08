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
and the accessor moved into the macro's projection. At head
`crates/viewer/src/forms.rs:190` says *"a verb with no word does not
parse"* instead.

So the quoted string is no longer anywhere in the tree.

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

