---
id: door-wiring-rows-justification-written-four-times
kind: issue
title: The function-pointer wiring rows' justification is written out at four door modules
status: closed
branch: scalar/hygiene
pr: 3449
opened: 2026-09-25
closed: 2026-09-29
priority: P4
cost: E
---


## What

The paragraph justifying the door wiring rows — "A row that compares
outputs cannot see a door re-pointed at a routine that agrees on the
fixture … Function-pointer identity is what `std::ptr::fn_addr_eq`
compares and is not a language guarantee …" — is written out four
times, once per door module:

- `crates/geom-brep/src/fitted_lane.rs` (`wiring_rows`' doc);
- `crates/geom-brep/src/offset_fit_lane.rs` (its `wiring_rows` doc);
- `crates/topo/src/chart_region.rs` (the `RegionLane` wiring rows);
- `crates/topo/src/props.rs` (the `QuadLane`/`ShellDoor` wiring rows).

Each copy can drift from the others, and a fifth door will copy it
again. Raised by LANE-4's R2 (S1).

## Proposed

State the argument once — beside LANE-4P's census
(`crates/topo/tests/certified_enclosure_impl_census.rs`, which already
knows every door and its helper) or in a crate README clause — and
have each module's `wiring_rows` doc cite it in one line.

## Cost

E: four doc comments.

## Closed (2026-09-29) — PR 3449 (SCALAR-HYGIENE)

Landed with the unit; the PR body says what was done for this row.
