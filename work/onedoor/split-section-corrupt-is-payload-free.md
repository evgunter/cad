---
id: split-section-corrupt-is-payload-free
kind: issue
title: SectionError::Corrupt is payload-free, and one of its sites is not a dangling key
status: closed
opened: 2026-11-12
priority: P3
cost: E
closed: 2026-11-14
pr: 6927
branch: onedoor/section-corrupt
---


Found by `topo-shared-cores-hosted-in-one-half`'s fix pass, which gave
`SplitFinishError`, `CarveError` and `JoinError` one statement of a key
that did not resolve (`topo::Unresolved`).

## What

`SectionError::Corrupt` (`crates/topo/src/splitting/section.rs`) is the
same payload-free arm that unit retired from `SplitFinishError`. It has
eight constructions:

- `plane_section`'s nesting maps `section_loops::NestFault::Torn(key)`
  to it and drops the key the nesting now carries.
- `section_walk` raises it seven times: for the loop, the walk of its
  cycle, each half-edge, its start vertex and that vertex's point (one `and_then` chain, so a
  dangling vertex and a dangling point read alike), its edge, the
  edge's curve, and `edge.claim(he)` answering `None`.

The last is not a dangling key: the edge resolved and does not claim
the half-edge, which is a broken edge/half-edge pairing. So the arm
covers two states.

## What closing it takes

`SectionError::Corrupt` carries `topo::Unresolved` (`Record` or
`Geometry` per site), and the unclaimed half gets an arm of its own.
Its `Display` ("the section traversal failed (corrupt body)") then says
what it found.

## Why it was left

`plane_section`'s error was outside that unit's list, and the second
state needs a new public arm.

## 2026-11-14: the arm panics and is retired (D2 row 4)

Closed the other way round from "What closing it takes": every one of
the eight raises is a record miss on the join's scratch body, so by D2
(*A torn body is a kernel bug*) none of them is a refusal. This is
`torn-body-refusal-families-beyond-the-six-doors`' `SectionError` line.

- `section_walk` reads its loop's members through
  `Body::loop_members_linked`, the start point through
  `linked_vertex_point`, the claim through `live::claimed` and the
  curve through `edge_curve_linked`. Each panic names the holding
  record and the key it read; the unclaimed half has its own message
  ("does not claim it in either slot"), so the two states this row
  found under one arm are told apart.
- The loop itself is the join's `CompletedSection::below_loop`; a miss
  panics naming the loop.
- `plane_section`'s nesting arm panics on `NestFault::Torn`, printing
  the `Unresolved` it used to drop.
- `SectionError::Corrupt` and its `Display` arm are gone. The
  lone-vertex below loop stays the typed `SectionInvariant`.
- `section::torn_walk` tears a cube loop one record at a time (the
  loop, a `next`, a claim, a curve, a vertex, a point) on
  `review_d18::assert_torn_op_panics`, each pinned by the holding
  record, the key and the premise. `review_d18`'s torn sweep does not
  gain the walk as a door: that needs `section_walk` visible
  crate-wide, which `[topo.homes]` (`engine-leaves.sh`) refuses.
- The walk is the claimed one (`loop_walk_of`, where it was
  `loop_cycle`), so a `next` that resolves and names another loop as
  its `parent_loop` is a torn walk too.
- `live::claimed`'s panic names `OPERATORS_KEEP_LINKS` beside the
  tier-1 premise, as the other link panics do, since the walk reads a
  mid-operation body.

## Closed (PR 6927, 2026-11-14)

DESIGN D2 governs: a torn body is a kernel bug. The eight `SectionError::Corrupt` constructions in `splitting/section.rs` therefore panic, naming the record, the key and the premise (`OPERATORS_KEEP_LINKS`), and the variant is retired. The reviewer traced every panic site to `plane_section`'s own scratch body: the walk reads only keys the join wrote, after every tolerance decision is already typed, so no valid body reaches them. The walk is now the claimed one (`loop_walk_of`), which adds the `parent_loop` premise. `NestFault::Torn` stands for the topo row's `SplitFinishError` line.

Model-compare row: `docs/model-compare-log/rows/PR-6927.md`.
