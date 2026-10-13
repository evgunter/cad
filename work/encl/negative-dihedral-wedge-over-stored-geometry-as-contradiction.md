---
id: negative-dihedral-wedge-over-stored-geometry-as-contradiction
kind: issue
title: geom-brep: is a definitely negative dihedral wedge over stored geometry a StoredDefinite::Contradiction?
status: open
opened: 2026-10-13
priority: P3
cost: E
---


(Filed by the ENCL implementer of `import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span`, from its Answered section's "Filed separately" list.)

## What

`geom_brep::DIHEDRAL_ARM` (`crates/geom-brep/src/dihedral.rs`) is `StoredDefinite::Contradiction`, so a definitely negative reading read at rest ends in `KERNEL_OR_FILE_DEFECT_ENDING` (`SizedDecision::recourse`, `crates/geom-brep/src/recourse.rs`). Its margin is the wedge the arm meters, `sin θ · arm`. Whether a definitely negative wedge over stored geometry is a contradiction between a stored description and its stored geometry, which no move of the geometry reaches (D4 ¶1 (iv)), or a configuration the lever does reach (`StoredDefinite::Lever`), has not been argued.

## Question

Which `StoredDefinite` does the arm's negative wedge belong to? Pin the answer with a row at rest.
