---
id: ks-admitted-floor-rises-above-2
kind: issue
title: K's admitted floor rises above 2 in the code ([verdicts], PR 6625)
status: open
opened: 2026-11-16
priority: P2
---

Ev ratified on PR 6625 (2026-11-16, "ok sounds good!") that K's admitted floor is **strictly above 2**. At K > 2 the at-rest gate alone makes a point's piece unique, at any chain length. At K = 2 two non-meeting pieces exactly 2ε apart still tie. DESIGN `[verdicts]` now says so, but the code still admits any K > 1.

- `crates/geom-core/src/tolerance.rs:135-137`: `admissible`'s `k > 1.0` becomes `k > 2.0`. Update its doc (`:128-133`), the `Tolerance.k` doc (`:283-295`), and `eps_min`'s doc if it names K near 1.
- **Delete the low-K note** at `tolerance.rs:289-291`: "Below `K ≈ 1.272` `extrude` admits chords the dihedral lever cannot resolve … refuse at rest (`SliverDihedral`)". Ev asked for it to go, and at K > 2 that regime is no longer admitted. Check whether its tests, which sit in the files listed by `git grep -l SliverDihedral`, pin behaviour that only K < 1.272 reaches. Retire those, or move them to an admitted K.
- **Admission rows at K ≤ 2:** `tolerance.rs`'s own rows at K = 1.5 and K = 1.0001 (`:1142`, `:1204-1224`) now read "not admitted". About 90 sites in crates spell a K of 1.x. Sort the ones that go through admission (`Tolerance::init`, `ENV_K`, `admissible`) from those that build a `Band` directly at a stated K, which need not change.
- **The three K = 2 fixtures** from the PR's audit: `crates/topo/src/splitting/classify.rs:434` (the tie itself), `crates/topo/src/boolean/join.rs:7685` and `crates/topo/src/pcurves/lift_rows.rs:82`. Move each just above 2, or retire it where its point was the tie.

**Done when:** `admissible(eps, 2.0)` is false and `admissible(eps, 2.0 + δ)` is true where the headroom allows; no doc names a K below 2 as admitted; and the default K = 10 runs unchanged at all three ε.
