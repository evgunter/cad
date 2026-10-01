---
id: union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant
kind: issue
title: boolean: a union with a definitely tilted cylinder boss refuses ClassificationInvariant (the volume backstop) where a frontier refusal is due
status: closed
opened: 2026-09-08
priority: P0
cost: H
closed: 2026-10-01
pr: 3611
branch: reach/volume-backstop
---


## Finding (BLEND unit 1's style review, PR 2123; reported to S-BOOL by the BLEND orchestrator)

A boss extruded from a `SketchPlane` tilted by a DEFINITE angle
(measured at 4e-3 rad and at 0.2 rad), standing on or penetrating a
box, refuses through the union as
`ClassificationInvariant { "volume backstop: mass properties refused on
a tier-valid planar body" }` — `crates/topo/src/boolean/ops.rs:744`–`:758`
maps `mass_properties_closed_form`'s refusal to `corrupt()`. The
untilted penetrating boss builds. So a tilted cylinder (its stored axis
not the world normal — the one public route to that is a tilted sketch
plane) reaches the boolean's row-4 "kernel invariant" message for what
reads as a row-2 frontier (a support-pair configuration the rebuild
does not carry): the sentence blames the kernel for a shape it does
not admit. In-band tilts are refused earlier and honestly at
`split_conic_plane_parallel` (margin 5e-9 on a 5ε tilt); the definite
tilt is the case with the wrong sentence.

Recorded by
`crates/sweep/tests/review_blend1_r1_probes.rs::r1_a_boss_on_an_in_band_tilted_sketch_plane_through_the_union`
(the in-band door) and the reviewer's report for the definite tilts;
a row for the definite case is owed here.

## Home

`work/bool/` — `crates/topo/src/boolean/*` is S-BOOL's; filed by the
BLEND orchestrator per `docs/prompts/implementer-discipline.md` §6.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## More evidence (CONTACT-2, 2026-09-25)

The tilt need not be the cylinder's. An UNtilted rod (`r = 0.5`, an
extruded circle over `z ∈ [0, 4]`) minus a box extruded from a sketch
plane tilted 20° about `x` through `(0, 0, 3.5)` — an oblique cut, the
cylinder's axis still the world `z` — refuses the same
`ClassificationInvariant { "volume backstop: mass properties refused on
a tier-valid planar body" }`. The body is not planar; its wall is
trimmed by an ellipse, which the closed-form lane the backstop calls
does not measure.

A sibling door has the same gap and a typed sentence: the same rod
split at `z = 0.5 + tan 20° · y` and `z = 3.5 + tan 20° · y` through
`topo::split` (which builds, certifies, and measures 3πR² through the
certified door), then flatted by `brick((-1, 1), (0.2, 1), (-1, 5))`,
refuses `Containment(VolumeUncertified)` — the containment door's
at-infinity orientation probe also reads the closed-form volume.
Pinned by `crates/sweep/tests/axis_lap.rs`
`an_oblique_cap_flats_through_its_ellipse_arc`.

## More evidence (GATHER, branch `gather/derive-cusp-legality`, 2026-09-28)

No tilt and no oblique cut at all: an extruded half-disk (the profile
from `(0, 4)` down the `x = 0` line to the origin and back along the
radius-2 arc centred `(0, 2)`, height 1) minus an axis-aligned box
(footprint `square(2, 2, 0.5)`, i.e. `x, y ∈ [1.5, 2.5]`, from
`z = 0.5` up 2) refuses the same `ClassificationInvariant { "volume
backstop: mass properties refused on a tier-valid planar body" }` at
the boolean node. The box's two vertical faces cut the vertical
cylinder wall along rulings and its floor along an arc, so the wall is
trimmed by lines and a circle only. The same box through the `.cusp()`
lune refuses identically, so the wedge arm is not involved; a notch in
the lune's flat wall (`square(0, 3, 0.2)`) builds and gathers.
Measured with a throwaway probe in `crates/editor-core/tests/m10_2_measure.rs`,
not committed.

## Closed (2026-10-01, PR 3611)

The volume backstop measured through the closed-form mass-properties
lane, which refuses faces trimmed by an ellipse arc, or by rulings and
a circle. It mapped that refusal to `ClassificationInvariant`.

- **What the backstop does now:** it measures at certifying scalars
  through the certified quadrature (`AtRestPolicy::gate_volume_backstop`).
  It refines past the reporting round until the margin's sign is
  decided, and refuses `VolumeUndecided` when the range is still open
  beyond the band at the last round. A dual runs nothing (DL3).
- **The repros:** the tilted boss (four poses) and the oblique rod cut
  now build. The notched half-disk refuses honestly as `VolumeUnmeasured`,
  and the missing measurement is PROPS's
  `a-notched-cylinder-wall-has-no-volume-measurement`.
- **Residue, each in its own file:**
  - QUAD's `quadrature-convergence-test-escalates-instead-of-refining`
    (an order-dependent refusal);
  - QUAD's `quadrature-interval-floor-grows-with-the-body-past-the-band`
    (correct results refused at kilometre scale);
  - PROPS's `not-iso-rectangle-names-off-surface-edges-and-inventory-gaps-alike`;
  - CONTACT's `at-infinity-probe-measures-in-closed-form-only` (the
    `axis_lap` sibling).
