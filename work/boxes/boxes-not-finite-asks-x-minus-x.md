---
id: boxes-not-finite-asks-x-minus-x
kind: issue
title: not_finite asks (x − x).is_poison(), which an unbounded Interval enclosure passes ([−∞,∞] is not poison), so a torus chart window end at infinity reads finite
status: open
opened: 2026-10-13
priority: P3
cost: E
---


`not_finite` (`crates/topo/src/boolean/boxes.rs:997-999`) reads
`(x − x).is_poison()`. Its doc says it is "the one expression that is
zero for every finite value and poison for `±∞` … in a trait with no
ordering and no `is_finite`". At `Interval`, an unbounded enclosure such
as `[MAX, ∞]`, `[0, ∞]` or `[−∞, ∞]` minus itself is `[−∞, ∞]`, which is
not poison. A scratch row on SYM-17's branch showed this. So the torus
chart window's ends read finite when they run to infinity.

The trait now has the question: `Real::is_bounded`
(`crates/geom-core/src/real.rs:385`, PR #4197) is a required method that
decides both endpoints at an enclosure. SYM-17 introduced it to fix the
same flaw at `RigidMotion::check_rigid`'s `0·tᵢ` probe (MINOR-1 on
`sym/17-review-delta`). The fix is likely `!x.is_bounded()`, with the
doc's premise ("no `is_finite`") restated.

Not yet shown reachable end to end: it needs a window end whose
enclosure is unbounded but not poison. Found by SYM-17's final pass, in
the sweep for `x − x` / `0·x` finiteness spellings.
