---
id: a-move-past-a-fitted-solids-reach-refuses-op-not-the-collapse
kind: issue
title: a shell thicker than a solid with a fitted face reaches refuses Op{edge: None, RechartFalsifies{IntervalNotForward}}, which names no collapse
status: open
opened: 2026-10-10
priority: P3
cost: M
---


Found by the dual review of PR 4524 (unit 18).

## Repro

`topo::shell(&finished(box_with_bumped_cap(0.0)), 0.6, Tol::witness())`. The fixture is `crates/sweep/tests/common/approx.rs`, `box_with_bumped_cap`.
- The box is `2 × 2 × 1`, so a wall of 0.6 runs the cavity's floor and ceiling past each other.
- `shell` refuses `Face { Op { edge: None, RechartFalsifies { IntervalNotForward } } }`.
- That names no edge and no collapse. An all-plane box of the same size refuses `WallClearance` instead.

## Where

The general door (`crates/topo/src/offset_general.rs`, `replace_face::offset_charts_staged`) plans the moved cap's rims as plane × fit sections. Their ends are read where the corners land, so a span that the move turns backwards reaches the re-chart (`move_points_then_rechart`). That is where the attach layer's certificate refuses it.
- The plane-only path never gets here: `shell`'s clearance gate refuses first.
- The re-anchor path is not the cause either: it reads `IntervalNotForward` as `ReanchorCollapse` only for edges that it re-anchors itself.

The review places this before PR 4524 at the door level. A single-chart move of the cap by −0.6 against its held sides does not reach it. That move refuses earlier, `ReanchorPastCarrierEnd`, on the u-moving iso image (`the-iso-row-arm-reads-a-u-moving-chart-image-as-a-u-row`). The `RechartFalsifies` names its edge inside the payload, `RechartFalsifies { edge }`, while the `Op` beside it carries `edge: None`.

## Owed

Either the clearance gate learns a fitted face's reach, or the door reads a derived section's span that runs backwards as the collapse it is, naming the edge and the offset. Then pin the repro.
