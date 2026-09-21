# NURBS — the plan

the spline doors: knot insertion, span metring, the loose net and what restrict composes

Opened 2026-09-20 by PROPS's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**19.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one` | E | nurbs_span_meter's d1 - d0 goes negative on a reversed domain, now indistinguishable from a collapsed span |
| P0 | `refine-dir-hairline-knot-insertion` | H | refine_dir's exact-equality insertion guard leaves knot pairs one ulp apart - de Boor divides by the hairline |
| P1 | `coefficient-vector-pairing-survivors` | D | The loose (knot vector, coefficient array) shape survives outside hull: evaluators, tensor grids, composition, a public green-integral door |
| P1 | `knot-mirror-symmetry-belongs-on-knotvector` | E | mirror_symmetric and KnotMirrorError sit on NurbsSurface, where a curve reversal cannot reach them |
| P1 | `mapped-curve-restrict-composes-placements-per-split` | H | MappedCurve::restrict composes the anchored rotation into the stored placement per split, re-applying rotation_about's diagonal enclosure each time — compose in the parameter, keep one placement |
| P1 | `parametric-polygon-loop-certifies-nothing` | H | A profile loop with a document parameter in a polygon vertex certifies nothing at any box width |

## Order

`refine-dir-hairline-knot-insertion` first: an exact-equality guard
that leaves knot pairs one ulp apart means de Boor divides by the
hairline, which is a live numeric failure rather than a looseness.
`nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one`
is class `E` and can ride with it.

Then `mapped-curve-restrict-composes-placements-per-split`, whose fix
(compose in the parameter, keep one placement) is stated in the row —
it is the widening that compounds, so it pays back per split.
`coefficient-vector-pairing-survivors` is the structural row and the
largest.

## Review posture

OPEN, for this program's first dispatch. PROPS inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
