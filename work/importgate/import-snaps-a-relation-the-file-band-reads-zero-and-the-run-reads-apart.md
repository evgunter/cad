---
id: import-snaps-a-relation-the-file-band-reads-zero-and-the-run-reads-apart
kind: issue
title: An imported relation the file band reads Zero and the run reads apart refuses until the snap catalogue makes it exact
status: closed
opened: 2026-11-18
closed: 2026-11-19
---



Found by `importgate/build` (PR 7128), which builds every imported cache
from the description the file band reads.

## What

D7 step 3's snap catalogue: a relation between two surfaces that the
file's band reads `Zero` (tangent, coaxial, a plane normal to an axis)
and the run's band reads apart is made exact, so the build constructs
the relation the file states. Until it lands,
`crates/step-import/src/build.rs` refuses such an edge
`StepImportError::NotYetSupported`, naming this row (`SNAP_LANE`):

- `analytic_pair`: the file band builds a section and the run's band
  reads another verdict (tangent, routed to the general rung for a pose
  the file band reads coaxial);
- `tangent`: the file band reads a tangency the run reads apart
  (`TangentLocusError::NotTangent`), or a coaxial tangency the run does
  not read coaxial;
- `spline_pair`: the file band builds a plane × wall or wall × wall
  section the run does not.

`nist/nist_ftc_09_asme1_rd.stp` refuses at every ε on it
(`tests/tier_gate.rs`'s `NIST09_SNAP`; its own row
`wild-nist-ftc-09-refuses-import-at-eps-1e-12`).

## What would close it

The catalogue's snaps, applied at interpretation and reported on the
ledger, and the `SNAP_LANE` arms built rather than refused.

- 2026-11-19 — Closed by PR 7135 (the snap catalogue, D7 step 3).
