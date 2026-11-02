---
id: conic-bend-side-reads-no-slack
kind: issue
title: A conic tied with a plane descends by geom_brep::conic_bend_side on the band alone, so over a cone's slack it reads no side
status: open
opened: 2026-11-02
priority: P3
cost: E
refs: [the-cone-takes-a-bounds-side-from-the-one-bound-side-reading, the-cone-sites-read-every-bounds-side-through-bound-read]
---


Left by `the-cone-takes-a-bounds-side-from-the-one-bound-side-reading` (review n4).

`bound_side::Tie::descend` decides a tie's parting on the caller's `link::metric::Reading`, but a conic tied with a plane face is descended by its certified lift (`geom_brep::conic_bend_side`), which decides inside geom-brep on the band alone and returns a verdict without its margin. Over a positive slack (a cone's reading, `link::Cone::reading`) `Tie::descend` therefore reads that arm no side at all (`Unresolved::Slack`), however far the lift clears the band: a conservatism that gives up decided sides at a cone site. Today the only cone-site reader is recl's held face (a plane), where a conic flank tied with it is that case; the suites meet none (the PR's decision diff). Lane 2 hands every sheet's side in through the same descent, so it will meet more.

Owed: `conic_bend_side` returns its lift's margin, or takes a slack, so the descent decides it over the slack as it does a parting (`Distance::departure`), or a statement that the case stays refused; a row of a conic flank tied with a held plane, its lift clearing band and slack, read `Leaves` over the slack.
