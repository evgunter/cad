---
id: the-pinned-profile-lift-launders-at-sym
kind: issue
title: The loft's enclosure arm re-enters its sections' f64 canonical form as constants (ValidatedProfile::lift_onto), the last half of the pinned profile lift's laundering at Sym
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [the-from-f64-gate-reads-every-generic-crate]
---

`ProfileLift::Pinned`, the `EvalOptions` default (`eval/mod.rs`), lifts the
profile's nominal `f64` elaboration through `from_f64`. The two lifts are
`eval/wire.rs`'s `placement.map(T::from_f64)` and `profile/src/validate.rs`'s
`ValidatedSegment::lift` and `ValidatedLoop::lift`. Both are listed
`launders(the-pinned-profile-lift-launders-at-sym)` in
`scripts/gates/from-f64-sites.tsv`.

At `Sym`, every profile length is therefore a `Lit`, while the same
variables are symbols in the node magnitudes (an extrude distance). A
`Zero` decided over that mix is a theorem about a function the document
does not compute.

`drive::lane_opts` and the Monte Carlo lane choose `Guided`, which
elaborates the profile at `T`. Nothing stops another `Sym` evaluation
from taking the default. `crates/editor-core/tests/revalue_corpus.rs`'s
`options` takes it, and so do the gating rows and the census:
- Both builds resolve the profile at the nominal
  (`program.resolve(op_env.lane.nominal)`), so they agree, and no row
  can see a laundering in the profile lowering.
- `profile`'s generic code, the lowering the symbolic lane registers
  arcs through, is never run at `Sym` by the instrument.
- The in-step move is not a dilation of the document, because a
  profile length does not move under it. That is why `plate_param`'s
  `bool_join_nearest` distance is exactly zero at `p0` and moves at
  `p1` under both moves.

Owed:
- The pinned lift refuses, or is not offered, at an analysis scalar.
- The gating rows and the census run `Guided`, each row's move and
  floor re-pinned. A row that parts where it compared before is read
  as a finding, not dropped.

## Lofts are now in this row's family (EDIT, 2026-10-11, PR 4741)

A loft reads its sections' profile values, so under the pinned lift it
launders the way an extrude does. A `Dual` seed on a parameter a
section reads gives a zero derivative where main refused typed
(`SeedPinnedSection`, retired), and a box over a section lofts the
nominal. Under the guided lift both are correct: the seed rides to the
walls, and the box refuses `SectionNotAPoint`
(`crates/editor-core/tests/eval10_section_reads_the_nominal.rs`).


## The plane's half of the lift moved door (2026-10-14, PATHS' `the-sketch-plane-is-its-frame`)

The pinned lift's sketch plane no longer crosses by
`placement.map(T::from_f64)` in `eval/wire.rs`: the plane is a frame,
and it crosses through `SketchPlane::embed` (`RigidMotion::embed`,
`from_f64` per component). So the `from-f64-sites.tsv` entry that
tracks this row moved from `crates/editor-core/src/eval/wire.rs` to
`crates/geom-core/src/linalg/rigid_motion.rs`, under the same owed
class; the loops' half is unchanged (`ValidatedProfile::lift_onto`).
`RigidMotion::embed_computed` (`from_computed`) is the frame's door for
a computed frame, if this row decides the pinned plane is one.


## Narrowed: the pinned lift is gone (2026-10-14, Ev's ruling A on #5068)

Ev ruled that `ProfileLift` retires: every evaluation elaborates profile
geometry at its own scalar, guided by the `f64` pass's record, and the
`f64` lane takes the first pass's validated form through a sealed
per-scalar fact (`editor_core::ProfileLane`), so no lane enters the
`f64` elaboration through `from_f64`. A frame crosses no scalar upward:
`RigidMotion::embed`/`embed_computed` and `SketchPlane::embed` are gone,
and the plane is minted at the lane (`frame_plane_lane`). Built on
PATHS' `the-sketch-plane-is-its-frame` (#5068).

What that closes here:

- **The plane's half**: closed. The `from-f64-sites.tsv` entry for
  `crates/geom-core/src/linalg/rigid_motion.rs` is gone with the door.
- **The evaluator's loops half**: closed. editor-core no longer calls
  `ValidatedProfile::lift_onto`; the `f64` lane takes the first pass's
  form at the same scalar (`ValidatedProfile::on_plane`), crossing
  nothing.
- **"The pinned lift refuses, or is not offered, at an analysis
  scalar"**: done, by not existing.
- **"The gating rows and the census run Guided"**: done, because the
  default IS the lane's elaboration. `revalue_corpus` and the rows this
  file named (`pinned_lift_validates_once`, now
  `f64_lane_validates_once`; the `m10_*_interval` rows;
  `e6_drive_over_a_loft_at_interval`'s pinned half) run Guided; what
  moved is in #5068's body under "Ruling applied".

What remains, and is now this row's whole subject:
`ValidatedProfile::lift_onto` keeps one production consumer, sweep's
loft at an enclosure scalar (`crates/sweep/src/loft.rs`, the
`enclosure` arm): it lifts the `f64` canonical form of its sections'
value channel onto its lane planes through `from_f64`. At
`Sym<Interval>` that canonical form is a function of the variables, so
it launders exactly as the pinned lift did, for a loft. The
`from-f64-sites.tsv` entry for `crates/profile/src/validate.rs` (four
sites) stays owed to this row, its reason re-pointed at the loft. The
fix this row owes: the enclosure arm reads its sections at the lane
(as its planes already are), or enters the canonical form through
`Real::from_computed` (`ConstructedLoop::lift_computed`'s convention).
