---
id: other-readers-map-a-declined-face-normal-to-a-kernel-fault
kind: issue
title: Readers outside the join map a face normal that face_outward_normal_at declines to a kernel fault, where a spline face is a limit
status: review
opened: 2026-11-26
priority: P1
cost: M
branch: lathe/declined-normal-readers
refs: [the-teapot-spouts-union-stops-at-section-loop-undecided-where-its-chords-hold-in-a-spline-wall]
---

## What

`face_normal::face_outward_normal_at` reads no normal for two
different reasons. One is a face whose kind has no arm at a point: a
spline, approximating or spindle sheet, which is a limit. The other is a
key that resolves to no face on a surface, which is a kernel defect.
`face_outward_normal_or_why` returns which, as `NoNormalAt`
(`UnreadKind` or `Unresolved`), and `face_outward_normal_at` is that
reading with the reason dropped.

The join's section-chord ladder (`boolean/join.rs` `held_normal`, for
the holding face and the chord's region) and `boolean/slit.rs`'s
edge-in-face reading match on the reason. The teapot's spout found the
join's case, where a spline wall holding a chord read as `JoinDesync`
(`the-teapot-spouts-union-stops-at-section-loop-undecided-where-its-chords-hold-in-a-spline-wall`).

These readers still map both reasons to a kernel fault:

- `boolean/fate.rs:836`, `normal_at`, the corner reading:
  `Ok(None)` → `ClassificationInvariant`. Likely reachable by an
  operand with spline walls.
- `boolean/reduce.rs:4660`, in `sheets_part`, a crossing on a boundary
  edge: `Ok(None)` → `JoinDesync` ("a face a crossing reads does not
  resolve"). Likely reachable by an operand with spline walls.
- `boolean/vtxfac.rs:1956`, `mint_node`: `Ok(None)` →
  `ClassificationInvariant`. Probably unreachable, since its nodes come
  from quadric-pair sections. Open: whether a spindle torus face reaches
  it.

`boolean/slit.rs:2494`'s foot reader is not in this class: its own
error (`FootNormalError::Unread` against `Pending`) tells them apart.

## Done when

Each site matches on `NoNormalAt` (through `face_outward_normal_or_why`).
An `UnreadKind` there refuses typed as that reading's limit, and
`Unresolved` stays the defect. For each site, a row either reaches it
with a spline-walled operand or states why none can.

## Census

Taken 2026-11-27 on `lathe/declined-normal-readers`, over every
production caller in `crates/topo/src` of `face_outward_normal_at`,
`face_outward_normal_or_why`, `face_outward_normal_at_foot` and
`spline_outward_normal_at`:
`grep -rn "face_outward_normal_at(\|face_outward_normal_or_why(\|spline_outward_normal_at(\|face_outward_normal_at_foot("`,
less tests and `face_normal.rs` itself.

**Misfiled, fixed.** Each now reads `face_outward_normal_or_why`, through
`BooleanError::of_declined_normal`. A spline sheet becomes
`CurvedBooleanUnsupported` naming the operand, face and kind, the limit;
a key that resolves to no face stays the site's own defect.

- `boolean/fate.rs` `normal_at`, the corner and piece-beside reading.
  Its row is `fate::declined_normal_rows`, in both operand labels, with
  the stale key.
- `boolean/reduce.rs` `sheets_part`, the crossing on a boundary edge.
  Same helper. No row reaches it with a spline face: it reads two
  tangent covering sheets at a boundary-edge crossing, and no fixture
  puts a spline face in a cover
  (`splice/sheets-part-on-a-spline-boundary-edge-has-no-row` is the
  boundary-edge half of the same gap).
- `boolean/vtxfac.rs` `mint_node`. Same helper, for one spelling of the
  class. No spline face can reach it: a node's faces are cylinder walls
  (`node::is_wall`), so only a stale key reads no normal there, and that
  stays `ClassificationInvariant`. A spindle face cannot reach it either,
  for the same reason.

**Already right:**
- `boolean/join.rs` `held_normal` and `spline_held_normal` (PR 7824);
- `boolean/slit.rs`: the edge-in-face walls read `NoNormalAt`; the
  floor's foot reads `FootNormalError::Unread` into `spline_floor`; and
  `spline_floor`;
- `boolean/vtxfac.rs`: the pierced face (a spline face read at its chart
  point, or `CurvedBooleanUnsupported`), and the held pierce's
  piercing face (`HeldPierceUnbuilt`);
- `census.rs`, the two foot readers (`FootNormalError` against a stale
  key's `Ok(None)`).

**Limit in the safe direction, not changed:**
- `boolean/overlap.rs` `normal_at`: a spline or approximating face is
  refused upstream (`OverlapRefusal::SplineCarrier`) before any normal
  is read. A spindle face, which the boolean does not admit where it
  meets the other operand, would read `StaleFace`.
- `census.rs` (the pair axis, the wedge, the touch floor, the interiors)
  and `validate.rs`'s pinch corners read a declined normal as unread or
  unbuilt, a limit. They read a stale key the same way. On an at-rest
  body that is a defect read as a limit, the opposite direction to this
  item, and these at-rest readers have no desync class to raise.
- `side_reader.rs` (two sites) is a diagnostic reader that skips a face
  with no normal.

**The sweep's blind spot.** Readers that never ask these doors, and call
`geom_brep::implicit_outward_normal` on a face's carrier, read poison on
a spline face. The class is filed with its hit list as
`onedoor/readers-of-the-implicit-outward-normal-read-poison-on-a-spline-face`.
