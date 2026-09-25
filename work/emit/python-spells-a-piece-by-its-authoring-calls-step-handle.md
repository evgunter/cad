---
id: python-spells-a-piece-by-its-authoring-calls-step-handle
kind: unit
title: Python spells a profile piece by the step handle its authoring call returned
status: open
opened: 2026-09-25
priority: P1
cost: D
---

## What

The second half of `profile-pieces-are-named-by-minted-step-ids`, split
off it (the coordinator's call on the build, 2026-09-25). The first half
gives every profile step a minted `StepId` and names a piece
`{ step, role }` (`crates/editor-core/src/names/README.md`, "N1, the
profile pieces"). Its Python surface is the raw one:

- `Doc.step_ids(profile)` answers the ids, one int per authored step;
- `Doc.pieces(profile)` answers each canonical segment's piece as opaque
  text;
- `DocEdit.set_program(node, outline, ids)` takes one kept id or `None`
  per new step;
- `band`, `band_pi`, `band_rim` and `meridian_vertex` take a piece's
  text;
- `SplitOutcome.step_map` / `InlineOutcome.step_map` pair the ids across
  a refactor.

So a Python author finds a piece by reading `Doc.pieces` at a canonical
position, which is the positional reading the rule exists to retire.

## Scope

- The authoring calls (`Open.at(...).line_to(...)`, the fillet and fused
  verbs, `circle`, `circle_split`) return a handle for the step they
  author, which the insert door's minted id backs once the program is in
  the document.
- A piece is spelled from that handle and a role (`Leg`, `RunIn`, `Arc`,
  `RunOut`, `Piece(k)`), and `set_program` keeps a step by its handle.
- `Doc.pieces` stays, as the reading for a caller that holds no handle.
- `pncad.pyi`, the binding census and the role-name rows move with it.

