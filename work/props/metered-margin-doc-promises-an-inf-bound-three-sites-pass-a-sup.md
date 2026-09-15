---
id: metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup
kind: issue
title: Margin::metered's doc promises a certified speed LOWER bound; three sites push a certified UPPER bound through it
status: closed
opened: 2026-09-12
closed: 2026-09-15
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


## Closed (RATE-PAIR, `rate-pair-in-geom-core`)

The door's contract and its uses no longer disagree, because the
contract is a type. `geom_core::{SupSpeed, InfSpeed}` (metres per
parameter unit, transparent newtypes beside `Margin`) carry the bound
direction; `Margin::metered` takes the `InfSpeed` its doc always
promised, and `Margin::metered_sup` is the sup-side sibling for
overshoot and escape metering, with the dimensional argument stated
once at the door instead of per call site.

All three sites named above moved onto the sup door, each of them
because its producer is now typed at the mint:
`pcurve_cache::trim_containment` (through `chart_arms_at`, which
answers a `SupSpeed` pair from `chart_stretch_sup`), the `pcurve_iso_*`
slack meters (through `nurbs_stretch_bounds`) and the v-channel of
`topo::pcurves::pcurve_loop_continuity` (through `v_meter`). A site
that reaches for the wrong door no longer compiles — the two
`compile_fail` doctests on `SupSpeed`, each with a twin that compiles,
are what says so — and the conversions are one operation each, so no
margin's bits moved.

`docs/predicate-dimension-audit.md`'s rows for those sites cite the
door that matches the tag.
