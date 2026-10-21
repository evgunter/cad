---
id: check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm
kind: issue
title: Check 9's lines_meet escalates an in-band side before reading a collinear pair, an order profile::seg dropped; unreachable through validate today
status: open
opened: 2026-10-21
priority: P3
cost: E
refs: [5542, check-9-lines-meet-reads-each-end-against-the-other-line-before-the-overlap]
---

## What

Found in the delta review of PR 5542 (CPAIR's one-line PR 3), which
moved `profile::seg::line_line` onto check 9's shape and then off it at
one point. `topo::validate`'s `lines_meet` (`crates/topo/src/validate.rs`)
returns `EdgePair::Unsure` on the first `Err` among its four
`ring_outer_meet_side` readings before it asks whether either pair reads
collinear (`collinear(&on_a)`, `collinear(&on_b)`). So when both of one
edge's ends lie within the band's zero of the other's carrier and one of
the other edge's ends stands in band of the first's carrier, it reports
`Unsure` where the overlap along the carrier would decide.

**At the function level it reproduces.** At ε `10⁻⁹`, K 10, with
`a = (0,0,0)→(1,0,0)` and `b = (1,0,0)→(x, y, 0)`, `lines_meet` returns
`Unsure` on `ring_outer_meet_side` for `(x, y)` = `(1.1, 0.5ε)`,
`(1.2, 0.5ε)` and `(1.3, 0.9ε)`. `b` lies within the band's zero of
`a`'s carrier, and `a`'s far end stands `len_a/len_b` times as far off
`b`'s, in band. These are the witnesses PR 5542 fixed in `seg`
(`rejections::a_short_segment_continuing_a_long_one_straight_within_eps_is_simple`).

**Through validate it does not reach, by argument (not built as a body).**
Check 9 compares a ring with its face's outer loop. Arm 5
(`segments_meet` → `lines_meet`) runs only after arms 1–3 have cleared
every vertex of each loop against the other. Take a collinear pair
whose gap does not read apart, so their projections overlap or touch
within the band:
- an end of one lies within the other's span on its carrier, so it is
  a vertex standing on the other loop;
- or the two spans touch within the band at an end.

Either way arms 1–3 read a vertex on the other loop, as a touch or in
band, and do not clear the pair. The witnesses above share a vertex,
which arms 1–3 catch outright. `seg` meets the case because its pair
pass reads adjacent segments of one loop through `line_line`; check 9
never reads adjacent edges.

## Proposed shape

The same order as PR 5542's `line_line`: read `collinear(&on_a)` and
`collinear(&on_b)` before propagating any side's `Err`. Both of one
edge's ends within the band's zero of the other's carrier put the whole
edge within it, so the overlap along that carrier decides the pair.
Land it so that the two doors stay one reading; PR 5542 left check 9's
order alone because nothing reaches it today.
