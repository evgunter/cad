---
id: value-channel-mints-a-nan-bearing-validated-profile
kind: issue
title: ValidatedProfile::value_channel mints a ValidatedProfile<f64> whose values may be NaN; documented, but the canonical-form type promises validated values
status: closed
opened: 2026-10-11
priority: P2
cost: M
closed: 2026-10-16
---

## Finding

Raised in #4741's review (style lane; coded unilateral). Not taken there.

`ValidatedProfile::value_channel` (`crates/profile/src/validate.rs:1670`)
maps every stored scalar through `SpanLocate::value_channel`, which
returns NaN for a value with no single point (an `Interval` wider than
a point). The result is a `ValidatedProfile<f64>` whose values may be
NaN. The type is the profile crate's canonical form, and its mint list
otherwise produces only validated values. The door's doc says this,
and its one consumer, `sweep::loft_profiles`, refuses a NaN-bearing
channel before reading it (`crates/sweep/src/loft.rs:1118`). Nothing
else stops a later caller from reading it as validated.

## The question

Should this be unrepresentable? Two shapes:

- `value_channel` returns `Option<ValidatedProfile<f64>>` (or a typed
  refusal naming the first non-point value), so the check lives in the
  mint and `profile_is_finite` in sweep goes away;
- or it stays as is, documented, if a second consumer needs the
  NaN-bearing form (none does today).

The first also closes half of
`a-section-not-a-point-is-spelled-two-ways-across-the-loft-doors`,
since the refusal would then have one source.

## Closed

The first shape. `ValidatedProfile::value_channel`
(`crates/profile/src/validate.rs`) returns
`Result<ValidatedProfile<f64>, ValueNotAPoint>`. Each value is read
through `SpanLocate::point_value`, and the first one with no single
point refuses typed, naming its site: the plane, a vertex, or a
segment (canonical indices). So `value_channel` no longer mints a
validated profile holding NaN.

At `f64` a NaN plane is now a refusal (`Plane`), where the old mint
passed it through. `Profile::validate` never reads the plane, so a form
validated over a NaN plane still exists. That is
`work/inlane/a-validated-profile-takes-its-plane-unvalidated.md`.

The one production consumer is `sweep::loft_profiles` (`loft.rs`'s
`SectionsAtT`):
- its "every input is a point" test reads the mint's `is_ok`;
- its channel read can no longer see a NaN.

Pinned in `crates/profile/tests/interval_lane.rs`:
- `the_value_channel_refuses_a_value_with_no_single_point`: a point
  form mints its corner and plane; a wide corner refuses at vertex 2;
  a wide plane refuses as the plane.
- `the_value_channel_refuses_an_arc_carrier_at_its_segment`: an arc's
  carrier from `atan` refuses at its segment.

The consumer's arm for a refusal it cannot see is `unreachable!`, with
the reason (D9's kernel-defect row).
