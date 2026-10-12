---
id: a-loft-is-the-nominal-surface-in-every-generic-lane
kind: issue
title: A loft or sweep is its nominal surface in every generic lane: Dual sees no derivative in a section's variables, Interval certifies the nominal over a box
status: closed
opened: 2026-10-11
priority: P1
cost: M
refs: [the-pinned-profile-lift-launders-at-sym, a-loft-takes-its-placements-as-f64]
pr: 4741
closed: 2026-10-11
---


Both designers who weighed `nurbs-weights-and-knots-are-the-scalar`'s
C6 fork (2026-10-11) found the same thing: a loft or a sweep is its
NOMINAL surface in every generic lane, not just at `Sym`. `skin.rs`
computes the walls in `f64` (`segment_curve`, `make_compatible`,
`skin_parameters`). `loft.rs`'s `assemble` lifts them by
`map_scalar(T::from_f64)`, about line 538. So:
- at `Dual`, a wall's derivative in any of its sections' variables is
  zero: the lift makes every datum a dual constant;
- at `Interval`, a body built over a parameter box certifies the
  nominal wall, not the family the box spans.

Either answer is wrong, not just unproven. A sensitivity read off a loft,
or a certificate over a toleranced loft, reports the nominal as if it
held over the box.

This is adjacent to `work/sym/derived-frame-placement-freezes-on-the-symbolic-lane.md`
but not the same: that one is a derived frame's placement, this one is
the wall's own data. It is also not the same as
`the-skin-computes-its-values-in-t`, which is the fix: that item makes
the values `T`; this one is the wrong-answer finding in the Dual and
Interval lanes, which stands until it lands. Owed: a pin, red today,
at each of `Dual` and `Interval`. A loft over a section whose radius is
a dual variable shows a nonzero wall derivative. A loft over a widened
radius encloses the radius's family at a sample.

## Ruling (orchestrator, 2026-10-11): the document feeds sections in `T`

A designer pair weighed whether the document should feed loft and
sweep sections in the lane scalar (`docs/DESIGN-FORK-LOG.md` row 115).
Both answered yes. This item is the document half, after the skin unit.

- **A loft reads each section operand's Profile value at `T`, as an
  extrude reads its profile.** It reads the loops and plane at `T`,
  and the pass-1 pieces and naming for wall naming. Nothing is
  re-resolved and nothing is read at `LaneEnv::nominal`. A section's
  placement is its profile's plane, so the loft takes no separate
  placement input.
- **Retired:** `section_of`, `SectionScalar`, `SeedPinnedSection`,
  `DerivedFrameSection` and their tags and tests, plus `LaneEnv::seed`
  if nothing else reads it. In the docs: DM1c's "a loft or sweep
  section on a derived frame refuses typed at any scalar but f64, since
  a section's geometry stays f64" (`crates/editor-core/REFERENCES.md`)
  and PP6's Sections bullet (`crates/editor-core/README.md`). DM1c's
  "a profile on a derived frame is placed at the lane scalar" stays.
- **Structure stays C6.** Every skin decision is taken under the
  library's rule, which is the skin unit's call: f64 selects, `T`
  re-verifies, and an indeterminate or flipped result refuses typed.
  The profile's own 2-D structure is already pinned by its PP1 record.
- **No `[ev]` PR.** `git log --all -S` finds no Ev words behind the
  section clause:
  - 3f3ef8c40f writes it as the orchestrator's consequence 2 of Ev's
    in-chat ruling A, recorded in 57d56934b7's spec amendment;
  - ruling A chose *place at the lane value* over *refuse off f64*;
  - the in-chat ratification of DM1-DM6 (0877790361) predates the
    clause.

  Ev's 2026-10-03 remark ("a bit shocked that parameters are f64 only
  now") points the same way. The unit's PR body says where it looked.
- **PP6 is shared with `the-pinned-profile-lift-launders-at-sym`.**
  That item owes PP6's sketch-plane rewrite (the pinned lift is the f64
  build, and every other scalar elaborates and places at that scalar).
  This unit owns the Sections bullet. Whichever lands second merges the
  other's text rather than re-writing it.
