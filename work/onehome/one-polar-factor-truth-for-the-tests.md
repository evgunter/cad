---
id: one-polar-factor-truth-for-the-tests
kind: issue
title: The polar factor's double-double truth is spelled twice in tests (conic_oracle::exact polar, ssi::residual polar_truth) beside production enclose_rotation
status: open
opened: 2026-11-21
priority: P2
cost: E
---


## What

Two test oracles each spell the polar factor of a stored frame in double-double, `R = M(I − ½G)`:
- `crates/geom-brep/src/ssi/residual.rs:637` `polar_truth` (the certified-partner rows of PR 7131);
- `crates/geom-brep/src/curve_face/conic_oracle/exact.rs:148` `polar` (the conic rows' exact oracle).

Production reads the same rotation as an enclosure, `crates/geom-core/src/interval.rs:684` `enclose_rotation`. That is one truth, written twice for tests. Give the double-double polar factor one test-support home (geom-core's, beside `enclose_rotation`), and have both oracles read it.

Filed from the second delta review of PR 7131.
