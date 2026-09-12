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
does not parse'"*, and all three parts of that miss. `forms.rs:190` did
not hold the const the row put there; at the time of writing the const
was one line above, and by the time this PR merged `forms.rs` had moved
twice more on `main` and it was one of five such lines further down
again. **Read every `forms.rs` number in this row as of the SHA beside
it and none of them as current** — that
exact line occurs **five** times in the file (`:49`, `:105`, `:128`,
`:189`, `:296`,
`grep -n "pub(crate) const ALL;" crates/viewer/src/forms.rs`), so a bare
number never identified it in the first place. The one it means is
`PathVerb`'s `const ALL`, the declaration inside the same `vocabulary!`
invocation that declares `pub(crate) enum PathVerb`, and that is how it
is named from here on. The sentence is not authored in `forms.rs` at
all: `PathVerb` is declared through a `vocabulary!` invocation
(`forms.rs`, the fourth of the five in the file), and the sentence is part
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

## Repaired 2026-09-11, in DOOR's file, on Ev's authorisation

Ev authorised CITE to repair this citation directly in
`work/door/viewer-pathverb-all-hand-written-seventeen.md` rather than
route it, in one PR, with no routing issue filed and no
`work/README.md` change. The authorisation covers this repair and the
two beside it (`d107-release-profile-job-lives-in-nightly`,
`loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`) and
nothing wider.

**What was done.** The quotation is re-cited to the GENERATOR — the doc
on `label` in the `macro_rules! vocabulary` body in
`crates/viewer/src/vocab.rs` — with the rendered doc on
`PathVerb::label` in `forms.rs` named beside it. The generator, because
`forms.rs` holds no copy of the sentence: a reader who greps `forms.rs`
for the quoted words finds nothing, which is the failure mode this row
exists to name. The historical quotation stays in DOOR's body, marked as
historical, with the replacement named next to it — the clause DOOR
draws from it turns on the MENU, which the replacement sentence does not
mention, so overwriting it would have left that clause resting on words
that do not reach it.

`app.rs:514` and `app.rs:557` are repaired to names in
`crates/viewer/src/forms.rs`: `pub(crate) enum PathVerb`, and the
`pub(crate) const ALL;` in the same `vocabulary!` invocation.
**`app.rs:4419` is deliberately NOT repaired.** Its subject was an
iteration of `profile::ArcMode::ALL` in the viewer, and no viewer file
iterates that any more; what carries the name `ArcMode` in the viewer
now is the viewer's OWN `vocabulary!` enum in `forms.rs`, iterated in
`crates/viewer/src/widgets.rs` (`for (option, label) in ArcMode::ALL`),
which is a different thing. Repointing would invent a subject, so the
citation is left in DOOR's body with a flag saying it no longer
resolves, and the consequence — that the sentence calling the arc-mode
half **fine** has lost its ground — is DOOR's, in the marked section.

**Where this row's own re-derivation of 2026-09-11 was wrong, and what
is corrected above.** Two things, both of them line-number claims made
inside a row about line-number claims:

- It said `PathVerb` is declared in *"the fourth of four"* `vocabulary!`
  invocations in `forms.rs`. There are **five** —
  `grep -n "^vocabulary! {" crates/viewer/src/forms.rs` prints `:36`,
  `:73`, `:108`, `:131`, `:267` — and fourth of five is right.
- It said `forms.rs:190` is `pub(crate) const ALL;`. It was not, at the
  base where that was checked: `:190` was blank and the const was `:189`
  (and after `main`'s `5045203` and `77631b4` the const is `:199` and
  `PathVerb` is `:155` — the number has now been wrong in three
  different ways in four days, which is the argument, not an aside)
  (`sed -n '189,190p' crates/viewer/src/forms.rs`), and the same line
  occurs five times in the file, so the number identified nothing even
  when it was one off. The paragraph above now names the const by the
  `vocabulary!` invocation that declares it.

The rest of that paragraph — that the sentence is authored in
`vocab.rs`, and that it reads *variant* rather than *verb* — holds at
this base.

**Three things the repair turned up that are DOOR's, recorded in DOOR's
file in a marked section and not decided here:** the transition table
declares twenty verbs now, not nineteen (`ContinueTo` is the new one);
`PathVerb::ALL`'s length is no longer a literal but
`vocabulary!(@count …)`, so `## Shape of the fix`'s premise moved; and
the viewer's `ArcMode` is now the viewer's own `vocabulary!` enum rather
than `profile::ArcMode`, so the sentence calling that half **fine** rests
on an anchoring the tree no longer has.

This row stays open until DOOR reads the marked section, because what it
asks for — a decision on whether the quotation should have been dropped
rather than re-quoted — was CODE-QUALITY's call to make and is now
DOOR's.
