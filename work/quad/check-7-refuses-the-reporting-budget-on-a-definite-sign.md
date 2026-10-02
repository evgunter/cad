---
id: check-7-refuses-the-reporting-budget-on-a-definite-sign
kind: issue
title: check 7 refuses QuadratureBudget at the reporting target on a rational blade whose volume sign is not in doubt
status: open
opened: 2026-10-02
priority: P0
---

Met while giving the tour lily's blades lanceolate sections
(`demos/tour/src/lily.rs`, SHOW unit `lily-lanceolate-blade-sections`).
Pinned live there as walls 15 and 16 of `lily::wall_probes`.

## What

`crates/topo/src/validate.rs`'s module doc ("What check 7 costs, and
what it cannot refuse") says a valid solid cannot fail check 7 on
quadrature budget while its sign is definite: the reporting target
`1024·ε` is the caller's refusal, not the body's. The lily's
arc-section blades contradict it. `validate_geometric_certificate`
refuses them with

```
[VolumeUncomputable { solid: SolidKey(1v1), source: Face { face: FaceKey(3v1),
  source: QuadratureBudget { width_len: 2.667e-5, target_len: 1.024e-6, rounds: 1 } } }]
```

on a swept lens blade whose volume is 3.134e-3 m³ (Pappus closed form,
and the kernel's own certified reading of the degree-2 twin agrees to
4.4e-7). `target_len` is the reporting target at the default ε, and
`rounds: 1` is the after-round-0 budget exit (`props::quad`'s
`last_round_refuses`). Nothing about this body's sign is in doubt.

**The body.** A lens section — two arcs meeting at two margins
(`lily::Lance::outline`, through the public `arc_to(Via { .. })`) —
carried by `sweep_body` along a cubic interpolation of nine points of
a circular arc (length 1.25, turn 0.40 rad), the skin fitted at
`v_degree` 3. Every lateral wall is rational (the arcs) with cubic
interior knots along the path.

## The knob grid (default ε, target 1.024e-6)

Swept lens blade (the lily's `LEAF_B`):

| stations | v-degree | other | gate |
|---|---|---|---|
| 4 | 3 | | number |
| 5 | 3 | | number |
| 9 | 3 | | REFUSES, width 2.67e-5 |
| 17 | 3 | | REFUSES, width 1.00e-3 |
| 33 | 3 | | REFUSES, width 1.48e-3 |
| 5, 9, 17 | 2 | | number |
| 9 | 3 | each arc split in two (`tangent_arc_to`) | REFUSES, 2.86e-5 |
| 9 | 3 | at the origin, along x | REFUSES, 5.19e-6 |
| 9 | 3 | spine nearly straight (turn 1e-3 rad) | REFUSES, 1.35e-2 |
| 9 | 3 | short blade (len 0.3) | REFUSES, 3.11e-3 |

Lofted lens blade (the lily's long leaf, tapering and rolling):

| stations | v-degree | gate |
|---|---|---|
| 5, 9, 17 | 3 | REFUSES (6.3e-6 to 1.2e-5) |
| 9, 17 | 2 | REFUSES (5.4e-6, 6.0e-6) |
| 33 | 2 | admitted, bracket [1.92e-2, 5.04e-2] |
| 17 | 3, arcs split | REFUSES, 3.45e-6 |
| 9 | 2, arcs split | admitted, bracket [1.99e-2, 4.96e-2] |

The three sepals at 33 stations, degree 2: one admitted (bracket
[1.66e-3, 3.06e-3]), two refuse (1.5e-5).

Two things in that grid are worth the program's attention beside the
headline:

- **More stations is WORSE at degree 3**: 2.7e-5 at 9, 1.0e-3 at 17,
  1.5e-3 at 33. A finer skin of the same solid should not move the
  width up by two orders.
- **The admitted rows carry brackets about 2.5× wide** — consistent
  with the sign settling at round 0 on a coarse enclosure, and the
  refusing rows being the ones whose sign did not settle there.

## Why P0

A normal verb fails on normal geometry: a loft, or a cubic sweep, of
an arc section is refused tier 3 at the DEFAULT ε. And it breaks the
invariant `crates/topo/src/validate.rs`'s module doc states ("What
check 7 costs, and what it cannot refuse", ≈260-273).

## It is a regression of a retired refusal

`work/export/rational-patch-flux-quadrature-budget.md`'s premise
correction (2026-09-17) records this very refusal — `QuadratureBudget`
on a rational wall at check 7 — as RETIRED by TCOST-K3's check-7 SIGN
certificate (PR #1703). On these bodies it is back.

## The mechanism (traced by the PR 3838 review, measured)

- `topo::props::sign_walk` runs round 0 of each face lane. The lane
  then asks `last_round_refuses` (`geom-brep` `props/quad.rs` ≈2964,
  called from `rational_patch_face` ≈3463) whether the schedule's LAST
  round can reach the REPORTING target `1024·ε`. On these walls it
  cannot, and the lane answers `Open { refusal }`.
- `topo::props::face_flux` turns that answer into `open_at = None`:
  the face is never refined again. The walk's sum is left at its
  round-0 width, which straddles zero, and the walk ends
  `Uncomputable` — the `VolumeUncomputable { QuadratureBudget }` above.
- At ε = 1e-6 the same body settles positive at round 3. The sign
  needed three more rounds; the reporting-target question cut them off.

So the sign walk is stopped by a question only the reporting caller
asks.

## The class

The same round-0 exit sits in all three patch lanes `sign_walk`
reaches:

- `rational_patch_face` (`props/quad.rs` ≈3463) — this row's bodies;
- `nurbs_patch_face_rounds` (≈3856);
- `trimmed_patch_face_rounds` (≈5101).

Each answers the reporting question inside the sign walk.

## A second consequence: tighter ε reads worse

The exit also freezes `measure()`'s fallback BRACKET at round 0. The
lily's degree-2 swept leaves pass the gate at every ε, but at
ε = 1e-12 the bracket `measure()` hands back is about 1.5% wide —
roughly 100× wider than the default-ε number's pad. Tightening ε buys
a worse answer.

## Position sensitivity

The cubic swept leaf's refusal width grows with its distance from the
origin: 1.9e-5, 2.7e-5, 2.1e-4, 2.8e-3 from the origin out to 10 m,
and at 10 m it refuses even at ε = 1e-6. That is the shape
`work/quad/quadrature-interval-floor-grows-with-the-body-past-the-band.md`
records for the interval floor; the two want reading together.

## Reproduce

`cargo test --release --bin demo-tour the_wall_list_still_stands`
in `demos/tour` runs walls 15 and 16. Neither refuses at
ε = 1e-6 (both blades pass the gate there), so the walls are
pinned at the default ε and tighter, and assert the pass at 1e-6.

## What it costs the scene

The lily's lofted blades (the long leaf, the three sepals) keep their
straight kite-and-rectangle sections, and its swept leaves are fitted
at degree 2 rather than the cubic the lofted blades use.
