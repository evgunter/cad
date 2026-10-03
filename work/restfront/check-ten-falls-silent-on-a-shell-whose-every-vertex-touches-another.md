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

## The one ladder (CLEAVE `cleave/ladders`, PR 3716)

The boolean's two witness walks are now one: `shell_witness.rs`
`complex_side`, read by the uncut-shell witness and by the join's
section-loop role resolution. It reads vertices, then each edge's
carrier at its parameter midpoint, then one certified interior point
per planar face. It passes over a witness that reads `OnBoundary`, or
one that reads too near the other boundary to say (`inconclusive`).
Check 10 asks the ladder's question against several shells at once and
sums the answers, so it cannot call `complex_side` as it stands. It
needs the ladder's witness sequence, which is internal to
`complex_side` today, so the fix would split witness generation out of
it. The same sweep filed this finding again as
`check-10-reads-a-shells-winding-at-vertices-only`. That row was
deleted in favour of this one.
