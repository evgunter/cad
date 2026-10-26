# GERM — the plan

The cone and torus operand lanes, and the containment arms that stop short of them.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched yet.

## The slate

**11 budget points** of dispatchable work against a ceiling of 30 (at the 2026-10-20 hand-back). The charter's six P0 rows are closed; the P3/P4 tail moved to GERMCERT and GERMTAIL.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-sliver-crossings-long-bisector-ellipse-is-served-past-representability` | M | A sliver crossing of equal cylinders serves the long bisector ellipse past what f64 evaluates within the band |
| P2 | `an-oblique-rod-clear-of-a-cone-refuses-the-extent-scan-at-1e-12` | E | an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms is red at CAD_TOLERANCE_EPS=1e-12 and 1e-6 on main (a rod clear: FallbackExtentUnsupported, near-tangent carriers) |
| P2 | `conic-section-just-past-its-band-fails-certification-downstream` | M | plane x cylinder and plane x cone mint a section whose construction error outgrows the band just past their degenerate trilean, and it refuses at certification instead of at the arm |

## Order

`an-oblique-rod-clear-of-a-cone-refuses-the-extent-scan-at-1e-12` first:
it is cheap, and its repro is one row. Next is
`conic-section-just-past-its-band-fails-certification-downstream`, a
design read on how far past the band a conic section may be served.
`a-sliver-crossings-long-bisector-ellipse-is-served-past-representability`
is the same question on the bisector; take the two together.
`c5-plane-torus-cone-cylinder-arms` moved to SECT on 2026-10-25.

## Review posture

OPEN, for this program's first dispatch. REACH inherited protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. This program's units are candidates where
the failure mode is a confident WRONG answer rather than a refusal —
the first orchestrator names which of its rows those are.
