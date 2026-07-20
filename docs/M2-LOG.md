# M2 Implementation Log

Orchestrator's running log for M2 (analytic geometry, extrude/revolve,
tessellation, STL). Same purpose and conventions as `docs/M1-LOG.md`.
L-numbering continues from M1 (no new L-decisions were minted in M1;
the counter stands at L7).

## Process conventions (inherited from M1, plus M2 changes)

- Orchestrator does central planning/design/meta-review; Fable
  subagents implement and review; one implementer + one adversarial
  e2e reviewer (real consumer programs, falsification assignments) +
  one fix pass per PR.
- **Overlapped pipeline (Evan, 2026-07-16)**: reviewer N and
  implementer N+1 launch simultaneously when implementer N reports
  (N+1 stacks on N's unreviewed branch); the fix pass is the only
  serialization point. Cross-PR conventions get pinned in binding
  specs, not discovered at review time.
- High-confidence design PRs self-merge with full writeups (Evan
  reviews retroactively); fundamental forks wait. All of M2-PLAN's
  forks were resolved pre-ratification (see the #24 conversation) —
  every planned PR is self-merge grade unless implementation
  surfaces something fork-shaped.
- Branches `ev/m2-<n>-<slug>`, stacked serially, merge commits only.
- Reviewer suites are promoted into CI as `review_m2_prN` tests after
  each fix pass.
- Reading notes: Mäntylä ch. 12/13 at
  `<main-checkout>/references/notes/mantyla-ch{12,13}-*.md`.

## Carried in from M1 (docs/M1-LOG.md "M1 EXIT")

- K's numeric value — first predicate telemetry from M2's geometric
  predicates; report due at M2 exit (PR 7).
- Tier 3 (geometric validator): D4 ¶2 residual certification + the
  material wedge-angle predicate — starts at PR 3.
- The L7 allowlist moment — predicted for PR 1; resolved NOT NEEDED
  (everything stayed supertrait-shaped; CI tripwire unchanged).
- M0 linalg watchlist — discharged in PR 1 (project/reject with
  documented association, axis-through-point rotation, branchless
  basis).
- Debug-O(n²) per-op validation cost — watch when swept bodies grow.

## PR 1 (geom-curves + geom-surfaces: analytic evaluators) — 2026-07-17

- Implemented per binding spec (Fable, isolated worktree). Two peer
  crates (surfaces does NOT depend on curves — iso-curve extraction
  is PR 3's layer); closed enums with Nurbs placeholders evaluating
  to all-poison (total, no panic); conventions documented once per
  crate: shared azimuthal frame (v_ref = axis × u_ref, seam at
  u_ref), sphere latitude (not colatitude) for cross-surface seam
  uniformity, cone v = slant length with the apex a true surface
  singularity (poison normal), sphere poles chart-only defects;
  normals are the chart's ∂u × ∂v — no "outward" contract, topology
  carries sense; unit fields are conventional data, unchecked.
- Real additions: `floor` (required) + `reduce_periodic` as a
  provided projection with fixed compositional body (inherits all
  three scalar contracts by construction; honest unclamped seam
  blur); floor's kink conventions mirror abs (f64 right-plateau
  tangent 0; interval [0,0] jump-free / [0,+∞] across a step — the
  step-function analogue of the straddle hull); `copysign` with
  both-argument poison (stricter than IEEE) and the min-style
  unchosen-branch tangent discard.
- Linalg watchlist discharged: project_onto/reject_from (documented
  association order), branchless Duff 2017 orthonormal basis (the
  M0 value-branch concern resolved by having no branch; equator
  discontinuity exactly at the sign bit of n.z, documented),
  rotation_about_axis (normalizes its axis internally — posture
  asymmetry with unchecked carrier fields, documented).
- **The L7 allowlist moment did not arrive** — no `Real +` bound
  anywhere; kink selectors stayed supertrait-shaped.
- **e2e review verdict: mergeable, zero blockers, 2 NITs** (doc-only:
  underflow-band honesty at chart singularities; project_onto
  overflow band). The [0,+∞] floor enclosure survived hand-derivation
  of the mean-value criterion + 20k-box empirical attack; [0,0] on
  endpoint-touching boxes proven exact; branchlessness swept
  line-by-line; all five chart normals re-derived independently;
  torus ∂uv hand-computed and matched; the bulge-arc → Circle
  composition dry-run verified in all four winding/reflex cases
  (with the near-full-period seam caveat handed to PR 2); implicit
  residual certification dry-run confirmed real trilean teeth (a
  1e-6-wrong cache is excluded at interval). Reviewer suites
  promoted as review_m2_pr1 integration tests (19 f64 + 11
  interval).
- For PR 2: end-vertex parameters near the seam (|bulge| → 0 or ∞)
  must classify via the sliver band, never raw comparison on t_end.
  For PR 3: certify with linearized residuals ((|P−c|²−r²)/2r vs the
  linear ε — dimensional honesty); Def decorations classify, they
  are not poison; never sample normal() at the cone apex.

## PR 2 (profile crate: bulge-chain sketches) — 2026-07-18

- Implemented per binding spec (Fable, isolated worktree; overlapped
  pipeline — implemented while PR 1 was under review, stacked).
  Bulge-chain representation ratified in the #24 conversation:
  ProfileVertex{pos, bulge}, closed by construction, winding
  invisible (containment-derived roles, internal canonicalization).
- **Spec conflict resolved toward the ratified record**: the
  orchestrator's spec gloss said positive bulge "bows left"; true
  DXF semantics (the ratified rationale) is positive = CCW sweep,
  center left of chord, apex bowing right — for minor arcs.
  Implementer chose true DXF; the reviewer independently re-derived
  AutoCAD's bulge/center/apex formulas and confirmed exact agreement
  (quarter-arc, major-arc via-point, two-arc circle). Import
  compatibility holds; sagitta s = L·b/2 proven exact for ALL θ.
- Canonical form (D9): outer first (CCW), holes in discovery order
  (CW), lex-min starting vertex through an EXACT-order band
  (min-subnormal — totality + transitivity over a tolerance band;
  lex-min uniqueness is guaranteed because duplicate vertices die at
  simplicity). Byte-invariant under rotation/reversal of every loop
  (proptest + reviewer's symmetric/ulp-tied attacks); NOT invariant
  under input loop reordering (documented).
- Trilean predicate inventory (~15 named predicates, one decide
  funnel, every margin meters through a stated lever arm — sagitta,
  clearance r−|h| ≈ r·φ²/2, sliver width 2A/P, chordal defect with
  its cos(θ/4) conditioning); exact tangency ⇒ TangentialContact;
  in-band ⇒ Escalated naming the leaf predicate. Ray-parity
  containment with a deterministic golden-angle retry schedule
  (grazes refuse the ray; exhaustion is a typed error — reviewer
  showed it requires exact 16-fold adversarial alignment).
- K-hook: thread-local recording funnel + Probe scalar (delegating
  f64 wrapper); bit-identical decisions by construction; one Cell
  write per decision in production (verified by review); per-predicate
  margin distributions ready for PR 7's K report.
- **e2e review verdict: mergeable, zero blockers, 3 SHOULDs
  (doc/error-typing), 4 NITs.** DXF independently verified; every
  simplicity attack correctly rejected (lens-crossing arcs, cocircular
  overlap, pinch, spike); enter-exit-same-arc parity hand-solved;
  lever arms audited (sagitta exact; translate-to-origin shoelace
  verified live at (1e8,1e8) with ε=1e-9). SHOULD-2's finding
  recorded honestly: near-full arcs had a false-Zero regime in
  arc_span (no wrong-accept path — every probe still rejected — but
  one mislabeled error type); fixed in the fix pass. Reviewer suites
  promoted as review_m2_pr2 (24 tests).
- For PR 4: axis = ±plane normal by turn sign is PR 4's convention to
  own and document; spans come from the stored bulge (θ = 4·atan|b|,
  the sanctioned re-inspection), never endpoint atan2. For PR 3: the
  smoothness handoff verified live — validated profiles present only
  definitely-smooth (exact carrier tangency) or definitely-corner
  joins; near-tangent joins die at profile validation.
- Deferred, named: D4 ¶4 session-box enforcement at construction
  sugar (first reachable-from-innocent-input site found here).

## PR 3 (EdgeGeometry + Newell + tier-3 start) — IMPLEMENTED, REVIEW PENDING — 2026-07-18

Implementation complete on `ev/m2-3-edgegeom` (stacked on PR 2, tip
`7bf450a`, pushed; PR 2's fix pass merged in). All gates green
(fmt/clippy/discipline; debug+release × default+interval; tests
ε-parameterized). **Adversarial review NOT yet run** — the next
orchestrator's first move. Implementation report highlights (binding
facts for the reviewer spec, PR 4/5 specs, and the fix pass):

- **New crate `geom-brep`**: `EdgeGeometry<T> = Intersection{s1, s2,
  witness} | MappedCurve(..) | Seam{surface}` (no Explicit, D2);
  geometry-arena key types moved here from topo (re-exported
  unchanged); geom-brep never resolves keys (lookup closures injected
  by Body — Q1 lineage scoping).
- **MappedCurve payload** (source+map fused, incoherent pairings
  unrepresentable): `PlacedSegment{segment, place}` (rims/meridians),
  `ExtrudedPoint{point, place, vec}` (struts, s ∈ [0,1]),
  `RevolvedPoint{point, place, axis_origin, axis_dir, angle}`.
  `SketchSegment = Line{a,b} | Arc{a,b,bulge}` — structurally split;
  no geom-brep→profile dependency (PR 4 maps ValidatedSegment
  field-for-field). All descriptions evaluate over s ∈ [0,1] affinely
  aligned with the carrier interval (certification enforces).
- **`EdgeCurve` is certified-by-construction** (private fields; only
  `EdgeCurve::certify` builds one — uncertified carriers
  unrepresentable). Cert schedule: 9 dyadic samples, documented check
  order (well-formedness → endpoint pinning → per-sample residuals →
  Intersection witness on both surfaces); linearized meter margins
  (plane (p−o)·n; quadrics (q²−r²)/2r; cone ρcosα−|h|sinα,
  cancellation-free); transversality at interior samples only
  (endpoints may sit on apex/poles). Parameter interval stored as a
  certified cache — documented reconciliation with vertices-derive-
  bounds (vertices stay authoritative via endpoint pinning; storing
  keeps full-period edges total).
- **Operator signatures**: `mev(site, point, curve)`, `mef(site,
  curve, surface: FaceSurface)`, `mekr(site, curve)`, `mfkrh(ring,
  surface)`; `FaceSurface = Inherit | New(Surface) | Shared(key)`;
  ops need `T: Decide`; certification inside the atomic precondition
  phase (`EulerOpError::Certification`, body untouched). Sugar:
  `mev_line`/`mef_chord`/`mekr_chord`/`mfkrh_plug` (self-loop sites →
  canonical circle, dispatch on vertex-key equality — structural).
  Setters `topo::attach::{set_face_surface, set_edge_curve}` for the
  mvfs seed cap + post-mint Intersection upgrades; `set_edge_curve`
  enforces description-adjacency (`DescriptionNotAdjacent`). mvfs
  seed face gets `Surface::Nurbs` (honest no-description; tier 3
  refuses it at rest).
- **Dihedral predicate** `classify_dihedral → Transverse | Smooth |
  Indeterminate`: margin sinθ·r meters; implicit-form gradients only;
  lever arm r = min(curvature arms, edge-chord extent); planes
  contribute f64::MAX not +∞ (interval-lane poison bug found by test
  and fixed — from_f64(∞) is NaI).
- **Tier 3 `validate_geometric`** (gated on tiers 1–2): no Nurbs
  surfaces at rest → per-edge re-certification + adjacency → planar
  vertex residuals → interior dihedral samples definite. Tier 1
  gained geometry-to-geometry referential integrity
  (`DanglingDescription`; descriptions anchor surfaces against
  orphan-removal). Documented not-checked: self-intersection, pcurves,
  material wedge SIDE (0-vs-2π lamina needs M3 pcurves — M2
  classifies the tangent-plane wedge only), prefer-intrinsic-as-
  validity, curved-face boundary containment.
- **Newell** translate-to-origin, certified, right-hand-of-next-order
  contract; the 1e8-offset pin shows the naive cross-sum ~0.3 rad
  wrong where translated Newell is exact.
- **Suite migrations**: ~380 call sites moved to sugar mechanically;
  two semantic rewrites (stale-anchor → stale-Shared-surface
  atomicity; degenerate-pillow → explicit full-period carriers).
  Full-period edges stay representable in topo (profile's ≥2-split is
  input-layer). New: tests/geometric_cube.rs (8), in-crate
  tier3_tests.rs (6), geom-brep 30 unit tests, interval lane extended.
- **Judgment calls to ratify in review/PR**: provenance unchanged
  (description IS the geometry-side provenance; no D5 duplication);
  mef defers adjacency to set_edge_curve + tier 3; K-telemetry from
  geom-brep/topo predicates is name-tagged but NOT wired to profile's
  thread-local funnel (deliberate non-dependency; PR 7 unifies —
  Probe-lane samples read <unnamed> until then).
- **For PR 4/5** (recorded verbatim from the report): mint swept
  edges as MappedCurve with real specs; upgrade corner joins to
  Intersection via set_edge_curve after both side surfaces exist
  (mint-time Intersection impossible — surfaces don't exist yet);
  classify_dihedral chooses (smooth ⇒ keep MappedCurve; sliver ⇒
  typed error); caps via set_face_surface + newell_plane (outer loop
  in next order ⇒ outward); cosurface splits share keys
  (Shared/Inherit); Seam{surface} is the u=0 iso-curve — PR 5 places
  each revolved surface's u_ref on the closing meridian (PR 5 is the
  first end-to-end Seam exerciser; topo-level Seam tests deliberately
  deferred to it); near-apex dihedral honestly escalates (arms
  collapse), apex/pole ENDPOINTS are fine (no gradient sampled).

## Design decisions with Evan, in-session (2026-07-19)

- **Rim edges upgrade to `Intersection` (Evan's call, resolving the
  PR 4 judgment-call flag)**: cap–wall rim edges do NOT stay
  `MappedCurve` — after both caps' planes are set, rims upgrade to
  `Intersection{cap plane, side surface, witness}` via the same
  `classify_dihedral` → `set_edge_curve` pattern as corner joins
  (uniform for normal extrusion: every rim is definitely transverse).
  Lands in the PR 4 fix pass; PR 5's spec inherits (revolve cap/wedge
  rims likewise; full-period latitude rims depend on the B2
  carrier-diameter lever-arm fix, in flight in the PR 3 fix pass).
- **Prefer-intrinsic gets tier-3 teeth (orchestrator proposal, Evan
  approved)**: at rest, every *definitely-transverse* edge must carry
  `Intersection`; definitely-smooth keeps `MappedCurve` (conventional
  split per D2); escalated dihedrals and `Seam` edges exempt — so
  ε-tightening can escalate but never flip valid→invalid. Rationale:
  an unenforced preference drifts silently — exactly the shape this
  project exists to kill; cost ≈ 0 (tier 3 already samples dihedrals
  per edge). Lands in the PR 4 fix pass (validator edit is in reach on
  the stack); ratification text folds into the M2-exit DESIGN.md sweep
  under D2, removing "prefer-intrinsic-as-validity" from tier 3's
  documented not-checked list.
- **Chordal-tolerance ≠ kernel-ε separation reconfirmed** by Evan
  ahead of PR 6 (already ratified in the #24 conversation; recorded
  here because he endorsed it explicitly in-session).
- **Usability scoping ratified into DESIGN.md (Evan-requested,
  2026-07-19, branch `mngr/plan-gui`)**: new "Beyond the kernel: the
  usability gap" section (four bands: kernel-side client services /
  the GUI as a second kernel-sized project / missing subsystems —
  assemblies, drawings+HLR, feature breadth / product
  infrastructure); sequencing stance **usable-as-a-library before
  any GUI work**; the interval-transcendentals reimplementation
  moved out of the roadmap into a new "Tabled (far future)" section
  so licensing hygiene never reads as preceding usability. Two
  design-now consequences amended into **M2-PLAN PR 6**: mesh
  entity back-references (per-triangle Face keys, per-segment Edge
  keys on boundary polylines) and an empty `Appearance` attribute
  container. Flagged for future design docs: **selection stability
  / persistent naming resolution (pre-M4, D1–D9 rigor)** and the
  GUI architecture (abstract edit-vocabulary layer vs. concrete
  interaction layer — discussion ongoing, nothing ratified beyond
  the layering intent).
- **`docs/GUI-DESIGN.md` created (Evan-ratified in the same
  conversation, second round)**: G1 three-layer architecture
  (kernel / headless `editor-core` / interaction) with type-level
  boundary rules (no arena keys past layer 2; transient state never
  in the document; preview/commit structural; layer 3
  headless-testable); G2 sketcher-as-nested-editor; micro-decisions
  (expression-drag refuses with affordance — replaceable;
  error presentation case-by-case over typed renderable values;
  preview may degrade chordal tolerance, never ε). `editor-core`
  added to DESIGN.md's crate table. **Pre-M4 blockers flagged: GQ1
  (solver/replay boundary — proposed witness-plus-certification, the
  `Intersection{witness}` pattern one level up; creates an ezpz
  bit-identity audit item) and GQ2 (partial-build semantics —
  proposed per-node result DAG)**; also GQ3 edit persistence, GQ4
  document scope, GQ5 units-in-expressions (at M4), GQ6 toolkit
  (re-survey at GUI time), GQ7 selection mechanics.
- **GUI-DESIGN round 3 (same conversation)**: GQ1's *rationale*
  settled — the witness is authoritative **branch selection**, not
  authoritative geometry (constraint systems have finitely many
  discrete solutions; "solve from scratch" delegates the choice to
  initial-guess heuristics — hidden state deciding topology;
  purity preserved because the witness is recipe data:
  `solution(constraints, params, witness)`; Jacobian degeneration =
  typed error with distance-to-singularity margin). Direction agreed
  in principle, mechanism details still the open part. **GQ2
  RATIFIED** (per-node result DAG; failures poison descendants only,
  independent subgraphs complete — Evan's addition). **GQ3
  RATIFIED** (all edits persisted in v1; snapshot + edit log; edit
  schema enters versioning discipline day one). **GQ5 RESOLVED** via
  D6 (expressions in raw meters/radians; unit strings are parse-time
  sugar; display unit = presentation metadata). GQ4 stays open
  (Evan unsure) — naming doc must flag every locality assumption.
- **GUI-DESIGN round 4 (same conversation)**: **GQ1 RATIFIED**
  (Evan: branch-selection framing is "the clear correct choice";
  mechanism details remain M4/M6 work under the committed
  direction). **GQ5 RE-RATIFIED, superseding round 3**: typed
  quantities in the expression sublanguage (Evan's revision — stored
  display units mean raw storage would know less than the data;
  conversion errors type-level impossible; canonical meters/radians
  underneath, units erase before kernel `T`; the dimension-algebra
  extent is a banked M4 decision, fold into D8 then). **GQ4:
  decide-now recommended; concrete proposal in the doc awaiting
  Evan** (one document = one part recipe, possibly multi-body;
  refs document-local, no document component in the stable-ref
  type; cross-doc refs = assembly-era wrapper (doc identity × local
  ref) — composition, never modification).

## Reference acquisitions (2026-07-18)

- Mäntylä ch. 14 notes (`mantyla-ch14-splitting-algorithm.md`) and
  ch. 15 notes (`mantyla-ch15-boolean-set-operations.md`, pp.
  263–300) in `<main-checkout>/references/notes/` — the M3 grounding.
  Ch. 15 headline: boolean pipeline composes ch. 12/14 machinery
  verbatim; ~half the special-case machinery is unprinted in the
  book; suspected sign-convention erratum between Program 15.7 and
  ch. 14's rule (a) — re-derive from first principles before porting;
  results are pseudomanifolds (validator needs a 3′ mode).
- **TOG 1986 paper acquired** (the ch. 15 notes' needed second
  witness): `references/mantyla-1986-boolean-operations-2-manifolds-
  tog.pdf` — real text layer (pdftotext works; no page rendering
  needed).

## State snapshot (handoff point, 2026-07-18)

- **Merged to main**: M2-PLAN (#24); PR 1 (#27, analytic evaluators);
  PR 2 (#28, profile crate). All zero-blocker reviews, suites
  promoted.
- **Implemented, review pending**: PR 3 on `ev/m2-3-edgegeom`
  (pushed, gates green) — see the PR 3 section above; the reviewer
  has not run.
- **Next orchestrator's first moves**: (1) spec + launch PR 3's
  adversarial reviewer (falsification targets: certification schedule
  soundness incl. the 9-sample sufficiency for the M2 carrier/surface
  pairs, the certified-by-construction claim from an external
  consumer, dihedral lever-arm honesty, the stored-interval
  reconciliation, suite-migration integrity — esp. the two semantic
  rewrites — and the PR 4/5 handoff claims); (2) in parallel, spec +
  launch PR 4 (extrude) stacked on `ev/m2-3-edgegeom` per the
  overlapped pipeline, consuming the "For PR 4/5" facts above plus
  PR 2's (axis = ±plane-normal by turn sign is PR 4's to own; spans
  from stored bulge θ = 4·atan|b|, never endpoint atan2); (3) after
  PR 3's review+fix: open its PR with full writeup, self-merge on
  green (option-(a) scope is ratified; only genuinely fork-shaped
  findings wait for Evan).
- **Standing process**: overlapped pipeline (fix pass = the only
  serialization point); high-confidence design PRs self-merge with
  writeups, forks wait; reviewer suites promote into CI; M2-PLAN PR
  sequence continues 4 (extrude) → 5 (revolve) → 6 (tessellation) →
  7 (STL + mass properties + K report + M2 exit).
- **Channels**: Evan may message via new GitHub issues or comments on
  any PR/issue (re-arm the monitor each session); usage-limit
  monitoring via the mngr events file (see orchestration-model
  memory); `mngr` CLI itself currently broken (azure plugin
  ImportError) — read events.jsonl directly.
