---
id: step-import-circle-promotion-has-no-map-obligation
kind: issue
title: the circle limb certifies locus and closure, not the map: a re-timed closed carrier promotes on locus alone
status: open
opened: 2026-09-04
priority: P1
cost: D
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
