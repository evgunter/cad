---
id: corner-config-recourse-and-policy-assert-a-default-for-any-tag
kind: issue
title: blend: CornerConfig's recourse and policy tables assert an answer for any tag added later
status: closed
opened: 2026-09-17
priority: P1
cost: D
closed: 2026-10-01
pr: 3690
branch: band/recourse-tables-decide-per-tag
---


## Finding

`escalation-recourse-dispatch-has-three-homes` (BLEND-15) fixed the two
Displays that route a recourse by predicate NAME. The same defect, keyed
on a SITE ENUM instead, survives two screens away.

`CornerConfig::recourse` (`crates/sweep/src/blend/mod.rs:523-528`):

```
    match self {
        Self::SeamVertex => FILLET3_SEAM_VERTEX_RECOURSE,
        _ => FILLET3_CORNER_RECOURSE,
    }
```

and `CornerConfig::policy` (`:504-516`) has the same shape. A variant
added to `CornerConfig` tomorrow gets the corner recourse and whatever
policy the `_` arm names, asserted, with nothing red: the author is not
obliged to decide, and the refusal a caller reads asserts advice nobody
paired with the tag. That is the item's own failure mode — the default
that speaks for a name (here a tag) nobody looked at.

Unlike the name-keyed tables, the compiler CAN close this one: an
exhaustive match over the enum makes a new variant a build error, which
is a stronger guard than a roster row. The `_` arm is what gives it up.

Not this unit: BLEND-15's spec fences it to the two predicate-name
tables, and `CornerConfig`'s tags are the carve verbs' vocabulary, not
the escalation's.

Found by unit 15's v6 review (R2 NOTE-3). Filed on `work/blend/`
because `work/carve/` does not exist on main at the time of filing.

## Re-homed at BLEND's exit (2026-09-17)

Filed by BLEND unit 15's fix pass after the cut branch was drawn, so it
missed the cut; moved here at BLEND's exit walk. `crates/sweep/src/blend/mod.rs`
is CARVE's ground.

## Closed (band/recourse-tables-decide-per-tag)

Both maps are exhaustive matches over the tag, so a new variant is a
build error in each. Deciding every tag moved one answer:
`DependentNormals` is the decided-Zero arm of
`fillet3_corner_independence`, so its recourse is that decision's lever
(`FILLET3_CORNER_INDEPENDENCE_RECOURSE`, shared with the in-band arm, D4
¶1 (iv)), not the corner sentence whose configuration clause it already
satisfies. Its policy was left as it stood; that it names a corner
patch the tag says it cannot have is
`dependent-normals-names-a-run-out-policy-the-tag-says-cannot-help`.
A corner support whose outward normal did not resolve (a face or stored
surface key that does not resolve) used to land on `DependentNormals`
through a zero normal; it now refuses as `BodyNotIntact` at that face,
so the tag is minted only by the independence decision.
