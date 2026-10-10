# SECTOR — the plan

CONTACT's P3 tail: sector-arm levers, self-touches and wedge joins, torn records, and the gate readers that drop a verdict.

Opened 2026-10-09 by CONTACT's closing cut on its priority seam
(`work/README.md`, Track size), when CONTACT measured 99.5 budget
points against 30 with its P0 units landed or parked (PRs 4363, 4368,
4372; CONTACT-13 parked on the D10 hold). Nothing dispatched.

## The slate

**28.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-cube-sunk-flush-into-a-block-refuses-ray-exhausted-on-every-order` | E | A cutter fully sunk inside a block with flush walls refuses Boolean(Containment(RayExhausted)) on every union order (docm8 split-fixture variant) |
| P3 | `a-curved-merge-group-with-a-dangling-seam-refuses-as-period-closure` | E | A curved merge group whose seam dangles without closing the period still refuses PeriodClosure, which names the wrong cause |
| P3 | `a-same-solid-self-overlap-seen-only-as-touches-is-not-a-self-crossing` | M | A solid whose own shells overlap while meeting only in touches is not caught by arm 2's self-crossing exception, so its pairs are read against an ill-formed material |
| P3 | `a-thin-fin-voids-boolean-refuses-tier-3-prime-with-an-edge-face-pierce` | M | A boolean against a void whose apex is a thin fin refuses tier 3′ with an undeclared edge-face pierce on the fin, on main too |
| P3 | `a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide` | M | A vertex pair whose faces agree at the shorter arm refuses as a coincidence where the long edges' far vertices decide the pose |
| P3 | `a-wedge-edge-on-a-block-top-from-its-corner-refuses-at-the-join` | M | A tilted wedge whose short edge lies on a block's top from the block's corner refuses at the join, with every reading definite |
| P3 | `boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm` | M | The boolean's bound-parallelism verdicts (parallel_same at the shorter arm, parallel_same_dir at 1 m) read a Zero as 'same ray' for long bounds that part by many bands |
| P3 | `census-arena-walks-read-a-torn-record-as-absent` | M | The census's own arena walks read a torn record as absent |
| P3 | `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid` | M | topo: census's dihedral read drops the arm's rung and verdict, and contact_verify mints its gate refusals as Invalid |
| P3 | `coplanar-lump-carrier-verdict-is-levered-at-the-sector-arm` | M | The coplanar lump's carrier verdict (vtxfac carrier_eq, recl require_same) is levered at the sector's shorter arm, so a face whose far vertices stand 2500 bands off is called one carrier |
| P3 | `degenerate-torus-operand-meets-the-declare-menu-and-a-false-solid-is-fine` | M | contact: a torus operand outside the ring convention reaches the Boolean's front door as CurvedPierceUnsupported's declare menu or Containment's 'the solid itself is fine' |
| P3 | `editor-core-never-reads-merge-skipped` | E | No code outside topo reads BooleanNaming::merge_skipped, so a document union whose merge stage skipped a group publishes the body with no trace |
| P3 | `pierce-germ-direction-within-is-levered-at-the-sector-arm` | E | The pierce germ direction's within test is levered at the sector's shorter arm, so a transition sector the germ-line gate reads at its reach refuses in band |
| P3 | `torus-walk-steps-over-null-edges-where-the-cone-refuses` | E | The torus chart walk steps over an uncertified boundary edge and guards the gap, while the cone walk refuses the same shape as CorruptFace |
| P3 | `unclaimed-half-edge-read-as-a-minus-half-in-contact` | E | a half-edge its own edge does not claim is read as that edge's minus half: 1 site(s) on this ground |

## Order

The three sector-arm lever rows (`pierce-germ-direction-within-…`, `coplanar-lump-carrier-verdict-…`, `boolean-bound-parallelism-verdicts-…`) are one lane: one arm length, three doors.

`contact-gate-readers-drop-the-arm-verdict-or-mint-invalid` names two doors. Its `contact_verify` half verifies declared contacts, which the D10 hold covers (CONTACTHOLD parks five `contact_verify` rows on `booleans-glue-on-zero`), so a lane takes the census half and parks the rest.

The rest batch by file.

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
