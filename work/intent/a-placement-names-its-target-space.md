---
id: a-placement-names-its-target-space
kind: unit
title: D′1: a placement names the Space it places into; Space a minted kind (the world, new_space(), a.space); every body's space derived from the recipe and SpaceMismatch decided statically; every bundle pinned, so the loose copy retires
status: spec
opened: 2026-10-22
priority: P0
cost: H
parent: a-placement-pins-by-written-values-without-a-mate
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
