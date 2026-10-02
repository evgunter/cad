---
id: a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync
kind: issue
title: A flush wedge sunk into a block, its continuation declared, refuses its intersect and subtract JoinDesync though every face is planar
status: open
opened: 2026-10-02
priority: P1
cost: M
---


Found by REACH (`reach/door-backstop`), measured on the merge of
`origin/main` at `71a5bbf6` (TANG #3795).

## Repro

`crates/topo/tests/door_backstop_settled_residue.rs`, the sunk pose.
The block is `3 × 4.5 × 1`. A parallelepiped is cornered at
`(0.5, 0.2, 1)` by a 5° wedge angle, 0.5 deep, and its top face is
tilted about the wedge's one edge by `±2ε`. That is the tilt the
declared door reads in band across both faces (`reduce.rs`
`a_coplanar_sectors_in_band_residue_builds_through_the_lump_where_the_door_bridges_it`).
The pair (block top, wedge top) is declared `Continuation`.

- `A ∪ B` builds at `vol(A)`, and `B ∖ A` is empty: both correct.
- `A ∩ B` and `A ∖ B`, at both tilt signs, refuse
  `JoinDesync { what: "neither section loop's regions hold a decisive
  witness" }` (`boolean/join.rs` `loop_roles`).

At ε = 1e-9, 1e-6 and 1e-12 alike. The oracle is box arithmetic:
`vol(A ∩ B) = sin 5° · 0.5`, `vol(A ∖ B) = 13.5 − sin 5° · 0.5`.

## Why it is a finding

`loop_roles`' doc says both loops fail to decide only where every
witness reads the other boundary or too near it, "which a crossing's
two flanks cannot both do unless their faces are all curved". Here
every face is planar. The section loops lie on the declared flush top,
and each loop's flanking regions read ON the other boundary. That is
the declared-flush case the doc names for one loop, reached by both.
The refusal is a `JoinDesync` (the kernel-defect class) on a legal
declared input. Before #3795 the same pose built both ops at the old
`5.5ε` tilt, which the door now contradicts. The `2ε` tilt was not
measured before #3795.
