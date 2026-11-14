---
id: a-body-buried-in-another-bodys-material-passes-a-clearance-bound
kind: issue
title: A body buried in another body's material, its faces more than c from the other's, passes clearance(a, b) ≥ c (and reads Exceeded for ≤ c): a clearance reads faces, not material
status: open
opened: 2026-11-14
priority: P1
cost: M
---



## Finding

From the design fork on `signed-penetration-depth` (fork row 267).
Both designers found this by reading the code; it has not been run.

A clearance assertion's sides are read as faces. Nothing on the
clearance path (`topo::clearance`, editor-core's `wire_clearance`)
reads material. Take a ball strictly inside a block's material, with
every face pair more than `c` apart:
- `clearance(ball, block) ≥ c` answers `Holds`;
- `≤ c` answers `Exceeded`.

Over a parameter box the drive certifies that for the whole box. Two
blocks that share material by a crossing read distance 0 and are
`Violated`. So shared material violates or holds depending on whether
a surface happens to cross.

## What E3 says (as changed by this fork's `[ev]` PR)

A body reference selects its material and a face reference its face.
The clearance is the distance between the two point sets.

## What a fix needs

1. **Pin first.** A small block inside a large one, with
   `clearance(a, b) ≥ 1 mm` over two body references, at `Interval` and
   at `f64`. Today it holds, and the pin must flip.
2. **After the sweep certifies the faces at least `c > 0` apart**, the
   boundaries cannot cross. So each shell of one side lies wholly
   inside or wholly outside the other body's material, and one
   inside/outside reading decides it (`point_in_solid`, generic over
   the lane scalar).
   - The reading runs only where the shell's box lies inside the other
     body's box.
   - Within a certified leaf, containment cannot change across the box
     without the boundaries meeting, so one reading at the `f64`
     rebuild decides it for the whole leaf. Argue this in the code.
   - A face selection inside a body's material reads the same way.
3. **Results.**
   - A buried shell violates `≥ c` and proves `≤ c`.
   - Its witness is "this point of `a` lies in `b`'s material".
   - A core inside a cavity still holds, because the containment door
     counts every shell.
   - A free-form wall the containment door refuses refuses typed. Only
     nested parts reach that refusal.

Also, as stale text: editor-core `clearance.rs`'s module header and
the doc comment on `an_interpenetration_is_reported_violated_not_refused`
(`m10_5_r2_probes_interval.rs`) cite `work/curved/signed-penetration-depth.md`.
Reword both to say that the coincidence is the intended witness (a
violating pair).
