---
id: metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup
kind: issue
title: Margin::metered's doc promises a certified speed LOWER bound; three sites push a certified UPPER bound through it
status: open
opened: 2026-09-12
---


## Finding

`crates/geom-core/src/predicate.rs`, `Margin::metered(span, rate)`: the
doc says `rate` is "a certified speed lower bound, a chart's
`param_rate`". Three shipped consumers pass a certified SUP bound —
`geom_brep::pcurve_cache::trim_containment` (via `chart_arms_at` /
`chart_stretch_sup`), the `pcurve_iso_*` slack meters (via
`nurbs_stretch_bounds`), and the v-channel of
`topo::pcurves::pcurve_loop_continuity` (via `v_meter`). Each is
correct — those sites meter an overshoot or an escape, where a sup is
the safe side — and each says so in prose beside the call
(`chart_stretch_sup`'s doc: "sup-side by construction and is NOT a
lower bound"). Nothing checks the direction: the door's contract and
its uses disagree, and the soundness argument lives in paragraphs.
Raised on PR 2457 (SCALAR's `[ev]` sitting) as evidence that a typed
rate needs a direction tag; whether the fix is a doc correction, a
second door, or the `SupSpeed`/`InfSpeed` pair proposed there is Ev's
to settle on that PR. Found by the SCALAR rate census, 2026-09-12.
