---
id: an-aborting-touch-waits-in-the-backstop-like-an-undecided-one
kind: issue
title: The cross-solid backstop waits on a touch whose reading aborts (Unreadable, OffInterior) as on an undecided one
status: open
opened: 2026-11-09
---


Found while carrying the touch's reason into the backstop (`onecopy/carrier-standing-2`, row `the-typed-refusal-reason-leaves-an-unused-second-spelling-and-unpinned-reads` item 11). The orchestrator ruled: file it, don't decide it there.

`link::TouchUnread::refusal` names two of its reasons as aborts, refusals of the whole query:
- `Unreadable`, a germ no later face rescues;
- `OffInterior`, two readings of the touch that disagree, a kernel defect.

The census's cross-solid backstop (`census.rs`, the `blocks` reading over `Undecided::is_undecided_touch`) still waits on both as undecided touches. A decided reason found later takes the finding's place, and the abort is reported only where nothing else is. `Corrupt`, the third abort, does not wait: it goes to `Undecided::CorruptInstance`.

So the backstop and the ladder disagree on what an abort is. To the ladder it stops the reading now. To the backstop it is one more undecided touch, and a decided touch can hide it, a kernel defect included.

**The question.** Should a touch whose reading aborts stop waiting, reported at once as the backstop's own refusal (as `Corrupt` is), or wait as it does?
- `Unreadable` is the common case: a curved face or edge at the touch.
- Changing it moves what the backstop reports wherever a curved touch and a decided one meet on one pair.

This is cross-cutting: it touches every abort the backstop meets, not one reason. It needs a ruling before code.
