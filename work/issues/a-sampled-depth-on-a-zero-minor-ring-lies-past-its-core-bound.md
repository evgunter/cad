---
id: a-sampled-depth-on-a-zero-minor-ring-lies-past-its-core-bound
kind: issue
title: census depth_gap_fuzz: a sample on a ring of minor radius 0 lies 7.3e-14 deeper than its f64 upper bound on a Core, past REL·scale (varying-seed red, seed 0xf23e6cc2b421332e)
status: open
opened: 2026-11-03
---


Filed by the PCERT orchestrator from a red hosted run on PR 6430 (E1e). The red is not E1e's: that head differs from its green parent `58b769fbf8` only by a doc comment in `crates/geom-brep/src/recourse.rs`, and E1e touches no census code.

## What

`topo census::depth_gap_fuzz::no_sampled_depth_lies_past_its_bounds` (`crates/topo/src/census/depth_gap_fuzz.rs:198`) seeds per run. On hosted run 41606346459 (job 125817462189, `test (kernel)`, default ε) it failed at draw 350:

- **The sample:** `Point3 { x: -0.1459783618620422, y: -0.16896468645510343, z: 0.9028877856079208 }`.
- **The piece:** `Ring { center: (-0.1435680721124965, -0.9712272601453655, 1.0906327723752076), normal: (0.1621148403439824, -0.24841092994385108, -1.0635785230039168), major: 0.8239412762430638, minor: 0.0 }`.
- **The miss:** the sample lies `7.30526750203353e-14` deeper than the f64 upper bound `1.4300958984235959`.
- **The bound's depth:** `Core { center: (-0.018109930354167858, -1.1634685971943752, 0.2675459727521021), axis: (0.16761070131833208, -0.2568323177241648, -1.0996349363793418), major: 1.9706783524599016 }`.

Reproduce with `CAD_FUZZ_SEED=0xf23e6cc2b421332e CAD_FUZZ_EFFORT=1`.

The row's tolerance is `REL = 1e-14` relative to the draw's scale, and its doc says the worst of four effort-400 runs stood 1.6e-15 past. This draw is about 7× past the tolerance. Either the f64 `sup_on` for a Ring piece against a Core depth is not an upper bound at `minor = 0` (a degenerate ring, a circle) with a non-unit normal and axis, or `REL` is too tight for that class.

## Not yet done

- Not reproduced locally.
- Not bisected.
- The owning program is not obvious from `work.py territory`.
- The row's last change was `f097a63b84` ("topo: the edge doors read the edge's cover").
