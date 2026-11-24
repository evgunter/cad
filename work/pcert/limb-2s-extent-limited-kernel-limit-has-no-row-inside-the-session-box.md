---
id: limb-2s-extent-limited-kernel-limit-has-no-row-inside-the-session-box
kind: issue
title: Limb 2's extent-limited kernel limit has no row inside the session box
status: open
opened: 2026-11-21
priority: P3
cost: E
---

Filed by ONEROUND's `oneround/box-certify-ask` (certification asks the session box of an edge before any check).

**What changed.** Two rows of `crates/geom-brep/tests/closed_form_limb2.rs` pinned limb 2's kernel limit (`AnalyticRung3Refusal::KernelLimit`) at its extent on 12 km edges: `a_twelve_kilometre_line_between_planes_refuses_as_the_kernel_limit` (an end enclosure `[0, 9.1e-12]` reaching zero) and `a_twelve_kilometre_edge_refuses_as_the_kernel_limit_at_its_extent` (its bound within `KERNEL_LIMIT_ROUNDING_FACTOR` of the net's width, its ending naming shorter edges). Both edges reach 8 km past the 1 km box, so certification now refuses them by the box first; the rows are renamed `..._is_refused_by_the_session_box` and pin that.

**What is left unpinned.** The extent-limited arm of `KernelLimit` (the bound is the net's own rounding at the edge's extent, the "shorter edges" ending) is pinned by no row that reaches it through a door. An edge inside the box is at most about 3.5 km long (the cube's diagonal); whether limb 2's rounding at ε = 1e-12 still passes the band at that length is not measured. Wanted: an in-box witness for the extent arm (a ~3 km diagonal edge at 1e-12, say), or a ruling that the arm is now reached only past the box, in which case its ending and its rows say so. `a_thin_torus_circle_refuses_as_the_kernel_limit_of_its_composite` still pins the composite arm.


