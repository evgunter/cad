---
id: refusal-rank-wildcards-the-display-fault-payload
kind: issue
title: Refusal::rank wildcards DisplayFault, so a new display fault takes a rank nobody chose
status: open
opened: 2026-09-06
refs: [2053]
---


Found by the style review of PR 2053 (`view/refusal-all`), whose
closing argument is that `Refusal` needs no `ALL` because
`impl Display for Refusal` and `Refusal::rank` are exhaustive with no
wildcard.

That argument is true of `Refusal`'s own arms — both matches name all
eighteen and neither carries a bare `_ =>`. It does not hold one level
down.

## The site

`crates/viewer/src/session/refuse.rs:280-285`:

    Self::Display(DisplayFault::NoFreeMove | DisplayFault::FreeMoveInFlight) => 2,
    Self::Display(_) => 1,

`DisplayFault` is a vocabulary of its own
(`crates/viewer/src/display.rs`), and two of its arms are hand-listed
here at rank 2 with a comment explaining why — *"the substantive
display refusals rank with the real failures, because 'this instance is
mate-constrained' is a decision about what the user tried"*. A
nineteenth `DisplayFault` silently takes rank 1. If it is a
gesture-order fault it belongs at 2, and nothing reds.

`Display for Refusal` forwards `Self::Display(fault) => write!(f,
"{fault}")`, so the RENDERING half of the compile-time obligation does
reach into the payload; only the RANK half does not. The two arms
listed by name are the evidence that the rank is a per-fault decision
rather than a default.

The same question applies to `Refusal::Edit(Box<EditError>)` and
`Refusal::SlotUnit(props::SlotUnitFault)`, which take one rank each for
a whole vocabulary; those are deliberate, but the `DisplayFault` split
shows the shape where it is not.
