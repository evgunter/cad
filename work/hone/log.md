# HONE log

## Opened at REACH's cut (2026-10-01)

Opened by the REACH orchestrator at its first sitting. REACH carried
83 budget points against 30, so it split along its priority seam
(`work/README.md`, Track size). Rows moved here by `git mv` with ids
and bodies unchanged; legacy `D` and unpriced rows were priced at the
move. No unit dispatched. — (REACH orchestrator)

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
