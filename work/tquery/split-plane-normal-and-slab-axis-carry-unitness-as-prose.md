---
id: split-plane-normal-and-slab-axis-carry-unitness-as-prose
kind: issue
title: SplitPlane.normal and slab_extent's axis carry a unit precondition as prose — the class the geom-core witness now types at function boundaries
status: open
opened: 2026-09-15
priority: P3
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

## The two sites

- `crates/topo/src/splitting/mod.rs`, `SplitPlane` (`:93`): "a point
  on the plane and its **unit** normal (conventional, unchecked — same
  posture as `Surface::Plane`)". `SplitPlane` is a VERB INPUT, not a
  geometry carrier, so the at-rest rule is borrowed rather than owed:
  its one production constructor with a witness in hand is
  `editor-core/src/verbs/split.rs` (`DatumValue::Plane { normal }` →
  `normal.get()`), and the other ~90 constructors are test fixtures
  spelling `Vec3::new(0.0, 0.0, 1.0)` and the like — which is why this
  is not the one-line take the sweep's rule allows: the field type
  change reaches every fixture in `sweep/tests`, `topo/tests`,
  `mesh/tests` and the exporters' `tests/common`.
- `crates/topo/src/boolean/boxes.rs`, `slab_extent` (`:455`): "**The
  premise is a UNIT axis**, which is what `Surface`'s own conic
  descriptions promise … State the premise if you add a constructor
  that does not hold it." The axis arrives as a `SpanBox<T>` read from
  the surface, so this is the carrier case; the sweep's grep could not
  see it at all (its parameter is not a `Vec3<`), and it is here from
  the ruling's survey rather than from the pattern.

## What taking it would look like

`SplitPlane { normal: UnitVec3<T> }` with the fixtures minting through
`UnitVec3::new(v, "<a test site name>", band)` — a decision every
fixture already implies. `slab_extent` waits on the carrier rule.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to TOPO (crates/topo/src/split.rs, euler.rs and the provenance graft are TOPO's paths) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
