---
id: two-cell-dimension-witness-ladders-join-roles-and-the-uncut-shell-have-drifted
kind: issue
title: Two cell-dimension witness ladders — join's section-loop role resolution and the uncut-shell witness — have drifted apart
status: review
opened: 2026-10-01
priority: P1
cost: M
branch: cleave/ladders
pr: 3716
---



## What

Two places ask the same question: on which side of the other operand
does this piece of boundary lie, read from its first point off that
operand's boundary. Each answers it with its own ladder over the cells
of the piece.

- `crates/topo/src/boolean/join.rs`, `resolve_roles_geometric` (its
  `Anchor` tiers). It tries vertices, then each region edge's CHORD
  midpoint, then each curved edge's carrier midpoint, then certified
  region-interior points. It probes BOTH section loops and
  cross-checks them: agreeing verdicts refuse `SectionLoopMixed`. The
  chord-midpoint tier is unsound for a curved edge, and its own doc
  says so (`work/zip/role-resolution-interior-tiers-certify-only-planar-region-faces`).
- `crates/topo/src/boolean/shell_witness.rs`, `shell_side`. It tries
  vertices (skipping contact vertices), then every edge's carrier
  midpoint (never a chord), then one certified interior point per
  planar face. It takes its first witness with no cross-check.

Since PR 3655 the two share their candidate generation
(`face_loop_points`, `triple_centroid`, `chord_midpoint`) and the
certificate (`certified_in_face`). The ladders themselves are still two
implementations of one logic, and they already differ in three ways:
which edge point they probe, whether contact vertices are skipped, and
whether a verdict is cross-checked.

## What a fix would be

One ladder that both callers run. Each caller states only what it adds:
join adds the two-loop cross-check, and the shell witness adds the
contact skip. The chord tier is either dropped or kept only for
straight edges, where the chord midpoint IS on the edge.
