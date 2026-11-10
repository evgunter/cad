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
- **`geom_brep::ssi::section::refined_sign`: not a ladder.** It is a refinement over hulls: a piece that straddles is halved and read again, never kept.
- **`topo::boolean::vtxfac::partner_side`: not `climb_agreeing`.** Bounds on both sides are an answer of their own (`PartnerRead::Across`).
  - `climb_agreeing` answers the decided agreement past a reading kept in band, without telling its caller one was kept. Here that reading could be the `Across` answer, unread.
  - So every bound is read first, and one in band refuses.
- **`topo::boolean::rim_wedge` (`FaceOnLocus::at`): not `climb_agreeing`,** for the same reason: rides both ways are an answer of their own (`Cover::RidesBothWays`).
