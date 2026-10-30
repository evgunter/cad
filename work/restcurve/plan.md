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

**30 budget points** against 30 (2026-10-30).

| pri | item | cost |
|---|---|---|
| P1 | `validate-tier3-curved-boundary-containment` | H |
| P3 | `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges` | H |
| P1 | `loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` | H, design |
| P1 (parked) | `tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity` | M |
| P3 | `a-sampled-edge-the-chart-singularity-reading-cannot-settle-is-exempt` | M |
| P3 | `a-loop-the-chart-singularity-walk-cannot-certify-is-exempt` | M |

## Order

1. **The chart singularity (P0) — closed by PR 6054.** The wrap-tip half (`tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity`) stays parked on the shared loop walk.
   Was: Its trigger (FOREIGN's import door)
   has fired; the row is re-aimed on the region reading and grown by
   `[ev]` PR 6014 (the wrap tip). Read the row's last two sections
   before the spec.
2. **One reading of a curved face's region: the shared loop walk and curved-boundary containment together**, weighed by one designer pair (they are one question), then built. The wrap tip follows.
   Was: **Curved-boundary containment**, the last unmarked deferral: it
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
