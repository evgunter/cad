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

## Measured (reflex-corner lane, 2026-10-02)

The probe re-run on JOIN-1's head 0b6e39ca (release,
`join1_r1_reflex_battery`, ∩ ∪ `a ∖ b`, 864 poses): 663 sound or
rightly empty, 159 `SeamOrientation`, 16 `Euler(FanStartMismatch)`, 25
`JoinDesync` (19 "B senses agree at a matched pair", 6 "every chord arc
separates a loose scaffolding pair"), 1 wrong body. Instrumented by
where each pose first goes wrong, not by its refusal, the refusals are
two groups.

**The strut order, 159 poses** (every `SeamOrientation`; profiles
`sqQ1`, `sqQ2`, `dUp`, `dLeft`, `eBot`, `eLeft`, `eRight`). The vertex
pair's two germs both lie in `a`'s 315° top face, so `insert` mints a
dangling strut there, and `bool_strut_order` decides which of its
halves faces which germ: the germ nearer the splice corner's arrival
edge (`+x`), compared by `dot(germ, +x)`. Measured into the face, the
top face reaches 315° from `+x`, and the cosine orders two angles only
within one half-turn. At `sqQ2`'s germs (`+y`, 270° in; `−x`, 180° in)
it picks `+y` as the nearer; the mirror pose `dDown` (germs 135° and
45° in) never refuses. The mint records the facing as the halves'
senses, so the error first shows at the zip
(`SeamOrientation`). The subdivided top face's twin sectors decide
which poses: a strut in the `[45°, 202.5°]` twin always has a germ past
the half-turn, so `sqQ2` refuses at every shear that dips below `a`'s
top, while `sqQ3` (straddling 202.5°) and `sqQ4` and `dDown` (inside
`[202.5°, 360°]`) never do. This is the window the ops docs named.

**The four-germ vertex pair, 41 refusals and the wrong body** (every
other refusal, and nothing sound): the corner keeps four germs and B's
null edges run in A's germ order. Filed as
`work/join/four-germ-vertex-pairs-run-b-in-a-order` (P0/H) with its
measurement and a fix that waits on ZIP's
`a-flush-declared-reflex-union-ships-the-wrong-volume`.

The wrong body is ZIP's P0 row's pose (`sqQ1`, (−0.5, 0.25), ∪,
volume 16 against 15.979). The REST zip builds it after the join
refuses; its first step wrong is the four-germ insertion (both rows
carry the trace).

## Built (reflex-corner lane, 2026-10-02)

`insert::strut_order` places each germ in the half-turn it lies in,
then compares cosines within it. Before and after over the whole probe
(∩ ∪ `a ∖ b`, 864 poses):

| outcome | JOIN-1 head | this branch |
|---|---|---|
| sound, or rightly empty | 663 | 822 |
| `SeamOrientation` | 159 | 0 |
| `Euler(FanStartMismatch)` | 16 | 16 |
| `JoinDesync` "B senses agree at a matched pair" | 19 | 19 |
| `JoinDesync` "every chord arc separates …" | 6 | 6 |
| a wrong body | 1 | 1 |

The battery's `b ∖ a` op moves 17 more poses from `SeamOrientation` to
sound. No pose of any JOIN-1 battery (`join1_r1_battery`, `_seam_`,
`_declared_`, `_tube_`, `_bored_capsule_`, `_reflex_`; 92 k lines)
changes otherwise.
