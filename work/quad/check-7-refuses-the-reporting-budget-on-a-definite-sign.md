---
id: check-7-refuses-the-reporting-budget-on-a-definite-sign
kind: issue
title: check 7 refuses QuadratureBudget at the reporting target on a rational blade whose volume sign is not in doubt
status: open
opened: 2026-10-02
priority: P2
---

Met while giving the tour lily's blades lanceolate sections
(`demos/tour/src/lily.rs`, SHOW unit `lily-lanceolate-blade-sections`).
Pinned live there as walls 14 and 15 of `lily::wall_probes`.

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

## Where to look (hypothesis, not measured)

The refusal carries `rounds: 1` and the REPORTING target. If check 7's
sign walk reaches the per-face lane's after-round-0 budget exit before
its own sign has settled, the exit is answering "can the last round
reach 1024·ε" — the reporting question — on behalf of a caller that
only needed the sign. The teapot's spout (`demos/tour/src/teapot.rs`)
is the contrast: `mass_properties` refuses it after round 0 at
ε = 1e-12, and tier 3 admits it, its sign settled at the round the
chase stops on. On the lily's blades the same exit fires inside the
gate.

## Reproduce

`cargo test --release --bin demo-tour the_wall_list_still_stands`
in `demos/tour` runs walls 14 and 15. Wall 14 does NOT refuse at
ε = 1e-6 (the degree-3 swept blade certifies there), so the walls are
pinned at the default ε, which is the ε every tour lane runs.

## What it costs the scene

The lily's lofted blades (the long leaf, the three sepals) keep their
straight kite-and-rectangle sections, and its swept leaves are fitted
at degree 2 rather than the cubic the lofted blades use.
