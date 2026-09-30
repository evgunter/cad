---
id: sentinel-markers-with-no-reader-are-grep-only
kind: issue
title: mesh7r1_probes' R1-DOOR-ONLY markers are grep-only: sentinels with no programmatic reader
status: open
opened: 2026-09-13
priority: P3
cost: E
---


## Finding

Raised by the delta review of PR 2517 as a class, not an instance:
that PR's unit turned on sentinel-delimited regions that a census
reads programmatically (`test_utils::source::sentinel_region`), and the
reviewer noted this file carries markers of the same SHAPE with no
reader at all.

`crates/mesh/tests/mesh7r1_probes.rs` has three
`// R1-DOOR-ONLY-BEGIN` markers (and their ends). Nothing reads them:
they are found by grep, by a person, when a person thinks to grep.

## Why it is worth a row rather than nothing

A marker with no reader has two failure modes a marker WITH one does
not. It can be deleted, moved, or unbalanced with nothing going red —
and it reads to the next person as if something were enforcing what it
delimits, which is the more expensive half. `test_utils::source`
already has the reader (`sentinel_region` panics on a missing or
inverted sentinel), so the distance from "grep-only" to "locatable
programmatically" is one row, in a crate whose tests already depend on
`test-utils`.

## Not this unit

`crates/mesh/tests/` is MESH's, TCOST's and TINT's. PR 2517 neither
read nor edited it beyond confirming the markers exist; whether the
regions deserve a guard, or the markers deserve deleting, is MESH's
call. Either answer closes this — what does not is leaving markers
that look like a mechanism and are not one.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to S-TINT (test-suite integrity: a seed-varying hosted gate and grep-only sentinels) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.
