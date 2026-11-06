---
id: geom-brep-keeps-eight-more-subdivision-walks-of-its-own
kind: issue
title: geom-brep keeps eight more hand-rolled subdivision walks beside geom_core::bisect_within, found by the sweep that ported the first eight
status: closed
opened: 2026-11-06
priority: P3
closed: 2026-11-06
pr: 6571
branch: onecopy/spline-walks
---


Found by `onecopy/spline-walks`. That branch ported `geom-brep-keeps-six-subdivision-walks-of-its-own`'s walks, plus `ssi/four`'s `row_crossing` and `covered` and `curve_pair/pose/window`'s `pieces`, onto `geom_core::bisect::bisect_within`. Its sweep matched `while let Some(..) = ...pop()` in `crates/geom-brep/src` and found eight more walks that keep their own stack:

Depth-first halving, the shape `bisect_within` serves. Each would port the way the first eight did: a wall counted in pieces read becomes a `Budget`; any other wall stays in the visit, with the read budget it implies.
- `pair_section/spline_implicit/sweep.rs` (the sweep's `stack`): `read > SSI_MAX_CELLS` → `Declined::Budget`, a cell budget.
- `pair_section/gap.rs` (`cells`): halving, with a floor refusal.
- `pair_section/ruling.rs` (`(lo, hi, depth)`): halving, with `Coverage` statistics.
- `pcurve_cache/projected.rs` (`parts`, `(sa, sb, depth)`): a `LEVER_DEPTH` cap and an `INCIDENCE_CUTS` wall.
- `pcurve_cache.rs` (`pending`, `((d, k), parent, clipped)`): a dyadic cover, counting `work`. Owned by `pcert`.

A different contract. Each needs a reason stated at its site, or a core walk of that shape:
- `pcurve_cache/spline_incidence.rs` (`queue`): a `BinaryHeap`, best first, not depth first.
- `chart_singularity.rs` (`pending`): roots read last first (the list is not reversed), and past `SAMPLED_BUDGET` it degrades (`over_budget`) rather than refusing.

Blind spot: the pattern matches only `while let … pop()`. A second pass looked for `let Some(..) = ….pop() else` and found nothing. It also looked for a `depth: u32|usize` parameter line in `crates/geom-brep/src`. Of those hits, `curve_pair/walk.rs`, `chart_singularity.rs` and `spline_incidence.rs` are piece fields, not recursion. It found one recursive walk:
- `pcurve_cache/projected.rs` `refine` (`depth: u32`, recursing on halves). It is depth-first halving, and the eighth walk here.

That pass cannot see a recursion whose depth is carried another way (a width, or a level folded into a struct).

## Outcome (`onecopy/spline-walks`)

Six walks are ported onto `geom_core::bisect::bisect_within`, each reading the same pieces in the same order and refusing at the same point:
- `pair_section/spline_implicit/sweep.rs`: `Budget::new(SSI_MAX_CELLS)`.
- `pair_section/gap.rs`: `GAP_CELLS`, in the visit.
- `pair_section/ruling.rs`: `COVERAGE_CELLS` / `COVERAGE_DEPTH`, in the visit; its roots last first, as read before.
- `pcurve_cache/projected.rs`: the `parts` walk (`LEVER_DEPTH`) and the recursive `refine` (`MAX_REFINE`), both depth caps in the visit.
- `chart_singularity.rs`. **Correction:** this walk is bisect-shaped. Its last-first order is only the order of its roots and halves, which the port passes unchanged. Its degrade-past-budget is a halving wall in the visit, which settles a piece rather than halving it.

Two stay apart, each with its reason stated at the site:
- `pcurve_cache/spline_incidence.rs`: best first from a `BinaryHeap`, not depth first.
- `pcurve_cache.rs`'s dyadic cover. **Correction:** the list above called it bisect-shaped, and it is not. A span is replaced by one span (its re-read clipped to the window) or by the one or two `halves` the window keeps, where a bisect step halves into exactly two.
