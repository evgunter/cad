---
id: material-pairing-gate-definite-zero-ends-as-unreadable
kind: issue
title: geom-brep/topo: the material-pairing gate's decided Zero reaches tier 3 as SliverDihedral{MaterialSide} with a flat defect ending that reads no margin
status: open
opened: 2026-10-09
priority: P3
cost: M
---



(Unit 1 of `hand-minted-invalid-gates-in-topo`'s scope.)

## What

`geom_brep::dihedral::classify_material_pairing_as` decides the material side through `decide_nonzero`, so its gate rejects a definite Zero. That verdict reaches tier 3 through `validate::MaterialStations::before_decision` → `Stopped` → `SliverDihedral { MaterialSide }`. The ending there is a flat defect ending that reads no margin, and the lead text says the side "could not be read consistently", which is false for a decided Zero. A genuinely in-band pairing gets the same defect ending.

Other readers of the same cause:
- `topo::census` `:2593` ends the tagged cause with declare/move;
- `splitting/finish.rs` `:832` reads it;
- `census` `:4241` drops it with `.ok()`.

Fold in `validate::MaterialStations::after_positive`'s cusp-side Zero (`validate.rs` ~5591, moved there by PR 4433). It is RESTFRONT's `validate-material-side-zero-mints-an-indeterminate`, and the same fix: `decide_nonzero` instead of a hand-minted INVALID.

## Repair shape

Let the gate's definite Zero carry its decided margin (`MarginDiag::rejected_sign` / `Refused::rejected`) and end as its own decision at every reader. A decided Zero side is a sliver wedge, so its ending is the lever with no tolerance unless D4 says the size is the user's. An in-band pairing keeps the escalation ending. Pin each reader's text. RESTFRONT's row closes with this unit.
