---
id: carry-a-grafted-shell-s-local-verdict
kind: issue
title: A later finish could carry a grafted shell's tier-3 local verdict; it waits on [topology.tiers] and fork C
status: open
opened: 2026-11-17
refs: [7079, a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs]
---


Recorded from the ONEHOME operand-coordinates lane (PR 7088's session),
2026-11-17, after the orchestrator parked the carry behind `[ev]` PR
7079. It refines
`work/onehome/a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs.md`:
that row asks for the carry; this one records what a carry would have
to be and why it waits.

## Why it waits

- **`[topology.tiers]`.** `docs/DESIGN.md`'s at-rest row reads "the
  scalar's `finish`, the one place a body's verdict is derived". Tier
  3′ says a finished body "passes both once, at the door that built
  it", and the only carry the text names is the isometry's: "An
  isometry, exact apart from rounding, keeps a finished body's
  verdict". Carrying a source's facts into the finish of a new body (an
  aggregate, a Boolean's result) goes past that clause, so it is Ev's
  call. The orchestrator raises it once 7079 is answered, with option 2
  below as the candidate.
- **Fork C (7079).** The two sites it would serve are the product
  gather's aggregate (`crates/editor-core/src/product.rs`, the one
  `T::finish` after the graft loop) and `distribute::fused`
  (`crates/editor-core/src/eval/wire/distribute.rs`). Both carry only
  if a graft is construction, the copies' data mapped into one body
  (C1, today's behaviour). Under C2, an aggregate that holds placed
  members and no grafted body, there is nothing to carry and the row
  closes empty.
- **Fork A (7079).** A Boolean's untouched faces are a third site. Under
  A1 they are images of the non-computing operand, mapped once at the
  join, so a carry there is a carry across a map, a question fork A
  decides.

## What the verdict holds today

`AtRestBody` (`crates/topo/src/validate.rs`, `struct AtRestBody`) keeps
the body, a `KeptSign` (check 7's sign certificate) and the held finish
record. It keeps no per-entity findings, so any carry is new machinery
in the gate: the finish takes finished sources and a key map, and for
each family it carries, a reading of what that family reads.

## The families, by what each reads

Read from the check headers in `tier3_local_checks_marked`
(`validate.rs`), not from a full audit of each family's reads. A carry
of a family is sound only where everything it reads is bit-identical to
a source's.

- **Per face, own data:**
  - check 1, surface implementedness: the face's surface;
  - check 3, planar corners on their plane: the surface, and the
    vertices of the face's loops;
  - check 5, planar containment between vertices: the surface and the
    face's edges;
  - check 6, loop-role winding (planar, curved and chart-singularity
    arms): the face's loops and their curves;
  - check 9, ring against outer disjointness, nesting, one region per
    face: all of the face's loops.
- **Per edge, its faces included:**
  - check 1's carrier half: the edge's curve;
  - check 2, carrier re-certification and description adjacency: the
    curve, both faces' surfaces and the end vertices;
  - check 4, the material wedge: the curve and both faces' surfaces;
  - check 8, the stored pcurve caches: the edge's uses and each face's
    chart.
- **Per vertex:** check 11, no joinable vertex at rest: the vertex and
  its incident edges' curves.
- **Never carried per entity**, because a graft adds neighbours:
  - check 7, the +V global orientation invariant;
  - check 10, shell winding, read per solid;
  - the tier-3′ coincidence census and contact-record certification;
  - the embedding (injectivity) question.

## Options

1. **A content-keyed per-entity carry.** Each family declares its read
   closure and is carried where the closure is bit-identical. This
   needs the closure audit across `validate.rs` (cost H) and the
   `[topology.tiers]` ruling.
2. **A shell's local verdict, carried across a disjoint graft**
   (recommended). A member shell grafted untouched, its keys only
   remapped, keeps checks 1–6, 8, 9 and 11. Checks 7 and 10, the census
   and embedding re-run over the aggregate. It is safe by construction,
   since no new neighbour enters a shell, and it covers the gather and
   `distribute::fused`. It assumes C1, and C2 deletes it. Cost M.
3. **Wait** for 7079 and the `[topology.tiers]` ruling (current).

## Gate

As the parent row: carry only behind a from-scratch agreement pin. On
the corpus' products and distributes, a finish that carries agrees with
one that re-derives, finding for finding.
