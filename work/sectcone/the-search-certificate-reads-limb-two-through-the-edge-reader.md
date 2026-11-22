---
id: the-search-certificate-reads-limb-two-through-the-edge-reader
kind: unit
title: The search certificate reads limbs 1-2 through the edge certificate's one limb-2 reader, so the cone converts to metres there and ToMeters retires
status: closed
opened: 2026-11-21
priority: P0
cost: H
parent: the-cone-has-no-implicit-enclosure-or-metres-conversion
branch: sectcone/one-limb-two-reader
closed: 2026-11-22
pr: 7350
---


Unit 1 of the P0's weighed design (`the-cone-has-no-implicit-enclosure-or-metres-conversion`, `## Weighed`). It does not wait on `[ev]` PR 7326, which asks what the cone stores.

**Review tier: dual** (two Opus reviewers; Ev 2026-11-21). It is an architectural merge of two certificate pipelines into one, with soundness at stake on every analytic kind. Difficulty letter: H.

## Spec

C2 (`crates/geom-brep/README.md`) says "the proof is the same at every door, a search's and an edge's at rest". Today the search's limbs 1–2 break that. `ssi::certify::analytic_limbs` runs `composite_form` → `geom_core::spline::compose::ImplicitSurface` (an `f64` form) → `ToMeters` (`Divisor`, `Torus`, `torus_meters_sup` / `torus_meters_floor`). It never learned the cone, and it bounds a linearized residual (`F/2R`) where the edge reader bounds distance. The edge certificate's reader (`pcurve_cache::limb2_offset_sup`, `pcurve_cache/projected.rs` `Meters`, `compose::CanonicalSurface` with enclosure-valued coefficients) already converts every analytic kind, the cone included (the lever `ρ cos α + (σz)_min sin α`, with `√|f|` where a part reaches the apex).

1. **The search reads limb 2 through that one reader.** The search's refinement (`ssi/refine.rs` `halve_spans`, `Refused.spans`) needs to know which parts did not clear, so the shared reader reports its uncleared parts. `OffsetRead` carries only bounds today. Reconcile the two halving schemes (the search's `subdivide`, the edge reader's `offset_rounds`) into one, or say why both must stay.
2. **The search's double-cone reading.** A pair's section is a section of carriers (C5: no face extent enters it), so the search reads the DOUBLE cone, where the edge reader reads one nappe. Add a nappe-free `Meters::Cone` reading (`cone_elevation` already takes `Option<Nappe>`).
3. **Retire the second pipeline:** `ToMeters`, `composite_form`, the search-side torus conversion, and `ImplicitSurface` if it then has no production caller. Limb 1 at the samples is implied by limb 2 (the edge door says so), so drop it if it becomes redundant, or say why it stays.
4. **Fix a drive-by defect of the same class:**
   - **The defect:** `pcurve_cache.rs` `twin_span_hulls` and `projected::Meters::of` (or wherever they now live) take `half_angle.sin_cos()` at the lane scalar and then cross it with `from_certified`. At `T = f64` that treats libm's rounded value as an exact point.
   - **The sound spelling:** `curve_face/composite.rs` `framed_form` crosses first and then takes `sin_cos` at `Interval`.
   - **The fix:** make every cone reader you touch use the sound spelling, and sweep for the shape.
5. **Ratified text:**
   - C2's "the cone has no such conversion at the search, which is why its rung-3 arms are unretired" and the search-side torus sentence are re-worded to the one reader.
   - DESIGN `[scalars]`'s "the analytic composite's implicit form is `f64`-only and untrusted … a WIDENED analytic operand refuses typed" becomes true of the new state, roughly: "the composite's coefficients are enclosures built at the lane scalar". It is agent-written: check `git log -S` and say in the PR what you found.
6. **Rows:** expect re-baselined margins, since distance bounds sit slightly above linearized residuals, rather than new refusals. Name every moved number. The rows this closes are `work/onehome/composite-form-takes-enclosure-valued-coefficients.md`, and possibly `work/select/the-ring-composite-refuses-an-interval-fitted-plane-from-exact-inputs.md`. Close them in this PR if they are closed: their programs' files, by moving them per `work/README.md`, or by setting their status with a `## Closed` note.

Out of scope: the tube's cone arm (sibling unit `the-tube-enclosure-has-a-cone-arm`, concurrent, in `ssi/enclose.rs`), the pair doors, `route`, and what the cone stores.

Territory: `ssi/*` is SSIARITH/SSIEDGE ground, and `pcurve_cache*` is PCERT's. Run `python3 scripts/work.py territory --base origin/main`, check those programs' live rows, and announce the seam in the PR.

## Closed (2026-11-22, PR 7350)

The search reads limbs 1–2 through the edge certificate's one reader (`pcurve_cache::limb2_offset_sup`). It reads the carrier's locus (`projected::Locus::Carrier`, the double cone on a cone) where the edge reads a face's nappe, and refinement reads the reader's uncleared parts. `ToMeters`, `composite_form` and the search-side torus conversion retired. The cone's angle crosses into `Interval` before sine and cosine through the one helper `implicit::cone_sin_cos`. The rest of this PR:

- **Dual review:** two Opus reviewers, both APPROVE-WITH-FIXES with no MAJOR. One demonstrated nightly red (the geom-core census pins) was fixed in the fix pass.
- **Rows filed:** `ssiarith/limb-two-has-two-decision-layers-over-one-reader`, and `nurbs/implicit-surfaces-cone-and-torus-arms-have-no-production-caller`.
- **Rows closed:** `onehome/composite-form-takes-enclosure-valued-coefficients` and `select/the-ring-composite-refuses-an-interval-fitted-plane-from-exact-inputs`.
