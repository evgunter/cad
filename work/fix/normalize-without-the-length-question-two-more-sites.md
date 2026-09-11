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
half of the direction family" and cite ONE instance,
`editor-core`'s `clearance::chart_frame`. This is a second, in a
`pub` `geom-brep` door. Whether the half stays declined is the
question; what is wrong today is that the doc claims one site and
there are at least two.

## Fences

`chart_region.rs` is S-BOOL's and CURVED's (`work/topo/program.md`'s
`keep_out`). `crates/geom-brep/*` is code-quality's geom-brep seam.
Neither is FIX ground; this row is filed here because FIX's sweep
found it, and it is the orchestrator's to route.
