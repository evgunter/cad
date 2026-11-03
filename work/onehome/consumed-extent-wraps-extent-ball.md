---
id: consumed-extent-wraps-extent-ball
kind: issue
title: topo::ConsumedExtent is a public newtype over ExtentBall, a third name for the consumed extent
status: open
opened: 2026-11-02
priority: P3
cost: E
---


Found by the `onehome/one-extent` unit (2026-11-02).
`topo::boolean::carrier_eq::ConsumedExtent` (public as
`topo::ConsumedExtent`) is `{ reach: ExtentBall }` with a `new` and a
test-only `arm`. Its readers (`carrier_eq`, `plane_eq`, `reduce`,
`vtxfac`, `recl`, `merge_faces`, `refusal_routes`, four `topo/tests`)
read only the ball. `geom_brep::Reach` is the one consumed-extent type
and `ExtentBall` its ball payload, so the wrapper is a third name for
one thing: delete it for `ExtentBall` (or `Reach::Ball`) and repoint
its 43 references.
