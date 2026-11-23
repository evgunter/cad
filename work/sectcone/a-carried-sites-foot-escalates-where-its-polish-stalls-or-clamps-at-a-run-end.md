---
id: a-carried-sites-foot-escalates-where-its-polish-stalls-or-clamps-at-a-run-end
kind: issue
title: A carried site's foot escalates where the polish stalls short of the foot or clamps at an interior run end, though the site is correctly placed
status: open
opened: 2026-11-23
priority: P3
---


`CarriedComponent::foot_along` (`crates/geom-brep/src/pair_section.rs`, PR 7408) certifies a site's witness `t` as its orthogonal foot. It decides `|t̂·(p − C(t))| / (1 − κr)` zero, and otherwise escalates, except at an open end the site stands past. Two correctly placed sites can escalate:

- **The polish stalls.** `Curve3::foot_within`'s Gauss–Newton (`POLISH_STEPS`) stops where its step rounds away. There the along-residual is about `d · 2e-8`, where `d = |p − C(t)|`, rather than zero. A site far off its carrier (`d` near the pad, 1e-2–1e-1 m on the measured fitted loops) at ε 1e-12 can therefore read a residual past the zero, and escalate though it is placed right. The measured fixtures read about 1e-14 at ε 1e-9.
- **The witness clamps at an interior run end.** `reads` keeps the least-gap foot over the tube pieces the site lies in. Where the site lies in one piece only but its foot lies past that piece's run (the component wanders within its pad past the run's normal plane), the witness is the run's end, and the reading escalates. The PR's row `a_sites_foot_is_its_orthogonal_foot_not_a_witness` builds exactly this case.

**Partly fixed in PR 7408's fix pass.** `CarriedComponent::polished` takes up to four Newton steps on the along-residual before certifying, kept within `pad + escalation` of the witness. That clears both cases where Newton converges inside that reach. What remains is a step rejected for leaving the reach, or a site past its carrier's curvature radius (`section_site_foot_bend`). Either still escalates a site that may be correctly placed.
