---
id: main-red-a-torus-carriers-limb-two-residual-exceeds-its-bound-by-rounding
kind: issue
title: main red (fuzz): ssi::certify a_torus_carriers_limb_two_holds_its_sampled_residual — residual 1.6963256895753353e-8 over its bound 1.6963256826300393e-8 (seed 0x16381c9c5b5fb469, eps 1e-6)
status: open
opened: 2026-11-06
priority: P1
cost: E
---


Found by PR 6553's hosted CI (test (kernel), at the 1e-6 eps step), and reproduced on origin/main by the orchestrator with the same seed:

```
CAD_FUZZ_SEED=0x16381c9c5b5fb469 CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6 \
  cargo test -p geom-brep --lib a_torus_carriers_limb_two_holds_its_sampled_residual
```

The failure: `crates/geom-brep/src/ssi/certify.rs:4240`, span 3 at t = 1/3. The residual is 1.6963256895753353e-8, its bound is 1.6963256826300393e-8, and the lift is 9.644291594131149e-11.

The torus is tiny (major radius 0.0038, minor radius 0.0020, about 1.27 from the origin). So the sampled residual exceeds the certified bound by about 7e-17, a relative 4e-9 of the bound.

A bound that a true residual exceeds is a soundness defect, however small. Find which term of the limb-two bound misses its own rounding (likely an absolute-vs-relative scale at a small torus far from the origin), fix it at its home, and pin this seed.

Note: ONECOPY's recent `Line` and normalise changes (PRs 6562, 6568) touched ssi and torus readings. Check whether one of them moved this.
