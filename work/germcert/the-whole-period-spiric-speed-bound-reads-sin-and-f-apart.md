---
id: the-whole-period-spiric-speed-bound-reads-sin-and-f-apart
kind: issue
title: The whole-period spiric speed bound reads sup|sin| and inf f apart, so it grows as 1/f_min at the pinch though the true sup stays under r·sqrt(1 + |d|/r)
status: open
opened: 2026-10-14
priority: P4
cost: M
refs: [the-spiric-winding-lever-decides-real-pinched-loops-zero]
---


Found by GERM's lane on `the-spiric-winding-lever-decides-real-pinched-loops-zero`
(`germ/spiric-lever-pinch`), 2026-10-14, in the sweep for that row's
shape. The fix there is local to `geom_brep`'s length bound; this is
the same shape where it is not.

## What

`geom::spiric_rate_bounds` (`crates/geom/src/curves.rs`, about :1031)
bounds the speed `|C′|² = r²(1 + d²sin²v/f²)` as
`r·√(1 + (d·sin_max/f(rho_lo))²)`. It reads sup `|sin v|` and inf `f`
apart. Over a whole period (`spiric_period_rate_bounds`, `sin_max = 1`,
`rho_lo = R − r`) that is `r(R − r)/f_min`, which grows like
`1/√gap` as the cut nears the inner equator. The true sup does not:
with `x = 1 + cos v` and `δ = R − r − |d| ≥ 0`,
`d²sin²v/f² = d²(2 − x)·x/((δ + r·x)(ρ + |d|)) ≤ |d|/r`, so the true
sup speed stays under `r·√(1 + |d|/r)` however thin the pinch.

`geom_brep::props::spiric::piece_speed` (the fix above) reads the two
together on each piece; its derivation is in its doc. The function
here takes no `R`, so it cannot do the same without a signature change.

## Readers (all sound; each pays the looseness)

- `crates/topo/src/carrier_speed.rs` `speed_range`: the sup speed per
  kind.
- `crates/topo/src/offset_axial.rs` `param_speeds`: the ceiling a
  Zero-admitting decision levers at.
- `crates/topo/src/splitting/containment.rs` `spiric_speed`: a loop's
  reach.
- `crates/topo/src/splitting/spiric_arc.rs` (about :182): the crossing
  row's per-piece speed and acceleration.
- `crates/geom-brep/src/props/spiric.rs` `span_rates`: the wall image's
  azimuth rate, read only by the wall's map-residual pad (`× env`,
  identically `0` on a spiric's own image today).
- `crates/mesh/src/sizing.rs` and `crates/step-export/src/writer.rs`
  read the acceleration term (`spiric_curvature_sup`), which has the
  same `1/f³` reading apart.

No reader is measured to refuse or mis-size because of it yet.

## Done when

The speed term (and, if it pays, the acceleration term) reads `sin`
and `f` together, or each reader that needs a tight bound moves to a
piecewise reading like `piece_speed`; and a near-tangent row shows the
whole-period bound within a small factor of the true sup. If the
reading lands in `spiric_rate_bounds` itself, `piece_speed`'s together
half becomes a call to it.

## Moved (2026-10-26)

From CURVED, which closed (Ev, in chat: "curved can close without a walk, and rows to germcert is good"; `docs/doc-ledger/curved-leaves-the-tracker.md`). Both rows came from GERM lanes and are about the spiric carrier's own facts.
