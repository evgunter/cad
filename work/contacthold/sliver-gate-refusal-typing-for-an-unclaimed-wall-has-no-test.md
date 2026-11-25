---
id: sliver-gate-refusal-typing-for-an-unclaimed-wall-has-no-test
kind: issue
title: gate_sliver_orientation's typing of a wall face with no reach (WallOutlineUnsupported / PartialConeFace) has no test: it needs an at-rest sliver operand bounded by a spiric or spline edge
status: closed
opened: 2026-10-18
priority: P3
cost: M
closed: 2026-11-25
---

## What

PR 5261 made `census::face_reach` answer `None` for a cylinder or cone
face with a boundary edge that has no axial claim (no sound box, or a
spiric). `crates/topo/src/boolean/reduce.rs` `gate_sliver_orientation`
then types that refusal as `PointInSolidError::WallOutlineUnsupported`
(cylinder) or `PartialConeFace` (cone), through
`census::boundary_unclaimable`, instead of `CorruptFace`. Nothing pins
that typing.

## The fixture it needs

An operand that passes the at-rest gate as a sliver (`AtRestBody::slivers`),
or one with no verdict (a dual's), holding a cylinder or cone wall whose
boundary includes an edge on a spiric carrier (a plane's section of a
torus seated on the wall) or a spline edge. Then any boolean against
it should refuse `BooleanError::Containment(WallOutlineUnsupported { face })`
or `PartialConeFace`. A dual-scalar operand may be the cheaper route,
since every one of its solids is read.

Moved from RESTREAD to CONTACTHOLD on 2026-11-20, by `git mv`, when RESTREAD merged into CONTACTHOLD: RESTREAD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

## Closed

Closed 2026-11-25 as **obsolete** (its premise is gone), in the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`), against main at `6003877ad`. `gate_sliver_orientation`, `census::boundary_unclaimable` and `AtRestBody::slivers` all have zero hits in `crates/`. `work/restfront/an-inside-out-slab-thin-enough-…-finishes-at-rest.md:108-111` (closed) records that the Boolean's sliver operand gate "is retired with" FLUSH's shell-role reader (check 7, PR 4990). `work/onehome/inside-out-operand-reading-…md:34` also says "`gate_sliver_orientation` is gone" The site whose typing the row wanted pinned no longer exists. If a dual's operand gate (`gate_unverdicted`) can still type `WallOutlineUnsupported`/`PartialConeFace`, that needs a new row against that code.
