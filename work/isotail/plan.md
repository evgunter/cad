# ISOTAIL — the plan

the trimmed and rational lane: loft rims and drives that refuse, the plane × NURBS refusals' wording, at-rest certificates and untested mints

Opened 2026-10-23 by ISO's priority-seam cut (`work/README.md`, Track
size). ISO closed every P0 and P1 row it kept (PRs 5629–5635, 5638,
5642, 5646) and was merged back into this track by the 2026-10-26
tracker sweep, so the trimmed and rational lane is one track again.
Nothing dispatched.

## The slate

**33 budget points** of dispatchable work against a ceiling of 30. It
is over by three because ISO's four rows share this track's one file
and its lane; the next sitting splits along the P2/P3 seam if it grows.

| pri | item | cost |
|---|---|---|
| P2 | `a-loft-drive-at-interval-refuses-every-leaf-since-the-one-polynomial-weight-test` | M, design |
| P2 | `a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim` | M |
| P2 | `a-moved-lofts-top-rim-refuses-its-at-rest-certificate` | M |
| P2 | `the-spline-lanes-lever-an-edges-transversality-at-the-edge-not-its-faces` | M |
| P3 | `plane-nurbs-refusal-arms-with-no-ending` | M |
| P3 | `plane-nurbs-not-transverse-says-coincident-on-a-zero-arm` | E |
| P3 | `plane-nurbs-limb-one-divergence-reports-a-sample-it-did-not-visit` | M, design |
| P3 | `plane-nurbs-limbs-reads-a-non-finite-plane-into-its-margins` | M |
| P3 | `a-general-image-ending-on-a-closed-charts-seam-takes-its-foot-on-the-first-branch` | M |
| P3 | `the-m8-4-mint-with-a-non-plane-pair-is-untested` | M |
| P3 | `non-separable-rational-interior-column` | M |
| P3 | `trimmed-lane-inner-rule-has-no-composite-arm` | M |
| P3 | `curved-trim-e2e-fixture-waits-for-a-producer` | D (legacy; re-price on touch) |
| P3 | `S351` | E |
| P4 | `the-tour-carries-two-patch-quadratures` | E |

## Order

The loft drive row first: it is a regression PR 5635 made (bisected in
its body), it costs the nightly about 16 minutes per row, and its fork
— whether a drive's weights reach the constancy test as plain
constants, or the rational lane certifies what the polynomial one did —
is weighed before a lane builds it. Then the arc-bulge loft, a loft the
GUI can author that refuses; the arc-rim arm's measured step (PR 5630)
is where it now stops, and its next measurement is `pick_arc_rim`'s own
reading at `HalfEdgeKey(3v1)`. The two at-rest P2 rows follow.

The four `plane-nurbs-*` refusal rows all edit `PlaneNurbsRefusal` in
`edge_nurbs.rs` and its `refusal` map, so they are one unit, or a short
chain on one branch, not four parallel lanes. The inner-rule row is
the composite outer rule's sibling one level in (PR 5634), on QUAD's
`props/quad/` by announced seam. `curved-trim-e2e-fixture-waits-for-a-producer`
waits on geometry nothing mints yet; read it before planning around
it.

## Review posture

Per unit at dispatch (`memories/orchestration-model.md`, review
tiers). ISO's lesson stands: a change to what a producer EMITS owes a
run of the slow set and of the tour at every ε row before merging, not
only the PR gate (`log.md`, 2026-10-24).
