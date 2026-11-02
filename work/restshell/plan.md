# RESTSHELL — the plan

RESTSHELL — solids and shells at rest: checks 7 and 10, overlapping material, orientation

Opened 2026-10-29 by RESTFRONT's split (133 budget points against 30;
`work/restfront/log.md`, 2026-10-29). Every row came from
`work/restfront/` by `git mv`, ids and bodies kept. The sibling
tracks, all on `crates/topo/src/validate.rs`: RESTFRONT (a planar
face's region), RESTCURVE (the region off the plane), RESTSHELL (solids
and shells), RESTGATE (what the at-rest gate admits on), RESTDATUM (a
carrier against its datum and its faces), RESTWORD (how validate
decides and words its refusals).

## The slate

**25 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P1 | `an-inside-out-slab-thin-enough-for-its-volume-to-be-in-band-finishes-at-rest` | M, design |
| P1 | `tier-3-admits-two-solids-of-one-body-whose-material-overlaps` | M |
| P2 | `check-ten-falls-silent-on-a-shell-whose-every-vertex-touches-another` | M |
| P3 | `check-10-is-silent-where-point-in-solid-refuses` | M |
| P3 | `check-10-is-quadratic-in-shells-per-solid` | M |
| P3 | `check-7-stops-body-wide-so-one-solids-defect-hides-anothers-orientation` | M |
| P3 | `tier-three-accepts-two-coincident-duplicate-shells-in-one-solid` | M |
| P3 | `shell-nesting-witness-refuses-on-a-point-scoped-reading` | E |
| P3 | `same-surface-faces-sharing-an-edge-need-not-carry-one-sense-bit` | M |

## Order

1. **The two P1s**: the thin inside-out slab (a design question first:
   which reading decides the side) and two solids' overlapping
   material (check against ATREST's census, which already refuses it
   at tier 3', before building a second reader).
2. **Check 10's silences**, then check 7's body-wide stop, the
   duplicate shells and the sense bit.
3. **Cost**: check 10's n(n−1) probes.

## Review posture

RESTFRONT's, unchanged: the orchestrator's read for P4 and prose rows,
a single review for a new check arm, and a dual pair for a change to
what tier 3 refuses on a body a kernel door produces
(`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.
