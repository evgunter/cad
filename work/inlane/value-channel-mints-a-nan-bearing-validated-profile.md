---
id: value-channel-mints-a-nan-bearing-validated-profile
kind: issue
title: ValidatedProfile::value_channel mints a ValidatedProfile<f64> whose values may be NaN; documented, but the canonical-form type promises validated values
status: open
opened: 2026-10-11
priority: P2
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
