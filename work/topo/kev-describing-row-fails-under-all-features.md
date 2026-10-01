---
id: kev-describing-row-fails-under-all-features
kind: issue
title: euler_kill's kev_describing_asks_the_survivors_point row fails tier-1 DanglingGeometry under --all-features, a feature set no CI row runs
status: open
opened: 2026-10-01
priority: P3
cost: M
---


Filed by the PCERT orchestrator, 2026-10-01, from two independent lanes.

`cargo nextest run -p topo --all-features --lib` fails
`euler_kill::tests::kev_describing_asks_the_survivors_point_only_where_a_question_needs_it`
with `kev postcondition: result is not tier-1 valid … DanglingGeometry
{ from: Vertex(2v1), to: Point(2v1) }` (raised from `surgery.rs`). The
same row passes without `--all-features`. Seen on a clean `origin/main`
worktree (f5480017) by the `pcurve-fit-refusal-drops-the-domain-doors-reason`
lane, and on PR 3612's head by its reviewer; that lane also saw
`review_d18::kill_anchors_on_a_few_torn_bodies` fail the same way on
its head.

No CI row runs `topo` with `--all-features` (the nightly passes it to
`viewer` only), so the gate cannot see this. Either a feature changes
what `kev` leaves behind — a real defect on a configuration a user can
build — or the row asserts something a feature legitimately changes.
Which is this row's work; the feature that flips it is the first fact
to establish.
