---
id: closure-guard-decides-a-sum-not-a-cover
kind: issue
title: The rim-only closure guard decides a span sum, not a cover: a half rim stated twice measures a whole cap
status: closed
closed: 2026-09-21
opened: 2026-09-20
---

`require_rim_only_closed` is the guard the sphere-pole-side unit added
to close its own MAJOR — an area answered for a rim that does not close.
It decides a **span SUM**, not a **cover**, so the defect it exists to
close is still reachable through the public `curved_face`.

## The mechanism

```rust
fn require_rim_only_closed<T: Decide>(du: T, arm: T, band: Band) -> Result<(), PropsError> {
    require_zero("props_rim_only_closed", Margin::levered(du - T::tau(), arm), band)
}
```

`du` comes from `du_of_rims`, which is documented as, and is, a
**per-group sum of arc spans** (grouped by rim level and traversal
direction). So every multiset of same-level, same-direction arcs whose
spans sum to a turn passes the guard, whatever part of the circle those
arcs actually cover.

## Executed, through the public door

Measured by the blinded reviewer of PR #2924 (ordinal 2408) on the cone
arm, and reproduced in this item's terms:

| boundary | covers | guard | area answered | truth |
|---|---|---|---|---|
| the same HALF rim stated twice (`u in [0, pi]`, same direction, twice) | half the circle | passes | `2.221441469079183e-4` | half that |
| two arcs `[0, 0.75*tau]` + `[0.5*tau, 0.75*tau]` | three quarters | passes | the whole cap | three quarters |

The four shapes the curved-residues unit red-firsts (half a rim; a
quarter rim; a full rim stated twice; a full rim plus an extra half arc)
are **instances** of the non-covering class, not the class. Each fails
the sum test; these pass it.

## Both call sites, and one is shipped

`require_rim_only_closed` has two callers: the cone arm added by PR
#2924, and the sphere's at `sphere_rim_only_pole_level`, which is **on
main today** (PR #2741). So a rim-only sphere cap whose boundary is a
half-arc stated twice is measured as a full cap on main. One shared
function, so one fix covers both arms — and it has to, because fixing
only the cone leaves the shipped defect standing in the very function
being generalised.

## Why this is the third time, which is the part worth keeping

The same premise has now failed at three depths:

1. `docs/PROPS-SPHERE-POLE-SIDE-SPEC.md` **asserted** that `du_of_rims`
   already sums a full rim to `2*pi`, rather than requiring the unit to
   decide it.
2. The sphere unit inherited that assertion and shipped a MAJOR through
   it — a half rim measured half the cap.
3. The fix added this guard, which checks the **sum** — the same premise
   one level down, and the thing this item is about.

A fix that compares a differently-shaped scalar is a fourth instance.
What the guard needs to decide is that the group's arcs **tile** the
circle: sorted, contiguous, no gap and no overlap, totalling a turn —
with every comparison that decides it going through the funnel under its
own predicate name, like every other decide in that file.

## Disposition

Being fixed in the curved-residues fix pass (PR #2924), because that PR
is generalising this function and leaving a known live defect in it
would be wrong. If the cover check needs interval machinery past what a
fix pass carries — arc normalisation, wrap-around, tolerance on the
joins — the lane stops and reports and this becomes its own unit with
its own spec.

## Refs

Found by the blinded review at ordinal 2408 (`work/props/logs/curved-review-brief-r2.md`),
probe row `r2_the_closure_guard_is_a_sum_not_a_cover`. Verified
independently by the PROPS orchestrator against `origin/main` before
dispatch.
