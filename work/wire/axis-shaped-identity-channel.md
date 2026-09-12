---
id: axis-shaped-identity-channel
kind: issue
title: Build the ratified axis-shaped identity channel: a per-component, placement-composing source for axis-flavoured facts
status: open
opened: 2026-09-12
refs: [2404, 1593, 1604]
---


## What

**Ratified design, unbuilt.** `docs/AXIS-DECLARATION-DESIGN.md`
(Ev, 2026-09-12, PR 2404). This row is the channel it ratifies.

A declaration names an **axis**; coaxiality between two carriers is
derived from it, and whether it still holds is decided by comparing
placement provenance — token equality, zero numerics. Absence of
provenance **refuses** (`crates/verbs/README.md` §3 P3's precedent).

## What is actually new

Not a reopening of §3 P1. P1's exclusion is scoped to **motion-invariant**
fields and an axis is not one, so this sits on `GeomSource`'s side of the
line P1 draws — where `SourceExpr::Placed` composition already exists in
the kernel. Read `docs/AXIS-DECLARATION-DESIGN.md`'s Round 3 before
re-deriving that; an earlier reading had it the other way and was wrong.

What IS new is **granularity**. `GeomSource` identifies a whole
description (a surface key); an axis is a *component* of one. Two
different cylinders sharing an axis come from different expressions, so
their `GeomSource`s differ and the shared axis is not derivable from
them. A per-component source that composes through placement is the
work.

## The staleness rule, ratified

| since the declaration | chains | verdict |
| --- | --- | --- |
| neither placed | equal | holds |
| both placed by one node/instance | equal outer wrappers | holds — relative pose unchanged |
| one placed, or both by different chains | differ | **stale** — refuses structurally, naming the placement node that broke it |

Row three under-claims (two chains composing to the same relative motion
refuse though coaxiality survives). That is the fail-loud direction and
the row is re-declarable; it was ratified knowing this.

## Territory, and why this is not one lane's diff

- `crates/verbs/` is **WIRE's** — the vocabulary and the README clause.
- `crates/topo/src/source.rs` is **TOPO's** — `GeomSource`/`SourceExpr`
  live there and a finer granularity changes them.
- `crates/topo/src/boolean/join.rs` is **S-BOOL's/CURVED's** —
  `cs_pair_frame` is where a declaration is consumed, and its own
  sentence already says so and stays accurate.
- `crates/editor-core/` — the attach side, and the document-level
  declaration node.

So this is a sequence with announced seams, not a unit. Cutting it is
the first task; do not dispatch it as one lane.

## Blocked on nothing, but read first

`work/topo/geom-source-absence-conflates-four-origins.md` decides what
"no provenance" means, and this channel's refusal arm depends on that
answer being implementable. It is not a blocker — the refusal is correct
either way — but a channel built before it will refuse on four
situations it cannot tell apart, one of which is a bug in the re-stamp.
