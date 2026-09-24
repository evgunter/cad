---
id: viewer-readme-two-marks-section-does-not-say-what-is-not-a-consumer
kind: issue
title: The viewer README's two-marks section records the consumer that was removed but not the look-alike that was refused
status: closed
closed: 2026-09-21
branch: vdoc/readme-attributions
opened: 2026-09-20
priority: P4
cost: E
---



Filed by a VNEWS lane (#2917) that had to reconstruct the argument the
section is missing. **Announced crossing**: the fix is in
`crates/viewer/README.md`, which is VDOC's, CHROME's and VIEW's — so
it is filed here and not written there.

## What the section records

`crates/viewer/README.md`, "The line is composed at two levels and
they are two marks", states the split, then "The third consumer was
the second level misread" records what happened when a site joined on
`frame::LIST_SEPARATOR` that should not have: the preferences file's
startup notices, removed at #2710. The test it gives is exact —
*"nothing counts them and no preamble introduces them, so there is no
enclosing sentence for them to be the items of"* — and it disposes of
the obvious counter-argument, that they *"happen to share a
subject"*.

## What it does not record, and what that cost

**The section says which consumer was removed. It does not say which
look-alikes were examined and refused.** `crates/viewer/src/` holds
one other site that spells the same two characters for its own
reasons: `seats::seat_line`'s panel label, joined with a literal
`"; "`. It is not a consumer, by the section's own test — it reaches
no `frame::Message`, nothing counts the seats and no preamble
introduces them.

Nothing said so, so a sweep read the shared spelling as a missing
reference and filed
`work/vnews/seat-line-spells-the-list-mark-as-a-literal` — a row that
asked for the substitution, was dispatched, was implemented, and was
caught in review. The argument that refuses it now exists at the site,
in `seat_line`'s doc comment, and in that closed row. **It does not
exist where the next sweep will look**, which is this section.

## The shape of the fix, and the thing to get right

One or two sentences after the third-consumer paragraph, saying that a
site sharing the spelling is not thereby a consumer and naming the one
that does. **The hazard is that this is a census**, and this register's
rule is that a population is re-derived by subject rather than copied:
if the line names `seat_line` alone and the mate panel's
`"pick a: node {}; pick b: node {}"` is left out (see
`work/vnews/mate-panel-hand-rolls-the-seat-line`), the section acquires
a second, smaller wrong population. Either re-derive both, or write
the TEST rather than the list — the test is what the sweep needed.

## Home

VDOC's: `crates/viewer/README.md`.

## Closed — the section states the TEST and disposes both look-alikes (#vdoc/readme-attributions)

**What was missing.** *"The third consumer was the second level
misread"* recorded the site that was removed and nothing about the
sites that were examined and refused, so the next sweep over the
shared spelling had nothing to read.

**The subject, re-derived.** `frame::LIST_SEPARATOR` is `"; "`
(`crates/viewer/src/frame.rs:758`). Two sites in `crates/viewer/src`
write those two characters between items of their own:

- `seats::seat_line`'s `.join("; ")` at
  `crates/viewer/src/seats.rs:373`, whose doc comment at `:351-361`
  already carries the argument that refuses it;
- `pane::create`'s mate-tool panel, `"pick a: node {}; pick b: node {}"`
  at `crates/viewer/src/pane/create.rs:134`.

Neither is a consumer: neither reaches a `frame::Message`, nothing
counts their items and no preamble introduces them — the section's own
test, applied forwards rather than backwards.

**New text — the test, because the list is not sweepable.** The row
warned that naming `seat_line` alone would mint a second, smaller wrong
population, so the section now states the membership test first and
disposes both sites under it, and says why the list is disposed rather
than swept. Both commands are printed and were run out of the file:

    rg -n '"; "' crates/viewer/src

prints **3** (the constant, `seat_line`'s join, and the doc comment
arguing about it) and cannot see the mate panel at all, whose mark is
inside a format string; and

    rg -n '"[^"]*; ' crates/viewer/src

prints **43**, every sentence in the crate that uses a semicolon. Both
exit 0, both extracted with `sed -n Np` and run as printed. There is no
pattern between them for *joins its own items*, which is precisely why
the section holds a test and not a census.

The mate panel's second spelling is `work/vnews/mate-panel-hand-rolls-
the-seat-line`, still open on VNEWS's slate and named at the sentence;
fixing that duplication would not make the site a consumer.

Re-derived on the merged tree at `fb60ba2b7f`.
