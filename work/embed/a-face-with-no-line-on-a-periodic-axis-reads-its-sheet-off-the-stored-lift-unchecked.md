---
id: a-face-with-no-line-on-a-periodic-axis-reads-its-sheet-off-the-stored-lift-unchecked
kind: issue
title: A face whose boundary holds no line on a periodic axis takes its sheet from the stored lift unchecked, so Sheets::Missed and the within clip rest on the lift alone there
status: open
opened: 2026-11-17
priority: P3
cost: M
---


## Finding

From EMBED lane half-sphere (PR 7086), at its review.

`face_extent` (`crates/topo/src/injective.rs`) reads the face's
coordinate lines against `Boundary::extent` before any reader takes
the extent. Where a line contradicts it, the face is not presented.

Some faces carry no such line, and for them nothing checks the extent:
- an edge whose image is no coordinate line (a harmonic, spiric wall
  or spline row);
- a line whose face side or value does not certify;
- an edge on the face twice.

A face whose boundary holds no line on a periodic axis therefore takes
its sheet there from the stored lift (`pcurves::Lift::read`) alone. On
that axis, three readers rest on the lift and nothing else:
`Sheets::Missed` (`sheets`, which reads the face as holding nothing),
the `within` clip in `present_chart`, and `placed`.

**Latent.** It needs a stored row off its sheet. Since ONEDOOR's
no-reset lift (PR 6900), no measured body has one: the revolved ball,
the revolved cone, the two-ball union and the filleted corner's ball
were each probed.

## What is needed

1. Say which at-rest check establishes that the stored lift is the
   face's boundary on one sheet, if any does. Candidates:
   `validate_pcurves`, or a fresh `decide_joint` at a pole or an apex
   checked against the stored element.
2. Build the case first: a loop through a pole or an apex made of
   curves that are not meridians, for instance two small circles
   through a pole. Then read whether its extent is on its sheet.
3. If no check establishes the lift, cross-check the extent there too.
   The extent of a curved row could be read against the face side its
   chart curve gives.

If a body is built that reaches it, re-band it P0: a face read as
holding nothing certifies any pair it is in.
