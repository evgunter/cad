# M6-3 spec — loft/sweep body assembly (binding)

Mandate (docs/M6-PLAN.md unit 3 + the ratified rider): assemble
the loft/sweep BODY from PR 10's surfaces, flip both tier-3
Nurbs-face gates, open certify's resolve for the new classes,
land the NURBS-patch flux door, B_SPLINE_SURFACE_WITH_KNOTS
export, and the analytic-chart pcurve completion (walk row 4);
plus `tube_along_arc` (Evan-ratified rider, #175 thread). Closes
walk rows 4, 5, 12(a); shape (iii)'s loft body and the cut-loft
row go green here. This spec is BINDING: deviations are numbered
and reported with executed blockers, never improvised. The
executed assembly design in docs/M5-LOG.md:1810-1843 (PR 9c
item 6) is part of this spec by reference — do not re-derive it.

## 1. Leg A — the builder

`sweep::loft_body` + `sweep::sweep_body` (names yours) consuming
`LoftGeometry` (skin.rs:733): topology is EXTRUDE'S with
different geometry (M5-LOG item 6(i); extrude.rs:690-825 is the
model; produce an `Extruded`-style key bundle). Binding points:
- Cap-wall edges: the wall's v=0/v=1 iso IS the placed sketch
  segment — carrier stays `Curve3::Line`/`Circle` under
  `MappedCurve::PlacedSegment`; certifies today; NOTHING new
  (item 6(ii)).
- Wall-wall seams: NEW `EdgeGeometry::IsoCurve { surface, u,
  v0, v1 }` (Copy-preserving, resolves through the surface
  arena), residual = the genuinely metric
  |carrier(t) − S(u, v0+(v1−v0)t)| at the CERT schedule,
  adjacency read as surface ∈ {fs_plus, fs_minus} (item 6(iii)).
  PR 10 §3's "Line-in-UV pcurve variant" materializes as the
  exact pcurve lane for these edges — iso pcurves are exact
  straight lines in UV; store through the PR 6 doors as an exact
  lane (a new `Pcurve` variant if needed; NOT the Fitted lane —
  these are exact, not fitted).
- Final pass: whole-body `mint_pcurves` re-mint; tier-3 green at
  rest is the builder's acceptance, not a follow-up.
- The Sweep NODE lane stays collapsed (PR 10 MAJ ruling): the
  joined-path composition lane is banked PAST M6. `wire_loft`/
  `wire_sweep` flip from `CurvedSolidFrontier` to running the
  builder; the node-layer sweep arm that cannot run keeps its
  honest refusal.

## 2. Leg B — certification opens (two tier-3 flips + resolve)

- Flip A (validate.rs:1578-1583, check 1): a DESCRIBED NURBS
  surface (finite control net) passes; the mvfs PLACEHOLDER
  (all-poison control points) keeps refusing. The check must
  distinguish the two states — the discriminator precedent is
  step-export/src/writer.rs:35-48 (finite-control test); put the
  distinction ON `Surface::Nurbs`/`NurbsSurface` as a method
  (e.g. `is_placeholder()`), not duplicated inline, and update
  the ValidationError variant docs/Display which currently
  conflate the two. There is ONE copy of the check
  (tier3_local_checks_marked) — flip once.
- Flip B (validate.rs:1683-1712, check 4): Nurbs-adjacent edges
  exempt BY KIND (the Seam exemption idiom, :1454-1456) —
  implicit-form gradients are poison on NURBS; document the
  exemption in the check-4 header alongside Seam's.
- Resolve (certify.rs:769-783): `EdgeGeometry::IsoCurve` gets
  its own check-1 lane (the residual above; two-tolerance,
  definite arms included). The resolve closure accepts
  `Surface::Nurbs` for descriptions that name it via IsoCurve;
  NURBS carriers under OTHER conventional descriptions
  (MappedCurve/Seam) STAY refused with the existing honest text
  ("nothing mints one") — verify that claim still holds after
  the builder lands and update if the builder mints one.

## 3. Leg C — the NURBS-patch flux door (+V)

Tier-3 check 7 routes NURBS faces to props::curved_face →
`PropsError::Unimplemented` (curved.rs:80). Binding scope:
- VOLUME flux only — that is what +V (check 7) consumes. The
  landing site is the pre-built lane: props/quad.rs:610
  `bspline_green_integral` (hull-bounded, non-rational), wired
  from the three named refusal sites in topo/src/props.rs
  (:506, :534 stays for conic-on-noncylinder until Leg E
  delivers, :553).
- RATIONAL walls (weights ≠ 1 — any arc-bearing profile skins
  to rational walls) REFUSE typed at the existing weights gate
  (quad.rs:620-626) with recourse text naming the rational
  extension as banked. Shape (iii)'s acceptance loft is
  therefore a POLYLINE-profile loft (non-rational walls,
  ≥3 sections, ≥1 non-affine pair — the R5 text does not
  require arcs). This is the honest partial: the walk-row-5
  honesty ("the surface-AREA half has no closed form for a
  rational patch") extends to rational volume flux; do not fake
  either.
- Surface AREA for NURBS faces: stays refused typed (no gate
  consumes it; document that fact at the refusal). If some gate
  DOES consume area for the new bodies, that is a reportable
  deviation with the executed blocker, not a silent scope grow.
- Two-tolerance on every new arm; the RingOnCurvedFace law
  (props.rs:221) applies to the new faces — a loft wall must be
  ring-free like every curved face.

## 4. Leg D — B_SPLINE_SURFACE_WITH_KNOTS export

Mirror the shipped CURVE precedent (writer.rs:327 + the
:859-1010 record pins): plain B_SPLINE_SURFACE_WITH_KNOTS, and
the RATIONAL complex-instance form when any weight ≠ 1 (write
both arms even though the first corpus body is non-rational —
the curve writer's precedent, cheap, and pinned at record
level). The placeholder keeps refusing (writer kind-namer
already distinguishes). Corpus mechanics (BINDING, the M6-1/
die_pips discipline):
- New fixture: the shape (iii) loft body (`loft_prism` or your
  better name) joins `fixture_corpus()` with a HAND-AUTHORED
  `.expect` sidecar carrying the derivation in its comment
  header (volume from the loft's closed form — a polyline loft
  between affine sections has piecewise-ruled walls; derive it,
  don't measure it), byte-golden pin, check_step.sh 15/15.
- The exactness table (m5_pr13_curved.rs:174-250) gains the row
  AND its `!text.contains("B_SPLINE")` assertion must be
  RESHAPED honestly (the corpus now legitimately contains
  B_SPLINE surface entities — scope the assertion to the bodies
  that claim native-analytic exactness, with the derivation).
- Reversed-face pins (91 → new count) move WITH derivations at
  both sites, never relaxed.
- `no_export_corpus_body_carries_a_nurbs_carrier_or_face`
  (m5_pr13_curved.rs:659): the FACE half flips via the S9
  pattern (history kept — second flip of this pin's lineage;
  say so in the note). The carrier half's status depends on
  whether the loft body's seams store NURBS carriers — pin
  whatever is true with the reason.

## 5. Leg E — analytic-chart pcurve completion (walk row 4)

Close the row's honesty: cone/sphere/torus charts certify and
mint. Binding route per (chart, carrier-class):
- Closed-form harmonic where the algebra exists: extend
  `chart_image_harmonic` (pcurve_cache.rs:1193) and
  `chart_pcurve` (:1615-1851; the sphere arm's two classes are
  the pattern — polar circles, meridian great circles; derive
  the cone and torus tables the same way).
- Where the image is azimuth-NON-harmonic (the sphere's general
  circles, torus Villarceau-class, etc.): route through the
  M6-2 `Pcurve::Fitted` + `certify_fitted` lane with
  `EnvelopeStatement::OnLocusHull` — the machinery exists
  post-#176; the mate operand comes from the edge's
  intensional description. Refusal remains ONLY where neither
  route is honest, with the class named.
- `chart_mints` (topo/src/pcurves.rs:203) routing: flip
  cone/sphere/torus to mint per the same table; Plane stays
  derive-on-demand (unchanged posture). Deliverable includes
  the resulting (chart × class → route) TABLE in the PR
  writeup, and the die's 8 sphere octants + ball/cone/donut
  corpus bodies carrying stored pcurves at rest (the walk-row
  sentence that becomes true).
- Multi-ε + Interval: the new chart lanes run the loud-skip
  interval suite pattern; band-relative placements throughout.
- NOT in scope: the cyl×sphere JOIN WINDOW wiring
  (run_azimuth_window's fitted consumers — banked past M6);
  completing the CHART lane is this unit's, the join lane is
  not. State the boundary in the writeup.

## 6. Leg F — the rider: `tube_along_arc`

`sweep::tube_along_arc(center: Point3, axis: Vec3, u_ref: Vec3,
radius/major: T, arc window (t0,t1) or Full, minor_radius: T)`
— exact world-coordinate signature is yours, but BINDING:
inputs are stored EXACTLY (no profile→bulge→radius arithmetic);
routed through the SAME torus-body machinery as revolve
(full.rs lamina case for Full, partial wedge caps for windows;
no semantic fork — factor, don't duplicate); ring-torus
convention R > r > 0 enforced by the same
axis_arc_clearance-style decide; one doc sentence that
two-door-built bodies may differ by ulps from revolve-built
(no cross-door bit-identity contract). Acceptance: a
tube_along_arc donut vs the corpus donut() — same census, same
tier-3, volume exact by Pappus, minor_radius stored bit-exact
(the 56-ulp drift retired for this door — pin it).

## 7. Corpus, tour, persistence, stale claims

- Shape (iii) + cut-loft: the loft body joins the Band 4 corpus
  with standard corpus/persistence/latency rows (corpus/mod.rs
  NODE_KINDS Loft/Sweep leave the at-zero comment — flip it);
  the cut-loft row runs end-to-end IF the boolean layer accepts
  it (PR 10 §5 contract verbatim: otherwise pin the TYPED
  refusal naming the missing layer — expected: PR 9c item 5's
  edge×NURBS-face sweep layer — never a silent skip).
- Tour: skinned.rs pin_frontier retire-on-closure panic FIRES
  by design when the frontier closes — retire the narration per
  its own instruction; tour ×3 ε green.
- Frontier flips: m5_pr10_frontier.rs both rows S9-flipped;
  m5_pr10_nodes.rs :224/:331; review_m5_pr10_sweep_node.rs:77.
- Stale-claims sweep (all sites enumerated in the substrate,
  §10 of the exploration report — the spec adopts that list):
  props/quad.rs:46,:603; mesh/chords.rs:208; mesh/trimmed.rs:27;
  step-export lib.rs:91,:241 + writer.rs:45,:541,:964,:994;
  topo/props.rs:508,:535,:553; demos/README.md:203;
  demos/tour/src/main.rs:56,:256; DESIGN.md:522-541 frontier
  entries. Every surviving refusal names its TRUE remaining
  blocker.

## 8. Non-goals (typed refusals stay, docs re-pointed)

Joined-path sweep node lane; rational-wall flux + rational
surface area (banked with recourse text); cyl×sphere/cyl×cyl
join windows; canal blend; curved REST; multi-shell curved STEP
classification (named DESIGN frontier, no scheduled unit);
NURBS extent test / NurbsExtentUnsupported retirement; any
recipe edge-SELECTION vocabulary (unit 5).

## 9. Acceptance summary (the review attacks exactly these)

1. Shape (iii) loft body: tier-3 green (both flips exercised),
   certified volume vs the derived closed form, watertight,
   persisted v2 + bit-replay, Band 4 corpus rows, STEP fixture
   + FreeCAD import, tour stop.
2. IsoCurve edges certify with the metric residual,
   two-tolerance both arms; iso pcurves exact-lane stored;
   whole-body mint green.
3. Flux door: non-rational NURBS volume certified
   (enclosure-asserted in the Interval lane, loud-skip
   pattern); rational refusal typed + trio-pinned.
4. Analytic-chart table delivered; die octants + ball/cone/
   donut carry stored pcurves at rest; multi-ε + Interval rows.
5. tube_along_arc: donut parity + bit-exact stored
   minor_radius pin.
6. Cut-loft row green or typed-refusal-pinned per §7.
7. All frontier flips S9-style with history; stale-claims sweep
   complete; corpus pins moved with derivations; no silent
   deviations.

## 10. Battery scope (memories/local-battery-scope.md)

Local: touched-crate suites at default ε (sweep, topo,
geom-brep, editor-core, step-export, mesh, demos build+tour) +
the Interval lane rows for sweep/topo/geom-brep + one
discipline() pass pre-final-push. NOTE: the workspace test-tree
layout may have changed (the ev/ci-test-collapse CI unit
aggregates [[test]] targets) — branch from CURRENT origin/main,
add new tests through the aggregator pattern you find there,
and do not fight it. Hosted CI is the only gate; multi-ε rides
it. CPU-pin canary before blaming slow builds
(memories/orchestration-model.md checklist; Vantage).
