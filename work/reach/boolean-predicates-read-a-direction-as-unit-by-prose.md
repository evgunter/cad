---
id: boolean-predicates-read-a-direction-as-unit-by-prose
kind: issue
title: three boolean predicates read a direction as unit by prose — contfp's plane normal, the germ facing sense, the sector directions
status: open
opened: 2026-09-15
priority: P3
cost: E
---



## Where this came from

The re-sweep of `unit-vector-witness-in-geom-core` (SCALAR) at its
merged base, with a pattern that reads multi-line signatures and
matches `unit|normali[sz]ed` case-insensitively. The class: a
function whose doc or parameter name asserts a unit-vector
precondition it does not check. `geom_core::UnitVec3<T>` carries that
fact across a function boundary; a function in the class takes the
witness the day its caller holds one. These three are in
`crates/topo/src/boolean/`, and none of their callers holds one — the
directions are read out of carriers or out of boolean-internal structs
whose fields are bare `Vec3`.

## The three sites

- `contain.rs`, `contfp` (`:126`): "classifies point `q` (already on
  the plane of `face`, with unit plane normal `normal`)". The callers
  pass `plane.normal` (`reduce.rs`, three sites; `ops.rs`, one) — a
  `Surface::Plane` field, the carrier case — and one test passes a
  constant.
- `join.rs`, `germs_face_each_other` (`:1106`): the facing margin is
  "unit germ dir · chord = cos × separation", in METRES only because
  `dir` is unit — `HalfGerm::dir` is a struct field, and a non-unit
  one scales the decided margins `bool_join_facing` and
  `bool_join_arc_facing` silently. A struct-field member, the shape
  the ruling's survey listed for `SplitPlane.normal`.
- `sectors.rs`, `parallel_same` (`:547`): "same-direction parallelism
  of two bound directions (unit-ish)". `u.cross(v).norm()` and
  `u.dot(v)` levered by `arm` — both margins scale with `|u||v|`, so
  a non-unit sector direction moves `bool_dir_parallel` and
  `bool_dir_same` without a refusal. `BoolSector::start` and `end`
  are fields.

## What taking it would look like

`HalfGerm::dir` and `BoolSector::{start, end}` as `UnitVec3<T>`, minted
where those structs are built (a decision each constructor already
implies); `contfp` waits on the carrier rule, as
`split-plane-normal-and-slab-axis-carry-unitness-as-prose` does.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.
