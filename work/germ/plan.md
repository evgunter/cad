# GERM — the plan

The cone and torus operand lanes, and the containment arms that stop short of them.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched yet.

## The slate

**25 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `VERBS-CONE` | H | cone and torus operand lanes |
| P0 | `c5-gate-admits-every-pose-of-an-implemented-pair` | D | The C5 gate reads PairRoute::implemented per KIND pair - a pose the arm would refuse passes the gate; sibling arms divide unguarded |
| P0 | `curved-face-containment-lacks-cone-torus` | H | curved_face_containment has no cone or torus arm while point_in_solid now answers both kinds |
| P0 | `ray-torus-root-search-finds-a-counterexample-at-eps-1e-12` | H | the ray-torus root search disagrees with its geometric oracle on a rare pose at eps = 1e-12 |
| P0 | `torus-coincident-pair-cannot-reach-the-covered-rung` | H | A coincident torus pair cannot reach the declared-cover rung: the sampled enclosure's chord-dip charge outruns every band |
| P0 | `torus-operand-gate-admission` | D | Torus onto boolean_arm_exists - the stem glue's door sequence after the box and the residual arm (a gate-policy unit, CURVED's) |

## Order

`ray-torus-root-search-finds-a-counterexample-at-eps-1e-12` first: it is
the only row here with a recorded counterexample, it is a wrong answer
rather than a refusal, and whatever it turns up about the torus root
search bears on `torus-coincident-pair-cannot-reach-the-covered-rung`
and `torus-operand-gate-admission` behind it.

Then `curved-face-containment-lacks-cone-torus`, which is the row that
makes the two containment doors agree, and `VERBS-CONE` behind it.
`c5-gate-admits-every-pose-of-an-implemented-pair` is structural and
can be specified in parallel with any of them.

## Review posture

OPEN, for this program's first dispatch. REACH inherited protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. This program's units are candidates where
the failure mode is a confident WRONG answer rather than a refusal —
the first orchestrator names which of its rows those are.
