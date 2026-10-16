---
id: the-skin-at-interval-encloses-the-nominal-skin
kind: issue
title: The skin at Interval encloses the nominal f64 skin: a section wider than a point refuses instead of enclosing its family
status: closed
opened: 2026-10-11
priority: P1
refs: [the-skin-computes-its-values-in-t, a-loft-is-the-nominal-surface-in-every-generic-lane]
cost: H
closed: 2026-10-11
---


Since `the-skin-computes-its-values-in-t` (#4726), a loft or sweep skins
its sections in their scalar at every projective scalar. At an
enclosure scalar (`Interval`, and `Dual`/`Sym` over it) it does not.
`sweep/src/loft.rs`'s `LoftBuild::enclosure` skins the sections'
`f64` value channel and lifts the walls through `from_computed`.
`SpanLocate::split_projective` (`geom-core/src/spline/locate.rs`,
`ProjectiveSplit` in `spline/projective.rs`) is the seam that chooses.

That is honest for a point section: the body encloses exactly the
`f64` skin of the section it was given. A section wider than a point
has no value channel (`SectionLoop::channel` maps it to NaN), so it
refuses typed at the `f64` structure pass
(`a_loft_over_a_section_wider_than_a_point_refuses_at_interval`). It is
still a narrowing: a toleranced loft over a parameter box cannot be
certified, only refused.

Owed, to enclose the family a wide section spans:
- a structure witness for the box: a nominal point that the
  sub-arc counts, merged knots, degree raise and span location are
  decided on, as C6 decides them;
- a span locator, and a knot algebra (elevation, refinement), that
  enclose at the certification scalar. The projective
  `apply_rational` is sealed against it (`ProjectiveScalar`), and a
  certificate holds a refinement only as homogeneous enclosures
  (`apply_certified`);
- a collocation solve whose averaged knots can be held where they are
  not points, or a parameterization fixed on the witness, so that
  `Knots::new`'s point rule still holds.

The enclosure half of `split_projective` is where this lands.
`a-loft-is-the-nominal-surface-in-every-generic-lane`'s `Interval`
enclosure pin waits on it.

Since #4741 the document's loft reads its sections at the lane scalar.
So at `Interval` it reaches this limit wherever a section or its
placement is wider than a point. A parameter box on a section's
dimension refuses `SectionNotAPoint`. So does rounding alone, under the
guided lift with no box at all:
- any arc section refuses `SectionNotAPoint`, because the arc's sweep
  (from `atan`) is an enclosure a few ulps wide;
- any section on a rotated authored frame refuses
  `PlacementNotAPoint`, because the frame's orthonormalisation widens
  the placement;
- a section on a derived frame refuses `PlacementNotAPoint`, because
  the frame's landed placement is an enclosure.

This item now gates every `Interval` loft with an arc or a rotated
frame.

**What the E6 drive does over these** (measured for #4741,
`crates/editor-core/tests/e6_drive_over_a_loft_at_interval.rs`). With a
box on a parameter no loft section reads:
- polygons on an unrotated frame certify in one leaf;
- an arc loft, or a polygon loft on a rotated frame, refuses at every
  leaf and bisects to its budget: 256 leaves refused as
  `Budget(Leaves)`, nothing certified.

With the box on a section's dimension the drive bisects the same way
(`m10_4_stackup_interval`'s
`a_loft_section_seed_rides_the_section_to_the_measure`). On the merge
base, all five shapes (the three above, plus a polygon and an arc
section boxed on its own dimension) certified nothing. Each refused in
one leaf as a `FlipCrossing` on a predicate-count divergence at the
loft (`chord_side` 224 against 168, `arc_apex_identity` 8 against 6),
with no verdict flipped. So no certification was lost: the unrotated
polygon gained one, and the others changed reason and cost 255 more
evaluations. Until this item lands, a terminal drive class for a
loft's not-a-point refusal would save those evaluations. It is
box-independent when the box reaches no section.

**Acceptance carried here** (orchestrator, 2026-10-11): this item also
carries `a-loft-is-the-nominal-surface-in-every-generic-lane`'s "a loft
on a face frame evaluates at `Interval`". When the enclosure skin
lands, flip both of these pins to evaluating and enclosing:
- `docm1_face_frame_interval`'s
  `a_section_on_a_derived_frame_refuses_a_wide_placement_at_interval`;
- `eval10_section_reads_the_nominal`'s
  `a_box_over_a_section_width_refuses_rather_than_lofting_the_nominal`;
- `e6_drive_over_a_loft_at_interval`'s two budget rows, and
  `m10_4_stackup_interval`'s drive tail on
  `a_loft_section_seed_rides_the_section_to_the_measure`.

## Closed

At an enclosure scalar a loft encloses the family its sections stand
for (`crates/sweep/src/loft.rs`, `LoftBuild::enclosure`):
- Sections and placements that are all points are a family of one.
  They loft as the lifted `f64` skin, as before.
- Wider ones are validated at `T` (`validate_at_t`, its decisions
  certified there) and skinned in `T`, under the projective half's
  code.

The structure witness is the decision itself, not a nominal point:
each structural positivity the skin asks (a chord that is not empty, a
row that moves, a parameter step that ascends, a pivot that is
nonzero) holds for every value the bracket stands for
(`SpanLocate::structure_positive`, which is `value_channel() > 0.0` at a
point scalar, so `f64` is unchanged). Finiteness is read through
`is_finite_length`. The knot algebra splits at
`SpanLocate::split_projective` in `make_compatible`. At an enclosure
scalar, curves that are already compatible come back as themselves,
and others refuse `SkinError::IncompatibleAtEnclosure`.

Flipped pins:
- `docm1_face_frame_interval`'s face-frame loft:
  `a_section_on_a_derived_frame_lofts_and_encloses_at_interval`.
- eval10's box over a section width:
  `a_box_over_a_section_width_lofts_the_family_it_spans`, at ε/256. At
  ε/16 the wall's pcurve residual escalates at the band, which the
  drive bisects.
- E6's rotated row, which now certifies in one leaf.
- m10_4's drive tail, which certifies in one leaf.

What a carrier cannot hold still refuses typed, and is
`an-enclosure-carrier-holds-point-weights-and-knots` (a design fork):
- every arc, as `SkinError::ArcWeightNotAPoint` (E6's arc row still
  bisects to its budget);
- an interior averaged knot wider than a point (`k > v_degree + 1`).

The library pins are in `crates/sweep/tests/a_loft_computes_its_walls_in_t.rs`
and `loft_profiles_door.rs`. `a-section-not-a-point-is-spelled-two-ways-across-the-loft-doors`
closes with this unit.
