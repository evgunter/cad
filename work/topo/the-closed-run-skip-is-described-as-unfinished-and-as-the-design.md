---
id: the-closed-run-skip-is-described-as-unfinished-and-as-the-design
kind: issue
title: The merge door's closed-run skip is described as unfinished work, and the sweep README calls keeping those cuts the design
status: open
opened: 2026-11-02
priority: P4
cost: E
---


## What

Two texts describe the same behaviour two ways.

- `crates/topo/src/merge_faces.rs`, `MergeCoplanarError::PeriodClosure`:
  a run whose merge would close a curved face's period "refuses here and
  the driver records a loud skip", because those shapes are ones "the
  exact-B-rep props cannot integrate yet", and its FLIP NOTE says
  re-evaluating the skip "belongs to the curved-face props owner":
  unfinished work.
- `crates/sweep/README.md`, "Walls: one per run": a run that is the
  whole closed loop of k ≥ 2 pieces "keeps its authored cuts (C12.5)":
  the design.

Naming leans on which one is true: a sunk boss's two wall pieces stay
two faces in a union exactly because the merge skips the closed run
(`emit_union_borders::a_sunk_boss_joined_to_its_plate_names_its_wall_pieces_apart`),
and a union's members that open the run only together are
`work/refs/a-union-whose-fold-glues-a-run-no-judgement-opened.md`. If the
skip is lifted when props integrate the closed form, those wall names
change. Settle which text states the rule and make the other agree with
it (the sweep README is CARVE's and LATHE's ground).
