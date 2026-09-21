---
id: limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has
kind: issue
title: limb 3's chart tube divides by a chart speed with neither the zero nor the non-finite guard plane_nurbs_ssi refuses on, and both collapses certify silently instead of refusing
status: open
opened: 2026-09-15
refs: [ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis]
priority: P0
cost: E
---


## Finding

`crates/geom-brep/src/ssi/certify.rs`, the plane × NURBS arm of the
tube ladder (`certify_rung3`, the `speed` closure ~`:862-880`), crosses
the rung radius into chart units through a local closure over the
derivative boxes:

```rust
let speed = |bx: Box3| {
    let m = bx.speed_sup();
    SupSpeed::new(if m > 0.0 { m } else { f64::NAN })
};
```

Its sibling — the same crossing on the same surface, in
`plane_nurbs_ssi` (`crates/geom-brep/src/ssi.rs`, the two guards above
the `SupSpeed::new` mint, ~`:985-997`) — refuses
`SsiError::UnsupportedCertificate` with a sentence naming the cause for
**both** members of the unusable class, and its comment states why
positive-finite is necessary. This site has neither guard, and neither
collapse is caught downstream either: **both end in a certificate, not
in a refusal.**

## What each collapse actually does (executed)

Measured on the unit's own certifiable wall (a single-span cubic × linear
NURBS patch against a cutting plane), `tube_radius = 1.875e-1`, live
speeds `su = 1.130884609498248`, `sv = 8.000000000000008e-1`:

- **zero (or negative) `m`** becomes the `f64::NAN` sentinel, so both
  pads are `NaN`. `probe_tube_chart` does **not** answer "no enclosure":
  it builds the span window `u0 = hu.lo() - NaN`, and
  `NurbsBoxes::deriv_box` → `NurbsBoxes::cells` clamps those ends
  (`f64::clamp` passes NaN through) and `KnotVector::span_at` lands a
  NaN parameter on the FIRST span — its own doc states that tie-break.
  The window is therefore neither poison nor empty. On this wall the
  ladder answers `Some((0.9610667880553895, 71))`, **bit-identical to
  the healthy pads**, and the run certifies. `TubeProbeSilent` never
  fires. On a multi-span wall the first-span landing is visible
  directly: `deriv_box(NaN, NaN, NaN, NaN, along_u)` returns
  `x ∈ [0.7799999999999987, 1.5600000000000016]` where the true domain
  box is `[0.7799999999999987, 1.6200000000000034]` — a strict subset,
  so limb 3's uniqueness proof is taken over a region nobody asked
  about.
- **non-finite `m`** passes `m > 0.0` unremarked and
  `SupSpeed::to_param(radius)` is exactly `+0`
  (`0x0000000000000000`), so the transversality is proved over the bare
  span hulls while the certificate reports `tube_radius` (metres) for
  the rung that was tried — a radius whose chart region was never
  padded. The `(0.0, 0.0)` probe and the `inf` probe are the same
  answer, bit for bit.

So the failure mode is **silence** — a certificate issued over a region
the pad never widened — and not, as this row first said, a refusal
wearing the wrong name. Both collapses are the class `plane_nurbs_ssi`
calls out by name one function away, and here neither of them reaches a
refusal at all, which is the worse half of D4 ¶3: the operation answers
`Ok` where it proved nothing.

Evidence: reproduced on branches `scalar/exhaust-r1-probes`
(`ssi/certify.rs::r1_collapse_probe`) and `scalar/exhaust-r2-probes`
(`r2_limb3_plant`, `R2_PLANT=zero|inf`), and re-run by SCALAR's fix pass
on the same wall. Neither probe is a shipped row: a row that pinned this
behaviour would pin a defect.

## Why it is not the neighbouring row

`ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`
is about the two sites disagreeing on the SHAPE of the pad (max-fold
versus per-axis) where both speeds are healthy. This is about what each
site does when a speed is not healthy, and the two fixes are
independent.

## Provenance

Found by SCALAR's `exhaustiveness-receipt-carries-its-lane` while typing
this site's two divisions through `SupSpeed::to_param` (the divisions
are typed; the guards are not this unit's). The first filing stated the
zero case as ending at `SsiError::TubeProbeSilent`; two independent
reviews of that PR ran it and found the ladder answers instead, and the
mechanism above is what execution shows. `ssi*` is TRIM's ground behind
PCURVE P-2 per PROPS' and BOOL's `keep_out`.
