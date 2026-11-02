---
id: a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex
kind: issue
title: A near-graze conic point is summed from its far centre, so its rounding is ~u·ρ; evaluated from the arc's vertex it is ~u·ρ·θ² and certifies the near-gap poses PR 6349 now refuses
status: open
opened: 2026-11-02
priority: P1
cost: M
refs: [6349, 6329]
---

From the delta review of PR 6349 (the conic frame unit), 2026-11-02.

## What

PR 6349 made every reader evaluate a circle or ellipse point the carrier's own way: `c + (u·(a cos) + v·(b sin))` (circle `c + (u cos + v sin)·r`). The residual mirrors' running bound now follows that chain faithfully, so the walk and the meter read one number. At a near-graze pose (θ ≈ 0, |c| ≈ ρ, the point near the origin) the in-plane offset is rounded at |E| ≈ ρ before it cancels against `c`, giving a per-coordinate bound of about 5u·ρ, against main's left-associated (off-carrier) chain at about 4u·ρ. Near-gap certified counts fell, measured against the merge base:
- ellipse k = 1.25, 0.999, 1.001: 17→7;
- ellipse k = 0.8: 7→1;
- circles: 17→7.

Every lost answer refuses typed (`Meter`) and every certified root is true. `the_near_gap_families_keep_their_certified_answers` floors the counts.

## The fix (well-conditioned, then treat as reals)

Evaluate relative to the graze vertex:

`P − o = (c + u·a − o) + u·a·(cos θ − 1) + v·b·sin θ`, with `cos θ − 1 = −2 sin²(θ/2)`.

The θ-dependent error drops from ~u·ρ to ~u·ρ·θ². The vertex term is one constant per carrier, not per-sample noise. Floor ~1–2 u·ρ. The walk and the meter both read this chain; the certificate is about the true curve, so `point_at`'s D9 bits are not what the near-gap reading needs.

This is the principle Ev is ruling on in PR 6329 ("make operations well-conditioned, then treat them as reals"). Settle it there first: whether the near-gap reading takes a re-posed evaluation, and where it lives (a `Conic` trait method beside `running_point`). Raise the floors to the new counts.
