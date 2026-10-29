---
id: an-analytic-face-may-cover-its-carrier-more-than-once
kind: issue
title: Nothing checks that an analytic face's parameter domain covers its carrier at most once, the premise the injectivity check's by-kind discharge rests on
status: open
opened: 2026-10-28
priority: P2
cost: M
refs: [self-intersection-drops-every-vertex-sharing-face-pair-globally]
---


## Finding

Both reviewers of PR #5955 (SELFX) flagged this as a NOTE.

`topo::injective::by_kind` discharges an analytic face (plane, cylinder, cone, sphere, torus with R > r) against itself, on the premise that the face is a region of its carrier's fundamental domain. That premise is: the face's loop, lifted to the chart, covers each carrier point at most once.

- The window does not enforce it: `full_turn` is one turn plus 2 ulps, and `by_kind` never reads the window.
- A face whose lifted loop winds more than one turn (a helical strip on a cylinder wider than its pitch, built by hand or imported) would be read injective while f : M → ℝ³ is not.

No builder in the tree makes one today; sweeps make NURBS walls. Import or validation is where a check belongs: either a validation clause, or `by_kind` reading the face's lifted span.
