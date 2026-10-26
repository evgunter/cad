---
id: boolean-conjunctions-of-two-readings-escalate-on-the-first-where-the-second-decides-false
kind: issue
title: The boolean's two-reading conjunctions (a? && b?) refuse on the first reading's escalation where the second decides false
status: open
opened: 2026-10-13
priority: P2
cost: M
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
`census::in_both` states: read both, a decided `false` on
either side drops the other's escalation.

## More candidates (unverified)

Found by the review of PR 5006. The pattern above has blind spots: it
misses conjunctions whose operands start with `!`, disjunctions (`||`,
where a decided `true` on either side settles it), and conjunctions
that wrap across lines. A second look found these. None of them is
checked yet for whether the order of the operands can change the answer:

- `offset_axial::` the carrier-hold test in the edge re-read,
  `!chart_moves(ca.distance, band)? && !chart_moves(cb.distance, band)?`
  (`offset_axial.rs`).
- `offset_axial::` the coaxial screens,
  `!sine(parallel(..), ..)? || !on_axis(..)?` (`offset_axial.rs`, four
  arms: cylinder, cone, sphere, torus).
- `editor_core::names::emit_topo` `walk_cover`,
  `!ahead(hi, start)? || !ahead(end, lo)?` and
  `!ahead(lo, start)? && !ahead(end, hi)?`, plus `on_line(..)? &&
  on_line(..)?` (twice) in the line-overlap reads. This file belongs to
  editor-core, not the boolean; it is listed here only as part of the
  same class.
- `boolean::solid_contain` the wall-section seat test,
  `zero(.., tilt)? || !zero(.., seat)?`, and the sphere iso-rim test,
  `!zero(..)? || !zero(..)? || !zero(..)?`. Both span several lines
  (`solid_contain.rs`).

None of these searches can see a conjunction that is built up across
statements, such as `let a = x?; if !a { return }; let b = y?;`. That
shape only shows up by reading the code.

## Slate triage (2026-10-26, CLEAVE)

- Banded P2: a conjunction escalates on its first reading where the second decides it: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
