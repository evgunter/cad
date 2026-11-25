---
id: the-one-shared-vertex-arm-reads-no-posed-presentation
kind: issue
title: The one-shared-vertex arm reads no posed presentation
status: open
priority: P2
cost: M
opened: 2026-11-23
---


## Finding

From EMBED's lane `embed/lune`, merging the landing train
`embed/wave3-land` at `f06547ccf53` (2026-11-23). The departure arm's
fourth separator (`departure` in `crates/topo/src/injective.rs`, the
plane through the pair's one shared vertex, with shared edges near)
re-presents each face against that vertex alone. It came from the
apex-separator unit. The pose door (`present_posed`) came from the lune
unit. The two met only at the merge.

- **What the arm reads now:** each face on its stored chart only
  (`Poses::none`, the store whose set is empty before any cell asks).
  Every pose stays open to separators 1 to 3.
- **Why it is shut:** neither review read a posed presentation
  re-presented against `{v}`.
  - Apex-separator's reviewer attacked the arm on stored-chart
    presentations: 960 mirrored and re-charted variants.
  - Lune's reviewer read poses under separators 1 to 3.
- **Measured** with the door open (lune at `1a3abc471e4`) and shut:
  the `injective::` unit set (151 rows) and
  `self_intersection_proves_injectivity` (75 rows) at ε = 1e-9, 1e-6
  and 1e-12 read the same verdict in every row and print the same
  receipts, so no row holds only through a pose in this arm.

## What it would take to open it

- **The superset argument for a posed rectangle against `{v}`:**
  - a pose places the cell by its own azimuth and latitude
    (`posed_cell`), and its boundary is the face's, lifted;
  - re-presented against the vertex alone, the posed rectangle must
    still hold the face's every point near `v`, and its corner or
    collapsed side must still be at `v`;
  - and the posed extent's sheets must read the same way they do for
    separators 1 to 3.
- **The mirrored-variant probe run with poses:** apex-separator's
  960-variant sweep, with each sphere face posed where its stored chart
  leaves it unread.
- **A pin:** a row red if the arm is handed the face's poses
  (`Poses::of`) without the argument above.
