---
id: a-window-end-is-metered-at-a-pointwise-speed-by-hand
kind: issue
title: curve_face's undecided_over meters a run against a window's end by hand, at the locus's pointwise speed, unseen by the span-end gate
status: open
opened: 2026-11-16
priority: P3
cost: E
---

Filed from PR 7027's review (F3).

`crates/geom-brep/src/curve_face.rs:347-356`, `Boundary::undecided_over`, decides whether a run of the span lies definitely past an undecided window (`curve_face_window`). It reads `Margin::of(gap * speed)` on the gaps `u.window.0 − b` and `a − u.window.1`. These are a parameter's gap from a window's end, metered at one `speed` the caller passes. That caller is `crates/topo/src/boolean/mod.rs` (~`:8684`): `geom::carrier_speed::speed_at_any(locus, mid_param(overlap))`, the locus's **pointwise** speed at the overlap's middle. That speed bounds no arc over the gap, the class PR 6767's `Span::end_gap` closed for span ends.

`scripts/gates/span-end-gap-sites.sh` does not see it: the window's ends are not named as a span's (`t0`, `t1`), and the speed is bound to a parameter.

**Done when**: the gap is metered over itself (`geom::carrier_speed::speed_between`), at the bound that makes the reading sound. It reads `Zero` as well as `Positive` as "past the window", and the window is already widened by the escalation. So the site must say which bound, the inf or the sup over the gap, makes "a zero gap misses the member" hold, or why the pointwise speed bounds it. The gate either finds the site or names it in its blind spot.
