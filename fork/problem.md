# Design question: how should the plane × torus section read its stand-off and cap gaps when the coordinates cannot resolve the zero band?

You are a DESIGNER for the `evgunter/cad` repository, a Rust B-rep CAD kernel. Read `docs/prompts/designer.md` and follow it. It is binding, and it sets your report's shape, including the "For Ev" and "For the orchestrator" sections. Also read `CLAUDE.md` and `docs/DESIGN.md` (D1–D10, especially D1's trilean, D4's tolerance reading and the D10 hold).

Work from main; you may read any code and run anything.

You may read `work/germ/plane-torus-section-gaps-decide-far-coordinates-uncharged.md` on MAIN, its "What" and "Measured" sections only. **Do not read its "The question" section**, and do not read `work/germ/torus-circle-arc-classifies-far-frame-carriers-uncharged.md`'s closing notes. Both name candidate answers, and this statement deliberately gives you none.

## The problem

`geom_brep::plane_torus_section` (`crates/geom-brep/src/intersect.rs`) sorts a plane × torus pair into its section cases by deciding two gaps:
- `pt_axis_plane_gap`: `d = n·q − n·c`, the plane's stand-off from the torus's centre;
- `pt_cap_gap`: `r − |h|`, with `h` the depth of the plane's crossing of the axis.

Both are differences of coordinates. Neither is charged for the rounding of those coordinates. At coordinates of about 1e5–1e6 each term is rounded to about 1e-11–1e-10, which is past the zero band at ε = 1e-12.

**Measured** (the far-frame row `ring_path::far_frame_rows::a_torus_circle_the_section_lane_mints_far_reads_its_kind_at_the_origin` on main):
- At ε 1e-12, of 3456 planes that hold the axis at the origin (meridian sections), 984 read far as spiric ovals and 312 escalate.
- Of 576 planes tangent to the tube's top or bottom circle, 102 read far as two concentric circles about 1e-5 apart in relative terms, 72 as empty, and 94 escalate.
- Nothing differs at ε unset or at 1e-6.
- The far sections are faithful to the stored data: the stored plane really does stand off the axis, or cross into the tube, by the rounding.
- What a boolean does with each far reading has not been traced past the section.

PR 5012 (`ring_path`'s far-frame fix) charged every path reading for the coordinates' resolution: four roundings of the farthest point's distance from the origin, `resolution` in `crates/topo/src/ring_path.rs`. A reading inside it reads Zero and stands down. Read that PR's approach in the code. Note what a Zero verdict means at `pt_axis_plane_gap` and `pt_cap_gap`, compared with what it means at the ring-path readings PR 5012 charged.

**The question:** what should the section decide when the gap is within the coordinates' resolution? Whose job is it, and what does each option do to the bodies built downstream?

Measure before you recommend. Trace a few far poses from each class through a boolean, in sweep, all six ops, at ε 1e-12, and report what each class builds or refuses on main today. If a choice is a design fork, say so plainly. If it moves ratified text in DESIGN.md or a design page, say which clause.
