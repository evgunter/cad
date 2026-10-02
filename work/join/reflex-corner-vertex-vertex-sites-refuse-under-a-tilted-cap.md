---
id: reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap
kind: issue
title: A vertex of the other operand on a 315-degree reflex corner under a tilted cap refuses in 326 of 720 probe cases (SeamOrientation, JoinDesync, UnpairedLooseEnds)
status: open
opened: 2026-10-02
priority: P0
cost: H
---

## What

Found in the PR 3770 fix pass, while looking for a witness of the
`bool_strut_order` reflex window that `boolean/ops.rs` "Known
limitations" describes.

`a` is the 315° reflex prism: `prism_z` over `(0,0) (2,2) (-2,2)
(-2,-2) (2,-2) (2,0)`, z∈[0,1]. `b` is a `prism_ops` prism, z∈(1,3),
described with `describe_as_intersections` and z-sheared by
`z' = z + sx·x + sy·y`, with sx, sy ∈ {−0.5, −0.25, 0, 0.25, 0.5}
(not both 0). Its bottom cap therefore passes through `a`'s reflex
corner `(0, 0, 1)`. Ten profiles put a vertex or an edge of `b` on
that corner:

- the four unit squares with a corner at the origin;
- three diamonds with a corner there;
- three profiles whose edge runs through it.

Each pair was run through ∩, ∖ and ∪ with `flush_declarations`.
**326 of 720 refuse**:

| refusal | count |
|---|---|
| `Join(UnpairedLooseEnds)` | 119 |
| `JoinDesync` ("every chord arc separates a loose scaffolding pair") | 99 |
| `SeamOrientation` | 87 |
| `Euler(FanStartMismatch)` | 12 |
| `RestZipUnsupported` | 9 |

Every profile refuses somewhere. One example: the unit square
`(0,0) (1,0) (1,1) (0,1)` with `(sx, sy) = (0.5, −0.25)` refuses
`SeamOrientation` under all three ops.

## Unmeasured

- The cause. The ops docs' reflex window for `bool_strut_order`
  (W > 3π/2, θ ∈ (π/2, W−π)) is a candidate. No refusal has been traced
  to it.
- Whether a convex corner under the same tilts behaves the same way.
- Whether any of these cases answered on main before PR 3770. That PR
  does not touch the vertex–vertex path (`boolean/insert.rs`).

The probe file is not committed. It is the loop above, in
`crates/topo/tests` style.

## Re-measured (JOIN-1 fix pass, PR 3790, 2026-10-02)

R1's rebuild of this probe (`crates/sweep/tests/join1_r1_probes.rs`
`join1_r1_reflex_battery`: twelve profiles — the four squares, four
diamonds and four profiles whose edge runs through the corner — the 24
shears, ∩, ∪ and `a ∖ b` with `flush_declarations`; 864 cases), release
build:

| outcome | main 0abf909cb | JOIN-1 fix-pass head |
|---|---|---|
| builds sound | 533 | 653 |
| `SeamOrientation` | 129 | 159 |
| `Join(UnpairedLooseEnds)` | 124 | 0 |
| `JoinDesync` "every chord arc separates a loose scaffolding pair" | 34 | 16 |
| `JoinDesync` "B senses agree at a matched pair" | 19 | 19 |
| `Euler(FanStartMismatch)` | 12 | 16 |
| `RestZipUnsupported` | 8 | 0 |
| a WRONG body | 5 | 1 |

So 326 refuse on main and 210 on the head. The one wrong body is the
`sqQ1` union at `(sx, sy) = (−0.5, 0.25)`, volume 16 against 15.979, the
same on main (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`);
the other four wrong on main now refuse `FanStartMismatch`. Where the
head's `SeamOrientation` rows came from is
`work/join/locus-matching-moves-frontier-refusals-to-join-desync`.
(`b ∖ a` with the same declarations refuses `InvalidDeclaration` or
`ContactContradicted` on both, the declarations being for the `(a, b)`
order; the battery's fourth op is left out of the table.)
