---
id: step-import-circle-promotion-has-no-map-obligation
kind: issue
title: the circle limb certifies locus and closure, not the map: a re-timed closed carrier promotes on locus alone
status: closed
opened: 2026-09-04
priority: P1
cost: H
closed: 2026-11-20
branch: exch/arc-map-obligation
---


`try_circle` (`crates/step-import/src/recognize_curve.rs`) certifies
the locus (INV-C1 + INV-C2) and full-period closure, but not the MAP:
a closed carrier re-timed around the circle — locus exact, angular
parameterization not affine in the carrier's parameter — promotes on
locus alone. The line limb gained exactly this obligation as INV-C5
in EXCH-H1 (the Greville linear-precision hull; a rational carrier
refuses), after a re-timed straight seam stranded at adoption; the
circle limb is the named sibling of that sweep. Measured TODAY the
fail mode is refusal-safe, not wrong-body: a promoted circle's chart
images go through the arc-rim construction, which certifies against
the chart and refuses loudly on a re-timed carrier. The obligation
still belongs in the certificate, not the consumer. This is the arc
lane of `step-import-curve-recognition-named-exclusions` (unit 2's
ground — open arcs need the same statement); a Greville-style hull
does not transpose directly to the rational-quadratic circle form, so
the certificate needs its own derivation there.

Found by the EXCH-H1 fix pass (PR #1798), from the adjudicated review
union.

## Closed

Not owed (EXCH unit B, `exch/arc-map-obligation`; the decision both
designers reached, logged on the EXCH orchestrator branch). A printed
carrier is evidence for its edge's description, not a cache
(`docs/DESIGN.md` `[import]`, step 2: the body is rebuilt from the
description), and nothing reads its parameter once it is promoted. So
a promotion owes its LOCUS (within ε_in of the analytic curve), its
COVERAGE (exactly the arc, segment or full turn between its ends) and
its ORIENTATION (the curve's sense), and not its timing. The re-timed
closed carrier this row describes is a correct promotion.

The map obligation is real where the kernel reads a spline it HOLDS
as an analytic map paired with a chart image: a wall's column read as
the straight line an affine iso image parameterizes. It lives there
now — `recognize_curve::straight_map` (INV-C5), read by the iso
rung's line arm (`build::column_match`), the section rung's straight
read and `knot_split`'s straight iso — and the iso rung carries an
edge on the column itself wherever the column does not read as that
map. Promotion itself (`try_line`, `try_circle`) certifies locus and
coverage only. The "refusal-safe at the arc rim" observation stands:
an arc rim on a spline wall certifies its own map against the chart.
