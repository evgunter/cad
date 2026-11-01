---
id: covered-endpoint-arms-read-a-non-convex-touch-at-the-ends-only
kind: issue
title: The covered endpoint arms read a torus line or an arc at its ends only, though either can touch twice
status: open
opened: 2026-10-02
priority: P3
cost: M
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

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the arms sit under `if covered` in reduce::curved_face_arm, and the cover is the declared-tangency channel stage 4 retires. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E does not close the gap, and widens its reach. The covered arms are unchanged (`crates/topo/src/boolean/reduce.rs:1630`, `covered`; the circle rung's `Ok(Sign::Zero) if covered` at `:1853`; `(Zero, Zero) if covered` at `:2004`). The cover is now also the glue door's auto-declaration, Zero-decided pairs and witness-verified tangencies, declared or not (`crates/topo/src/boolean/glue.rs:40`). So a covered torus line or arc reaches these arms in undeclared scenes too. The fix's two shapes stand.

## The settle stage now reaches the circle arm too (CONTACTHOLD, 2026-10-19)

`contacthold/two-tubes-zero-band` defers a covered circle with no end
on the carrier and its interior unseen, as a line was, so the
fragments `settle_deferred` reads include arcs. That reach already
existed in the other operand order (the other direction's vertex
splitting the arc before this direction reads it): the fragments the
settle stage reads are the ones the swapped order's second direction
reads. Measured on that branch: refusing the circle branch's
ends-only `Recorded` reading outright fails 19 committed sweep rows
beyond the new ones (kissing discs, externally tangent walls, the
sphere-capped tube, the strut cover stacks), so the first fix shape
retires builds; the second, a touch set certified from the cover's own
locus, is the one that keeps them.

## A first fixture: a rod on a three-quarter puck (CONTACTHOLD, 2026-10-19)

The PR 5389 review's `probe_rod_on_three_quarter_puck`
(`crates/sweep/tests/review_5389_probes.rs`, branch
`contacthold/two-tubes-zero-band-review`): a puck revolved 1.5π and
spun −π/4, a rod `r = 0.25` lying along `x` on its top face `0.5·ε`
above it, the rod's seam off the contact ruling, the rod ending at
`T2 = (1, 0, 0)` on the puck's rim. The plane × cylinder cover admits
two touches of the rim circle with the ruling, `T1` mid-arc and `T2`.
On main both orders build ∪, record `T2` and not `T1`, and tier 3′
reports `CensusUnsupported`: the rim arc read at its ends only. PR
5389's deferral settles through the same ends-only reading, so it
shares the gap rather than adding to it.
