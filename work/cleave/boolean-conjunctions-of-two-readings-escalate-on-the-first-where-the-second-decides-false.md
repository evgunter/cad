---
id: boolean-conjunctions-of-two-readings-escalate-on-the-first-where-the-second-decides-false
kind: issue
title: The boolean's two-reading conjunctions (a? && b?) refuse on the first reading's escalation where the second decides false
status: open
opened: 2026-10-13
---



Found by the §5 sweep of CONTACTHOLD's one-tangent-interiors-reading
lane, which fixed the same shape in `boolean::line_through_both_faces`
(each station reads both faces; either deciding out settles it).
Pattern swept: `\)\? && [a-z_:]+\(.*\)\?` over `crates/topo/src`.

Each hit below conjoins two fallible readings with `?`, so an
escalation on the left returns `Err` even where the right reading
would decide `false` and settle the conjunction; the answer then
depends on which reading is written first.

- `boolean::rim_wedge` `seam_cover_ride`'s `inside(s - lo)? &&
  inside(hi - s)?` (`rim_wedge.rs`, the `rides` loop).
- `boolean::insert` the two tangent-germ side tests
  `within(sa, ±d, ..)? && within(sb, ±d, ..)?` (`insert.rs`, two
  places, `plus`/`minus`).
- `boolean::sectors` `parallel_same(a.start, ..)? &&
  parallel_same(a.end, ..)?` (`straight`/`crossed`) and the
  non-coplanar sector hit `within(sa, ±d, ..)? && within(sb, ±d, ..)?`
  (`sectors.rs`).

Not measured on a fixture; read from the code. The fix is the rule
`census::curved::in_both` states: read both, a decided `false` on
either side drops the other's escalation.
