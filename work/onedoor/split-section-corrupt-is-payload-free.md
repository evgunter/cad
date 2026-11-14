---
id: split-section-corrupt-is-payload-free
kind: issue
title: SectionError::Corrupt is payload-free, and one of its sites is not a dangling key
status: open
opened: 2026-11-12
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
- `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`
  walks every loop of its torn bodies through `section_walk`, floored
  on a premise panic.
