---
id: sphere-side-reader-keeps-an-unencoded-arm-for-a-pole-off-the-loop-on-each-side
kind: issue
title: The sphere side readers keep arms for a pole off every loop of a face, which tier 3 now refuses
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Filed by RESTCURVE's `validity-refuses-an-interior-chart-singularity` (branch `restcurve/chart-singularity-is-a-vertex`), on NEARPOINT's slate because the sphere arms of `crates/geom-brep/src/props/curved.rs` are NEARPOINT's.

## What

Tier 3 now refuses every sphere face with a pole in its interior or inside an edge (`topo::ValidationError::ChartSingularityNotAVertex`, read by `geom_brep::chart_singularity::locate`). Two sphere readers keep an arm whose only remaining subject is that state:

- `MaterialSign::Unencoded`'s second bullet and `pole_free_side` / `agreed_side` (`curved.rs`, near `:3632`/`:3647`): a one-loop face with a pole off its loop on each side answers `Unencoded`. Through a valid body that loop does not exist.
- `sphere_region_material_sign` (near `:3741`): "a pole on no member's loop is not read". FLUX's note on the parent row: once the rule is enforced, the region reader may read such a pole as outside the region.

## What a fix owes

Retire or narrow the arms (or say why a raw door still needs them), citing the tier-3 rule rather than re-reading where a pole lies; the check-6 curved-arm comment in `crates/topo/src/validate.rs` and `CurvedSenseInverted`'s exemption list follow whatever the readers decide.
