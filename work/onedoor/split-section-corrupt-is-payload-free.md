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
