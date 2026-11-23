---
id: validated-profile-lift-onto-is-a-fixture-door-and-value-channel-has-no-production-reader
kind: issue
title: ValidatedProfile::lift_onto is a fixture door now, and ValidatedProfile::value_channel has no production reader
status: open
opened: 2026-11-21
priority: P3
cost: E
refs: [f64-built-data-is-built-at-t]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-21.

The loft's enclosure arm was the one production caller of `ValidatedProfile::lift_onto` (`crates/profile/src/validate.rs`) and of `ValidatedProfile::value_channel`. It builds in `T` now and calls neither.

- **`lift_onto`** is behind `test-support`, listed `fixture` in `scripts/gates/from-f64-sites.tsv`. Its callers are test fixtures that want a validated profile at `Interval` or `Dual64` holding the `f64` form's points and decisions: `crates/profile/tests/` (`validated_map`, `cusp_joints`, `rejections`, `recorded_directions`, `one_segment_loop`) and `crates/sweep/tests/` (`copied_carriers_at_interval`, `band_closed_mixed_chain_shapes`). `validated_map` is a suite about the door itself (that the lift equals a re-validation). A fixture that validated at its own scalar would need no door, and would meet the escalations the lift skips; whether those rows want that is the question this row holds.
- **`value_channel`** (the `T` to `f64` direction) is read by `crates/sweep/tests/loft_profiles_door.rs` alone. DESIGN `[unrepresentable]`: a conversion is justified by necessity, and "that it has callers is no evidence"; this one has none in a shipped build. Delete it, or say what needs it.
