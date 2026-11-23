---
id: a-plane-poses-in-plane-origin-enters-the-mate-coset
kind: issue
title: A Plane (or Axis) pose's representative origin, which the kind forgets, enters the mate solve's frame and coset
status: closed
opened: 2026-10-19
priority: P1
cost: M
refs: [flag-and-pivot-pose-kinds]
branch: msolve/plane-pose-holds-its-invariants
pr: 5624
closed: 2026-10-23
---


Found by a FORK-REST designer (INTENT stage 3 D, 2026-10-19), from code reading; not measured.

A `Plane` pose forgets in-plane motion and an `Axis` pose a slide along itself (D10 `[variables.poses]`), yet the representative frame the solve reads carries the face's own point as its origin: `mate::solve::frame_of` and `mate_coset` (`crates/editor-core/src/mate/solve.rs`) feed that origin into the map, where exact arithmetic would cancel it again. In f64 it does not quite cancel, so a copy stood on its own start cap against the world's xy solves to the identity only to rounding. On a box lane a parameter that moves the in-plane origin (and not the plane) widens the map, though the plane, and so the placement, does not move. That widening is the "known gap" D's handoff records against `mate::anchor::Anchors::fixed`.

The suggested direction: read each pose kind through a canonical representative before the coset (a plane by its normal and the foot of the perpendicular from its space's origin; an axis by its direction and the foot from the origin), so a kind's forgotten freedoms cannot reach the map. `[coincidence.structural]` already reads poses modulo their kind's symmetry. To measure: a stood copy's map at f64, Interval, Dual and Sym under such a representative, and the m10 box maps' widths.

## Decided (2026-10-23, designer pair; INTENT orchestrator)

A blinded designer pair weighed this; the record is on `analysis/design-fork/msolve-plane-pose-origin`. Both designers recommend the same final state (both "likely"), so it is not a fork for Ev: the flaw is in the value, not the solve.

- **The value holds exactly its kind's invariants.** `PoseValue::Plane { normal, offset }` (the plane `p·normal = offset`, with `offset` a Length) and `PoseValue::Axis { dir, foot }`, where `foot` is the line's point nearest its space's origin, projected by the variant's one constructor. `Point`, `Direction` and `Frame` already hold only their invariants.
- **Rejected:** a canonical representative chosen inside `frame_of` alone, which was this row's first suggestion. It protects one reader by discipline. The others keep reading the stored point: the arm in `bundle_arm` and `unobservable::point_of` (the one exact-arithmetic effect), the axis chart's spin line, `Subgroup::Cylindrical`, `PoseDef::Meet`, and mirror and split planes. Measured on the box lane, it also does not remove the tilted-plane width.
- **What the premise got right and wrong** (measured with scratch crates calling `mate_coset` at f64 and Interval):
  - The f64 leak is rounding only (2–4e-16).
  - The in-plane leak reaches a placed map on the box lane when the target is tilted: about 2 m wide today against 6e-14 with a canonical value, for a copy whose normal is exactly an axis.
  - A tilted copy normal whose stored origin is boxed keeps the dependency width of `origin·normal`. That width is the kernel surface's input width, which no representation recovers.
  - The "copy stood on its own start cap" example is gone: `stand()` is now the identity bundle.
- **Constructions compute in the canonical form:**
  - a slide moves a plane's offset by `by·(along·normal)`;
  - a spin or a carried motion transforms the invariants;
  - a flip negates the normal and the offset;
  - `Meet` takes the meeting line's point nearest the origin;
  - mirror and split read `offset·normal`.
- **Unobservable moves:** the plane case of the unobservable-move report should then read `along·normal` directly.
- **The solve in closed form** (one designer; take it if it fits D′2's `settle`). `settle` builds the placed map from the chart's axes, the two offsets and the values, as the frame-lane path already does for axis-aligned charts, instead of correcting a representative. With the canonical value alone, a tilted chart still costs about 2.6e-3 of width at half-width 1e-3. With both, the solve adds no width beyond its inputs.
- **Rows owed:**
  - a box-lane document with a mate to a tilted face whose stored point is moved in-plane by a variable;
  - the m10 box maps re-measured;
  - stored bits that move at rounding level, re-baselined with their cause.
- **No ratified text changes.** One optional sentence for `[variables.poses]` ("a pose's value holds exactly its kind's invariants") would be new binding text, so it is not added here. The type documentation in `pose.rs` carries the rule.
- **After D′2,** which rewrites `wire/pose.rs`, `value.rs` and `settle`, and deletes `anchor.rs`, so the `Anchors::fixed` "known gap" above goes with it. When the flag and pivot kinds land, the rule extends to them as "the point of the kind's fixed locus nearest the space's origin".
