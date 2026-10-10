---
id: a-covered-arc-lying-on-a-carrier-places-its-ends-with-no-margin
kind: issue
title: A covered arc lying on a partner's carrier places its ends with no margin, so a surviving record of one would refuse
status: open
opened: 2026-10-10
---


## What

The reduction's curved edge-face lane has a covered arm for an arc
that lies on the partner face's carrier (`crates/topo/src/boolean/reduce.rs`,
the `covered && !on_carrier` block that asks `wall_crossing` for
`SpanVerdict::LiesOn` and `parents_distinct_from`). It hands the arc's
two ends to `lying_on` as `ends: [(u, pu, None), (v, pv, None)]`, so an
end placed `In` the curved face is pushed through
`vertex_on_curved_face_at` with no margin.

The arc was decided on the carrier: `CircleRoots::OnSurface` is a Zero
of `constant_residual_roots` (`circle_roots.rs`) or of the torus
meridian deviation (`circle_torus.rs`). So each end is on the carrier
by that decision. Since stage 4 B2 (`contact-records-cite-their-decision`), a
surviving record whose decisions all carry no margin refuses
`ClassificationInvariant` in `ops::cite_rows` ("a contact record
surviving into the result cites no decision"). A record minted on this
lane that survives would therefore refuse where the op built before B2.

The uncovered `(Zero, Zero)` arm of the same lane hands the ends' own
`bool_vertex_face_side` margins (`m1`, `m2`), so it is not affected.

## Reach

Not reached. The B2 dual review (R1) looked for a surviving case and
found none: a cylinder's rim kissing a tilted block refuses
`CurvedRestUnrecorded` first, and a peg-in-hole rim collapses as an
incidence. No corpus, test, tour or Python scene reaches the refusal.

## The fix

Carry the `OnSurface` decision's margin out of the circle root doors
(`CircleRoots::OnSurface` and the line door's `Constant`), and hand it
to `lying_on` as each end's margin. That margin is the decision that put
the arc, and so its ends, on the carrier. About 36 sites name
`OnSurface`.
