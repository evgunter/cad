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
