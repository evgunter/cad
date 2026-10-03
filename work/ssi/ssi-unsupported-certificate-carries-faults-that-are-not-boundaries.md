---
id: ssi-unsupported-certificate-carries-faults-that-are-not-boundaries
kind: issue
title: geom-brep: SsiError::UnsupportedCertificate is raised for kernel faults and a degenerate operand as well as per-arm boundaries, so those sites end in no way through yet
status: open
opened: 2026-10-02
---

(SSI implementer, from the PR "SSI: every SsiError arm ends in the
recourse its decision earns", which closed
`ssi-refusals-whose-decision-has-no-ending`.)

## What

`SsiError::UnsupportedCertificate` is documented as "a documented
per-arm boundary (C12.1), never a fallback", and that PR ends it as such:
`NOT_YET_ENDING`, "There is no way through yet". Most raise sites fit
that: cone and torus have no implicit form, a widened analytic operand,
NURBS × NURBS, a NURBS operand with no traced pcurve
(`NURBS_LIMBS_NEED_PCURVE`), and a chart tube against a non-plane
(`CHART_TUBE_NEEDS_PLANE`). Several do not:

- **The kernel's own data, refused.** Its fault is the kernel's, so the
  ending should be the defect ending.
  - `crates/geom-brep/src/ssi/certify.rs`, `analytic_limbs`: "the fitted
    carrier's enclosure data is malformed" and "the implicit composite
    refused the fitted carrier".
  - `nurbs_limbs`: the carrier's, the pcurve's and the NURBS operand's
    "enclosure data is malformed", and "the tensor composite refused the
    carrier/pcurve pair".
  - `crates/geom-brep/src/ssi.rs`, `trace_plane_nurbs_uncertified`: "the
    ℝ⁴ trace must produce both pcurves". This is unreachable, since
    `fit_branch` given charts always returns both.
- **A degenerate operand.** These have a geometry lever.
  - `crates/geom-brep/src/ssi/exhaust.rs`, `sweep_r3`: a sphere or
    cylinder of zero radius.
  - `sweep_chart_plane`: a NURBS weight whose rational denominator
    underflows.

## Repair shape

Raise each non-boundary site as the refusal it is: a kernel-defect arm,
or the operand's own degeneracy with its lever. Only the true
boundaries should keep `UnsupportedCertificate` and its not-yet ending.
This is a type change, so it stayed out of the endings PR.
