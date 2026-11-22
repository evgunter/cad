---
id: the-cone-torus-pair-door
kind: unit
title: Cone × torus in general pose has its rung-3 door: a pose on the section value with its tangency and apex margins, the ssi door, route flipped, the join carried
status: spec
opened: 2026-11-22
priority: P0
cost: H
parent: cone-torus-rung-three-arm-waits-on-the-cone
branch: sectcone/cone-torus-door
---


**Review tier: dual** (two Opus reviewers; Ev 2026-11-21). The pose's tangency decision and the apex margin are certified logic on a quartic pair. Difficulty letter: H.

## Spec

`crates/geom-brep/src/intersect.rs` `route(Cone, Torus)` answers `implemented: false`. Both halves of the ℝ³ certificate now take the cone: the tube arm (PR 7358) and limb 2 at the search through the one reader (PR 7350). The pattern to follow end to end is sphere × torus:
- the pose: `SphereTorusPose` in `pair_section/torus.rs`;
- the door: `ssi::sphere_torus_ssi` (pair gate, the pose's own tangency decision before any rung, then `past_the_gap` and the ℝ³ march);
- the carried value: `PairPose::SphereTorus` in `pair_section.rs`;
- the join: its `rides_carried` arm in topo's `join.rs`.

1. **A cone × torus pose on `geom_brep::pair_section`** decides its Zero margins in closed form where a closed form exists:
   - **Tangency** between the cone and the torus.
   - **The apex on the torus:** C5's node clause (`[ev]` PR 6432). The form on the torus's tangent plane gives node, touch or refusal, never R-tan.
   - **The coaxial pose:** stays `cone_torus_section`'s closed form (do not route it to the march).
   - **The parallel-axis pose:** has its own row (`a-cone-against-a-parallel-axis-torus-has-no-section-arm`, P3). Refuse it typed if this unit doesn't take it, and say which.
   
   The pose is an instance of the section object, never a per-pair frame. Components are `ClassCarrier::General`, marched by `PairSection::carry`.
2. **`ssi::cone_torus_ssi`**, the door, following `sphere_torus_ssi`: a tangent pose refuses before any rung. Flip `route(Cone, Torus)` to `implemented: true`, and update its note.
3. **The join carries the pose:** topo's `rides_carried` and whatever else sphere × torus needed; the section table holds it.
4. **Witnesses:**
   - lily wall 2 (`demos/tour/src/lily.rs` `wall_probes` pins `CurvedPairUnsupported { site: InteriorLoopGuard, kind: Cone, other_kind: Torus }`). It should now build, or move to a later door; re-pin with the reason, and push a `[render]` commit if frames move;
   - the meridian-triangle pose in `the-cone-has-no-implicit-enclosure-or-metres-conversion` (2026-11-20 note: the torus `R = 2, r = 1` about `y`, and the triangle revolved about the line through `(2, 0, 0)` along `z`), at the default ε, 1e-6 and 1e-12, every op in both orders.
   
   Add pose rows against a traced oracle: crossing, tangent, apex node, apex touch, clear. Each must be able to go red.

Out of scope: cone × sphere and cone × cone (`cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door`, sequenced after PR 7357), and the torus's own thread (SECTTORUS).

Territory: `pair_section/torus*.rs` and the torus thread are SECTTORUS's, and `ssi*` is SSIARITH/SSIEDGE's. Run `python3 scripts/work.py territory --base origin/main`, read SECTTORUS's live rows, and announce the seam.
