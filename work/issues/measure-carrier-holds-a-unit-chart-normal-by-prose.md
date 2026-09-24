---
id: measure-carrier-holds-a-unit-chart-normal-by-prose
kind: issue
title: editor-core eval/measure.rs Carrier::Plane holds its chart normal as unit by prose — read from stored geometry, so no witness is minted
status: open
opened: 2026-09-15
priority: P1
cost: E
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
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
