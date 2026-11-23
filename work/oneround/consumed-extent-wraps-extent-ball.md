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

## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.
