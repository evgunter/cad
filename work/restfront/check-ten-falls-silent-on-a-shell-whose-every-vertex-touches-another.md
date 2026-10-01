---
id: check-ten-falls-silent-on-a-shell-whose-every-vertex-touches-another
kind: issue
title: Check 10 reads a shell's winding only at its vertices, so a shell whose every vertex touches another shell is left unchecked where an edge or face interior point would decide it
status: open
opened: 2026-10-01
priority: P2
cost: M
---



## What

`validate.rs`, `shell_winding_errors` (check 10), reads the winding the
other shells put on a shell at "the first vertex of it that touches no
other shell". A vertex that answers `OnBoundary` is passed over, and the
shell is silent when every vertex touches. The function's doc states
this silence and lists it in `validate_geometric`'s not-yet-checked
residue, so nothing is mislabeled. But a shell whose vertices all lie on
another shell's boundary can still have edges or face interiors off it
(the four-walls-flush shape), and those points would decide the winding.
The boolean's uncut-shell witness reads them
(`crates/topo/src/boolean/shell_witness.rs`, found by CLEAVE's
`a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted`).

## Measured

Not measured. This comes from reading the code during CLEAVE's sweep
for vertex-only witness walks.

## What a fix would be

Take the witness candidates from `shell_witness` after the vertices.
That would shrink check 10's documented silence to shells with no
witness off every other shell.
