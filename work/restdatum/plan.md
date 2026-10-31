# RESTDATUM — the plan

RESTDATUM — checks 1 and 4: a carrier read against its datum and against its faces

Opened 2026-10-29 by RESTFRONT's split (133 budget points against 30;
`work/restfront/log.md`, 2026-10-29). Every row came from
`work/restfront/` by `git mv`, ids and bodies kept. The sibling
tracks, all on `crates/topo/src/validate.rs`: RESTFRONT (a planar
face's region), RESTCURVE (the region off the plane), RESTSHELL (solids
and shells), RESTGATE (what the at-rest gate admits on), RESTDATUM (a
carrier against its datum and its faces), RESTWORD (how validate
decides and words its refusals).

## The slate

**16 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P2 | `tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis` | M |
| P3 | `transverse-not-intrinsic-reads-declared-where-d2-says-declared-is-exempt` | M |
| P3 | `a-cusps-second-order-arm-caps-at-the-curvature-arm-where-its-walls-part-definitely` | M, design |
| P3 | `tier-3s-planar-arm-never-reads-an-edges-carrier-ends` | E |
| P3 | `check-4-gives-no-arm-verdict-to-a-mixed-or-nurbs-adjacent-edge` | M |
| P4 | `unlevered-frame-conventions-are-uncertified-at-rest` | M |

## Order

1. **The frame check's band** (P2): it refuses a valid section ellipse.
2. **Check 4's arm**: the carrier ends, the mixed and NURBS-adjacent
   edges, the cusp's second-order cap (designers first).
3. **The conventions**: `transverse-not-intrinsic-…` reads D2's text
   against the check; if the fix changes what D2 decides it is a design
   fork (designers, then `[ev]`), and if it only brings the check in
   line with D2 it lands as code. Then the unlevered frames.

## Review posture

RESTFRONT's, unchanged: the orchestrator's read for P4 and prose rows,
a single review for a new check arm, and a dual pair for a change to
what tier 3 refuses on a body a kernel door produces
(`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.
