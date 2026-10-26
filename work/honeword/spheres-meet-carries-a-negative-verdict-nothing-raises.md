---
id: spheres-meet-carries-a-negative-verdict-nothing-raises
kind: issue
title: SpheresMeet carries a Refused verdict whose Negative arm nothing raises: a crossing sphere pair is cut in, so the refusal is a decided-zero touch only
status: open
opened: 2026-10-08
priority: P3
cost: E
---


Found by JOIN's `join/sphere-pair-whole-circle`, which cuts in the
circle two crossing spheres meet in off every edge (the sphere arm of
`boolean::ops::sphere_extent_scan`, `SphereCutIn`).

## What

`BooleanError::SpheresMeet { verdict: geom_brep::recourse::Refused }`
was raised at two sites in `sphere_extent_scan`: a decided-zero touch
(`Refused::Zero`) and a crossing whose circle the section certificate
placed inside both faces (`Refused::Negative`). The crossing is now cut
in, so only the zero site raises it, and the `Negative` arm is a state
the type admits that nothing produces. These readers still carry it:

- `SpheresMeet`'s `Display` (the "cross" wording);
- `boolean::offer_rows`' `BooleanErrorKind::SpheresMeet` row, which
  mints both arms;
- `editor-core/tests/refusal_concision.rs`' `SpheresMeet` fixture
  (`Refused::Negative { margin: -0.5 }`) and its `NO_TOLERANCE_PASSES`
  entry.

## What a fix owes

Narrow the payload to the decided zero (`Classified`), drop the
`Negative` reading from the `Display` and the offer row, and re-pin
the concision fixture on the zero.
