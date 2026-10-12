# INSIDE — the plan

Containment and point-in-solid: the decision an escalation carries, the walls and charts the door refuses, and the P1/P2 touch reads.

Opened 2026-10-09 by CONTACT's closing cut on its priority seam
(`work/README.md`, Track size), when CONTACT measured 99.5 budget
points against 30 with its P0 units landed or parked (PRs 4363, 4368,
4372; CONTACT-13 parked on the D10 hold). Nothing dispatched.

## The slate

**30 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `revolved-tube-wall-refuses-bool-wall-trim-period` | D | point_in_solid refuses most probes of a fully revolved tube: bool_wall_trim_period escalates Invalid on a wall face |
| P1 | `two-copies-of-a-pierce-carry-edges-that-run-within-the-band` | M | A near-tangent two-run pierce leaves its copies apart on one point, and the two edges leaving them run within the band for a stretch the census passes, at 89 poses main refused |
| P2 | `a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone` | M | A solid touching itself at a point reads its touch star from its vertex there alone, so a second solid resting at that point refuses MixedTouch |
| P2 | `a-touch-at-a-saddle-corner-refuses-unanalysed` | D | A touch at a saddle corner (an L's inner corner) that no plane separates refuses TouchUnanalysed, so a block seated in that corner on the floor cannot certify |
| P2 | `census-containment-escalation-drops-its-decision` | M | topo: four doors drop or ignore the containment decision a refusal carries (census, ring re-homing, rim wedge, sphere region) |
| P2 | `point-in-solid-escalation-carries-no-decision` | H | topo: PointInSolidError::Escalated carries no decision, and its refusals end in the coincidence menu with no declaration door |
| P2 | `point-in-solid-refuses-a-ringed-cylinder-wall` | M | point_in_solid's wall outline refuses any ringed cylinder wall, so a later union member's containment probe stops on a pierced boss |
| P2 | `the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start` | M | The census's collinear edge-edge lane reads two edges' line offset at the second edge's start, so its overlap verdict depends on arena order: a body whose edges part by 21 bands reads an undeclared overlap |
| P2 | `the-census-material-probe-reads-only-vertices-so-a-flush-nested-solid-is-undecided` | M | The census material probe reads only an inner solid's vertices, so a solid nested flush on every side its vertices touch is CensusUndecidable(AllOn) where a face interior would decide it |
| P2 | `torus-split-lead-escalates-a-legitimately-small-resolvent-root` | D | line_torus_roots' rung 3 (bool_ray_torus_split_lead) escalates every ray whose odd coefficient is small but decided nonzero, not only a rounding-scale contradiction |
| P3 | `cone-chart-trim-reads-a-tilted-section-as-its-vertex-window` | M | cone_chart_trim folds the slant window over boundary vertices with no edge-class check, so a cone face bounded by a tilted planar section would be misread; no door builds one today |

## Order

The two P1 rows first.

`census-containment-escalation-drops-its-decision` and `point-in-solid-escalation-carries-no-decision` are one lane: both carry a containment decision to the door that renders it, under D4 ¶1 (i), and CONTACT-10 (PR 4363) built the types they extend (`ContainDecision`, `LoopDecision`).

The census rows (`…-collinear-lane-reads-the-offset-…`, `…-material-probe-reads-only-vertices-…`) touch RESTREAD's file; announce the seam.

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
