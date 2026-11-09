---
id: the-exact-unit-sign-of-a-value-has-no-home
kind: issue
title: T::one().copysign(x), the exact ±1 sign of a value, is spelled at 7 sites with no home; and PR 6610's census-fix leftovers
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6615
---


Filed from PR 6610's FULL review (MERGE).

## The ±1 sign (S1)

`T::one().copysign(x)` is spelled at conic_arc.rs:296 and :355, tangent.rs:95, surfaces.rs:672 (`meridian_side`), arms.rs:928, sector_face.rs:196, and vec.rs:718 inside `turned_toward`. PR 6610 gave the vector turn a home (`Vec3::turned_toward`) and swept the scalar sign transfers as "not this shape". These 7 are one shape among themselves, so give them one home (e.g. `Real::unit_sign`). Also look at `select_le_zero(-one, one)`-style sign picks.

## Also from the review

- **S2.** `boxes::cover_box` takes `Option<&Cover>`, though 4 of its 5 callers convert with `.as_ref().ok()`. Its signature is shaped for the minority caller. The same "a held cover's box, or nothing" role is spelled with the opposite failure direction (skip, not poison) at sweep/blend/reach.rs:1816, step-import/knot_split.rs:754 and geom-brep/ssi.rs:3484.
- **S4.** `onto_nappe_of` now mints 2 `SymOp::Copysign` atoms under `Sym`, up from 1. The source census went down, but the DAG atom count went up. Check sym12's per-document census.
- **S6.** No row pins `trig_jets` refusing an uncertified `t`, despite its doc's promise at taylor.rs:221.
- **The `ruling_from` identity** now rests on cos α ≥ 0, where `copysign` used to take the abs. Note it at the site.
- **TCOST row wording** (work/tcost/census-files-each-cut-a-files-shipped-part-by-hand.md:3,13-21). It says "eight" but lists 10.
- **CIW row option A.** It overstates: only certified_endpoint_census carries `gated_to!`, so the other three need markers.
