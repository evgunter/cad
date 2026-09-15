---
id: limb-3-chart-tube-speed-has-neither-guard-its-sibling-site-has
kind: issue
title: limb 3's chart tube divides by a chart speed with neither the zero nor the non-finite guard plane_nurbs_ssi refuses on, so both collapses reach a refusal that names a different cause
status: open
opened: 2026-09-15
refs: [ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis]
---


## Finding

`crates/geom-brep/src/ssi/certify.rs` (~`:862-871`), the plane × NURBS
arm of the tube ladder, crosses the rung radius into chart units
through a local closure over the derivative boxes:

```rust
let speed = |bx: Box3| {
    let m = (…).sqrt();
    SupSpeed::new(if m > 0.0 { m } else { f64::NAN })
};
```

Its sibling — the same crossing on the same surface, in
`plane_nurbs_ssi` (`crates/geom-brep/src/ssi.rs`, ~`:987-999`) —
refuses `SsiError::UnsupportedCertificate` with a sentence naming the
cause for **both** members of the unusable class, and its comment
states why positive-finite is necessary. This site has neither guard:

- **zero (or negative) `m`** becomes a `f64::NAN` sentinel that nothing
  here catches. It propagates into `probe_tube_chart`'s span windows
  (`u0 = hu.lo() - radius_uv.0`), so every rung answers with no
  enclosure and the ladder ends at `SsiError::TubeProbeSilent` — *"the
  ladder had rungs, but no rung's tube probe produced an enclosure"* —
  which is true and is not the cause. `TubeLadderEmpty`'s own doc
  records the last time a structural refusal wore another variant's
  costume here; `memories/refusal-text-is-not-cause.md` is the standing
  rule.
- **non-finite `m`** passes `m > 0.0` unremarked. The pad is then
  exactly `0`, so the transversality is proved over the bare span hulls
  while the certificate reports `tube_radius` (metres) for the rung
  that was tried — a radius whose chart region was never probed.

Both are the class `plane_nurbs_ssi` calls out by name, one function
away, and neither reaches a refusal that says so.

## Why it is not the neighbouring row

`ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`
is about the two sites disagreeing on the SHAPE of the pad (max-fold
versus per-axis) where both speeds are healthy. This is about what each
site does when a speed is not healthy, and the two fixes are
independent.

## Provenance

Found by SCALAR's `exhaustiveness-receipt-carries-its-lane` while
typing this site's two divisions through `SupSpeed::to_param` (the
divisions are typed; the guards are not this unit's). `ssi*` is TRIM's
ground behind PCURVE P-2 per PROPS' and BOOL's `keep_out`.
