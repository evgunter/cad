---
id: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
kind: issue
title: Carriers hold their directions and frame as UnitVec3 / RigidMotion, and check 1's frame half goes
status: open
opened: 2026-11-16
priority: P1
cost: H
refs: [7006, check-1-reads-a-turning-frames-length-off-its-bracket]
---


Decision 1 of fork 274 (PR 7006, merged), built against `docs/DESIGN.md`
[scalars] as now worded. What a lane scalar takes from `f64` is a choice,
never a claim. A frame is a claim: it asserts that it is a rotation, so it
"is built at the lane scalar, from its own inputs and those choices, and is
held as the claim it is (a frame as a `RigidMotion` or `UnitVec3`, never as
its entries), so no gate reads it back from entries that, at a box scalar,
no longer carry it". [placement.frame] puts the same frame in a function
of the operation's inputs.

## What changes

- **Carriers hold their frames as claims.** The directions and frames of
  the `geom` carriers become `UnitVec3` / `RigidMotion`, not raw `Vec3`
  entries: the `axis`/`u_ref` frame of a cylinder, sphere, torus,
  circle, ellipse or spiric, and the directions of a line and a plane.
  Every construction site mints them at the lane scalar.
- **Check 1's frame half goes.** Tier-3 check 1 stops reading the frame
  back from its entries:
  - `geom::convention::frame_margins` (`crates/geom/src/convention.rs:163`)
    and the `representability_margins` frame terms built on it;
  - `frame_defect` (`crates/geom-brep/src/pcurve_cache.rs:6504`), its
    `EnvelopeTerm::Frame` charge (`pcurve_cache.rs:6408`), and the
    projected lane's check (`pcurve_cache/projected.rs:2735`);
  - `implicit.rs`'s `unit_defect` reads of `u_ref` and `axis`
    (`crates/geom-brep/src/implicit.rs:2017`).
- **The code docs change with it:**
  - `geom`'s at-rest paragraph (`crates/geom/src/lib.rs:71`, "At rest,
    `topo`'s tier-3 check 1 certifies…");
  - the "What is NOT this type" paragraphs of `UnitVec3`
    (`crates/geom-core/src/linalg/unit_vec.rs:62`) and `RigidMotion`
    (`crates/geom-core/src/linalg/rigid_motion.rs:200`);
  - the `bounds_allowlist` entry that admits the frame read
    (`crates/geom-core/src/real.rs:1369`, checked by
    `scripts/gates/bounds-allowlist.sh`).

## Open point to settle in the build

**The f64 → box lift.** A carrier minted at `f64` and read at a box
scalar needs its `UnitVec3` / `RigidMotion` lifted so that it is still
the claim, not entries re-read. The build settles how the lift carries
the claim (re-minted at the lane scalar from the construction's inputs,
or carried as a certified rotation) and states it at the types.

## Closes with it

`check-1-reads-a-turning-frames-length-off-its-bracket`. There a frame
that truly turns over the box reads non-unit at first order in the box,
because check 1 reads its length off its bracket. With no frame half
left there is nothing to read.

## Done when

Every carrier's directions and frame are `UnitVec3` / `RigidMotion`, and
check 1 has no frame half. The sites above are gone or re-worded, and
the turning-frame row's case builds.
