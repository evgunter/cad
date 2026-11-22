---
id: the-tube-enclosure-has-a-cone-arm
kind: unit
title: The ℝ³ tube enclosures (implicit_enclosure, implicit_gradient_enclosure) and exhaustion have a cone arm: the double cone's elevation, gated off the apex
status: closed
opened: 2026-11-21
priority: P0
cost: H
parent: the-cone-has-no-implicit-enclosure-or-metres-conversion
branch: sectcone/cone-tube-arm
closed: 2026-11-22
pr: 7358
---


Unit 2 of the P0's weighed design (`the-cone-has-no-implicit-enclosure-or-metres-conversion`, `## Weighed`). It does not wait on `[ev]` PR 7326.

**Review tier: dual** (two Opus reviewers; Ev 2026-11-21). The gate's soundness is the whole content: "no second sheet in a box" underwrites Krawczyk's uniqueness. Difficulty letter: M.

## Spec

`crates/geom-brep/src/ssi/enclose.rs` `implicit_enclosure` and `implicit_gradient_enclosure` refuse the cone. Give them a cone arm.

1. **The function:**
   - **The function is** limb 1's own, the double cone's elevation `g = ρ cos α − |h| sin α`. Here `ρ` is the distance from the axis and `h` the height above the apex along it: `implicit_residual`'s / `cone_elevation`'s cone arm, a signed distance in metres.
   - **Its gradient is** `ŵ cos α − sgn(h)·â sin α`, which is unit.
   - **How to enclose it:** form `w = q − â·h` directly (the cylinder arm's reason), take `ρ = √|w|²` (C9 admits `√`), and use the mean-value form about the box centre met with the direct form, as the torus arm does.
   - **The angle:** `sin α` and `cos α` are crossed into `Interval` and THEN taken (`cross(half_angle).sin_cos()`, as `curve_face/composite.rs` does), never taken at `f64` and crossed.
2. **The gate.** A box is enclosed only where `h`'s enclosure excludes zero and `|w|`'s excludes zero; otherwise both enclosures refuse. This is the analogue of the torus's `A > 0`. Inside a gated box only one nappe exists, so the slice argument's "no second sheet" holds of the double cone, the B-rep carrier. Ungated, a box near the apex can hold points of both nappes without touching the axis. Write the row that goes red if the gate is dropped.
3. **The apex.** A box at the apex refuses. C5's node clause (`crates/geom-brep/README.md`; `[ev]` PR 6432) owns the point. Check how a rung-3 ℝ³ chain ends on a node today. If no ℝ³ node end exists, refuse typed and file the gap. Do not invent one.
4. **Exclusion** (`ssi::exhaust`, which reads only the sign): cells far from the surface that straddle the apex plane, where the gate fails, read the sign of the quadric `Q = cos²α·ρ² − sin²α·h²`, which has the double cone's sign exactly. Otherwise exhaustion refines them forever.
5. **`has_implicit_enclosure`** lists the cone. It repeats the roster of `pair_section::traced_in_r3`, so derive one from the other if they mean the same thing. A sibling lane (`sectcone/cone-ruled-side`) may be editing `traced_in_r3`; coordinate by merging main.
6. **Rows:** enclosure soundness at sampled points on both nappes and near the apex, the gate's refusal, and exclusion that terminates on a far straddling cell. Fuzz these per implementer discipline §8.

`route(Cone, *)` stays `implemented: false`: the pair doors are `cone-torus-rung-three-arm-waits-on-the-cone` and `cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door`. Out of scope: limb 2 (sibling unit `the-search-certificate-reads-limb-two-through-the-edge-reader`, concurrent in `ssi/certify.rs` and `pcurve_cache`).

Territory: `ssi/*` is SSIARITH/SSIEDGE ground. Run `python3 scripts/work.py territory --base origin/main` and announce the seam.

## Closed (2026-11-22, PR 7358)

The cone has its arm in the ℝ³ tube enclosures: the double cone's elevation, gated on `h` and `|w|²`. Exclusion reads the quadric off the gate. The apex refuses typed; the ℝ³ node-end gap is `ssiedge/an-r3-tube-chain-has-no-node-end`. Dual review: two Opus reviewers, both APPROVE-WITH-FIXES with no MAJOR, and the fix pass took the union. The cone's quadric spelled in seven scalars is `germ/the-cones-quadric-is-spelled-in-seven-scalars`.
