---
id: an-in-band-certification-of-a-minted-edge-other-than-its-face-residual-reads-as-a-defect
kind: issue
title: An in-band certification of a minted edge other than its face residual reads as a kernel defect (describe pass) or as the fit's last resort (section describer)
status: open
opened: 2026-10-11
priority: P2
cost: M
---


## What

Found by the sweep of `cleave/folded-graze-join-desync`. That branch
types a minted edge's carrier read in band of one of its faces
(`CertCheck::SurfaceResidual` / `WitnessSurfaceResidual` escalated) as
`Escalated { BooleanDecision::EdgeOnFaces }` at `boolean/ops.rs`
`of_attachment`, the one in-band certification the near-tangent census
measured. Two neighbours of it are unmeasured:

- **The describe pass's other in-band checks.** `describe_edges` sends
  every other `CertifyError::Escalated` (`Transversality`,
  `TransversalityArm`, `TangentPlanes`, `TangentSecondOrder`,
  `TangentParallel`, `TangentHull`, `TangentTube`, the plane × NURBS
  and rung-3 rows) to `BooleanError::EdgeUndescribed`, which ends in
  the kernel-defect ending. Each is a `Value` margin in band, which D9
  row 1 calls the input's. They are not mapped onto the seam's own
  decisions (`SeamWedge`, `LeverArm(Seam)`, `SeamJet`) because the
  pass sets differ: the describe pass's seam reading passes a decided
  smooth seam, while the certification of an `Intersection` refuses
  one (`NotTransverse`), so the seam's tolerance offer would not hold
  there.
- **The splitting section describer** (`splitting/finish.rs`, the
  `classify_dihedral` … `set_edge_curve(…)?` arm): its
  `EulerOpError::Certification` reaches the user through the
  certification table at a build, where a `SurfaceResidual` in band
  ends in the fit's last resort ("loosen the tolerance … may be a
  kernel bug"). On a chord between vertices the split placed, that is
  the story `EdgeOnFaces` corrects at the Boolean.

## Owed

Measure whether either is reachable (a near-tangent census over the
split door and over seam poses at `|d| ≈ ε`), and type what is with
its own decision and lever.
