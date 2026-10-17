---
id: a-validated-profile-takes-its-plane-unvalidated
kind: issue
title: Profile::validate and ValidatedProfile::lift_onto take the sketch plane as given, so a validated profile can hold a NaN or infinite plane
status: open
opened: 2026-10-16
priority: P3
cost: E
refs: [value-channel-mints-a-nan-bearing-validated-profile]
---


Raised in PR 5238's review (MINOR, executed). Not taken there.

`Profile::new(SketchPlane::new(translation(0, 0, NaN)), quad).validate(tol)`
at `f64` succeeds, and the `ValidatedProfile<f64>` it returns holds
z = NaN in its plane (`crates/profile/src/validate.rs`). Validation is
2-D and never reads the plane, and `ValidatedProfile::lift_onto` takes
its plane as given. Since PR 5238, `value_channel` refuses such a form
(`ValueNotAPoint::Plane`), and `sweep`'s loft doors refuse it first
(`LoftError::PlacementNotFinite`). Any other reader of the plane gets
the NaN.

Repair shape: `validate` (and its guided and recording siblings) and
`lift_onto` refuse a plane whose components are not
`Real::is_bounded`, with a typed `ProfileError` arm. That is a new
refusal on a door every body operation passes through, so the arm
needs a row per door.
