---
id: span-reach-passes-the-whole-turn-by-an-ulp
kind: issue
title: span_reach_hunt: a whole-turn ellipse arc's span reach passes lever_from by one ulp (fuzz counterexample, reproduces on main)
status: open
opened: 2026-11-03
priority: P2
cost: E
---


Found by the frame-readings unit's full geom-brep run (slow set included, PR 6463). It reproduces on main (`origin/main` at the unit's merge) with the same seed and the same numbers, so it is not that PR's.

`crates/geom-brep/tests/span_reach_hunt.rs` `a_conic_arcs_span_reach_is_never_short_of_its_farthest_point`, with `CAD_FUZZ_SEED=0x8b7ff4d7a91c64d1 CAD_FUZZ_EFFORT=1`. An ellipse with equal semi-axes `1000` over `[8.5169, 17.5946]` (more than a turn), read from its own centre: `Reach::span_reach_from` gives `1000.0000000000002`, one ulp past `Reach::lever_from` (`1000`). That breaks the assertion "never past `lever_from`". `span_reach_from`'s docs say that over a turn or more both read the whole conic, one from the span's start and the other from the carrier's zero. The two covers' windows start at different parameters, and their rounding differs by an ulp.

Fix: either the third assertion allows the two covers' rounding (a `tol` like the first two get), or `span_reach` over a whole turn reads `lever_from` itself. Then pin this seed as a deterministic row (discipline §8).
