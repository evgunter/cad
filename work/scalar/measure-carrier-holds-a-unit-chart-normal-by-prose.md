---
id: measure-carrier-holds-a-unit-chart-normal-by-prose
kind: issue
title: editor-core eval/measure.rs Carrier::Plane holds its chart normal as unit by prose — read from stored geometry, so no witness is minted
status: parked
opened: 2026-09-15
priority: P1
cost: E
blocked_on: [carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes]
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is SCALAR's `unit-vector-invariants-carried-as-prose` (ruled on PR 2457; `docs/doc-ledger/scalar-leaves-the-tracker.md`)
§RATIFIED). The class: a function whose doc or parameter name asserts
a unit-vector precondition it does not check. `geom_core::UnitVec3<T>`
now exists to carry that fact across a function boundary — minted by
the normalizing constructor (`UnitVec3::new(v, site, band)`: decide the
length under the band, divide), by exact negation, by `sin_cos`, and by
`geom-core`'s `frame.rs` ladders. A function in the class takes the
witness the day its caller holds one; until then the precondition
stays prose, and this row is where that is recorded rather than in a
merged PR body.

The geometry CARRIER fields (`Line.dir`, `Plane.normal`, the conic
axes) stay bare under `geom/src/lib.rs`'s at-rest rule by the ruling;
a parameter that is read straight out of such a field is in the class
but its caller holds no witness, so the take waits on either a
decision at the read (a `UnitVec3::new` under a name the reader owns)
or the carrier rule changing, which is not this row's call.

## The site

`crates/editor-core/src/eval/measure.rs`, `Carrier::Plane` (`:109`
onward): "a point on it, its UNIT chart normal, and the face's OUTWARD
normal". Both are read from stored geometry (`topo::readback`'s rule
1: report what the body stores), and the closed forms below (`angle`,
`gap`) read them as unit — an angle from a dot product is an angle
only between unit vectors. The carrier case, one read away from the
field.

## Home

`work/issues/` because `crates/editor-core/src/eval/measure.rs` is in
no open program's `paths` (`scripts/work.py territory` names no
owner; WIRE holds `eval/wire.rs` only).


## Added at the fix pass (re-sweep at the merged base)

Two functions in the same file read the same carriers' directions as
unit, and are members with `Carrier::Plane`:

- `axis_offset` (`:596`): "the component of the separation orthogonal
  to a UNIT direction" — `rel − axis·(rel·axis)`, a projection only
  for unit `axis`; the axis is read out of the stored cylinder or
  cone.
- `parallel` (`:375`): "the sine of two unit directions' disagreement,
  levered at the arm" — `u.cross(v).norm()` levered by `arm` into a
  DECIDED margin, so a non-unit direction scales the margin silently,
  the failure the witness makes unrepresentable. The same shape as
  `mate/coset.rs`'s `parallel`
  (`work/msolve/subgroup-directions-are-unit-by-prose.md`).

Moved from `work/issues/` to `work/scalar/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Re-measured and parked (2026-11-19, SCALAR orchestrator)

Read against main `00a5903654`. The row's premise has been overtaken:
it waits "on either a decision at the read … or the carrier rule
changing", and the carrier rule changed. Fork 274 (PR 7006, decided
2026-11-16) has `docs/DESIGN.md` say a carrier holds its directions and
frame as the witness types, and ONEHOME's
`carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes`
(P1 H) builds it, naming "the directions of a line and a plane". Once
that lands, `plane_of` and `carrier_of` read a `UnitVec3` out of the
stored carrier and the take here is mechanical. Minting at the read now
would add a site name, a band and a refusal path that the carrier
change then deletes. So this row is parked on that one.

What is true of the sites today:

- `axis_offset` is gone (`31e76cd42c`): `Carrier::Cylinder` holds a
  `geom::Line` and the arm reads `geom::Line::distance`, which is
  correct for any nonzero direction.
- `Carrier::Plane.normal`, `Carrier::Line.dir` and `Pose::Plane.normal`
  are still bare `Vec3`s their docs call unit.
- `parallel` forwards to the public
  `geom_brep::directions_parallel` (`carrier_pose.rs`), whose doc says
  "two unit directions" and whose margin
  `Margin::levered(a.cross(b).norm(), arm)` scales with a non-unit
  input. The prose precondition lives in that door, so the take is
  there as well as here.
- Two of `parallel`'s four callers (`pose_distance`, `axes_parallel`)
  hold a `UnitVec3` from `pose::AxisLine::dir()` and strip it with
  `.get()`; `Pose::of` strips two more. Those stop stripping in the
  same change.
- `angle` is `atan2(|u×v|, u·v)`, which does not need unit inputs; its
  "unit" prose is not load-bearing.
