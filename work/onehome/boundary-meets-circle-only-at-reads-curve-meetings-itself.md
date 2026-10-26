---
id: boundary-meets-circle-only-at-reads-curve-meetings-itself
kind: issue
title: reduce::boundary_meets_circle_only_at decides where a face's boundary meets a circle by plane sides and off-circle rows, a second reading of the curve-pair meeting question
status: open
opened: 2026-10-14
priority: P1
cost: M
refs: [5227]
---

Found by the sweep of stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227), which made `carrier_cross::boundary_crossing` a caller of `geom_brep::curve_pair::meet`.

`reduce::boundary_meets_circle_only_at` (`crates/topo/src/boolean/reduce.rs`) certifies that a face's boundary meets a circle nowhere but at given vertices. That is the curve-pair meeting question, asked of each boundary edge against the circle. It answers it with its own rows:
- `bool_arc_plane_side`: each vertex against the circle's plane;
- `bool_arc_boundary_off_circle`: a point in the plane against the circle (`splitting::containment::circle_miss`);
- a line's one crossing of the plane, and a conic's crossings by `splitting::plane_crossing_lane`.

`meet` reads the same pairs through its one `(CurveKind, CurveKind)` table (C5, D3 "One reading per question"). The two can decide one fact differently in the band:
- `meet` polishes a plane crossing to the carriers' closest pair, where this function reads the plane crossing itself, which lies `h/σ` from the meeting at a shallow angle;
- a conic lying in the circle's plane is read here only through a parallel plane decided off.

A route: build the circle's span from the arc, read each boundary edge against it with `meet`, and accept only meetings whose hit on the boundary edge is an `End` at one of `at`. The caller's `false` stays its frontier.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: a second answer to `curve_pair::meet`'s question: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
