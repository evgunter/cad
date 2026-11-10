---
id: agreeing-loops-left-beside-climb-agreeing
kind: issue
title: Five loops read every element and compare the answers by hand, beside ladder::climb_agreeing
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
---


Moved from `question-mark-ladders-the-gate-registered` when it closed (QML T6, `onecopy/refusal-standing`): the list PR 6606's review left there, unported.

Loops that read every element and compare the answers, beside the ones QML ported. Each is a candidate for `ladder::climb_agreeing`;
read each before porting it. In two of them a disagreement is a valid
answer of its own, not a fault (`climb_agreeing` leaves that to its
caller):

- `topo::link::second_order` (≈`link/second_order.rs:151`);
- `geom_brep::curve_face::composite::one_signed` (≈`composite.rs:462`);
- `geom_brep::ssi::section` (≈`ssi/section.rs:424`);
- `topo::boolean::vtxfac` (≈`vtxfac.rs:301`): a disagreement maps to a
  valid answer;
- `topo::boolean::rim_wedge` (≈`rim_wedge.rs:500`): a disagreement maps
  to a valid answer.

## Done when

Each is ported to `climb_agreeing`, or dispositioned in place with why it is not an agreement read.

## Closed (2026-11-10, `onecopy/agreeing-loops`)

One loop is ported, and four are dispositioned at their sites.

- **`topo::link::second_order` (`thorns`): ported** to `climb_agreeing` over the cycle's (side, vertex) pairs.
  - `beside`'s side answers, and its `None` aborts. A disagreement, or a pair that does not read, leaves the polygon `TouchUnread::Unreadable` as before.
  - Nothing is kept in band (`beside` returns no reason), so the reads, the exits and the K records are the old loop's.
  - Reach over topo and sweep `ci`: 139 climbs, all identical: 78 agreed, 40 disagree, 16 unread, 5 no pairs.
- **`geom_brep::curve_face::composite::one_signed`: not a ladder.** It is a fold over certified enclosures. No value is a band decision, so none is kept, and the sign needs every one.
- **`geom_brep::ssi::section::refined_sign`: not a ladder.** It is a refinement over hulls: a piece that straddles is halved and read again, not kept. Which refusal surfaces can depend on read order, and no outcome rests on it.
- **`topo::boolean::vtxfac::partner_side` and `topo::boolean::rim_wedge` (`FaceOnLocus::at`): ladders for the disagreement,** not `climb_agreeing` (PR 6750's review, F1).
  - Two decided rungs that differ answer (`PartnerRead::Across`, `Cover::RidesBothWays`), and only an agreement needs every rung.
  - Both refuse on the first reading in band, ahead of a decided disagreement. The ladder has no combinator for that shape.
  - Filed as `the-ladder-has-no-climb-for-a-disagreement-that-answers`, with both sites' measured reach. `partner_side`'s refusals occur in one unit test only, and none has a decided disagreement; `FaceOnLocus::at` has none in suite. Each site's comment names the row as a scheduled deviation.
- **`topo::boolean::contain::loop_circle`** (review F2): escalated on a later arc after an earlier edge had decided "not one circle", the ladder rule inverted.
  - **Ported** to `climb(Any)` over the edges against the first circle, an identity: any edge off it answers `None`, and an arc in band refuses only where every other edge is on the circle.
  - **Reach:** a probe, with a positive control of 1728 calls in topo alone, saw 0 in-band refusals in the topo, sweep and editor-core `ci` sets at either ε. So no in-suite outcome moves. Its pin is on `t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer`.
