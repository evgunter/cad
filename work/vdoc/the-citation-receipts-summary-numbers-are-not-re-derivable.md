---
id: the-citation-receipts-summary-numbers-are-not-re-derivable
kind: issue
title: The citation receipt's summary numbers do not re-derive: 60 has no enumeration rule, six move by -13 is five, twelve lines is eight, two public items is three
status: open
opened: 2026-09-06
refs: [2089]
---



Found by the style review of #2089. The receipt itself is good work —
the eighteen stale citations are real and the eleven that predate the
branch all check out by reading the line on `bc44531e1`. What does not
survive is every SUMMARY number wrapped around it, and this is the
second consecutive VIEW PR for which that is true
(`citation-repoint-shifted-a-number-the-lane-knew-was-wrong`, #2083).

## Four numbers

**1. "60 citations checked" has no enumeration rule.** A
`path.rs:line` regex over the eight item files the branch edits returns
51; counting the comma-lists (`edge-cost-claims…:13,19`,
`run-on-whitespace…:9,23`, `error-types…:73,87`), the second numbers in
`pickindex.rs:422`/`459` and `forms.rs:55` (`view/const-all`: `:74`) as
separate citations returns 56; the new `work/view/log.md` section adds
9 more, most of them restatements of the same coordinates. 60 sits
inside that range and cannot be landed on. A receipt is a number a
second reader can re-derive; this one needs its rule stated — which
files, whether `log.md` counts, whether `:a,b` is one citation or two.

**2. "The six `marks.rs` citations … all move by −13" — five do.**
`:286`→`:273`, `:200`→`:187`, `:224-227`→`:211-214`,
`:118-121`→`:105-108` and `:261`→`:248` are all exactly −13. The sixth,
`the-point3-to-gpu-corner-cast-is-at-three-sites`'s `:132-135`, became
`:119-121` — a three-line span, not a shifted four-line one, because
`marks.rs:135` on `bc44531e1` was `#[derive(Clone, Debug, Default,
PartialEq)]` and never part of the quoted doc sentence. The
re-derivation is right and the rule stated over it is wrong. It also
means that citation was **already** wrong on `main`, so on the lane's
own accounting the "eleven that predate this branch" is twelve, and
this one is filed in the wrong bucket.

**3. "Twelve lines argued what happens at the `u64` ceiling … Six lines
instead of ten."** Nothing in either tree is twelve. On `bc44531e1`,
`next`'s whole rustdoc block is ten lines (`generation.rs:33-42`) and
the ceiling paragraph inside it is eight (`:35-42`). On head the block
is eight (`:40-47`) and the paragraph six (`:42-47`). "Six instead of
ten" compares the new paragraph against the old BLOCK. The sentence
appears in the PR body, in `generation-get-has-no-reader`'s `## Closed`
and in `work/view/log.md`, twelve-line version included.

**4. "the module is 51 lines with two public items, both of which have
readers."** Three public items: `Generation` (`generation.rs:34`),
`Generation::FIRST` (`:38`, 25 readers) and `Generation::next` (`:48`,
one — `session.rs:1814`). All three do have readers, so the claim's
point stands and only its count is wrong.

## The 51 is the more interesting number

`crates/viewer/src/generation.rs` is 51 lines on `bc44531e1` **and** 51
lines on head. Deleting `get` and trimming `next`'s paragraph removed
seven lines; the new *"The counter itself is not readable"* paragraph
at `:27-32` put six back, plus its blank. A unit whose finding was "a
`pub` item with no reader is a third of a leaf module's API" closed by
replacing four lines of code with six lines of prose about the four
lines not being there, at exactly zero net. That is worth saying in the
close, and the close says the opposite by quoting 51 as an outcome.

## Where else to look

Every `## Closed` section and `log.md` entry this program has written
that quotes a line count, a citation count or a "moves by N" rule.
The two rows here are #2083's and #2089's; a fix pass that corrects
only #2089's is a half-fix.

## Confidence

`sure` on all four counts and on the 51/51 — each is one `sed -n` and
one `git show` away. `likely` that the right correction is to state the
enumeration rule beside the count rather than to restate the count.
