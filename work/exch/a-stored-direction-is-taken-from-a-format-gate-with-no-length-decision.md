---
id: a-stored-direction-is-taken-from-a-format-gate-with-no-length-decision
kind: issue
title: step-import stores line and ruling directions taken from checked_direction, direction_and_length or a bare len2 > 0.0, with no length decision
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by the review of the plane-normal witness unit (the sweep of
`recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness`
missed these).

**The class:** a direction the importer stores in a carrier, or uses to
place one, taken from `Vec3::checked_direction` /
`Vec3::direction_and_length` (format gates only: finite, not
underflowed) or behind a bare `len2.is_finite() && len2 > 0.0`, with no
length decision through the funnel — so a chord a few ulps long names a
direction as confidently as a metre-long one. The decided spelling is
`UnitVec3::new` under a name the reader owns (at the file band where
the length is a distance), or `RigidMotion::from_axis_and_reference` /
`gram_schmidt` for a frame.

Sites (`crates/step-import/src/`):

- `build.rs`, `line_on_column` (was `line_iso_match`): the `len2.is_finite() && len2 > 0.0`
  gate, which admits the chords whose `direction_and_length` feeds
  `Curve3::line` in `iso_of_wall` and `cap_row`.
- `build.rs`, `deferred_carrier` and `rebuilt_in_surface`:
  `(p_end - p_start).checked_direction()` → `Curve3::line`.
- `build.rs`, `generators_through`: `across.checked_direction()` → the
  ruling's radial and origin.
- `normalize/singularity.rs`, the seam-azimuth fallback (`radial`
  closure): `r.normalize()` after a `not_apart` read at the file band —
  decided, but not minted, so the stored azimuth carries no witness.
- `recognize_curve.rs`, `try_line` and `straight_map` (the chord) and
  `try_circle` (the
  chord, the three-sample normal, the radial `u_ref`): estimator
  directions, the circle's also filed as
  `circle-recognition-normalizes-its-plane-normal-with-no-mint`. Another
  lane was editing this file when the row was filed.

The surface recognizer's `try_plane` jet read (`patch_jet_mid`) has no
copy in `recognize_curve.rs`; `chart_flipped` shares it.
