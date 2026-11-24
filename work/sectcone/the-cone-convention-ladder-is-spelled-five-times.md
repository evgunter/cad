---
id: the-cone-convention-ladder-is-spelled-five-times
kind: issue
title: The cone's half-angle convention ladder (sin α, cos α levered, ConeAtTheBand / DegenerateOperand) is spelled at six sites under six prefixes
status: open
opened: 2026-11-22
---


Found in PR 7357's review. Each of these sites decides a cone's half-angle against its convention `0 < α < π/2`, the same way: `sin α`, then `cos α`, each levered across an extent; `Zero` gives `SectionError::ConeAtTheBand` and negative gives `DegenerateOperand`. Each site spells the ladder itself under its own prefix:
- `intersect.rs` `plane_cone_section` (~2374, `pn_aperture_sin` / `pn_aperture_cos`);
- `intersect.rs` `coaxial_cone_sphere` (~3195, `kos_aperture_*`);
- `intersect.rs` `coaxial_cone_torus` (~3346, `kot_aperture_*`);
- `intersect.rs` `cone_cylinder_section` (~3596, `coc_aperture_*`, beside `coc_seam_off_axis`);
- `pair_section/cones.rs` `carrier` (`kc_cone_aperture_*`, `ConeDecision::{ApertureSin, ApertureCos}`);
- `pair_section/cone_torus.rs` `aperture` (`kt_aperture_*`, `ConeTorusRefusal::Aperture`), added by PR 7395.

The prefixes are K-funnel names per site, so one home must either take the names as an argument or the sites must agree on one name pair. That choice touches k-lint baselines, which is why PR 7357 left the fifth copy beside the four, and PR 7395 the sixth beside the five, rather than moving them. What the class could not match by this grep: a cone convention ladder that is not written with the `aperture` word, for example one reading `tan α`.
