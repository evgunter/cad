---
id: the-scalar-lift-convention-mints-doors-faster-than-consumers
kind: issue
title: Measured: of the eight map_scalar rungs scalar_lift.rs's convention names, one has a production consumer outside its crate; both top rungs have no non-test consumer anywhere
status: open
opened: 2026-09-12
---


## Finding

Filed by WIRE's `placement-prose` lane (PR body names the branch), out of
fence and on PROPS's slate because `crates/geom/src/*` and
`crates/geom-core/src/*` are PROPS's territory. The evidence and the full
per-rung table live on
`work/wire/frame-linear-generic-door-has-no-consumers.md` under
*"Measured (2026-09-12)"* — this row exists so the `geom`-side decision is
scheduled rather than left in a PR body, and so it survives WIRE closing.

`crates/geom/src/scalar_lift.rs`'s module docs state the convention:
*"One name, `map_scalar` on every geometry type and `map` on every leaf;
a reader looking for 'where does this crate lift X' finds it on X."*
That is a real argument for minting a door an in-tree site does not want.
The measurement is what it costs today.

Instrument, run once per rung and reverted: demote `pub` to `pub(crate)`,
`cargo check --workspace --all-targets`, read the `dead_code` warning and
the `E0624` list. An `E0624` naming a `tests/` file is a test-only
consumer; one naming another crate's `src/` is a production consumer.

Of the **eight** `map_scalar` rungs the module names:

- `NurbsSurface::map_scalar` — **the only one** with a production
  consumer outside its crate (`crates/sweep/src/loft.rs:328`).
- `Curve3::map_scalar` and `Surface::map_scalar`, the two **top** rungs,
  the ones the module is named for — `warning: method 'map_scalar' is
  never used` on the lib target, and every `E0624` inside `geom`'s own
  `tests/`. No non-test consumer anywhere.
- `NurbsCurve2`/`NurbsCurve3::map_scalar` — dead *because* `Curve3`'s is:
  its one lib caller is the rung above.
- `SurfaceDescription::map_scalar` and `ApproxSurface::map_scalar` —
  clean at `pub(crate)`: in-crate callers only, nothing outside `geom`
  names them, not even a test. Their `pub` is unearned.
- `Profile::map_scalar` / `ProfileLoop::map_scalar` (`crates/profile`,
  BOOL's ground, already filed on
  `work/wire/frame-linear-generic-door-has-no-consumers.md`) — test-only
  outside their crate.

Of the six leaf `map` doors it names:

- `Vec2::map` — `warning: method 'map' is never used` and **no errors at
  all**: zero consumers workspace-wide, tests included.
- `Mat3::map` — exactly one consumer workspace-wide,
  `crates/editor-core/src/placement.rs`'s `Frame::linear`, which is
  itself a door with no consumers (that row's subject). If `Frame::linear`
  is deleted, `Mat3::map` has none.
- `Vec3::map` — 15 call sites, **all** inside `scalar_lift.rs`'s own dead
  ladder.
- `Point2::map`, `Point3::map`, `Affine3::map` — live.

## The decision this asks for

Not "delete these". The convention may well be right, and a library that
lifts `Curve3` but not `Surface` would be worse than one that lifts both
ahead of demand. What the measurement blocks is settling a door's fate by
*citing* the convention, because as practised the convention is mostly
unconsumed — seven of eight `map_scalar` rungs and two of six leaf `map`
doors have no production consumer outside their crate. Ratifying "the
convention owes the door" ratifies that, and it should be decided on
purpose.

The counter-position is already written down elsewhere in the tree, at
the same altitude and with the same instrument:
`crates/pncad/src/lib.rs` refuses to re-export `bvh` — *"No demo scene,
no export corpus, and no document-layer path names it, and that
measurement is what decides the re-export. Re-export it the day a
consumer needs it."* Two doors, two opposite rules, neither aware of the
other. That is the thing to resolve; it is a `DESIGN.md`-shaped question
and probably Ev's.

Citations accurate at `e25946743`.
