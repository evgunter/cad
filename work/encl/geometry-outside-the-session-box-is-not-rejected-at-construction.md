---
id: geometry-outside-the-session-box-is-not-rejected-at-construction
kind: issue
title: D4 ¶4's 'geometry outside the range is rejected at construction' is unimplemented: no door refuses a coordinate beyond the 1 km session box
status: open
opened: 2026-10-12
---

(Filed by the ENCL lane on `encl/eps-floor`, which gave ε its floor from the box and was told not to build this.)

## What

D4 ¶4 (`docs/DESIGN.md`) reads: "the session box is 1 km, no coordinate larger in magnitude; geometry outside the range is rejected at construction". The box now has one home, geom-core's `tolerance::SESSION_BOX_EXTENT`, and the floor `tolerance::EPS_MIN` is derived from it. Nothing reads the extent as a bound on geometry: no constructor, builder door or import door refuses a coordinate beyond 1 km.

`EPS_MIN` is only honest while the box holds. Past 1 km a coordinate's f64 spacing exceeds ε_min, so a run at ε_min decides lengths its coordinates cannot state.

## Where

- `crates/geom-core/src/tolerance.rs`, `SESSION_BOX_EXTENT`: read by `EPS_MIN`, by error text in `ToleranceError`'s and `editor-core`'s `EditError::InvalidTolerance` Display, and by nothing that sees a point.
- `crates/profile/src/lib.rs`, the crate docs' "Deferred (named): session-box enforcement (D4 ¶4)" paragraph, and `crates/profile/src/sugar.rs`, `bulge_from_via`'s "Session-box gap" note: construction sugar mints a ~1e15 m radius carrier that validates.
- `crates/profile/tests/review_m2_pr2.rs`, `annulus_at_ten_kilometers_validates`: a row that pins a 1e4 m scene validating. It is outside the box and moves when this lands.
- `crates/step-import/src/lib.rs`, `import_step`: an imported body is placed and adopted with no extent check.
- `crates/geom-core/src/linalg/vec.rs` and `affine.rs`: several docs argue "far outside the session box (D4 ¶4)" for overflow postures. They assume a box that no door enforces.

## Repair shape

There are two candidate homes for the check: a construction-time check at the doors that mint geometry (sketch/profile, builders, import placement), or a validator check at rest (`topo::validate`). D4 ¶4 says "at construction". The box's lever arm (`Band::angular_at`'s conservative universal arm) and the SSI scale levers (`work/ssimarch/ssi-scale-levers-cite-a-size-range-with-no-number.md`) then have a refusal to cite. Weigh where the check lives and what a box-crossing refusal names before implementing.
