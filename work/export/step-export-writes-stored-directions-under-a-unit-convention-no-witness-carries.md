---
id: step-export-writes-stored-directions-under-a-unit-convention-no-witness-carries
kind: issue
title: step-export's direction and axis2_placement emit stored carrier directions as unit by convention, with no witness carrying it
status: parked
opened: 2026-11-20
priority: P3
cost: E
blocked_on: [7131]
---


Re-homed from EXCH's `recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness` (closed by PR 7166), whose sweep named these as the carrier case on the way out.

`crates/step-export/src/writer.rs`, `direction`: "unit by the conventions of every stored normal/axis/dir — emitted as stored, never renormalized"; and `axis2_placement`: "every kernel frame stores `axis` unit, `ref_dir` unit and ⊥ `axis` … no renormalization". Both rest on an at-rest carrier convention no type carries. `[ev]` PR 7131 (open) changes how carriers hold their frames (`RigidMotion`); once it lands, these writers either read the witness the carrier then holds, or the convention stays prose and that is recorded at the claim site. Parked on it.

Signed: (EXCH orchestrator)