- **The wrong answer at `Interval` is live until this lands.**
  `section_of` checks only `lane.seed`, so a parameter box that reaches
  a section is resolved at the nominal. The lane then certifies the
  nominal wall over the box. If this unit does not follow the skin
  unit promptly, an interim typed refusal for a box that reaches a
  section is owed.
- **Acceptance:** the two pins above, a nonzero `Dual` wall derivative
  and an `Interval` enclosure of the family. Also: a loft on a face
  frame evaluates at `Interval`, `Dual` and `Sym`, and the f64 build is
  bit-identical. The `Interval` enclosure pin waits on
  `the-skin-at-interval-encloses-the-nominal-skin`; the `Dual` pin
  does not.
- `a-loft-takes-its-placements-as-f64` dissolves here: the loft has no
  placement input to take as f64.

## What the library offers this unit (PR 4726, the skin unit)

- `sweep::loft_body`/`sweep_body` take sections (`ProfileLoop<T>` or
  `ConstructedLoop<T>`) and placements in `T`. The structure is
  selected at `f64` on their value channel (`SectionLoop::channel`, with
  NaN where there is no point). Each section is then re-validated at
  `T` along that record (`SectionLoop::validate_guided`), and the skin
  runs in `T` at every projective scalar. At `Dual` a library loft over
  a dual bulge carries the wall's derivative
  (`sweep/tests/a_loft_computes_its_walls_in_t.rs`).
- At an enclosure scalar (`Interval`, and `Dual`/`Sym` over it) the
  library loft encloses the `f64` skin of the value channel
  (`SpanLocate::split_projective`). A section wider than a point
  therefore refuses at the structure pass
  (`a_loft_over_a_section_wider_than_a_point_refuses_at_interval`).
  A placement wider than a point refuses by name
  (`LoftError::PlacementNotAPoint`). Enclosing the family over a box
  is `the-skin-at-interval-encloses-the-nominal-skin`. A nominal
  witness alone would not do it: in the enclosure half the `f64`
  skin of the channel is the walls' VALUES, not only their structure,
  so a section answering `SectionLoop::channel()` with its nominal
  would certify the nominal over the box, the defect this item names.
- The library keeps a separate `places` argument (Python, demos, the
  test corpus). A document section that carries its own plane composes
  onto it: the plane's placement is the `places[i]` it passes.
- editor-core's `wire_loft` lifts its `f64` sections and placements
  at the door through `from_computed` until this unit lands.

Closed by #4741, the document half. A loft reads each section
operand's Profile value at `T` (`wire.rs`'s `section_value`, through
`sweep::loft_profiles`); `section_of`, `SectionScalar`,
`SeedPinnedSection`, `DerivedFrameSection`, `LaneEnv::seed` and
`LiftRefusal::PinnedSection` are retired, and DM1c's and PP6's section
clauses are rewritten.
- At `Dual`, a seed on a section's radius moves the lofted wall
  (`eval10_section_reads_the_nominal`'s
  `a_seed_on_a_section_radius_moves_the_lofted_wall`).
- At `Interval` under the guided lift, a box reaching a section
  refuses typed (`SectionNotAPoint`) rather than lofting the nominal
  over the box. So does a section with no box whose values are a few
  ulps wide: an arc's sweep, and the plane of a rotated authored frame
  (`PlacementNotAPoint`). Main lofted both. The E6 drive over such a
  document now bisects to its budget on refused leaves. Main certified
  none of these lofts either: it refused in one leaf as a
  `FlipCrossing` on a predicate-count divergence. An unrotated polygon
  loft now certifies where main refused it
  (`e6_drive_over_a_loft_at_interval`).
- Under the pinned lift a section's values are its `f64` form lifted,
  as an extrude's profile is: a box still lofts the nominal, and a
  `Dual` seed on a section gives a zero derivative where main refused
  typed (`SeedPinnedSection`). Both belong to
  `the-pinned-profile-lift-launders-at-sym`.
- The Monte Carlo lane (guided, one box per sample) now lofts each
  sample's sections rather than the nominal.
- A face-frame loft evaluates at `Dual` and `Sym`. At `Interval` it
  refuses `PlacementNotAPoint`, because the frame's landed placement
  is an enclosure.

The `Interval` enclosure of the family, and the face-frame loft at
`Interval`, wait on `the-skin-at-interval-encloses-the-nominal-skin`.
