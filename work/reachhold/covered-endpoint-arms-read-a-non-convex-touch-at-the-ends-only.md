---
id: covered-endpoint-arms-read-a-non-convex-touch-at-the-ends-only
kind: issue
title: The covered endpoint arms read a torus line or an arc at its ends only, though either can touch twice
status: parked
opened: 2026-10-02
priority: P3
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---

Found by the dual review of PR 3846 (r1 NOTE 3); analysis on that PR's
branch after its merge of `origin/main`. Not built: no fixture yet.

## The gap

`reduce::curved_face_arm` takes a COVERED pair with an end on the
carrier into an endpoint posture that records the on-carrier ends and
never looks inside the span:

- the line arms `(Zero, Positive)`, `(Positive, Zero)` and
  `(Zero, Zero)` under `if covered`;
- the circle rung's covered branch (`Ok(Sign::Zero) if covered`),
  whose ends feed `Placement::declared`.

That is sound when the residual along the edge is CONVEX: the cover puts
the edge in one closed side of the carrier, so the residual is
nonnegative, and a nonnegative convex function's zero set is a single
point or interval, which an on-carrier end already accounts for. A line
against a cylinder wall or a sphere has that.

A line against a TORUS does not (its residual carries `−2Rρ`, concave
in the line parameter), and neither does any CIRCLE (against a sphere
it is a first harmonic; against a wall or a torus a degree-2
trigonometric polynomial). A covered torus line or arc can touch the
carrier at an end AND again strictly inside the span. The arm records
the end and reports the pair done, and the interior touch is never
recorded, so no event is minted for it.

## Reach

- On `origin/main` before PR 3846: reached whenever the other operand's
  vertex put one touch at a fragment's end.
- PR 3846 scopes its deferral to a line against a wall or a sphere,
  precisely so that settling routes no torus line or arc into these
  arms. The pre-existing reach stands.

## What a fix needs

Either refuse typed in these arms for the non-convex kinds unless the
span's interior is certified touch-free, or certify the touch set from
the cover's own structure (a verified `Tangent`'s witness locus, a
structural tangency's edge). The first may retire builds that rely on
these arms today. Arcs lying ON a continued carrier take the
carrier-identity rung, a different case, but the posture is shared. So
the fix starts by measuring which committed rows reach the arms with a
torus line or a non-coincident arc.
