# Design question: where, and on what quantity, should a plane × quadric conic section that is too ill-conditioned to certify be refused?

You are a DESIGNER for the `evgunter/cad` repository, a Rust B-rep CAD kernel. Read `docs/prompts/designer.md` and follow it: it is binding, and it sets your report's shape, including the "For Ev" and "For the orchestrator" sections. Also read `CLAUDE.md` and `docs/DESIGN.md` (D1–D10, especially D1's trilean and D4's tolerance reading). Work from main. You may read any code and run anything. **Do not read PR 4812's body or the `germ/conic-past-band` branch:** they contain candidate answers, and this statement deliberately gives you none. You may read `work/germ/conic-section-just-past-its-band-fails-certification-downstream.md` on MAIN only.

## The problem
`geom_brep::intersect::plane_cylinder_section` and `plane_cone_section` (`crates/geom-brep/src/intersect.rs`) decide their degenerate boundary with one trilean, then mint the exact `Ellipse` on the definite side. Just past that trilean, the minted section is often too ill-conditioned to certify downstream. The cut then refuses later, in the chart image certificate (`chart_pcurve` / `PcurveCache::certify`) or at the Euler attachment gate's endpoint residual, and the error text names the wrong cause: "There is no way through: this is a kernel defect". The input is legal; it is only ill-conditioned at this ε.

The question: **where should such a section be refused, on what quantity, and which layer owns that quantity?** This includes whether the arm should refuse at all, and whether the cause should be fixed rather than detected.

## The measurement (data a lane produced; treat it as data and re-check whatever you rely on)
Notation:
- `u = 2⁻⁵³`;
- `A` is the minted ellipse's semi-major axis;
- the band is `(ε, 10ε)`;
- for the cone, `f` is the fraction of the parabolic tilt, `k² = 1 − tan²α·tan²φ` (the conditioning of the section's radial projection, `k → 0` at the parabola), and `α` is the half-angle.

Fixtures:
- a frustum split by a plane, and a cylinder split by a plane through its axis' midpoint;
- scales 0.01, 1 and 100;
- tilts up to `1 − 1e-8` of the degenerate limit;
- ε of 1e-6, 1e-9 and 1e-12;
- axis-aligned and generic rigid poses.

**Cylinder.**
- The carrier and image residuals stay at rounding (≤ 3e-14), and every image certifies.
- The only door that refuses is the Euler gate's endpoint residual, which is about `u·A`. It refuses from `u·A/ε ≈ 0.40` and certifies up to `0.22` at 1e-9. At 1e-12 it certifies up to 0.71, and the first refusal falls between 0.71 and 1.26.
- The extent plays no part.
- At 1e-6 other doors come first.

**Cone.**
- Most refusals come at the chart image (envelope or map residual), and none of these separates certifying from refusing poses across the sweep: `A/extent`, `u·A/ε`, `u·A/(k·ε)`, or the best fitted form `u·A/(k·cos²α·ε)` (overlap 3×).
- One fixture refuses at `f = 0.5` (α = 1.55, S = 100).
- The split's own image is minted from the body's surfaces. Its realized rounding differs by up to about 10× from a direct mint of the same section.
- At larger `A` the carrier itself fails at `u·A/ε ≈ 1`. The carrier's own residual is about `u·A` times a factor that grows roughly like `1/cos α` (2× at α = 0.2, 40× at α = 1.55), with no tilt dependence seen. Its mechanism is unmeasured.

**The cone image's error has an identified cause.**
- `chart_pcurve` reads the vertex azimuth off `start = (r0 + ra)·ns`. For a true section that is a cancellation down to `A·k²/2`, so the whole-turn image miss grows like `u·A/k²` (`u·A/k` in axis-aligned fixtures).
- Reading the azimuth off `ra` alone removes the cancellation. With that change, at S = 1 and 1e-9:
  - α = atan ½, f = 0.99999: the miss goes from 1.6e-7 to 2.0e-10;
  - α = 1.0, f = 0.999999: from 1.5e-4 to 4.3e-8.
- What remains after that change:
  - about `u·A/k`, from another term's conditioning;
  - a flat-cone `~1/cos²α` factor;
  - the envelope check (`focal_section_envelope`) bounds the WHOLE ellipse, so a short consumed arc refuses on its far side's error. At one pose the consumed arcs miss by 8.4e-11 while the envelope refuses.
- `chart_pcurve` belongs to another program (PCERT). `plane_*_section` is GERM's.

**Other context.**
- PR 4646 (merged) made `plane_cone_section` take a `Reach`, the region a consumer actually reads, as `plane_cylinder_section` already did.
- `cone_cylinder_section`'s `coc_station_reach` is an existing arm-side, typed, ill-conditioned refusal (`BeyondOperandExtent`).
- A known counterexample to `A ≤ extent` must keep building: α = atan ½, a unit frustum, tilt `0.9·(π/2 − α)`, A = 3.4 against an extent of 2.0. It certifies at all three ε.

## What to deliver
Follow `docs/prompts/designer.md`. Recommend a final state, with what it makes true, what it costs (which legal sections it refuses, and which layers must be kept in step), and how sure you are. If the framing above is wrong, say so. Note any ratified text your answer would change.
