---
id: profile-has-no-scalar-lift-door
kind: issue
title: crates/profile/ carries no map_scalar, so loft's end_profile hand-spells the ProfileLoop lift three rungs deep
status: open
opened: 2026-09-04
refs: [D320, D321, 1782]
---

## Finding

`crates/sweep/src/loft.rs:227-235`'s `end_profile` lifts a `Section`
(a `ProfileLoop<f64>` list) to `T` by hand, three rungs deep: it walks
`lp.vertices()`, rebuilds each `ProfileVertex` from its parts, rebuilds
each `ProfileLoop` from the vertex vector, and re-attaches the tangent
joints. Only the innermost rung reaches a library door — `v.pos()` now
goes through `Point2::map` (`crates/geom-core/src/linalg/point.rs:46`),
landed with `D320`/`D321` in PR 1782. The two rungs above it are still
spelled out.

This is `D320`'s defect one type up. The reason it could not close with
`D320` is that **the door does not exist**: `crates/profile/` carries no
`map_scalar` on `ProfileVertex`, `ProfileLoop` or `Section`, so there is
nothing for the call site to delegate to. `crates/geom/src/scalar_lift.rs:12-23`
states the convention this violates — *"One name, `map_scalar` on every
geometry type and `map` on every leaf; a reader looking for 'where does
this crate lift X' finds it on X."*

**Not a find-the-copies sweep.** `crates/sweep/src/loft.rs:227-235` is
the only site in the workspace that lifts a stored `ProfileVertex<f64>`
to another scalar; the work is minting the door, not hunting siblings.

**Two fences.** `crates/profile/` is **Track V**'s territory and the call
site is **Track T**'s, so the door and its one consumer cannot land in a
single track's lane under the partition rule. Whoever staffs this rows it
on the track that owns the half it starts from.

## Was

Filed by the style review of PR 1782 (`D320`/`D321`), which recorded the
`end_profile` row as "fixed" when only the innermost rung had moved.
