---
id: agreeing-loops-left-beside-climb-agreeing
kind: issue
title: Five loops read every element and compare the answers by hand, beside ladder::climb_agreeing
status: open
opened: 2026-11-10
priority: P3
cost: M
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
