---
id: an-aborting-touch-waits-in-the-backstop-like-an-undecided-one
kind: issue
title: The cross-solid backstop waits on a touch whose reading aborts (Unreadable, OffInterior) as on an undecided one
status: closed
opened: 2026-11-09
closed: 2026-11-10
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

## Closed (2026-11-10, `onecopy/backstop-abort`): ruled "at once"

The orchestrator ruled one meaning of an abort, the ladder's: the first touch whose reading aborts ends the pair's reading and is reported as the backstop's finding. A later decided crossing does not hide it. "As `Corrupt` is" was the other candidate, and it would leave that hole open for a defect read (`OffInterior`, a contradicted `EdgeTangent`) reaching the backstop.

**The change.**
- `census.rs`'s `blocks` now hands its readings, in scan order and read lazily, to one pure function, `blocked`. The readings are the pair's findings, then its recorded and shared-key sites.
- `blocked` returns:
  - the first crossing;
  - else the first aborting touch, at once;
  - else the first other reason ahead of the first waiting touch.
- **The standing** is the touch's own, `Undecided::aborts` over `TouchUnread::refusal()`, so a future abort cause joins without a list.

**Measured before building** (the topo, sweep and editor-core `ci` sets, both ε):
- 123 backstop readings per ε met an aborting touch, all `Unreadable` (now `Curved(..)`).
- None ended in a crossing or `Stands`. Each lost today to another undecided reason (`Unexamined` 121, `LoneLoop` 2).
- Every changed reading stays `Blocked::Undecided`, so the finding stays `CensusUndecidable { MayOverlap }` with only its `what` text changing. No verdict moves.

**Moved and re-pinned.** Three sweep rows read the solid pair's finding through `common/touch_union.rs`'s `only_curved_within_reach`, which now accepts the curved touch's sentence beside `Unexamined`, with the reason at the helper. The rows are `two_discs_kissing_externally_touch_at_every_op`, `externally_tangent_walls_touch_at_every_op` and `a_same_side_tangent_touch_answers_every_op`.

**Pinned:** `census::blocked_rows::an_aborting_touch_is_reported_ahead_of_a_later_crossing`. An abort scanned before a decided crossing reports the abort, and it is red under the old rule.
