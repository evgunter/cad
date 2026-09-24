---
id: lane-4p-door-pins-in-one-shape
kind: unit
title: LANE-4P: the four door values' pins in one shape; the certified-enclosure census counts doors as well as scalars
status: closed
opened: 2026-09-24
closed: 2026-09-24
branch: scalar/lane-4p
pr: 3165
---


## What

The first of LANE-4's two cuts (H5 ruling 3's last trait). The four
door values — `geom_brep::OffsetFitLane`, `topo::QuadLane`,
`topo::RegionLane`, `topo::ShellDoor` — get their fn-pointer wiring
pinned in one helper shape each, and
`crates/topo/tests/certified_enclosure_impl_census.rs` learns the axis
it lacks: it enumerates door VALUES (every door constructor in
`crates/*/src`) and requires a roster entry for each, with each entry
saying whether its door is formed at every certifying scalar or at
`f64` only. Takes the filed row
`the-shell-door-is-a-third-door-value-the-certified-enclosure-census-does-not-know`.
Test code and prose only. Spec: `docs/LANE-4P-SPEC.md` (deleted at
merge). Survey: `/home/user/scalar-briefs/survey-lane4.md` §7.

**Review tier: SINGLE** (one Opus reviewer) — small, test-only, no
design decision; the risk is a census matcher that looks strict and is
blind, which the red-first rows and a reviewer's own plants check.

## Closed (2026-09-24) — PR 3165

Each of the four door values (`OffsetFitLane`, `QuadLane`, `RegionLane`,
`ShellDoor`) has one `Result`-returning wiring helper called by one row
per scalar it is formed at; the shell door's pointer pin moved out of
the policy gate rows (one policy comparison kept: it catches a literal
door in a policy arm). `certified_enclosure_impl_census.rs` reads four
`Roster { door, file, helper, formed }` entries (`formed` = certifying
scalars or `F64Only(reason)`), enumerates door VALUES by their
constructors (incl. `where`-clause bounds, `Option<Self>`/`Result<Self,
_>` constructors, lifetime-parameterised impls; unreadable heads red),
requires every fn-pointer field to be compared in the helper, carries a
`NOT_A_DOOR` exemption list (empty), and collects every failure before
it reds. Single full Opus review, APPROVE-WITH-FIXES 0/3/3; the fix pass
took all twelve items (one shared-reader change in `test_utils::source`,
`where_at`). Head `4c473313d5`, run 36013029381 green. Filed: TINT's
three impl-block walks and `source::line`'s column-0 off-by-one; SCALAR's
`QuadLane` formation sites. LANE-4 lands `FittedLane<T>` in this shape.
