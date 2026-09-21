---
id: viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep
kind: issue
title: the hand-rolled vector-op sweep missed its own dot half, and one datum row asserts something no value can break
status: dispatched
opened: 2026-09-16
priority: P3
cost: E
branch: vgeom/deletions
---


Found by the VIEW review of #2783, which is also the unit that ran the
sweep this row is about.

## The sweep named three patterns and the diff deleted four things

#2783's class sweep stated its patterns as *"a local normalize, a
hand-rolled cross, a least-aligned-axis branch"*. The diff deleted
**four** helpers from `crates/viewer/src/datums.rs` — `unit`, `cross`,
`dot` and the seed rule — so `dot` was a member of the class the sweep
was sweeping for and was never swept for.

The hit that escaped: `crates/viewer/src/display.rs`, inside
`is_rigid` —

```
let dot = |a: [f64; 3], b: [f64; 3]| a[0]*b[0] + a[1]*b[1] + a[2]*b[2];
```

the same shape as the deleted `datums::dot`, one file over. Its likely
disposition is the one `scene.rs`'s array cross already got — the data
are `[f64; 3]` and not `Vec3`, so there is no door to call without a
lowering — but **a hit with no line is a claim, not a receipt**, and
the sweep's table is the receipt.

`crates/viewer/src/sketch.rs:1038` (`tip_mark`'s `diagonal`) is a
second miss, from the follow-up `.hypot(` pass #2783 ran to close its
own stated `sqrt()` blind spot. Its disposition is clean — it is
`is_finite() && > 0.0` guarded and it multiplies rather than divides —
so this one costs only the receipt.

## The class underneath, which is not this crate's to fix alone

Both misses are array-lowered: `[f64; 3]` values that cannot reach a
`Vec3` door without a conversion. `work/props/geom-core-linalg-has-no-
array-doors.md:73-76` already lists the lowering sites, and
`crates/viewer/src/scene.rs:1423`'s array cross is already filed to
MESH. So the disposition of the `display.rs` hit probably belongs with
that row rather than here; what belongs here is that VIEW's sweep did
not list it.

## An assertion no runtime value can break

Separately, `crates/viewer/tests/datum_draw.rs:1327-1332` does

```
if along == axis_index { continue; }
assert_ne!(along, axis_index);
```

The `continue` makes the `assert_ne!` unreachable-false by
construction, so the row's only live check is the `unwrap_or_else`
panic on `position(…)`. `docs/prompts/implementer-discipline.md` §2:
an assertion no runtime value can break is documentation, and deleting
it is the repair.

**Where**: `crates/viewer/src/display.rs` (`is_rigid`'s `dot`),
`crates/viewer/src/sketch.rs:1038`, `crates/viewer/tests/datum_draw.rs:1327-1332`.

**Confidence**: sure on all three sites; unsure whether the
`display.rs` hit should be fixed in this crate or wait on the array-door
question PROPS holds.
