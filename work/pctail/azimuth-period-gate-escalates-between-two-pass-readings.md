---
id: azimuth-period-gate-escalates-between-two-pass-readings
kind: issue
title: pcurve_azimuth_period passes on Positive | Zero and escalates the band between them, the shape check 5's in-band clearance had
status: open
opened: 2026-10-03
priority: P2
cost: M
---

Found by CLEAVE's `cleave/steep-tube-eps` (PR 3981) sweep and raised by
its review. Unmeasured: no pose is known to refuse here.

`crates/geom-brep/src/pcurve_cache.rs` decides `pcurve_azimuth_period`
at four sites: the harmonic lane (`run_closed_form_checks`), the
cone-section lane, the spiric lane, and the fitted/general lane over
the control-net box. Each decides `Margin::levered(τ − extent, arm)`,
passes on `Sign::Positive | Sign::Zero`, refuses
`AzimuthPeriodExceeded` on `Negative`, and escalates `Err`. A headroom
in `(ε, K·ε)` escalates even though both readings it lies between
pass. That is the shape of check 5's in-band clearance, which PR 3981
fixed with `escape`.

**Why it was not fixed with 3981.**
- On the closed-form lanes the margin is the edge's OWN distance from a
  full turn, a near-closure of its two ends. That is a real sliver, so
  the band may be saying something true there.
- `the_azimuth_gate_ends_as_the_edge_winding_gate` pins its in-band
  story to the edge certifier's `CertCheck::ParamWinding`, a decided
  posture (D4 ¶1 (iv)).
- On the fitted/general lane the extent is a conservative control-net
  box, so the clearance there is the box's and not the edge's, which
  is the same reasoning as check 5.

## Owed

Measure first. Does any corpus pose read a headroom in band, and on
which lane? Decide per lane: is the in-band headroom a near-closure the
band should refuse, or a box clearance it should not? The
ParamWinding twin moves with the closed-form answer.

**PCERT overlap.** Ev ratified (PR 3919, DESIGN-FORK-LOG row 54) that
joints and closure state the integer branch and winding. The per-row
azimuth extent is PCERT's ground
(`work/pcert/pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin.md`),
so check that unit before acting here.

## PCERT B2a (PR 5244, 2026-10-17): the closed-form lanes' gates are gone

The harmonic lane's gate went with the stored windings, and B2a's fix
pass drops the cone-section (focal) and spiric-wall lanes' too: each of
their carriers (a circle, an ellipse, a spiric) has its interval held to
one period by the edge certificate's `interval_span_winding`
(`CertifyError::WindingExceeded`), and each image's angles advance
monotonically one turn per carrier period, so the gate restated the
edge's own bound. The one site left is the projected lane
(`pcurve_cache/projected.rs`), over the swept box of a spline net or a
sphere's general circle: the box clearance this item names for the
fitted/general lane, now the item's whole subject.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a headroom between two passing readings escalates: interval and error propagation, P2 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to PCTAIL (`work/pctail/`) by `git mv`, id and body unchanged: whose ground is `geom-brep`'s `pcurve_cache.rs` (the row names its PCERT overlap). CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
