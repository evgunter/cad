---
id: normalize-without-the-length-question-two-more-sites
kind: issue
title: two sites normalize a vector whose length no door asked to be finite — the shapes the director-doors sweep found and did not take
status: open
opened: 2026-09-11
---



## Where this came from

The sweep obligation of
`two-d-director-doors-skip-the-finiteness-question`, whose five doors
share ONE shape — a length whose SIGN is decided and whose vector is
then normalized. That unit closed all five. Its grep turned up two
sites that are NOT that shape and were deliberately not taken there;
they are here so the disclosure is scheduled rather than buried in a
merged PR body.

Neither is measured end to end. Both are argued from the arithmetic,
and that is exactly the state the parent unit's rows 3 and 5 were in
before they were executed — one of which then turned out to be a
different defect from the one filed. **Execute before fixing.**

## Site 1 — the decided quantity has the norm in its DENOMINATOR

`crates/topo/src/chart_region.rs:3101-3113`. The collinear lane decides

```
off_a = |perp_dot(r, qp)| / |r|      off_b = |perp_dot(s, pq)| / |s|
decide("chart_region_collinear_offset", Margin::of(max(off_a, off_b)))
```

and its `Sign::Zero` arm then normalizes `r` (`let rhat = r.normalize()`).
`|r|` is never asked to be a number. A chart polygon edge past
`Vec2::normalize`'s ~1e154 band makes `|r| = ∞`, so a finite
`perp_dot` divides to exactly 0, the max is 0, the offset decides
definitely Zero — and `rhat` is the zero vector, which the span
comparisons below then read. The rung above it
(`chart_region_parallel`, `Margin::over_lever(denom, min(|r|,|s|))`)
has the same denominator and reaches `Zero` the same way.

This is a REAL sibling of the parent class and not a member of it: the
parent's doors decide the length itself, so asking `is_finite_length`
of the decided quantity closes them. Here the decided quantity is a
ratio, and an infinite lever makes the margin SMALLER rather than
larger — the failure is a spurious Zero, not a spurious Positive.

## Site 2 — the second instance of the declined half

`crates/geom-brep/src/enters.rs:202-212`. `enters_material` decides
the caller-named `arm`, then normalizes `dir`, whose own length is
never decided or asked about at all. A `dir` past the overflow band
normalizes to zero, the levered margin is 0, and the door answers
`Tangent` — definite, and about nothing.

`geom_core::is_finite_length`'s docs name this shape as "the declined
half of the direction family". They cited two instances,
`editor-core`'s `clearance::chart_frame` and this one; **the first is
gone** — PROPS's sign-hull unit retired the clearance engine's planar
re-chart with both its doors, because the stored frame refines at the
equator now, so the doc's list is this site alone. Whether the half
stays declined is still the question; what is left to answer is a
`pub` `geom-brep` door that normalizes a `dir` nobody asked to be
finite.

## Fences

`chart_region.rs` is S-BOOL's and CURVED's (`work/topo/program.md`'s
`keep_out`). `crates/geom-brep/*` is code-quality's geom-brep seam.
Neither is FIX ground; this row is filed here because FIX's sweep
found it, and it is the orchestrator's to route.

## AMENDED 2026-09-11 — site 1 was EXECUTED, and the mechanism above is wrong

A reviewer ran site 1 rather than reading it. **The defect is real and
is worse than this file claimed. The mechanism this file names is not
the one that fires, and the repair it implies would not close the
row.** Both halves are recorded because the second is the more useful.

**What reproduces.** Through `proper_crossings` with two-point loops:
two collinear overlapping segments at finite scale give
`Err(TouchingBoundary)`; the same configuration scaled by 1e199 gives
**`Ok(0 crossings)`**. A touching boundary silently lost — the class,
and worse than a mis-named refusal.

**What actually fires.** Not the spurious `Zero` predicted above. The
offset rung decides a **correct** `Zero`, and then `rhat =
r.normalize()` is `(0, 0)`, which drives `s0 = s1 = 0`, so `overlap =
0`, so `chart_region_collinear_overlap` decides `Zero` and the lane
`continue`s past the crossing. The spurious-`Zero`-from-an-infinite-
lever path was constructible only for genuinely SEPARATED parallel
edges, where the outcome happens to be right anyway; transverse pairs
with both norms infinite escalate at `chart_region_parallel` on a NaN
`∞/∞`.

**The operative consequence: a fix at the offset rung would not close
this row.** The repair belongs where the collapsed `rhat` is USED, not
where the offset is decided. Anyone taking this unit should start
from the reviewer's reproduction, not from the argument above.

**The generalisation, because this is twice in one day on this slate.**
An argued reproduction is two claims — *a defect exists* and *this
mechanism produces it* — and the first survives far more often than
the second. The other instance is the parent unit's own row 3, filed
as "both arms normalize to zero after a definite-positive decision"
and measured as something narrower arriving by a different route. This
is an argument for EXECUTING before filing, not for distrusting filed
rows: both rows named real defects.

## What the sweep that found these two actually covered

The parent unit's pattern (a norm or `Margin` within 12 lines above a
`.normalize()`) returned **58 hits**, all dispositioned. A
shape-blind grep finds **186** `.normalize()` call sites in non-test
source across `crates/`, `demos/` and `tools/`. The difference is
exactly the four blind spots that PR disclosed — two-step norms,
decide and normalize more than 12 lines apart or split across
functions, normalization inside a helper, trait/macro expansion — so
the disclosure is honest and the gap is accounted for.

**"58 hits, all dispositioned" is not coverage of the family**, and
this row exists partly to stop that reading hardening. `topo` (78
sites) and `geom-brep` (21) are where the next look belongs;
`geom-brep/src/enters.rs` was found there, which is why this row has
two sites and not one.
