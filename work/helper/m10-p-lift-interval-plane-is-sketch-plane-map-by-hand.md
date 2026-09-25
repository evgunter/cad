---
id: m10-p-lift-interval-plane-is-sketch-plane-map-by-hand
kind: issue
title: editor-core/tests/m10_p_lift.rs interval_plane rebuilds a SketchPlane from twelve from_f64 calls, which is plane.map(Interval::from_f64)
status: closed
opened: 2026-09-08
refs: [2139, D385]
priority: P3
cost: E
closed: 2026-09-24
---

(EVAL orchestrator) From EVAL-1's style review (PR 2139, S2), filed
onto S-TCOST's slate because the file is in its fence.
`crates/editor-core/tests/m10_p_lift.rs:207`–`:224` `interval_plane`
is `SketchPlane::new(Affine3::from_parts(Mat3::from_cols(v(c0), v(c1),
v(c2)), v(translation)))` with `Interval::from_f64` per component —
`plane.map(Interval::from_f64)` by another name, the walk EVAL-1
retired from `editor-core/src`. Not on `D385`'s list, which names
`m10_p_fence.rs:397` and `cert3r1_dump.rs:97` in this crate; one line
to fix and a `D385` sibling. Citations accurate at `bd2fe4289`.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane D)

**VERDICT: REPRODUCES** — unchanged, including the line number.

`crates/editor-core/tests/m10_p_lift.rs`'s `interval_plane` is still at
`:207`, still
`profile::SketchPlane::new(Affine3::from_parts(Mat3::from_cols(v(a.linear.c0),
v(a.linear.c1), v(a.linear.c2)), v(a.translation)))` with a local
`v: Vec3<f64> -> Vec3<Interval>` closure spelling
`Interval::from_f64` per component — **twelve calls**, three per vector
across four vectors, exactly as the row counts.

**The one-line replacement exists and is documented as such.**
`SketchPlane::map` is at `crates/profile/src/lib.rs:645`
(`pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> SketchPlane<U>`), and
its own doc ten lines above names this case verbatim: *"`plane.map(S::from_f64)`
— the `f64` frame lifted whole."* `Affine3::map` sits under it at
`crates/geom-core/src/linalg/affine.rs:47`, and the spelling is already
used in `crates/sweep/src/loft.rs` and `crates/editor-core/src/placement.rs`.

**`D385`'s two siblings in this crate are also live** (line drift only):
`crates/editor-core/tests/m10_p_fence.rs:539` and
`crates/editor-core/tests/cert3r1_dump.rs:96`, both the
`let pt = |p: Point2<f64>| Point2::new(T::from_f64(p.x), T::from_f64(p.y))`
hand-lift.

**Command.** `grep -n -A22 'fn interval_plane' crates/editor-core/tests/m10_p_lift.rs`;
`grep -rn 'pub fn map' crates/profile/src/lib.rs crates/geom-core/src/linalg/affine.rs`.

**Blind spot.** Only the named file and `D385`'s two named siblings were
checked; no crate-wide sweep for other hand-written `from_f64` frame
lifts was run, so this says nothing about the class's floor.

## Closed (2026-09-24, `dup/scalar-lift-home`)

Folded by S-DUP's componentwise-scalar-lift unit, whose census put its
inner `Vec3` closure among the members: `interval_plane` is deleted and
its one caller reads `plane.map(Interval::from_f64)` — the door this row
named. `SketchPlane` is `Copy`, so the borrow went with the function.
Closed from S-DUP's branch as a drive-by on HELPER's ground, announced
in the PR.
