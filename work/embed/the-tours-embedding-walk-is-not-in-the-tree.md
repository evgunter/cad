---
id: the-tours-embedding-walk-is-not-in-the-tree
kind: issue
title: The tour's embedding walk is not in the tree
status: closed
opened: 2026-11-18
priority: P2
cost: E
closed: 2026-11-18
pr: 7127
branch: embed/tour-walk
---


## Finding

Every EMBED engine lane and reviewer measures a change by walking the
tour and asking `topo::clearance::embedded` of every finished body, and
quotes the result ("82 of 85 hold", with each body's candidates,
discharged, refused and splits). The program that prints those figures,
`demos/tour/examples/embed_walk.rs`, exists only on the measuring branch
`embed/conic-vertex-measure`, together with the two visibility changes
in `demos/tour/src/lib.rs` it needs (`walk_tour` and `TourStop::name`
are private on main). So:

- two reviewers in a row could not re-run a PR's tour figures, because
  the tool that produced them is not in the tree they review;
- each lane fetches the tool by branch name, and the branch also carries
  a measuring switch in `crates/topo/src/shell.rs` that is no part of
  the walk.

No CI job runs the walk, so the figures are only as reproducible as the
tool is findable.

## Resolution

`demos/tour/examples/embed_walk.rs` lands on main with `walk_tour` and
`TourStop::name` public; its module doc states the question, the band,
the budgets, the output format and the run command, and
`demos/README.md` ("Run the tour") points at it. The verdict lines go
to stdout under a `WALK` prefix and the per-body wall time to stderr, so
a captured report diffs clean between two runs of one tree.

Not done here: the walk gates nothing in CI.
