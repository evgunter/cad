# HONE log

## Opened at REACH's cut (2026-10-01)

Opened by the REACH orchestrator at its first sitting. REACH carried
83 budget points against 30, so it split along its priority seam
(`work/README.md`, Track size). Rows moved here by `git mv` with ids
and bodies unchanged; legacy `D` and unpriced rows were priced at the
move. No unit dispatched. — (REACH orchestrator)

- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`, the last unit of that program): the D4 ¶1 (i) recourse GRAMMAR moved in `geom-core`, so refusal text changed across the tree. `COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE` and `SPLIT_PLANE_RECOURSE` lost their unvalued `", or lower the tolerance"` tail and are now the LEVERS alone; `DEFINITE_COINCIDENCE_RECOURSE` retired into `COINCIDENCE_RECOURSE` (with the tail gone the two were one string). The valued conditional arm has one home, `geom_core::Indeterminate::ending(levers)`, composed through `MarginDiag::sized_recourse`: a site that holds an escalation gets "Recourse: {levers}, or, if this size is intended, tighten the tolerance below {m/K} m", and loses the offer exactly where the margin gives no value. `Indeterminate`'s own `Display` (and `under`) therefore renders a LABELLED recourse now, with each margin kind's first lever folded inside it, so `test_utils::refusal::recourse_markers` counts 1 where it counted 0. `MarginDiag`'s invalid rendering says "NaN or a refused enclosure", not "poisoned". Assertions written as `contains(COINCIDENCE_RECOURSE)` followed the constants; literal pins of "lower the tolerance" did not and were re-baselined. (PROPS implementer)
- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`): mechanical edits in the Boolean's files for the recourse-grammar change. `topo/src/boolean/mod.rs` and `refusal_routes.rs` use `geom_core::COINCIDENCE_RECOURSE` where they used the retired `DEFINITE_COINCIDENCE_RECOURSE` (same string). `topo/src/boolean/contain.rs`' `ContainError::RayExhausted` drops "or lower the tolerance", keeping "move the point off the boundary" — it carries no margin, so there is no value to offer and D4 ¶1 (i) offers nothing unvalued. `solid_contain.rs`' props pattern gained `..` for `PropsError::Escalated`'s new `check` field. (PROPS implementer)
## A note from CLEAVE on `split-section-spur-guard-skips-curved-spurs` (2026-10-01)

The row may be moot; its owner decides. Its one measured route to a
spur, a one-sided tangency along a straight edge rerun mirrored, is
gone: rule (b) now classifies a convex in-plane edge with its
material, so the contact mints no null edges
(`split-cannot-declare-an-exact-tangency-with-its-target`,
`splitting/rules.rs`).

The curved analogue the row names cannot arise the same way. A circle
or ellipse edge lying in the split plane has its planar partner face
in that plane too, and the split gate admits only planes and
cylinders. Rule (a) then sends both of the sector's edges with the
material, before rule (b) sees them.

What still reaches the zero-area net is a curved face's graze along a
ruling, and a ruling is straight, so the straight-tip guard covers
it. Measured on the two-arc cylinder: the seam (x = 0.5) and the wall
(y = 0.5) both refuse `DegenerateSection`, in both orientations.

Not measured: a graze whose contact meets a real section. Every
construction tried needs a cylinder ∪ brick, and the union refuses
that pair today (`CurvedSectorSideUnsupported`). — (CLEAVE,
cleave-tangency)
- 2026-10-02 — Seam note from TQUERY: PR 3768 (merged) types `SplitPlane.normal` as `geom_core::UnitVec3`. Mint one with `topo::test_support::split_plane(origin, dir, tol)` in tests, or `UnitVec3::new(v, site, band)` in code. A `SplitPlane { normal: Vec3 }` literal on an open branch stops compiling. The section join lanes carry the witness end to end, so `chord_join::SectionPlane` is gone. The boolean decides each germ plane's normal at the read (`BOOL_GERM_PLANE_NORMAL`), and a degenerate germ normal refuses `JoinDesync`. Paths touched on your ground are listed in the PR body. (TQUERY orchestrator)
- 2026-10-02 — `split-hands-out-a-body-without-running-tier-3` moved to `work/tquery/` by TQUERY: PR 3797 answers its question (split gates its sides at tier 2, typed; tier 3 decided out, with the measurement), under TQUERY's `validate-passes-a-body-with-a-zero-width-slit-face`. (TQUERY lane)
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4366, merged): `geom_brep::enters::LeverEscalation`'s `rung` and `diag` are private; read them with `rung()`/`diag()`, re-quote only through `with_diag`, which keeps the gate's verdict. `BooleanError::of_lever_rung(gate, read, rung, diag)` is the one boolean spelling. The dihedral lever-arm decision is told in one shape ("long enough, for how its faces curve, to measure their angle"), with the lever "clearly longer and no face curves tightly there"; pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` (it reads source literals: a natural "long enough … angle … face" wording elsewhere trips it). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: dispatching `encl/rim-wedge-one-walk` (P3, `rim-wedge-walks-the-second-order-stations-by-hand`). It touches `topo::boolean::rim_wedge::classify_shared_rim`'s station loop, which will read through `geom_brep::second_order_walk` with a shared material hook. No answer may move. A unit on `rim_wedge.rs` should wait for it or coordinate. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4433, `encl/rim-wedge-one-walk`, merged at `0bdb2baeae`): `geom_brep::second_order_walk` now takes its stations (`impl IntoIterator<Item = (Point3, Vec3)>`) instead of `carrier, t0, t1`. `topo::validate::MaterialStations` (`pub(crate)`, `new`, `outcome`) is the one home of the per-station material reads, shared by tier 3's check 4 and `boolean::rim_wedge::classify_shared_rim`. The rim's own station loop and its cusp-side Zero mint are gone; the Zero arm is now `MaterialStations::after_positive`. Tier 3 reads `interior_stations` once for checks 4 and 5. A source scan (`tier3_tests::check_4_and_the_rim_route_…`) reds on a second walk in either file's production code. No answer or text moved. (ENCL orchestrator)
