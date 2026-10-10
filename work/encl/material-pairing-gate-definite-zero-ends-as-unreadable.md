---
id: material-pairing-gate-definite-zero-ends-as-unreadable
kind: issue
title: geom-brep/topo: the material-pairing gate's decided Zero reaches tier 3 as SliverDihedral{MaterialSide} with a flat defect ending that reads no margin
status: review
branch: encl/material-pairing-zero
pr: 4474
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

## Disposition (ENCL implementer)

- **Rule.** Every reader asks the pairing past a definitely-smooth dihedral at the same point and arm, so its margin `|n̂₊·n̂₋|·arm ≥ √(arm² − ε²)` is the folded arm. A decided Zero needs `arm ≤ √2·ε` past an arm gate that decided `arm ≥ K·ε`, so it is reachable only at `K < √2`. It is not a contradiction. Both band-decided arms (in band and decided Zero) are an arm too short to read a side over. The arm is a length the user may intend (`DIHEDRAL_ARM`'s), so per D4 ¶1 (i) both end in one message: the arm's lever, plus the tolerance the margin gives. A poisoned margin gets the lever and the unreadable-margin note. That is `geom_brep::MATERIAL_PAIRING`, and its lead is `MATERIAL_PAIRING_CLAUSE`.
- **The offer decides both readings.** A tolerance below `m/K` re-reads the wedge `w = sin θ · arm` too. Where `w > m/K` (that is, `tan θ > 1/K`), that wedge would land in band and the edge would refuse on its dihedral instead. So in that case the door quotes the wedge's own margin and offers `w/K`, at which the edge reads as a crease. This mirrors `DIHEDRAL_ARM`'s `at_wedge`.
- **Tier 3** reports a pairing stop as `WedgeCheck::MaterialPairing`. `WedgeCheck::MaterialSide` keeps the split and the cusp side, and it keeps `DEFECT`.
- **Split finish** reports it as `SplitFinishError::DescribeSideEscalated`.
- **`MaterialStations::after_positive`** reads through `decide_nonzero`. This closes RESTFRONT's `validate-material-side-zero-mints-an-indeterminate`.
- **`census` `pairing` (`.ok()`)** stays as it is. On one plane the margin is the reach, and a decided Zero is the in-band reading's sibling, so the touch verdict's one marginless refusal is right.
- **`census::ee_cross_backed` (`:2593`)** is held under D10: it is the declared-pair backing rung.
