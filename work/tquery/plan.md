# TQUERY — the plan

the split and query doors that refuse geometry the kernel can build

Opened 2026-09-20 by TOPO's priority-seam cut (`work/README.md`, Track
size). Nothing dispatched yet.

## The slate

Carrying **20.5 budget points** of dispatchable work against a ceiling of
30 — about one sitting, which is what the cut was for.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `rim-of-refuses-extruded-multi-arc-rims` | H | topo::query::rim_of refuses every multi-arc rim extrude mints: the arcs' carrier circles are not bit-identical |
| P0 | `split-refuses-cylindrical-feature-box` | H | topo::split refuses a box with one cylindrical feature in both orientations — and the second refusal reports CircularAxes where the closed form gives a-b = 0.049 m |
| P1 | `curve-kind-placement-disagrees-with-the-ratified-seat-clause` | D | The ratified VERB-SEAT S1 puts CurveKind beside Curve3; the code has kept it in topo::query since SEAT-2 |
| P1 | `split-edge-cannot-carry-a-fitted-or-general-pcurve-row` | H | split_edge carries only the Decide-door pcurve lanes; a Fitted/General row is left as found because its certification doors carry the PcurveFittedLane bound |
| P3 | `rim-of-flattens-a-dangling-curve-key` | E | rim_of answers NotAnArc{kind:None} for a dangling curve key, the arm whose doc says null scaffold |
| P3 | `split-edges-key-retention-direction-is-pinned-by-no-row` | E | topo: split_edge's key-retention DIRECTION is pinned by no row |
| P3 | `split-plane-normal-and-slab-axis-carry-unitness-as-prose` | E | SplitPlane.normal and slab_extent's axis carry a unit precondition as prose — the class the geom-core witness now types at function boundaries |

## Order

`split-refuses-cylindrical-feature-box` first: it is the row a user meets, it has a closed form on the record (a - b = 0.049 m against a `CircularAxes` refusal), and the diagnosis is likely to explain `rim-of-refuses-extruded-multi-arc-rims` too, whose cause — carrier circles that are not bit-identical — is the same class. The four small rows behind them are class `E` and ride whichever unit opens their file.

## Review posture

OPEN, for this program's first dispatch. TOPO ran the full v6 dual on
kernel units; Ev took S-TCOST off the protocol entirely on 2026-09-12
and protocol v7 (`docs/MODEL-AB-LOG.md`) runs the dual on triaged-in
units only. Nobody has re-asked the question for this ground, so the
first orchestrator answers it here rather than inheriting an answer.
