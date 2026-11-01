---
id: direction-doors-refuse-finite-directions-normalize-now-answers
kind: issue
title: Direction doors still refuse finite out-of-range vectors that Vec3::normalize now answers
status: open
opened: 2026-10-21
refs: [normalize-overflow-yields-zero-axis]
priority: P2
cost: M
design: true
---

Found while landing `normalize-overflow-yields-zero-axis`, whose fix
makes `Vec3::normalize` / `Vec2::normalize` (and
`Vec3::orthonormal_basis`) prescale by an exact power of two
(`normalizing_scale` in `crates/geom-core/src/linalg/vec.rs`), so a
finite vector whose squares leave the format — `(1e200, 0, 0)`,
`(1e-170, 0, 0)` — normalizes to its direction instead of to the zero
vector or to `±∞`.

The doors that decide a direction's length still refuse those same
finite vectors, and the reason their docs gave for refusing was the
collapse that is now gone:

- `decide_direction_by` (`crates/geom-core/src/linalg/unit_vec.rs`,
  behind `UnitVec3::new` and `decide_unit_direction`) refuses a finite
  vector whose norm overflowed as `UnitVec3Error::NonFiniteLength` and
  one whose norm flushed to zero as `UnitVec3Error::UnderflowedLength`.
  Consumers that map those arms: `sweep::revolve::axis`
  (`RevolveError::NonFiniteAxis` / `UnderflowedAxis`), `sweep::skin`,
  `topo::boolean::join`, `linalg::frame` (`FrameError::NonFiniteLength`
  / `UnderflowedLength`), `editor-core`'s `eval/wire.rs` and
  `eval/wire/pose.rs`, and `editor-core`'s `mate/coset.rs` (INTENT's
  ground under the D10 hold).
- `topo`'s `sector_shape` rung 0 (`NonFiniteChord`, and its underflow
  twin) refuses an overflowed or underflowed chord before normalizing
  it.
- `step-import`'s `entities.rs`: the DIRECTION door (the
  `ratios.norm()` gate, "a zero triple names no direction") and
  AXIS2_PLACEMENT_3D's reference-direction door (the
  `perpendicular.norm()` gate, "a parallel pair states no frame") test
  `norm.is_finite() && norm > 0.0` on the unscaled norm. They refuse
  `(1e200, 0, 0)` under a false cause (it is neither zero nor
  parallel), and `(1e-170, 0, 0)` as "a zero triple".
- `profile`'s `unit_from_components` (`crates/profile/src/path.rs`) has
  the finiteness gate but no underflow gate: `(1e-170, 0)` squares to
  zero and refuses as `ZeroDirection`, a false cause.

Under DUAL-DESIGN DL6 a refusal is legal only where the input poses no
real question, and `(1e200, 0, 0)` names a direction. Two readings to
settle, per door:

1. the door's length question is asked of the prescaled vector (its
   `norm_witness` and `normalizing_scale`), so a finite vector decides
   `Positive` and normalizes, and the two arms narrow to a poisoned or
   infinite component; or
2. the refusal stays, because the door's real question is a length
   measured in the session box (D4 ¶4: 1 km) rather than a direction,
   and its docs say that instead of naming the collapse.

Which reading holds may differ by door — a datum normal is a direction,
a chord's length is a length — and the funnel names (`datum_unit_norm`,
`eval_direction_norm`, `path_director_norm`) would start receiving
samples the gates now keep out, which `docs/K-REPORT.md` counts.
`profile`'s `unit_from_components` and `arc_fillet::carrier_tangent`
divide by a hand-spelled norm rather than calling `normalize`, so they
still collapse past ~1e154 and their refusals still prevent a wrong
answer; under reading 1 they would respell onto `normalize`.

**The lanes already disagree on one document.** A linear pattern with
direction `(1e200, 0, 0)` refuses at `f64`
(`m4_pr2_wire::non_finite_pattern_direction_refuses_at_the_direction_door`)
and, at the interval scalar — where `is_finite_length` waves an
overflowed enclosure through by design — now places its three
instances at `x = 0, 2, 4`
(`m4_pr2_wire::a_non_finite_pattern_direction_places_its_instances_at_the_interval_scalar`).
Before the prescale the interval lane failed downstream on a collapsed,
wide direction. The ruling here decides which of the two answers the
other lane converges on.
