---
id: a-cones-parity-reference-is-off-its-sectors-plane-by-the-tie
kind: issue
title: A gathered cone's parity reference stands off its sector's plane through p by the tie, scaled out to q's distance
status: open
opened: 2026-11-02
priority: P3
cost: E
refs: [the-cone-takes-a-bounds-side-from-the-one-bound-side-reading]
---


Found by `link::cone_fuzz::a_given_side_never_contradicts_the_exact_oracle` while it was being written (ties up to `1.5·δ`, `δ` up to `100·Kε`, far past what the boolean builds).

`link::Cone::reads_given`'s parity reads `q` along the arc to a reference `w` on a sector `T`, taking the material as the side behind `T` at `w`; the argument needs `w` on `T`'s plane through `p`, so the arc meets that plane only at `w`. `w` is built from `T`'s rays relative to `p` (`(a − p) + (b − p)`, or the bisector) and carried out to `q`'s distance (`ArcReach::reference`). A gathered cone's rays are fixed at points of faces whose apex stands up to the tie off `p` (`Cone::gathered`; `sectors::line_fans`' opposite ray is `2·apex − l`), so a ray's point stands off `T`'s plane through `p` by up to twice the tie, and the reference by that much times `|q − p| / |c|`. Nothing checks `w`'s side of `T`. Witness (seed `0x7402f11c0fd69872` of the fuzz before its ties were held to `0.9·zero`, ε 1e-9): a line cone of two flanks a hair off a half-turn, tie `2.3e-7`; `q` reads `In` by the first reference and, where the slack refuses the first two references' crossings, `Out` by the third, whose `w` stands `1.35e-6` off `T`'s plane while `q` stands `−6.4e-7` off it, so the arc crosses `T` uncounted.

Not reachable at the ties the boolean builds as far as the suites and fuzzes show (a vertex pair decided one point stands within the zero band, `w`'s offset then within the slack the readings already widen by), but nothing states the bound. Owed: a reference read only where `w` reads on `T`'s plane over the reading (or projected onto it and checked inside `T`), or a stated bound on the offset; the witness pinned.
