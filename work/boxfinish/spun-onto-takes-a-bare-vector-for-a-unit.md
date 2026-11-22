---
id: spun-onto-takes-a-bare-vector-for-a-unit
kind: issue
title: RigidMotion::spun_onto takes a bare vector for a unit, and the sweep door hands it a table's written radial
status: open
opened: 2026-11-22
---


Found by `boxfinish/signed-radius`, which added the door, on the
orchestrator's question about ONEHOME's frame claim (fork 274 decision 1,
`work/onehome/carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes.md`,
and PR 7323, which removes check 1's frame half).

**The door.** `geom_core::RigidMotion::spun_onto(c: Vec2<T>)` turns a
frame onto `c` with no arithmetic on `c`; `sweep`'s
`Placing::frame_onto(Unit2<T>)` is its one caller, and
`swept::sketch_direction` hands it an arc's recorded start radial
(`profile::SegmentKind::Arc { radials }`, wrapped as `Unit2(start)`). The
extrude's and the loft's arc carriers and cylinder walls are turned so.
`RigidMotion::spun_toward`, which every line still goes through,
normalizes first.

It is un-normalized on purpose: over a box the frame's `u` is then the
recorded radial placed, node for node, so `centre + radius·u` is the
placed vertex as a theorem; through `spun_toward` the radial's own
length is a root the tier holds as an atom (measured on the merge of
main: the arcs' start endpoints stop folding).

**What reaches it, and what makes each a unit.**

| radial | made by | a unit by |
|---|---|---|
| the circle verb's `(±1, 0)` | `path::circle_loop` | literals |
| the split circle's `(cos θ, sin θ)` | `circle_split`, at the lane scalar | `sin_cos` at the scalar: encloses a unit |
| a bulge leg's, a via, tangent or onto arc's | the lowering (`Arc2::from_chord_radials`) and the verbs | a formula whose squared length is one as an identity |
| a half turn's `[r, −r]` in the sweep | `swept` | its source radial |
| one validation minted | `seg::mint_radial`, the offset over its measured norm | a normalization |
| **one a table wrote** | `RawLoop::new_directed`, a test-only door (`raw_door!`, compiled under `test` / `test-support`) | **a decision**: `segment_direction` accepts it where `‖c + r·d − p‖` is within the band, which bounds `(|d| − 1)·r`, and stores it as written |

Every row but the last encloses a unit at every point of its box, so
`spun_onto` of it encloses a rotation, and at `f64` is one to rounding.
The last is a unit only at the radius's lever: a written radial may be
stretched by `ε/r`. `validate.rs`'s `SegmentKind` doc says so and leans
on check 1 ("a radial accepted here is one check 1 accepts as a `u_ref`
at that radius"). PR 7323 deletes that check on the claim that every
frame mint yields a rotation in exact arithmetic and none takes raw
entries; `spun_onto` of a written radial is a mint that does not.

**A radial lifted across scalars** is the shipped row.
`ValidatedProfile::lift_onto` (and `SegmentKind::map`,
`ProfileLoop::map_scalar`, `ConstructedLoop::lift_computed`) passes a
radial's two `f64` numbers through `from_f64`: a unit to rounding, and
at `Interval` a point that holds no unit. Traced: `lift_onto`'s one
production caller, the loft's enclosure arm (`loft.rs`, `enclosure`),
hands the lifted sections to `assemble` and to the stacking door, and
both build each rim through `placed_segment_spec`, whose arc arm is
`frame_onto(sketch_direction(seg))`. So a loft of point sections that
hold arcs, at an enclosure scalar, reached `spun_onto` with a lifted
radial. Closed in this unit: `ValidatedSegment::lift` normalizes an
arc's radials at the target scalar, so what the loft turns onto encloses
the unit the `f64` radial rounds. `map_scalar` and `lift_computed` feed
a profile that is validated again at its scalar, whose radials the
verbs or the mint make there.

The table door being test-only, the hole it leaves is a statement about
types: no shipped build holds a written radial. The row
`sweep`'s `a_tables_stretched_radial_is_stored_in_the_walls_frame_as_written`
pins it as a fixture.

So on main as it stands the written radial is still read by check 1 at
the finish. Once 7323 lands nothing reads it: at an enclosure scalar its
rounding is taken as zero, and at `f64` 7323's measured rounding charges
it only where it is levered.

**Why it is not closed here.** The argument above is a comment: neither
`spun_onto`'s `Vec2` nor the sweep's `Unit2(…)` tells a constructed unit
from a written one, and the validated profile does not either (a
`ProfileLoop` a verb built and a `RawLoop` a table wrote both arrive as
`radials: [Vec2; 2]`). Telling them apart is a type in the profile's
direction record (which constructions record a direction, and what a
table's written one is), which is the direction mint's, under `[ev]`
PR 6993.

**Repair shape.** The validated arc says which its radials are (made by
a construction or a normalization; or written and accepted at a lever),
`frame_onto` takes only the first as a type, and a written radial goes
through `spun_toward`, as every direction does on main today. Or the
table door normalizes what it accepts.

**The tests that say it is closed.**
- A `RawLoop::new_directed` arc whose start radial is stretched by half
  of `ε/r` validates, and the frame its extrude stores is a rotation to
  rounding at `f64` (its `u_ref` squared less one within a few ulps).
- `spun_onto`'s argument is a type no written direction inhabits.

**Answered (2026-11-22).** Two designers read this row: it is the 2-D
case of fork 274, a recorded direction held as a unit witness
(`UnitVec2`) that `spun_onto` takes, built by
`work/boxfinish/a-recorded-direction-claims-a-unit-and-is-held-as-two-numbers.md`.
