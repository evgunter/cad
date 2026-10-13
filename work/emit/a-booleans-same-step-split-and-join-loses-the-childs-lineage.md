---
id: a-booleans-same-step-split-and-join-loses-the-childs-lineage
kind: issue
title: A boolean step that splits an edge and joins the child away in the same step records no lineage for the child, so the Borders chase stops at it
status: open
opened: 2026-10-13
priority: P3
cost: M
---


## What

`Obstacles::record` (`names/borders.rs`) keeps `split_from` lineage in two ways:
- from each step's edges that are live at the end of that step;
- from the discarded faces' boundary chains.

Suppose a boolean step splits an edge, and its own output stage joins the new child away as `gone`. Neither source holds that child. A chase that goes from `kept` to `joined_from` then stops dead at it.

The Split op exports exactly this lineage (`SplitNaming::joined_lineage`, topo `splitting/finish.rs`), but `BooleanNaming` has no equivalent.

This was found by the PR 4949 review. The corpus does not reach it, so it is open whether any document does.

**Owed:**
- a fixture;
- a joined lineage export from the boolean, mirroring the Split's.
