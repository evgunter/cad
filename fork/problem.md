# Design question: what should a plane × cylinder (and plane × cone) section do at a tilt where the plane is decided off square to the axis, but the section's ellipse is decided to have equal axes?

You are a DESIGNER for the `evgunter/cad` repository, a Rust B-rep CAD kernel. Read `docs/prompts/designer.md` and follow it. It is binding, and it sets your report's shape, including the "For Ev" and "For the orchestrator" sections. Also read `CLAUDE.md` and `docs/DESIGN.md` (D1–D10, especially D1's trilean, D3 and D4's tolerance reading and recourse rules).

Work from main. You may read any code and run anything.

**What you may read:**
- `work/germ/circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band.md` on MAIN, its "What" section only.
- The measurement probe `crates/sweep/examples/circular_axes_gap_probe.rs` on branch `germ/circular-axes-band-gap`. Read and run it.

**What you may not read:**
- That item's "Fix" section, on main or on any branch.
- PR 5156's body or comments.
- Any other file on `germ/circular-axes-band-gap` beyond the probe.

They name candidate answers, and this statement deliberately gives you none.

## The problem

`geom_brep::plane_cylinder_section` (`crates/geom-brep/src/intersect.rs`) reads the plane's tilt θ off square to the cylinder's axis twice, as two different quantities, and the two readings disagree near square:
- `pc_rim_alignment` decides `r·sin θ` against the band. Within it, the table serves the cylinder's rim circle (`PlaneCylinderSection::Rim`).
- Past it, the table builds an ellipse through `geom::Curve3::ellipse`, whose `ellipse_axes_distinct` decides `a − b = r·(sec θ − 1) ≈ r·θ²/2` against the band.

So at tilts with `r·sin θ` decided past the band but `a − b` decided Zero (roughly ε/r < θ < √(2ε/r)), the table returns `Carrier(CircularAxes)`. Through `topo::splitting::split` or a Boolean, that renders as a recourse ("construct the Circle carrier …") which no caller of a split or Boolean can act on. `plane_cone_section`'s axis-normal arm (`pn_axis_normal` → ellipse) has the same shape, and so does `cylinder_cylinder_section`'s bisector ellipse.

**Measured** (the probe, at r ∈ {1e-2, 1, 1e2} and ε ∈ {1e-6, 1e-9, 1e-12}):
- Of 25 log-spaced tilts across the region, 8–17 land in this gap, on the cylinder and the cone alike.
- In the gap, the cylinder's rim circle misses the true section by up to thousands of ε.

**The question:** what should the table decide in this span, and how does that sit with D3, D4 and the public section types' documented meaning? Consider:
- what each answer serves or refuses;
- what the user's lever is;
- what downstream consumers of the section (`join.rs`, `chord_join.rs`, `offset_derive.rs`, the wall's pcurve) would have to do.

Measure before you recommend: derive and measure the miss of anything you would serve, along the whole section, and trace a few gap poses through a split and a Boolean at ε 1e-12. If a choice is a design fork, say so plainly. If it moves ratified text in DESIGN.md, a design page, or a public type's documented contract, say which.
