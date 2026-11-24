---
id: an-extent-short-of-a-turn-enclosed-a-rounding-past-it-meets-its-own-copy
kind: issue
title: A face's extent short of a whole turn, enclosed a rounding past it, is read by sheets as meeting its own copy a turn back, so the wall is presented at no cell holding its start (one mirrored ring is 1.4e-15 from it)
status: open
opened: 2026-11-24
priority: P2
cost: M
refs: [a-generic-mirror-refuses-at-interval-a-body-that-holds-at-f64]
---

## Finding

Found by EMBED's lane `embed/clip-vertex` (2026-11-24) tracing why a
pinned refusal became a hold when main was merged into the landing
branch (`embed/wave3-land-next` `cc2a7327b8`), with prints at
`topo::injective::sheets` on two trees.

`sheets` (`crates/topo/src/injective.rs`) reads a face's extent on a
periodic axis from `face_extent` (the stored lift), and counts the
copies of it, a whole turn apart, that a cell meets by more than
`chart_slack` = `few_ulps(2π)` = `8 ε · 2π` = `1.116e-14`. Two copies or
more is `Unpresented::Local`: "the face enters the cell more than once".

A face whose extent is SHORT of a whole turn can still be counted
twice, because the extent is an enclosure and its high end can lie past
`2π` by the enclosure's own rounding. The copy a turn back then reaches
into any cell that starts at the extent's low end by that rounding, and
where the rounding is wider than the slack the cell meets two copies.
Every smaller cell holding the wall's start meets them too, so the
wall is presented at no depth there and the pair ends at the budget or
`Uncertified { Departure }`.

**Measured**, the generically mirrored near-full ring
(`self_intersection_proves_injectivity::near_full_ring`, a ring
revolved `τ − 10⁻³`, then `topo::mirror_body` across the plane through
`(0.1, −0.2, 0.4)` with normal `(1, 2, 3)`), `Interval`, its inner wall
`3v1`, root cell `(−5e-324, 6.283185307179587)` in the angle:

| tree | the wall's extent in the angle | past `2π` by | slack | copies met | reading |
|---|---|---|---|---|---|
| `0a1e9944db` (before main) | `(0.0009999999999799059, 6.283185307179604)` | `1.78e-14` | `1.116e-14` | 2 | `Local`; the pair `3v1 × 4v1` refuses `Uncertified { Edge(6v1), Departure }`, 143 of 164 traced cells at `sheets` |
| `cc2a7327b8` (main `17e78d9c90`) | `(0.0009999999999919904, 6.283185307179596)` | `9.8e-15` | `1.116e-14` | 1 | presented; the pair certifies at the root cell by the carrier arm, and the body holds |

So today's hold is `1.4e-15` inside the slack: an enclosure `15%` wider
at the extent's high end reads `Local` again. The true extent is a
milliradian short of a turn; the face has no points in the overlap.

## What it costs

A refusal on a valid body, at `Interval`, wherever a face's stored
extent on a periodic axis is within a rounding of a whole turn at one
end and its enclosure is wider than `1.1e-14` there. No wrong verdict:
missing the copy a turn back is right (the face is not there), and
counting it only refuses. **Count**: one body traced (above), and it
holds today. Not counted: which census bodies sit near the margin; a
print of `(extent high end − 2π)` against the slack over the census
would say.

Not the unmirrored ring: `a_ring_a_milliradian_short_of_a_full_turn_holds_across_its_gap`
holds at both scalars in three poses on both trees.

## What is needed

`sheets` deciding the copies by what the extent IS, not by its
enclosure's outer end: an extent whose width is decided less than a
turn meets a cell that lies within one turn at most once, whatever its
enclosure's ends round to. The gap is `10⁻³` here, eleven orders above
the rounding, so the width is decidable. Not a wider slack: that is the
same concession moved, and it loses a real second entry sooner.

## Unprobed

Which of main's changes tightened the stored extent (main through
`4ab816bbfb` already reads so; conic-symmetry, PR 7640, is ruled out by
a scratch merge without it; a unit direction held exactly, PR 7349, is
a guess from the numbers). Whether `f64` has the same shape: at `f64`
the extent is a point pair and the same ring refuses for another cause
(`work/pcert/a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64.md`).

## Valid or in band

Valid: the mirror image of a ring that holds unmirrored.

## Evidence

- `self_intersection_proves_injectivity::a_generic_mirror_refuses_the_ring_at_f64_and_the_block_at_interval`
  asserts the ring holds at `Interval`; it goes red if the margin is
  lost.
- `self_intersection_proves_injectivity::a_flap_through_the_mirrored_rings_wall_never_holds`:
  the negative beside it (`Violated` through each wall, both scalars).

