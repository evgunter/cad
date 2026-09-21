---
id: viewer-readme-two-marks-section-does-not-say-what-is-not-a-consumer
kind: issue
title: The viewer README's two-marks section records the consumer that was removed but not the look-alike that was refused
status: open
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
