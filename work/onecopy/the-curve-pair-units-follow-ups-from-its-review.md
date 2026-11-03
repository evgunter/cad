---
id: the-curve-pair-units-follow-ups-from-its-review
kind: issue
title: PR 6327 follow-ups: an edge's end vertices spelled beside query::edge_ends, the split point branch-selected twice, read_boundary's lone-vertex filter wider than its reason
status: open
opened: 2026-11-02
priority: P1
cost: E
refs: [6327]
---

From the delta review of PR 6327 (`boundary-meets-circle-only-at-reads-curve-meetings-itself`, `point-on-circle-is-a-second-point-on-carrier-reading`), recorded at merge.

1. **An edge's end vertices, spelled twice.** `carrier_cross::FaceBoundary`'s `BoundaryEdge::ends` derives an edge's two vertices by hand (`linked(..).start` of `he_plus` and `he_minus`, moved from `reduce.rs`). `query::edge_ends` already does it through `he_plus`/`next`, and the new spelling now sits in the boundary pass's shared input type. Read `query::edge_ends`.
2. **The split point's parameter, branch-selected twice.** On a circle, `reduce::split_other_at_point` takes `foot.t()`, which `angle_near(.., mid)` already placed near the middle, then re-branches it with `Span::new(..).branch(..)` (a `curve_pair_span_branch` compare). Every other carrier goes through `point_param`, so one parameter has two routes. Choose one: the foot's own placement, or the span's branch, for every kind.
3. **`read_boundary`'s lone-vertex filter is wider than its reason.** It answers `None` on any face with a lone vertex (`.filter(|b| b.lone.is_empty())`) because the pre-pass after a `Meets` refuses `EmptyLoop` (`contain.rs`'s `curved_boundary_containment`). A `Clear` answer needs no pre-pass, so the filter is more conservative than the reason requires. The gate `loop-boundary-discards.sh` cannot see a `.filter` on a field, and its register entry was deleted when the discard moved here, so this deferral is now audited only in prose.
4. **A type hole.** `carrier_cross::boundary_fault`'s `_ =>` arm only ever receives `Desync` (`BoundaryRefusal::of` puts only `Desync` and `Contradiction` into `Fault`). Name the arm.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
