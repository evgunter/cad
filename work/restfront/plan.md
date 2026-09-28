# RESTFRONT — the plan

the at-rest validator's not-yet-checked list, and what its checks cost

Opened 2026-09-27 by ATREST's close
(`docs/doc-ledger/atrest-leaves-the-tracker.md`). Every row came from
`work/atrest/` by `git mv`, ids kept.

## The slate

**29 budget points** against a ceiling of 30. One row is parked on a
trigger outside this program and does not count.

| pri | item | cost | status |
|---|---|---|---|
| P0 | `validity-refuses-an-interior-chart-singularity` | H | **parked** on `work/exch/import-normalizes-the-rim-only-cap` |
| P1 | `check-9-does-not-check-a-ring-nested-inside-another-ring` | D | open |
| P1 | `validate-tier3-curved-boundary-containment` | H | open |
| P2 | `check-6-planar-arm-skips-ellipse-and-nurbs-loops` (spiric and NURBS remain) | D | open |
| P3 | `check-10-is-silent-where-point-in-solid-refuses` | D | open |
| P3 | `check-10-is-quadratic-in-shells-per-solid` | M | open |
| P3 | `check-7-stops-body-wide-so-one-solids-defect-hides-anothers-orientation` | M | open |
| P3 | `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges` | H | open |
| P4 | `unlevered-frame-conventions-are-uncertified-at-rest` | D | open |
| P4 | `an-ellipse-stored-minor-over-major-passes-tier-3` | E | open |
| — | `transverse-not-intrinsic-reads-declared-where-d2-says-declared-is-exempt` | (unpriced, 2.5) | open |

## Order

1. **Ring-in-ring nesting (check 9), then curved-boundary containment.**
   Both are "a ring outside where it should be" on ground the nesting
   arm does not reach yet; the first uses the carrier walk ATREST-9 and
   ATREST-12 built unchanged, and the second needs a walk on a curved
   chart. The curved row is the last unmarked deferral in the list.
2. **The silences that hide a refusal**: check 10 where point-in-solid
   refuses, check 7's body-wide stop, check 9's meeting arms off a plane.
   Each is a place a defect passes because another check could not
   answer; each row says whether the fix is to answer or to name the
   silence in the verdict.
3. **Check 6's remaining carriers** (spiric, NURBS) — follows the shared
   winding in `loop_winding.rs`, not a second winding here.
4. **Cost**: check 10's n(n−1) probes, once a real body makes it matter.
5. **The P4 conventions** (unit frames, ellipse ordering, the D2
   declared/derived reading). `transverse-not-intrinsic-…` reads D2's
   text against the check; if the fix changes what D2 decides it is a
   design fork (designers first, then `[ev]`), and if it only brings the
   check in line with D2 it lands as a code change.

## Review posture

As ATREST's: the orchestrator's read for P4 and prose rows, a single
review for a new check arm, and a dual pair for a change to what tier 3
refuses on a body a kernel door produces
(`memories/orchestration-model.md`). Gate on local CI while hosted
queues, `[skip ci]` on verified heads, batch units into one PR where
convenient (Ev, 2026-09-26).

No exit criteria are set: the program closes when its slate is empty.
