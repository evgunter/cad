---
id: a-carried-sites-foot-escalates-where-its-polish-stalls-or-clamps-at-a-run-end
kind: issue
title: A carried site's foot escalates where the polish stalls short of the foot or clamps at an interior run end, though the site is correctly placed
status: dispatched
opened: 2026-11-23
<<<<<<< HEAD
priority: P0
branch: sectcone/foot-bend-escalation
=======
priority: P3
>>>>>>> origin/main
---


`CarriedComponent::foot_along` (`crates/geom-brep/src/pair_section.rs`, PR 7408) certifies a site's witness `t` as its orthogonal foot. It decides `|t̂·(p − C(t))| / (1 − κr)` zero, and otherwise escalates, except at an open end the site stands past. Two correctly placed sites can escalate:

- **The polish stalls.** `Curve3::foot_within`'s Gauss–Newton (`POLISH_STEPS`) stops where its step rounds away. There the along-residual is about `d · 2e-8`, where `d = |p − C(t)|`, rather than zero. A site far off its carrier (`d` near the pad, 1e-2–1e-1 m on the measured fitted loops) at ε 1e-12 can therefore read a residual past the zero, and escalate though it is placed right. The measured fixtures read about 1e-14 at ε 1e-9.
- **The witness clamps at an interior run end.** `reads` keeps the least-gap foot over the tube pieces the site lies in. Where the site lies in one piece only but its foot lies past that piece's run (the component wanders within its pad past the run's normal plane), the witness is the run's end, and the reading escalates. The PR's row `a_sites_foot_is_its_orthogonal_foot_not_a_witness` builds exactly this case.

**Partly fixed in PR 7408's fix pass.** `CarriedComponent::polished` takes up to four Newton steps on the along-residual before certifying, kept within `pad + escalation` of the witness. That clears both cases where Newton converges inside that reach. What remains is a step rejected for leaving the reach, or a site past its carrier's curvature radius (`section_site_foot_bend`). Either still escalates a site that may be correctly placed.

## 2026-11-23: a built body it refuses (filed by EMBED; re-banded P0)

A slow-set row of EMBED's suite is red on main since PR 7408 merged (`323c16d098`), reproduced on main `74feb4851c`:

`cargo nextest run --profile ci --ignore-default-filter -p sweep --test all -E 'test(=self_intersection_proves_injectivity::a_notched_rod_holds_across_its_reflex_corners_in_every_pose)'`

fails in the fixture's boolean, before any embedding question is asked:

`the op builds: Err(Escalated { decision: Coincidence(Section, Moot), diag: Indeterminate { margin: MarginDiag(Invalid), terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("section_site_foot_bend") } })`

The body is a radius-1 rod cut by a slab (`notched` in `crates/sweep/tests/self_intersection_proves_injectivity.rs`), built in the poses the row lists: as built, turned about a generic axis, a hundred units up and across its axis, a quarter turn about `x`, at the session box's corner and turned there, mirrored, and at radii 0.02 and 30. Which pose is the first to escalate is not yet read (the panic is in the shared `built` helper and names none). The row was green on main at `1bd32ee664` (EMBED's PR 7491, merged five minutes before 7408); PR 7408's branch did not hold the row, and it is in the slow set, so neither PR's gate ran the two together.

Re-banded P0 on that evidence: a cut of a cylinder by a box that built yesterday is refused today (a normal verb on normal geometry), where the row as filed knew only of fitted loops far off their carriers. EMBED does not re-pin its row at the refusal. If the escalation is right for this body, say so here and EMBED pins the pose refusing at the build with this row named.

**Which case (20:05Z, same day):** the first, "as built": radius 1, not mirrored, no pose applied. The escalation is raised inside `notched_rod_of::<f64>(1.0, false)` while the body is being built, so every pose of the row meets it. Read by EMBED's merge lane with a temporary print in the row's loop, on the second landing's branch at main `74feb4851c`.

**A second built body (22:27Z, same day).** `sweep::all verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar` (slow set; a test of another program's, not EMBED's) fails on main `e3fdefa369` with the same refusal, from the Boolean under the ring lane at the `Interval` scalar:

`the ring lane refused: Escalated { decision: Coincidence(Section, Moot), diag: Indeterminate { margin: MarginDiag(Invalid), terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 }, predicate: Some("section_site_foot_bend") } }`

`cargo nextest run -p sweep --test all --profile ci --ignore-default-filter -E 'test(/verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar/)'`. Not checked by EMBED: whether it was green before PR 7408 (it was not among the reds EMBED's lanes listed on the landing branch before today's merges of main).

## 2026-11-24: the built bodies were a straight carrier at `Interval` (FOOTBEND)

Both built bodies refuse for one cause, which is neither of this row's two: a plane × cylinder ruling (`PlaneCylinderSection::ParallelLines`, a `Curve3::Line`) read at `Interval`. `CarriedComponent::foot_along` decided `section_site_foot_bend` on `1/κ − r`, with `κ` from `curvature_hi_over`, which is exactly `[0, 0]` on a line; `1 / [0, 0]` is no enclosure, so the margin read `Invalid` on every site of every ruling. At `f64` the same quotient is `+∞` and decides. The notched rod's slab faces `x = ±0.3` cut the rod's wall in rulings, and the row builds the rod at `f64` and its `Interval` twin (`notched_rod_of::<Interval>`), so the twin is the build that refused. The ring lane's bar faces `y = ±¼` cut the pipe in rulings too, and the lane is `Interval` throughout.

The fix raises the curvature bound to `f64::MIN_POSITIVE` before its reciprocal (still a bound on the curvature, so still sound), so a straight carrier's radius is finite and past any reach, and a curvature enclosure `[0, k]` gives a radius `[1/k, …]` instead of a refusal. A refused bound (`NaN`, `NaI`) stays refused. The fast row is `pair_section::tests::a_site_on_a_ruling_decides_its_foot_at_every_scalar`. The slow rows are `self_intersection_proves_injectivity::a_notched_rod_holds_across_its_reflex_corners_in_every_pose` and `verbs_germarms_interval::the_ring_lane_builds_at_the_certified_scalar`.

The two cases this row was filed for, a polish that stalls short of the foot and a witness clamped at an interior run end on a fitted loop far off its carrier, have a different cause, and they stay open here. No built body is known to meet them, so the P0 band, which was set on the built bodies' evidence, lapses back to this row's filing band, P3.
