# RESTFRONT — the plan

a planar face's region at rest: ring nesting, loop simplicity, check 9's planar arms

Opened 2026-09-27 by ATREST's close
(`docs/doc-ledger/atrest-leaves-the-tracker.md`). Split on 2026-10-29
at 133 budget points against 30 (`log.md`): the rest of the slate went
to RESTCURVE (the region off the plane), RESTSHELL (solids and shells),
RESTGATE (what the at-rest gate admits on), RESTDATUM (a carrier
against its datum and its faces) and RESTWORD (how validate decides
and words its refusals).

## The slate

**19 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P1 | `check-9-does-not-check-a-ring-nested-inside-another-ring` | M |
| P1 | `tier3-never-checks-a-planar-loop-is-simple` | M |
| P1 | `check-9-lines-meet-falls-through-to-a-definite-meet-off-both-spans` | E |
| P2 | `validators-accept-a-lamina-slit-or-zero-area-membrane-face` | M |
| P2 | `check-6-planar-arm-skips-ellipse-and-nurbs-loops` (spiric and NURBS) | M |
| P3 | `a-corner-is-a-slice-has-three-homes` | M |
| P3 | `check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm` | E |
| P3 | `check-9-reads-an-internal-tangencys-trims-at-the-radical-foot` | E |
| P4 | `validate-rs-reads-the-doubled-cusp-as-the-kissing-union-two-slit-edges` | E |

## Order

1. **The wrong answers in check 9's line arm** (`lines_meet`'s
   fall-through Meet, then its order), with the radical-foot trims: all
   three are E rows on one function and ride one unit.
2. **Ring-in-ring nesting, then loop simplicity, then the slit and the
   membrane.** All three are "a planar region that is not what its
   loops say"; ring-in-ring uses the carrier walk ATREST-9 and ATREST-12
   built unchanged, and simplicity reads edge interiors against each
   other, which the slit row needs too. Read the three together before
   speccing, and check JOIN's corner arm (`pinch_corner_errors`), which
   closed the repeated-vertex half of simplicity.
3. **Check 6's remaining carriers** (spiric, NURBS), following the shared
   winding in `loop_winding.rs`, not a second winding here.
4. **The corner rule's three homes**, and the doubled-cusp prose.

## Review posture

The orchestrator's read for P4 and prose rows, a single review for a
new check arm, and a dual pair for a change to what tier 3 refuses on a
body a kernel door produces (`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.
