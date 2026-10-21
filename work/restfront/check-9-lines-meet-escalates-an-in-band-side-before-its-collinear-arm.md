---
id: check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm
kind: issue
title: "Check 9's lines_meet escalates an in-band side before reading a collinear pair, an order profile::seg dropped; reachable only in a sliver, with the same verdict"
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

**Through validate it is reachable only in a sliver, with the same
verdict.** Check 9 compares a ring with its face's outer loop, and arm 5
(`segments_meet` → `lines_meet`) runs only after arms 1–3 have cleared
every vertex of each loop against the other. A collinear pair whose
gap does not read apart has a vertex within the band of the other loop
almost everywhere, which arms 1–3 read first. PR 5542's delta review
found the sliver that passes them, through `lamina_with_ring`:
- outer `(0,0) (4,0) (4,−2) (10,−2) (10,10) (0,10)`;
- ring `(4+g, h₀ε) (5, h₁ε) (5,1) (4+g,1)`;
- at `g = 1.000001·Kε`, check 9 escalates `ring_outer_meet_side`.

With collinear-first it would escalate `ring_outer_meet_overlap`
instead. The verdict is the same, an escalation, under a different
predicate: the gap screen and the overlap's upper bound read the same
`extent + noise`, so a pair the gap did not read apart cannot read
definitely disjoint in the overlap. The defect is the two doors' order
differing, not a wrong verdict.

## Proposed shape

The same order as PR 5542's `line_line`: read `collinear(&on_a)` and
`collinear(&on_b)` before propagating any side's `Err`. Both of one
edge's ends within the band's zero of the other's carrier put the whole
edge within it, so the overlap along that carrier decides the pair.
Land it so that the two doors stay one reading. PR 5542 left check 9's
order alone because the only reach is that sliver, with the same verdict.
