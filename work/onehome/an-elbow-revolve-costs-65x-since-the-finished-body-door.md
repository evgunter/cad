---
id: an-elbow-revolve-costs-65x-since-the-finished-body-door
kind: issue
title: An elbow revolve costs 65x since the finished body became the doors' one constructor
status: open
opened: 2026-11-10
priority: P2
cost: M
---

Filed by `pcert/spline-certificate-door` (PR 6531's cost re-take), bisected
on main's first-parent line.

## What

`sweep::all pinned_corners_at_the_loft_door::an_elbow_builds_from_every_spelling`
(`crates/sweep/tests/pinned_corners_at_the_loft_door.rs`): the unit circle
turned about an axis outside it, in its plane or tilted 0.3 rad out of it,
authored from every 45° and in both senses, every spelling built.

At opt-level 1, `--test-threads 1`, one row at a time:

- `40e3f0f7f971901203be7778cde7ace23763f01a` (main before the merge): 0.188 s;
- `80a5227c5a368278c3c0740908f4186de75292f5`, the merge of PR 6371
  (`onehome/finished-body`: "the scalar's finish is the one constructor of a
  finished body; doors take and return it, consumer re-gates go"): 15.05 s;
- main at `bc788a7b8`: 14.8–15.4 s.

The bisect's other points agree: every first-parent commit up to
`40e3f0f7f` reads about 0.2 s, every one from `80a5227c5` on about 15 s. The
row is unchanged between them (`git diff f6e97bd62 bc788a7b8 --
crates/sweep/tests/pinned_corners_at_the_loft_door.rs` is empty), so the
cost is the kernel's.

## Where to look

Not profiled. PR 6371 changed `crates/sweep/src/revolve/mod.rs`, `loft.rs`,
`extrude.rs` and `test_support.rs`; the elbow is a revolve, so the finish its
door now runs is the first suspect (a tier-3 gate per build, where the row
builds 16 spellings). If the finish is meant to cost that, the row belongs in
the slow set and the cost is the design's; otherwise the finish repeats work
the door has already done.

## Fix

Profile the row at `80a5227c5` against `40e3f0f7f`, and either cut the
repeated work or record the cost as the finish's (and move the row to the
slow set, which it now passes the 0.5 s bar for).
