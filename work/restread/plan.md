# RESTREAD — the plan

The census's near-band reads in its crossing and edge-on-face lanes, the cross-solid curved pairs, and the P1 point and seam readings.

Opened 2026-10-09 by CONTACT's closing cut on its priority seam
(`work/README.md`, Track size), when CONTACT measured 99.5 budget
points against 30 with its P0 units landed or parked (PRs 4363, 4368,
4372; CONTACT-13 parked on the D10 hold). Nothing dispatched.

## The slate

**29.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `the-census-crossing-lane-escalates-a-line-gap-where-the-segments-lie-far-apart` | M | The census's crossing lane escalates the two edges' line-to-line gap before it reads where the lines meet, so segments 1e-2 to 1.5 apart escalate pm_census_ee_gap |
| P0 | `the-census-crossing-lane-misplaces-a-shared-points-crossing-on-a-near-collinear-pair` | M | The census's crossing lane reads a near-collinear pair's crossing through a closed form whose f64 error is ulp·|d|/θ: pairs meeting only at their shared point escalate pm_census_ee_span at ε = 1e-9 (982 escalations) and read a definite EdgeEdgeCross at ε = 1e-12 (845) |
| P0 | `the-census-edge-face-cut-escalates-a-line-gap-at-a-boundary-vertex-beyond-the-edge` | E | The census's edge-face cut and vertex-edge lanes escalate a vertex's gap off the edge's line before reading its span, so a face vertex 0.58 to 1.0 beyond the edge's end escalates pm_census_ef_cut_gap |
| P0 | `the-census-edge-face-lane-escalates-a-plane-residual-at-an-end-far-outside-the-face` | M | The census's edge-face and vertex-face lanes escalate a plane residual in band before reading the face's region, so an end 0.58 outside the face escalates pm_census_ef_residual |
| P0 | `the-census-parallel-test-reads-two-edges-through-one-point-end-to-end-as-a-near-parallel-pair` | M | The census's parallel test levers the two edges' line angle at the shorter length, so two edges continuing each other through one point, or lying 3e-8 apart, escalate pm_census_ee_parallel |
| P1 | `at-infinity-probe-measures-in-closed-form-only` | M | point-in-solid's at-infinity probe measures in closed form only, so an obliquely trimmed wall refuses VolumeUncertified |
| P1 | `census-backstop-separates-curved-pairs-only-along-world-axes` | M | The census backstop clears a curved cross-solid pair only by a gap along a world axis, so a pair's verdict depends on how the body is turned |
| P1 | `census-cross-solid-curved-pairs-undecidable-on-shell-results` | H | The census's cross-solid backstop answers CensusUndecidable for curved solids within reach of each other, so 26 multi-solid shell results fail the empty-contact tier 3' they should pass |
| P1 | `seam-description-reads-a-dihedral-at-the-seams-own-length` | M | The boolean's seam description reads a seam's dihedral at the seam's own length, so a short seam between faces that part far away is left a scaffold at rest |
| P1 | `torus-face-bounded-by-an-oblique-circle-refuses-point-classification` | H | A torus face bounded by a circle off both chart families refuses point classification (PartialTorusFace), where a sphere face so bounded is now read |
| P3 | `census-face-reach-returns-a-nan-box-for-an-unclaimable-boundary-edge` | E | census::face_reach returns a NaN-ended Some for a cylinder or cone face with an unclaimable boundary edge, against its own None contract |

## Order

The five P0 rows first. They are near-band reads JOIN and TANG found in the census's straight lanes, and they batch into two lanes: the crossing lane (`…-crossing-lane-escalates-a-line-gap-…`, `…-crossing-lane-misplaces-…`, `…-parallel-test-reads-…`) and the edge-on-face lane (`…-edge-face-cut-escalates-…`, `…-edge-face-lane-escalates-a-plane-residual-…`). CONTACT-12's lesson stands for both: a Zero that abandons a reading and a Zero that is a verdict want different bounds, so each brief asks, per verdict, which bound makes it sound.

Then `census-cross-solid-curved-pairs-undecidable-on-shell-results` (H): REACHHOLD's `boolean-door-runs-the-census-over-its-result` waits on it.

The rest in any order; `census-face-reach-returns-a-nan-box-…` rides the first census lane.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D9 and D10 as ratified; the DEV-1 set; the C6
  interference-fit era; the merge door's `RecordsASkip` doctrine (#2105).
- Shared ground: crates/topo/src/census.rs and crates/topo/src/boolean/* are shared with CONTACT's sibling successors (RESTREAD INSIDE SECTOR CONTACTHOLD) and with REACH's. Run scripts/work.py territory on your branch and announce the seam in the PR.

## Review posture

CONTACT's, inherited: the review tiers of `memories/orchestration-model.md`
(style, full single, or a dual under `docs/DUAL-REVIEW-PROTOCOL.md`,
logged in `docs/DUAL-REVIEW-LOG.md`; a class-H unit draws a concurrent
pair, a class-M unit a urandom draw), with an independent verifier on
any unit a review passes with fixes.

The lesson CONTACT's reviews taught, stated so that briefs carry it:
**nearly every defect they found was a lever or a bound that is
conservative for one verdict and unsound for another** (a Zero that
abandons a ray, against a Zero that is a verdict; an interiority Zero
metred at an ellipse's minor semi-axis, DR-111). A brief for any unit
that levers a `decide` asks, per verdict, which bound makes that
verdict sound.

A lane measures its row's repro on current main before it builds, and
reads the row against the D10 hold (`work/intent/plan.md`): a row that
turns out to need declared pairs or contact parks on its INTENT stage
row and moves to CONTACTHOLD.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
