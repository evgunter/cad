---
id: the-import-door-has-no-ending-for-a-margin-between-eps-in-and-k-eps-in
kind: issue
title: the import door's shared in-file ending has no sentence for a margin in (ε_in, K·ε_in), so a reading the file's band cannot call falls through to a run-tolerance offer that would not decide it
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by FOREIGN's review and fix pass of PR 6007 (`step-import` `normalize/singularity.rs`, the `SingularityUndecided { band: File }` arm).

`geom_core::MarginDiag::sized_recourse_in_file` (`crates/geom-core/src/predicate.rs`) gives its "uncertainty declared below X" sentence only where the margin's nearer end lies within ε_in. A margin past ε_in but inside the file's band (ε_in, K·ε_in) falls through to the run-tolerance offer ("tighten the tolerance below m/K"). That offer is false for a reading decided at the file's band: measured on PR 6007's `polefrustum` cell at 1e-12 (margin 9e-10, ε_in 1e-10), a run tolerance below 9e-11 still leaves the file's band unable to call it, and the import still refuses. The true lever is a smaller declared ε_in (any ε_in below m/K decides it).

PR 6007 therefore ends that arm in a hand-written `LeverOnly` (`SINGULARITY_IN_FILE_BAND`) with no value. Owed: one in-file ending for a margin in (ε_in, K·ε_in), naming the ε_in that would decide it, and that arm routed through it (its two pinned texts in `step-import`'s `tier_gate.rs`, `POLEBAND_POLE_IN_FILE_BAND` and `POLEFRUSTUM_POLE_IN_FILE_BAND`, re-pinned). Other import-door decisions read at the file's band are candidates of the same class.

Signed: (FOREIGN orchestrator)
