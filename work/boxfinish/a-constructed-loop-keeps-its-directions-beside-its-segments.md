---
id: a-constructed-loop-keeps-its-directions-beside-its-segments
kind: issue
title: ProfileLoop keeps a direction beside each segment's kind, in step by checks and by hand across reversal; for a constructed loop that is an invariant a type would hold
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found by PR 7384's review (2026-11-22). That PR moved a validated
segment's directions into its kind's arm (`SegmentKind::Line { director }`,
`SegmentKind::Arc { radials }`), so the validated form cannot pair a
line with radials. One layer up the same shape survives:
`profile::ProfileLoop { segments: Vec<Segment>, directions:
Vec<Option<SegmentDirections>> }` keeps a direction beside each kind, in
step by `seg::directions_match_kind` (a construction owes its kind),
`seg::check_directions`' `RecordedDirection::Kind` arm (a table), and by
hand across reversal (`ProfileLoop::reversed`, `SegmentDirections::reversed`).

For a table that is an input check: a table can be written the wrong
way round, and validation refuses it typed. For a `ConstructedLoop`
(the verbs' own output) it is an invariant, and a type would hold it: a
constructed segment carrying its directions in the arm of its kind,
as the validated one now does.

A design question now with designers will reshape the same record:
what a recorded direction is as a type
(`work/boxfinish/a-recorded-direction-claims-a-unit-and-is-held-as-two-numbers.md`).
Take this row after it, or with it.

**One more of the class, from PR 7384's second review**: the loft's
clearance record `sweep::loft::stacking::Piece`
(`crates/sweep/src/loft/stacking.rs:240`) keeps `line: bool` beside the
placed `carrier: Curve3<T>`, the bool read off the segment's kind
(line 288) and the carrier off the same segment, so the two can be
written to disagree. The carrier's own class is the one fact; the bool
is a second copy of it.
