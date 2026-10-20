---
id: the-dihedral-arm-noun-is-spelled-twice-and-its-docs-still-say-edge-length
kind: issue
title: geom-brep: the dihedral arm's noun is spelled in ArmCollapsed's Negative render and in CertCheck::TransversalityArm, and ArmCollapsed's and DIHEDRAL_ARM_CLAUSE's docs still say the edge is too short
status: closed
pr: 5437
opened: 2026-10-19
priority: P4
cost: E
branch: encl/one-noun-per-decision
closed: 2026-10-20
---


(Filed by the ENCL orchestrator from the PR 5157 delta review and its main-merge lane.)

## What

`crates/geom-brep/src/certify.rs`:
- **The arm's noun is written twice.** `CertifyError::ArmCollapsed`'s Negative Display spells "the length the faces' angle is measured over is below zero". `CertCheck::TransversalityArm`'s name (`CertCheck::name`) spells "the length its faces' angle is measured over" separately. These are two spellings of the one noun `DIHEDRAL_ARM` decides, and they already differ ("the faces'" vs "its faces'"). The Zero arm composes `DIHEDRAL_ARM_CLAUSE` (`crates/geom-brep/src/dihedral.rs`), which TANG's U-lever reworded to the faces' extent. The Negative arm's noun was not reworded with it.
- **The variant doc is stale.** `ArmCollapsed`'s doc still says the "edge is too short, or a face curves too tightly there". Since the U-lever change, the arm is the faces' extent clamped by curvature, not the edge's length.

## Repair shape

One home for the arm's noun, read by the check name and by both arms of the Display. Re-word the variant doc to the faces' extent. Sweep `rg -n "too short|edge's length|own length" crates/geom-brep/src/{certify,dihedral,recourse}.rs` for other docs the U-lever change left behind.

Overlaps `dihedral-arm-lever-says-curves-tightly-of-a-radius-below-zero` (the note's radius clause), which the proven-zero implementation retires. Land after it, or together.
