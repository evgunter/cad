---
id: a-placement-names-its-target-space
kind: unit
title: D′1: a placement names the Space it places into; Space a minted kind (the world, new_space(), a.space); every body's space derived from the recipe and SpaceMismatch decided statically; every bundle pinned, so the loose copy retires
status: closed
opened: 2026-10-22
priority: P0
cost: H
parent: a-placement-pins-by-written-values-without-a-mate
closed: 2026-10-22
pr: 5592
branch: intent/dprime1-placement-target-space
---



## What

INTENT stage 3, D′1. Spec: `docs/INTENT-STAGE3-SPEC.md` §5′.1,
which governs where this body and it differ. It builds PR 5494's
`[variables.kinds]` *Spaces* and `[placement.spaces]` (fork-log rows
189 and 191):

- `VarKind::Space`, minted three ways: the world's (`Node::World`),
  `new_space()` (`Node::Space`, new) and each construction's `space`
  output (`a.space`).
- `Place` and `InstantiatePart` read a `target`, a `Space` read,
  always written.
- `Doc::space_of` derives every body's space from the recipe;
  `Doc::space_faults` names every read across two spaces. The edit
  door, the façade's builder and Python refuse `SpaceMismatch`; a
  re-target leaves the reads it breaks unresolved, and the document
  still loads (spec §13 on `validate_document`).
- `unpinned` checks every bundle, so `mate::solve::Space`,
  `Space::Loose`, the loose copy and every reader of it, and
  `TargetsDiffer` retire.
- The viewer shows a set of spaces at display frames (DI5).

Interims it leaves for D′2 and D′3: `Target::World`, admitted only
where the target is the world; `Place`'s list of shapes; the
gathered product; an instance copying its part's world placements.

## Done when

The spec's §5′.1 rows, and tests 18 and 23. Geometry unmoved; the
wire, content key (VERSION 18) and var tables move.

**Absorbs:** `a-bundle-folds-in-the-joined-space`,
`a-placement-relates-two-frames`.

## Closed (2026-10-22)

Built on `intent/dprime1-placement-target-space`.

- `VarKind::Space`, minted by the world (`VarId::WORLD`, every
  document's, off the wire), `Node::Space` and each construction's
  `space` output (its last port).
- `Place` and `InstantiatePart` read a `target`; `SetParam` of the
  `target` slot re-targets, reporting `Maintenance::StrandedSpace`.
- `Doc::space_of`, `node_space`, `space_fault(s)` derive every space
  from the recipe; the edit door refuses `EditError::SpaceMismatch`
  for a node written whole, and evaluation refuses
  `NodeErrorKind::SpaceMismatch` for a stranded one. A re-point is
  reported rather than refused, as a re-target is, so a split or
  inline that re-points a node's reads one at a time passes through.
- Every bundle pins its copy; bundles are written whole
  (`DocEdit::SetConstraints`). `mate::solve::Space`, the loose copy,
  `TargetsDiffer`, `loose_spaces`, `ExportError::Loose` and
  `AssemblyError::Space` are gone.
- The viewer shows its spaces by their variables (DI5).

Left open: `incremental-constraint-edits-cannot-succeed-on-a-pinned-bundle`,
`deleting-a-space-strands-the-placements-into-it`,
`the-viewer-does-not-show-a-constructions-own-space`, and the census
of spaces other than the world (`a-parts-loose-spaces-and-member-reads-drop-its-census-rows`).
