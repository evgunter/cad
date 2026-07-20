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

## PR 3 (EdgeGeometry + Newell + tier-3 start) — 2026-07-18/19

Implementation complete on `ev/m2-3-edgegeom` (stacked on PR 2;
implementation tip `7bf450a`). Adversarial review + fix pass follow at
the end of this section. Implementation report highlights (binding
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

### PR 3 adversarial review (2026-07-19) — verdict: 2 BLOCKERS, 3 SHOULDs, 4 NITs

Reviewer ran eight falsification assignments as executed programs
(suites on `review/m2-3` @ `44427d4`; `survives_*` promotable as-is,
`finding_*` pinning defects for the fix pass to flip):

- **B1**: interval lane refused ALL inexact geometry —
  `norm_squared = self.dot(self)` squares straddling-zero enclosures
  through plain interval `Mul` (spurious negative lo), `sqrt` clamps,
  decoration degrades below Def, `Decide` reads poison. The PR's own
  interval cube passed only by being exactly dyadic. The PR 4
  implementer hit the same bug independently (convergent diagnosis).
- **B2**: collapsed lever arm (zero chord: self-loop/full-period
  edges) classified *definitely Smooth* — refused the full-period
  Intersection rims PR 5 needs, made tier 3's dihedral pass vacuous
  on self-loop edges, and near-apex sub-ε arms read definitely Smooth
  (falsifying the implementation report's honest-escalation claim).
- **S1**: 9-sample winding aliasing — intervals wrong by exactly 8kτ
  certify (executed counterexample family), reachable via public
  setter and accepted by tier 3. **S2**: Intersection interval
  side/winding unverified between endpoints (complementary arc and
  1.5-winding certify). **S3**: planar-face boundary containment
  unchecked (off-plane half-circle edge passed tier 3).
- **N1** reversed intervals certify vs the ratified he_plus-forward
  convention; **N2** zero-length edges certifiable; **N3**
  near-collinear Newell normals noise-determined but certified;
  **N4** raw-op precondition coverage shadowed by sugar.
- **Survived** (executed attacks): certified-by-construction from an
  external consumer (no leaks/mutators; atomicity proven by deep
  before/after snapshots on six failure paths); the mini-extrude e2e
  running PR 4's promised recipe clean through tiers 1–3 at f64/Dual
  (recipe validated pre-PR-4); Newell 1e8 translate-to-origin pin;
  the tier-3 corruption sweep; suite-migration audit (both semantic
  rewrites faithful).

### PR 3 fix pass (2026-07-19) — all findings closed, tip `e160079`

- **B1**: `Vec2/Vec3::norm_squared` → per-component tight `powi(2)`
  (inari pown enclosure: straddling components square to [0, hi],
  decoration stays Com). Bit-identity: unconditional for f64 and the
  Dual VALUE channel; the fix-pass report over-claimed the derivative
  channel too, but PR 4's reviewer produced executed witnesses of
  derivative-channel divergence at subnormals (3 vs 4 min-subnormals)
  and 2x-overflow (∞ vs finite) — harmless since tangents never
  decide (D8), and the in-code doc was already correctly scoped to
  the value channel. Byte-identical to PR 4's coordination patch.
  Sibling audit: torus `implicit_residual` fixed (d²+h² straddle);
  radius/bulge squares left (definitely-nonzero singletons). No new
  Real surface. The interval lane now runs the FULL mini-extrude e2e
  incl. all nine Intersection upgrades.
- **B2**: new `edge_extent` — for circles max(chord, r·(1−cos(Δt/2))),
  a certified lower bound on the point-set diameter reaching 2r at
  full period (lower bound = safe direction: smaller arm escalates
  more, never misclassifies); collapsed-arm gate in
  `classify_dihedral` (predicate `dihedral_arm`): no displacement
  scale ⇒ escalate, never classify. The 90° full-period
  plane×cylinder rim now certifies as Intersection — **PR 5's
  dependency confirmed working**.
- **S1**: circle-carrier span bound (τ−Δt)·r decided in meters ⇒
  `WindingExceeded` (kills the whole 8kτ family: with 0 < Δt ≤ τ no
  k≠0 alias is representable); interval lane refuses by DETECTION
  (clean Com decoration), not blanket poison.
- **S2**: witness contract sharpened — **the stored witness IS the
  mid-parameter point** (`WitnessMidpoint` check). Refines D2's
  "selected by the witness" (verifiable sharpening, not a
  contradiction; DESIGN.md untouched — folds into the M2-exit sweep).
  Obligation on PR 4/5: mint witness = carrier(mid). Residual freedom
  documented: circles determined up to joint whole-period translation
  (geometrically invisible); which connected component stays M3's.
- **S3**: tier 3 check 5 — dihedral-pass carrier samples classified
  against adjacent planar faces (`PlanarBoundaryResidual`); curved
  containment stays documented-M3.
- **N1** enforce (`IntervalNotForward`); **N2** zero-length REFUSED at
  the same forward-span gate (no legitimate M2 construction needs
  them); **N3** documented ("What certification does NOT pin");
  **N4** raw mev/mef precondition tests restored.
- Suites permanent as `review_m2_pr3*` (geom-brep 19, topo 13; no
  test deleted). Full matrix green foreground.
- For PR 4 fix pass: drop the B1 `#[ignore]` + sweep-crate interval
  caveat after merging; mint witnesses at carrier(mid); near-full-
  period spans escalate (PR 2 handoff's certification-side
  counterpart); attach surfaces before curved re-descriptions.

## PR 4 (sweep crate: extrude) — 2026-07-19/20

- Implemented per binding spec (Fable, isolated worktree; overlapped
  pipeline — implemented while PR 3 was under review, stacked on its
  unreviewed branch). New crate `crates/sweep`:
  `extrude(&ValidatedProfile, Extrusion{Vector|Distance}) →
  Extruded{solid, shell, top, bottom, side_faces, strut_edges}`
  (key-bundle return for PR 6/7 addressing); closed typed
  `ExtrudeError`; §12.3 sweep re-derived under CCW (all six
  mirror-check sites recorded in the report, incl. the "seed face is
  the swept face" mirror of the book's choice and the PR 5 grounding
  for rsweep/lamina-opening/loopglue); holes via bridge `mev` +
  `kemr` Empty ring + strut chain + closing `mef` + same-shell
  `kfmrh` (the genus supplier).
- Conventions owned and documented once (lib.rs): extrusion vector
  trilean-classified (normal component = the meter margin; oblique ⇒
  typed error, deferred — sheared arcs sweep elliptic cylinders,
  outside D3); the +n cap carries canonical winding (reversal
  involution iff w·n < 0); arc carrier axis = turn-signed plane
  normal; spans θ = 4·atan|b| from stored bulge (never atan2).
- Notable judgment calls: NO separate placement parameter (the
  profile carries its plane — a second one would be a
  representation-consistency condition, PR 2's ratified lesson); cap
  Newell over vertices + exact-sagitta arc apexes (2-vertex loops
  under-determine a plane from vertices alone); kemr/kfmrh not
  mekr/mfkrh (the spec's parenthetical named the wrong duals).
- **Plan correction (intrinsic math, not a fork)**: "genus 0 with
  rings" in M2-PLAN PR 4 is impossible — a through-holed extrusion
  has genus h (square + 2-vertex hole: v−e+f−r = 12−18+8−2 = 0 ⇒
  g = 1). Reviewer confirmed independently for h ∈ {1,2,3} against
  the tier-1 component E–P validator.
- The implementer independently found B1 (before the PR 3 review
  report arrived — convergent diagnosis) and applied the byte-
  identical norm_squared patch, then reverted it per orchestrator
  coordination (PR 3 fix pass owns the fix).

### PR 4 adversarial review (2026-07-19/20) — verdict: MERGEABLE, zero blockers, 1 SHOULD + 1 doc-SHOULD + 2 NITs

- Nine assignments, all executed (suites `review_m2_pr4*`, 26 tests):
  Euler hand-traces re-derived from topo's actual association rules
  (digon/2-arc-hole/multi-hole executed incl. bitwise hand-traced
  cycles); orientation via three oracles (tiers, cap Newell, an
  independent divergence-fan signed volume — exact +4.5 on the
  all-planar L both directions); canonical reversal maps pinned
  bitwise; dihedral band sweep at the strut arm never wrongly
  definite; cap Newell survives 1e8-offset arcs at all ε; interval
  post-revert honest; determinism incl. debug-vs-release byte-
  identical dumps; sub-ε oblique vector used bitwise as-given (no
  hidden snapping); genus-h confirmed.
- **SHOULD-1**: wrap-cosurface sharing short-circuited by prev-join
  precedence — a ≥3-arc same-carrier run crossing the canonical
  start split one identical-by-construction cylinder into two keys
  (convention violation, not corruption — body stayed tier-valid).
- **doc-SHOULD-2**: the interval caveat was over-broad — even
  rectilinear holed profiles refused pre-B1-fix (the hole-planting
  bridge chord is diagonal ⇒ non-dyadic direction); the ring surgery
  itself proven sound at Interval via an axis-aligned staircase.
- **NIT-1 (routed to PR 3)**: powi(2) Dual DERIVATIVE channel is not
  bitwise-equal to the dot form (subnormal + 2x-overflow witnesses,
  executed) — value channel unconditionally bit-identical; tangents
  never decide, so doc-scoping only. **NIT-2**: CosurfaceEscalated
  unreachable from validated profiles (profile simplicity classifies
  the same displacements first) — defense-in-depth, needed a doc note.

### PR 4 fix pass (2026-07-20) — tip `03ff10b`, all items closed

- **SHOULD-1 fixed by a third shape** (justified over both reviewer
  proposals): precompute ALL n consecutive-pair cosurface predicates
  (incl. the wrap pair) before minting any wall; share-with-faces[0]
  when the forward chain reaches segment 0 through the wrap. No
  re-keying, no arena mutation; 2-arc wrap behavior bit-preserved;
  predicate decisions byte-identical (only evaluation order and
  previously-short-circuited pairs changed, observable only on the
  defense-in-depth CosurfaceEscalated edge).
- **Interval lane un-gated**: B1 ignore + caveat removed; the pre-fix
  honesty pins flipped to REQUIRE tier-valid builds (diagonal-bridge
  holed profile and rotated non-dyadic placement both build through
  tiers 1–3 at Interval, all ε rows).
- **Evan's rim decision landed**: new phase 6 upgrades every rim
  (both caps, outer + ring loops) to Intersection; witness =
  carrier(mid) computed with the certification schedule's own
  association (chord midpoints fail on arcs — the bulge height);
  every rim in the whole suite classified definitely Transverse (no
  Smooth, no escalation, all ε rows); new typed `SliverRim` error;
  Smooth arm kept total (documented believed-unreachable).
- **Evan's tier-3 enforcement landed**: `TransverseNotIntrinsic`
  from the dihedral pass's existing per-sample classes (no second
  classification); Smooth conventional; Seam exempt by kind;
  escalation exempt (never flips valid→invalid); mixed sample sets
  conservatively unenforced (documented). Fallout: honest body
  upgrades across geometric_cube / interval_body / review_m2_pr3
  suites (e.g. the cube pins exactly 12 TransverseNotIntrinsic
  pre-upgrade, the prism e2e exactly 9); L-prism now asserts 18
  Intersections (6 struts + 12 rims). No check weakened.
- Suites permanent as `review_m2_pr4*`; the Dual-derivative pin kept
  under `finding_` (it pins a true scope bound, not a defect). Full
  matrix green foreground incl. debug-vs-release byte-identity.

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
- **GUI-DESIGN round 5 (same conversation): GQ4 RATIFIED** — the
  round-4 proposal plus **Evan's uniformity-principle synthesis**:
  B (global refs) discarded; C (one workspace) absorbed as "the
  document boundary is a namespace/versioning seam, not a change of
  formalism" — an assembly document is a recipe DAG of the same
  shape (instantiate-part/mates/patterns as ordinary feature
  nodes), so GQ1 applies to mates verbatim (same finite-discrete-
  solutions structure), GQ2/GQ3/naming/undo transfer with zero new
  machinery. Alternatives-considered record written into the doc
  (three axes; axis-3 binding semantics deferred to assembly design
  with Cargo.lock-style pinned-plus-explicit-update as the
  unratified leading candidate; in-context modeling noted as
  landing on the same wrapper-plus-pin extension point). **All
  GQ1–GQ5 now closed; remaining pre-M4 design work: GQ1 mechanism
  details + the selection-stability/naming doc.**
- **GUI-DESIGN round 6 (same conversation)**: axis-3 binding
  semantics **RATIFIED in direction** by Evan —
  pinned-with-explicit-update (Cargo.lock model: wrapper holds the
  pin, update is a recorded DocEdit, assemblies are self-contained
  reproducible values); pin representation / update granularity /
  conflict surfacing are assembly-design work. Also delivered
  in-conversation (round 6, NOT yet ratified — awaiting Evan's
  pushback): the "more for free" list (naming localized to reified
  predicate flips via M0 key-identity — proposed pillar of the
  naming doc; content-keyed cache transfer across rebuilds from D9
  bit-determinism; intensional-equality-before-numeric-coincidence
  as an M3 boolean principle; scalar-generic editor-core evaluation
  service) and the danger map (flip-handling semantics; boolean
  coincidence; ε-vs-persistence/assembly uniform-ε rules; SE(3)
  mate witnesses; pattern-index provenance; early corpus benchmark).
- **GUI-DESIGN round 7 (same conversation)**: Evan ratified the
  round-6 proposals — new **"Banked principles" subsection in
  DESIGN.md's Beyond-the-kernel section**: naming-localized-to-
  predicate-flips (naming-doc pillar; margin warnings noted
  far-future), content-keyed cache transfer (M2 PR 6 amended: keep
  per-face patches separable, no keying machinery yet),
  scalar-generic editor-core evaluation, ε rules **plus Evan's
  change-ε addition** (`SetTolerance` as a recorded DocEdit; apply =
  replay + D9 structural diff; any predicate-verdict change = typed
  error requiring explicit resolution — same diff machinery as the
  naming pillar), SE(3)/pattern-index/corpus-at-M4 flags.
  **Still PROPOSED pending Evan**: coincidence-resolved-
  descriptively-before-numerically (his round-7 question answered
  in-conversation: tiers (a) shared key and (b) exact description
  equality ARE "knowing in advance" — D9 bit-identity extends (b)
  across constructions sharing parameter expressions; tier (c)
  numeric classification exists only for definitionally unrelated
  geometry where advance knowledge is impossible in principle and
  sliver escalation is correct).
- **GUI-DESIGN round 8 (same conversation): coincidence principle
  RATIFIED in Evan's strengthened form — "structural or declared,
  never inferred from values."** Evan's explicit-intent revision
  fixed a latent defect in the round-6 proposal: bit-equal-
  descriptions-as-coincidence is an UNMARGINED predicate (equal vs.
  one-ulp cliff, no escalation band — a Q1 violation), and value
  equality is not evidence of intent. Final ladder: (a) shared key =
  structural; (b) equal independent descriptions do NOT glue —
  coincidence intent must be recipe data (shared surface or explicit
  relation declaration); detection is diagnostic/affordance only;
  (c) near-coincidence of unrelated definitions = typed sliver
  error, resolved by an explicit repair/adoption operation (D7
  machinery natively — reported displacement, like import healing).
  Payoffs: no silent-guess gluing anywhere; naming pillar airtight
  (predicate flips remain the only topology-change sites). **All
  banked principles now ratified; nothing in the usability/GUI
  conversation remains pending.**
- **GUI-DESIGN round 9 (same conversation): six further risk-
  reduction principles ratified by Evan into Banked principles** —
  (1) fillet/blend validity as reified margined predicates in the
  feature definition (pre-M5; enables M6 fillet-valid-over-a-
  parameter-box certification; extends the naming pillar to blend
  corners); (2) SSI completeness contract — marching finds,
  interval subdivision certifies exhaustiveness; certification is
  an at-rest tier obligation, preview may march uncertified;
  (3) non-manifold boolean results = typed errors at M3 (Evan chose
  typed error; silent body-splitting rejected as inexplicit);
  (4) expression sublanguage total/finite by charter (anti-OpenSCAD;
  Turing-ish stays in the generator layer); (5) DOF diagnosis =
  structural combinatorial layer + GQ1 singularity-margin layer,
  never conflated ("degenerate configuration" ≠ "over-constrained";
  ezpz boundary = numbers only); (6) bit-exact float persistence
  (Ryu round-trip suffices for finite values; explicit NaN/inf
  policy; replay-identity CI test). Residual risks acknowledged as
  execution-not-design: blend-corner geometry, tuning dials,
  rebuild latency (corpus defends), GUI scope.
- **PR #32 orchestrator review incorporated (2026-07-20)**:
  (1) Vertex keys added to the PR 6 mesh back-references
  (Evan-agreed; completes triangle→face, segment→edge,
  endpoint→vertex at zero construction cost). (2) Appearance
  revised to the review's **option (B)** — field-less named type,
  NO keyed container at M2; the home is type + module + doc
  contract ("attach here, keyed by stable names, from M4") —
  resolving the two-names tension (arena-keyed container = fake
  durability + migration debt); design session concurs, Evan was
  still weighing at review time — flip is one line if he lands
  otherwise. (3) The PR 3 "S2" witness-aliasing lesson carried into
  GUI-DESIGN GQ1 (the witness contract must pin which point).
  Orchestrator will fold final PR 6 amendment text into the
  implementer spec; branch synced with main at incorporation time.

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
