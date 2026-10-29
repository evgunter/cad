# RESTCURVE — the plan

RESTCURVE — the face region off the plane: chart singularities, curved containment, curved meeting arms

Opened 2026-10-29 by RESTFRONT's split (133 budget points against 30;
`work/restfront/log.md`, 2026-10-29). Every row came from
`work/restfront/` by `git mv`, ids and bodies kept. The sibling
tracks, all on `crates/topo/src/validate.rs`: RESTFRONT (a planar
face's region), RESTCURVE (the region off the plane), RESTSHELL (solids
and shells), RESTGATE (what the at-rest gate admits on), RESTDATUM (a
carrier against its datum and its faces), RESTWORD (how validate
decides and words its refusals).

## The slate

**28 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P0 | `validity-refuses-an-interior-chart-singularity` | H |
| P1 | `validate-tier3-curved-boundary-containment` | H |
| P2 | `ring-and-scaffold-refusals-read-three-ways` | E |
| P3 | `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges` | H |
| P3 | `the-corner-slice-arm-is-silent-on-cones-nurbs-faces-and-first-order-ties` | M |

## Order

1. **The chart singularity (P0).** Its trigger (FOREIGN's import door)
   has fired; the row is re-aimed on the region reading and grown by
   `[ev]` PR 6014 (the wrap tip). Read the row's last two sections
   before the spec.
2. **Curved-boundary containment**, the last unmarked deferral: it
   needs a walk on a curved chart, which the P0's region reading may
   share.
3. **Check 9's arms off the plane** (meeting, then corner), and the
   ring-on-a-curved-face wording, which the containment walk decides.

## Review posture

RESTFRONT's, unchanged: the orchestrator's read for P4 and prose rows,
a single review for a new check arm, and a dual pair for a change to
what tier 3 refuses on a body a kernel door produces
(`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.
