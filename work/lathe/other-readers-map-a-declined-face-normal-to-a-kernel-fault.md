---
id: other-readers-map-a-declined-face-normal-to-a-kernel-fault
kind: issue
title: Readers outside the join map a face normal that face_outward_normal_at declines to a kernel fault, where a spline face is a limit
status: open
opened: 2026-11-26
priority: P1
cost: M
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
