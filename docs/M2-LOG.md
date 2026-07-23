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

## PR 5 (sweep crate: revolve) — 2026-07-20

- Implemented per binding spec (Fable, fresh worktree off main — the
  first attempt was killed 3× by the 64k output-token-per-response
  limit; the fresh spec baked in the chunked-writes discipline that
  fixed it, see the memories update). Files:
  `sweep/src/revolve/{mod,axis,surfaces,partial,full,upgrade}.rs`.
  API: `revolve(&ValidatedProfile, RevolveAxis{origin,dir},
  Revolution::{Full|Partial(signed θ)}) → Revolved` (key bundle);
  in-sketch axis, r = (p−origin)·ê_r, profile must sit in r ≥ 0;
  θ > 0 sweeps toward −n (cap-outwardness derivation, never the
  book's matrotate sign — mirror-check 4).
- Owned conventions: shared azimuthal frame (every revolution
  surface axis = +a₃, u_ref = u₃ anchored on the placed axis ⇒
  u = 0 IS the profile half-plane, meridians are literally the
  seam); full period minted bitwise-identical at the seam (zip
  pairs by construction-record keys, never geometric matching);
  plane-wall meridians keep MappedCurve (`SeamOnNonPeriodic`
  refusal); rim carriers θ-signed for forward intervals.
- **Washer op-sequence (the full.rs-referenced hand-trace)**:
  mvfs → 3×mev chain → closing mef (start disc, transient Nurbs) →
  4×mev full-period rim struts (interval (0,τ], endpoint = start
  bitwise) → 4×mef walls (copied chain C₀..C₃; 2 cylinders + 2
  plane annuli) → kfmrh(start_disc, seed) — copied chain demoted to
  ring, THE genus supplier → zip: mekr(Cycles{E₃⁻@v0, C₀⁺@v0ʳ},
  self_loop_circle_at) + kev(N₀⁺); for j=1..3 mef(E_{j−1}⁻, C_j⁺) +
  kev(N_j⁺) + kef(C_{j−1}⁺); final kef(C₃⁺) → upgrades (cylinder
  meridians → Seam, plane meridians conventional, 4 rims →
  Intersection, witness = antipode). End V4 E8 F4 R0 ⇒ g = 1. Zip
  null edges carry self_loop_circle_at scaffolding (endpoints pin —
  distinct bitwise-coincident vertices, doc widened in the fix
  pass). Op-for-op the book's loopglue with Inherit replacing temp
  face −1 and explicit keys replacing list-head reliance.
- **Two-band wire case (novel — the book's ball is broken at poles,
  Problem 12.2 unsolved there)**: one-band wire sweeps leave poles
  valence-1 (tier-2 ScaffoldingStrutVertex ban — ratified behavior
  found by the body's own postcondition). Axis-touching full
  revolves therefore sweep two π-bands: band 1 = ordinary π-sweep;
  band 2 = per-interior-vertex rim-closing mefs carve the π..2π
  walls out of the surviving wire face. Poles get valence 2 (angle-0
  + angle-π meridians); angle-π meridians are conventional (not
  Seam — u ≠ 0). Ball V2 E2 F2 g0 (one sphere key); cone V4 E6 F4
  g0. Consequence for the exit sweep: the "minimal" V2/E1/F1 sphere
  is unrepresentable at rest — a line in the D-doc sweep.
- Axis-contact trilean classes (exact on-axis / sliver band typed
  error / generic), half-plane check, surface catalog by trilean
  parallel/perpendicular/on-axis classification; scope calls:
  full-revolve-with-holes ⇒ typed `FullRevolveHoles` (per-hole seam
  surgery unexercised by the acceptance set; revisit on demand);
  `UnsupportedToroid` conservatively refuses arcs whose CARRIER
  crosses the axis even when the arc stays clear (D3 ring-torus
  rule upstream).
- **e2e review verdict: MERGEABLE, zero blockers, 1 docs-SHOULD
  (this section — the branch predated main's log sections), 2 NITs,
  1 OBS.** All ten falsification assignments SURVIVED, executed:
  two-band construction attacked across 5 wire shapes (4-segment
  dome, split cylinder, megaphone, silo, ball) with band-pair
  single-key assertions; washer zip lineage pinned (survivors =
  exactly the 4 chain edges + 4 rims; wall loops hold their
  meridian twice); forged Seams refused (SeamSide / 
  SeamOnNonPeriodic) incl. under rotated placement + oblique axis;
  witness bitwise = mid-parameter antipode, start-point witness
  refused e2e; the implementer's volume oracle audited — found to
  be sign-only (coned polyhedron), magnitude supplied by the
  reviewer's independent Pappus line integral (<1e-6 rel) + a
  revolved-mesh ball check; trilean bands exercised at every ε row
  incl. a negative false-positive hunt; interval lane REQUIRED
  tier-valid on all shapes + dome/donut/non-dyadic wedge; D9
  debug↔release dumps byte-identical (the flagged-unasserted item,
  done); all 14 error variants reachable or verified-honest
  defensive; mirror-check-5 subsumption verified (the book's
  opening exists only because plain rsweep needs a wire; the zip's
  mekr+kev plays that role — final entity sets identical).
- **Fix pass (tip 27219b0)**: NIT-1 radial_extent now folds
  arc-interior radial extrema, comparison-free via copysign gating
  (off-arc candidates negated, never win the max); NIT-2
  self_loop_circle_at doc widened to the zip's
  distinct-coincident-vertices shape; Seam variant doc aligned to
  the SPATIAL definition (u_ref half-plane meridian — on
  mirror-nappe cones chart u=0 is the spatial-π meridian; found by
  the PR 6 implementer, certification was already spatial and
  self-consistent); review suites promoted by merge
  (review_m2_pr5 + interval, names kept); full matrix green.
- For PR 6: wire bands are u∈(0,π)/(π,2π) patches (valence-2 poles
  with TWO meridian boundaries); plane meridians are MappedCurve —
  never key seam handling off edge kind alone; full-period rims are
  self-loops — ONE chord-point set per edge. For PR 7: use
  Pappus/divergence for mass properties, NOT the sign-only fan
  oracle; the reviewer's meridian_pappus_volume is a starting
  point.

## PR 6 (mesh crate: tessellation) — 2026-07-20

- Implemented per binding spec (Fable, isolated worktree; overlapped
  pipeline — implemented while PR 5 was under review, stacked; the
  branch also merged main mid-flight to pick up the #32 amendments,
  which the spec had wrongly claimed were in its base). New crate
  `crates/mesh`: `tessellate(&Body<f64>, δ) → Result<Mesh, _>`;
  `Mesh` = positions + per-face `FacePatch`es (separable, ratified)
  + `BoundaryPolyline`s with Edge/Vertex keys (+ per-triangle Face
  keys — the full #32 back-reference chain); NO Appearance artifact
  (ratified final). Files: types/chords/walk/curved/planar/
  tessellate/validate.
- **Per-triangle exact-form deviation certificates** — the export
  promise is certified per emitted triangle (typed
  `CertificateExceeded`, never an uncertified mesh): plane 0;
  cylinder/sphere r − dist (exact for inward deviation; outward
  bounded by the vertex-on-carrier ε residual — documented promise
  is δ+ε); cone cosα·sinα·v_max·(1−cos(Δu/2)) (radial-sum
  contraction, both nappes); torus (3/4)(R+2r)·L² (chart Hessian
  bound, ~4× conservative). Sizing is a δ/2 heuristic; the
  certificate is the guarantee (it caught a live cone mirror-nappe
  bug during development). Honest δ-vs-ε: ε read exactly once (pole
  vertex identification), never for sizing.
- **Watertightness**: chord points once per edge (endpoints bitwise
  vertex points), every polyline segment a CDT constraint in both
  adjacent faces; the ratified pure-function invariant stated
  verbatim in lib.rs (per-face tessellation = f(surface, loops,
  per-edge chords, δ) — the incremental-retessellation memo-key
  contract). Seam welds by identical 3-D ids at u=0/2π; poles enter
  the CDT as repeated-id corner copies whose degenerate triangles
  drop ⇒ valence-correct fans; `Surface::normal` never called
  (winding from UV orientation + shoelace flip — the ∂u→0 pole
  poison is unreachable).
- **spade 2.15.1** (CDT): age/license policy pass; determinism
  audited at source level (hashbrown confined to unused modules;
  insertion order fixed: boundary walk then grid row-major;
  panic-on-crossing pre-checked into a typed error).
- Judgment calls: f64-only API (display/export layer; branch-heavy;
  D8 replay reaches display through the f64 lane; interval
  workspace rows still pass via feature passthrough); display-layer
  comparisons deliberately NOT Q1 predicates (documented list —
  none decide kernel topology; certificates + check_mesh + oracles
  are the backstop).
- **e2e review verdict: 1 BLOCKER, 1 SHOULD, 3 NITs — everything
  else survived** (5 suites / 38 tests, promoted as
  `review_m2_pr6_*`). Survived under execution: all four
  certificate bounds independently re-derived + attacked with
  from-scratch distance oracles (45 samples/triangle × 13 bodies ×
  δ sweep — zero violations of δ+ε); check_mesh audited with six
  hand-broken meshes (all rejected typed); debug↔release AND
  ε-row {1e-6,1e-9,1e-12} meshes bitwise-identical; outward-shell
  assumption verified unbreakable through the M2 public API; the
  PR 5 Seam-doc spatial-definition finding independently confirmed.
  BLOCKER: partial revolves θ ∈ (3π/2, 2π) with a pole junction in
  a rim-anchored loop failed typed (`Triangulation`) — the
  junction's meridian column unwrapped nearest prev_u, but past
  3π/2 the wrong branch is closer; polygon self-crossed; the
  crossing pre-check fired. Root-caused with fix direction by the
  reviewer; the certificates' fail-loud posture held (never a bad
  mesh). SHOULD: tessellation wall-clock ~quadratic in per-face
  point count (spade insertion path) — documented, carried to PR 7
  as fine-δ STL guidance. NITs: sphere certificate margin thin at
  the equator; check_mesh combinatorial-only (doc); 0.1-rad closure
  snap silent.
- **Fix pass (tip 8233eeb)**: the final meridian traversal now
  unwraps nearest the loop's closing anchor (`out[0].u`) — exact by
  construction for every wedge angle (the anchor lies on that
  meridian plane analytically); previously-passing shapes
  byte-identical (old/new branches coincide there). Flipped test +
  extended sweeps: θ ∈ {3π/2+0.01 … 2π−0.01} × {cone apex wedge,
  dome-cap rim+pole loop} × δ, with exact analytic volume/area
  oracles. Sphere grid sizes at δ_s/1.25 (margin factor, removes
  the equator trap). Closure snap 0.1 rad → 1e-9 +
  `debug_assert!` (release falls through to the typed error —
  closed enum preserved). Perf + check_mesh doc notes added. Full
  matrix green (64/64 suites per row).
- For PR 7: consume `Mesh.positions` + `patches[].triangles`
  (outward winding guaranteed); drop keys for STL;
  `validate::{signed_volume, triangle_count, check_mesh}` public
  (pre-flight); mesh bitwise-deterministic incl. debug/release ⇒
  byte-identical STL needs only a deterministic writer; mass
  properties via Pappus/divergence over the EXACT B-rep (per plan),
  never the mesh fan; fine-δ exports pay the quadratic CDT cost.

## PR 7 (stl crate + mass properties + K report) — 2026-07-20/21

- Implemented per binding spec (Fable, isolated worktree; branch
  `ev/m2-7-stl` off post-#41 main, tip `2be24f2`). Layout:
  `geom-brep/src/props/` (key-free per-face closed forms — the
  geom-brep pattern, lookup closures injected); `topo/src/props.rs`
  (`mass_properties(&Body<T>) → {volume, surface_area}`; living in
  topo lets tier 3 consume it with zero new inter-crate edges); new
  `crates/stl` (lib depends on mesh only); recorder moved to
  `geom_core::k_stats`; harness `sweep/tests/k_report.rs`;
  `scripts/check_admesh.sh` + CI `watertight` job; `docs/K-REPORT.md`
  + raw CSVs.
- **Closed forms** via per-face anchor split ∮p·n = ∮(p−c_f)·n +
  c_f·A⃗_f with the vector area A⃗_f = (1/2)∮(p−ref)×dp exact per edge
  (line + arc forms): plane flux = origin·A⃗ (rings by stored
  winding); cylinder s_f·r·Area + o·A⃗; cone flux = apex·A⃗ (anchor at
  apex needs no interior sign); sphere s_f·R·Area + c·A⃗ with Area =
  R²Δu(sin v₁ − sin v₀); torus closed form confirmed numerically.
  Iso-rectangle verification is STRUCTURAL from stored data only
  (carrier axes, minted param spans, circle centers/radii; zero
  atan2); anything outside the M2 inventory ⇒ typed PropsError, no
  quadrature fallback. f64 ≤1e-12 rel on all acceptance shapes
  (incl. donut 2π²Rr², τ−0.01 wedge, axis wedge); interval enclosures
  contain analytic values at ≤1e-9 width; Pappus + mesh signed_volume
  cross-checks as test oracles only (the sign-only fan prohibition
  held).
- **+V invariant** into tier 3 as check 7: margin V/A_total (a
  length — mean boundary displacement; dimensionally honest lever),
  Negative ⇒ NegativeVolume; Zero AND escalated exempt (orientation
  probe, not thinness gate; never-flips posture); VolumeUncomputable
  ⇒ invalid (every at-rest M2 body computes); gated on an
  otherwise-clean tier-3 report.
- **K unification**: funnel + Probe + MarginSample moved to
  geom-core; profile::k_stats a re-export shim; geom-brep/sweep(×2)/
  topo funnels delegate; zero sign_within call sites outside
  geom-core; decisions bit-identical (one added Cell write before an
  unchanged sign_within); `<unnamed>` unreachable (asserted over the
  full harness corpus). **Evan's #41 addendum**: AMBIGUITY_K const →
  run-configured `Tolerance.k` (env CAD_AMBIGUITY_K, default 10,
  finite >1 validated, OnceLock; re-exec test proves K=25 reaches
  Band::linear).
- **K data** (docs/K-REPORT.md DRAFT): 13,282 samples/row ×
  {1e-6,1e-9,1e-12}, 63 predicates; zero indeterminate/invalid, zero
  escalation-band landings; bimodal margins (zero-side ≤8.9e-16,
  min definite |m|/ε = 1e4); counterfactual K ∈ {3,10,30,100} all
  decision-equivalent. Draft: keep K=10, scoped (native corpus
  well-conditioned; D7 import is the future data source).
- **STL**: streams mesh order exactly (no snap/dedup/reorder),
  constant 80-byte header (never "solid"-prefixed), explicit
  to_le_bytes, ASCII floats = shortest-round-trip Display;
  byte-identity pinned across repeat builds, ε rows, debug↔release
  (print_stl_hashes oracle), ASCII↔binary parse-back. admesh gate
  check-only (no repair counted as success); dry-run clean on all 7
  acceptance STLs.
- **Implementer findings**: (1) coarse-δ cone apex fans emit
  exactly-collinear triangles (distinct indices, zero area —
  invisible to PR 6's id-degenerate drop and combinatorial
  check_mesh; live at δ=0.05); writer refuses typed
  (DegenerateTriangle); mesh-side fix deliberately deferred to a
  PR 6 follow-up. (2) f32 narrowing makes sliver triangles exactly
  collinear ⇒ as-written-vertex normals impossible; normals stay
  f64-winding-derived; admesh "Normals fixed" un-gated with
  rationale (reversed/backwards strict). (3) Third interval x·x
  negative-lo poison occurrence (props_rim_level; same class as PR
  3 B1 + torus sibling) — fixed with tight powi(2); banked as
  memories/interval-square-poison.md.
- Gates all green at 2be24f2: 777 tests/row × 3 ε rows, 884 × 2
  interval lanes, clippy -D warnings both feature sets, fmt,
  discipline grep, admesh dry-run. Deviations flagged: sequential
  supervised gate script (10-min cap); param-spans-instead-of-bulge
  re-inspection (satisfies the no-atan2 intent); per-body δ in
  exact_vs_mesh (quadratic-CDT cost).
### PR 7 adversarial review (2026-07-21) — verdict: MERGEABLE after fix pass; zero BLOCKERs, 1 SHOULD, 3 NITs

Ten falsification assignments, all executed (review branch `review/m2-7`
@ `a0ceddd`; six suites promotable as-is; one test intentionally failing
at tip, flipped by the fix pass):

- **F1 SHOULD**: the props iso-rectangle gate checks carrier SHAPE but
  not incidence-on-surface — an off-axis "rim" with correct radius and
  axis-parallel carrier is silently accepted (flux of a curve nowhere
  on the surface); same class across cone/sphere/torus/meridian cases.
  Unreachable via tier-3-validated bodies (re-certification pins
  incidence) but `mass_properties` is public on unvalidated bodies ⇒
  silent wrong volume there. **F2 NIT**: stale "single tolerance env
  var" docs (ENV_K exists). **F3 NIT**: M2-LOG PR 7 section absent from
  the impl branch (handled on `ev/m2-exit` — this section). **F4
  NIT/OBS**: torus s_f vertex-tag contract is load-bearing (lying tags
  flip the sign; trust boundary to be stated on `LoopEdge`).
- **Survived (executed)**: every closed form vs an independent Simpson
  oracle (incl. cone both-nappes no-s_f proof and a from-scratch torus
  re-derivation) ≤1e-9 rel; public-op shapes vs the reviewer's OWN
  closed forms ≤1e-12 (cup, frustum+bore, quarter-donut, pac-man,
  two-hole plate, groove/bump washers); no legal path to a silent
  wrong s_f; inventory-escape ⇒ typed + VolumeUncomputable; a mirrored
  cube forged through PUBLIC Euler ops reaches NegativeVolume (the
  check is stronger than the implementer's "no public path" claim —
  claim corrected, check vindicated) incl. at 1e6 scale; gating
  unmaskable; the thin-inverted-slab exemption boundary pinned
  executable (V/A ∈ (ε,Kε) escalates and passes — the ratified
  orientation-probe posture); base↔tip decision bit-identity (FNV
  probe over 8 bodies incl. 143k-tri donut); powi-fix f64
  bit-identity; zero sign_within sites outside geom-core; K=25
  band-reach re-exec; K CSVs byte-reproduced from scratch at all 3 ε
  rows with every reported number re-derived independently
  (13,282/row, 63 predicates, 0 in-band; counterfactual table
  confirmed); STL parsed by independent spec-derived parsers (byte
  layout, LE, constant header, f32-cast identity, unit outward
  normals, ASCII↔binary bit-agreement, subnormal round-trips);
  determinism incl. debug↔release print_stl_hashes both profiles run
  by the reviewer; admesh gate non-vacuous under 5 byte-level
  mutation classes (a normal-only flip FAILS as "reversed" — cannot
  hide under the un-gated "Normals fixed"); consumer e2e (vase +
  bracket, public API only) through to external verification; full
  gate matrix re-run green. OBS: 3 refusal-path predicates never fire
  on the all-valid corpus (scoping sentence added to K-REPORT in the
  fix pass).

### PR 7 fix pass (2026-07-21) — all findings closed, tip `69a396f`

- Review suites promoted by merge (c4b1235, names kept; 6 suites).
- **F1**: five new certified incidence residuals in props (all `props_*`
  named, joining the K funnel; closed-form math untouched):
  rim axis-parallel (‖n_c × â‖·r_c, lever = rim radius), rim
  center-on-axis (‖w − â(w·â)‖ — kills the off-axis counterexamples
  and the sphere w∦axis gap), cylinder meridian-on-surface (radial
  offset at interval start), cone meridian-apex (apex-to-line
  distance), torus meridian-plane ((n_c·ρ̂)·ρ, lever ρ ≈ R; n_c now
  pinned ∥ τ̂). Sphere meridians needed nothing (center + radius
  already complete). Review test flipped to require the exact typed
  variant; module doc rewritten to state precisely what is and is not
  certified. **Valid geometry proven untouched**: pre/post acceptance
  STL exports + V/A values byte-identical; all 1e-12 closed-form pins
  green at every ε row.
- **F2** both stale tolerance-doc sites corrected (ENV_EPS + ENV_K);
  **F4** `LoopEdge` "Trust boundary: vertex tags" doc section (tags
  are trusted data; no residual catches a tag lie; torus s_f is the
  load-bearing consumer; topo's flattening correct by construction).
- K-REPORT gains the refusal-corpus scoping bullet (3 refusal-path
  predicates never fire on the all-valid corpus; adversarial-corpus
  data awaits D7/M3).
- Full matrix green foreground (post-crash re-run): fmt, clippy -D
  warnings both feature sets, 3 ε rows (78 suites each), interval
  lane per package group, admesh dry-run (user-local dpkg extraction;
  script unmodified).
- Session note: a WSL host crash interrupted both the fix pass and
  the parallel M3 PR 1 implementer mid-run and emptied 11 loose
  objects in the shared git store; repaired by empty-object deletion
  + re-fetch (fsck clean), both agents resumed from transcripts, no
  work lost. Push-after-every-commit re-affirmed as non-optional
  (the M3 branch had 3 unpushed commits at crash time).

## M3-PLAN drafted and ratified mid-M2 (2026-07-20/21, PR #42)

At Evan's prompting ("any reason not to get started on planning M3
now?"), the M3 work order was drafted during PR 7's review window and
**ratified same-day** (#42, merged b52d8df). Grounded in a new
second-witness synthesis (`references/notes/m3-grounding-synthesis.md`:
ch. 14/15 notes cross-examined against the TOG 1986 paper — which
confirms our CCW/outward convention, supplies the unprinted srecledges
machinery via its Tables II/III, but contradicts the book's rule (b)
table; zero code listings, no proofs). Fork resolutions with Evan on
#42: curved intersections defer to M5 as a unit (the ellipse-at-first-
oblique-cut argument; no speculative curved-readiness abstraction in
M3); non-manifold = non-representable while 3′ touching is typed
success under the explicit-intent invariant (Evan's condition,
three-part text in the plan); ∅ a typed success value. K dropped from
the fork list (Evan: no approval needed; empirically reasonable value
self-merges).

## M2 EXIT (2026-07-21) — exit-criteria walk (M2-PLAN final section)

- **Extruded and revolved parts (incl. ringed profile, genus-1
  revolve, the ball, the cone) end-to-end from profile data through
  public ops only** — DONE: L-prism, holed extrusion (g=1), washer
  (g=1), ball, cone, donut, partial/axis wedges; acceptance + review
  suites across PRs 4–7.
- **Tier-1 after every op; tier-2 + tier-3 (residual certification +
  orientation/volume invariant) at rest** — DONE: tier 3 implemented
  PRs 3–7 (residuals, dihedral classification, prefer-intrinsic
  enforcement, planar-boundary containment, +V invariant with the
  exemption boundary pinned executable).
- **Watertight STL exports verified externally** — DONE: admesh
  0.98.4 check-only CI gate (`watertight` job), all 7 acceptance STLs
  clean; gate proven non-vacuous under 5 byte-level mutation classes.
- **Mass properties match closed forms within certified bounds** —
  DONE: f64 ≤1e-12 rel; interval enclosures contain analytic values
  (width ≤1e-9); reviewer's independent closed forms and quadrature
  agree.
- **CI green at ε ∈ {1e-6, 1e-9, 1e-12} + interval lane** — DONE:
  78 suites/row, 0 failures; interval lane green (geometry
  evaluators instantiate at Interval throughout).
- **K report delivered, outcome ratified into Q1 residue** — DONE:
  docs/K-REPORT.md FINAL (keep K = 10; now run-configured
  `Tolerance.k` per Evan's #41 direction); Q1 residue closed in this
  branch's DESIGN.md edit.
- **New conventions ratified into DESIGN.md at exit** — DONE in this
  branch: witness-midpoint (D2), prefer-intrinsic teeth (D2 +
  tier 3), M2 structural conventions (single-shell/voids, minimal
  sphere, parameterizations, profile format), chordal-δ ≠ kernel-ε
  (D4).
- Carried out of M2: mesh-side collinear-apex-fan drop at coarse δ
  (PR 6 follow-up; the STL writer refuses typed meanwhile);
  refusal-path K telemetry awaits an adversarial corpus (D7/M3);
  debug-O(n²) per-op validation cost watch continues.

**M2 is COMPLETE** upon this branch + PR #43 merging.

## Design decisions with Evan, in-session (2026-07-19/20)

- **`FullRevolveHoles` is permanent; voids are born only from
  booleans (Evan, 2026-07-20)**: PR 5's typed refusal of
  full-revolving holed profiles is upgraded from a scope deferral to
  a standing rule, with the invariant **sweeps emit single-shell
  bodies**. Rationale: in a full revolve a hole's swept walls touch
  nothing (no caps, no wedge faces) — the cavity boundary is a
  disconnected interior shell, so direct support would mean revolve
  emitting multi-shell bodies with internal voids, silently breaking
  M2 machinery documented against the no-voids assumption
  (tessellation's outward-shell orientation rule) a milestone before
  M3's boolean/void machinery exists. `revolve(outer) −
  revolve(hole-as-outer)` produces the same solid through the front
  door. Ergonomics, if ever wanted: an M4 recipe-layer sugar node
  ("revolve holed profile" ⇒ revolve + subtract) — sugar above the
  kernel, never a new kernel emission mode. Ratify the invariant
  into DESIGN.md at the M2-exit sweep; point the error text at the
  boolean route once M3 lands. (`UnsupportedToroid` likewise stays:
  it is a D3 ring-torus boundary, not a scope cut — spindle tori
  have no representation to land in.)

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
- **Appearance FINAL (Evan, 2026-07-20): full deferral — no M2
  artifact at all**, superseding option (B). Evan's question ("what
  does (B) buy over deferring?") exposed the flaw in (B)'s premise:
  its only content beyond deferral is a grep-able tripwire type,
  but the type's correct home is the document layer (`editor-core`),
  which doesn't exist until M4-era work — landing it at M2 means
  parking it in `topo`/`mesh`, modeling the exact layering mistake
  it was meant to prevent. What survives is the contract, ratified
  in DESIGN.md Band 1: display attributes attach in the document
  layer, keyed by stable names, from M4, and nowhere in any form
  before that. **PR 6 implementer spec: drop the appearance item
  entirely** (back-references item unchanged, incl. Vertex keys).

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

## State snapshot (handoff point, 2026-07-20)

- **Merged to main**: M2-PLAN (#24); PR 1 (#27); PR 2 (#28); PR 3
  (#31, EdgeGeometry/tier-3 + fix pass); PR 4 (#33, extrude + rim
  upgrades + tier-3 prefer-intrinsic enforcement); PR 5 (#37,
  revolve incl. the two-band pole construction); PR 6 (#39,
  certified tessellation + back-references); the GUI/usability
  design PR (#32, Evan-signed); docs/memories PRs #30, #34–36, #38.
  All reviews zero-blockers-after-fix-pass; all reviewer suites
  promoted (`review_m2_pr1..6*`).
- **Pipeline drained**: nothing in flight, no live background
  agents, no unmerged work branches. This session's orchestrator
  hands off here (context length), per [[orchestrator-handoff]].
- **Successor's assignment: PR 7 (STL + mass properties + K report
  + M2 exit)** per M2-PLAN's PR 7 entry + exit criteria. Inherited
  facts, binding: mass properties via the ch. 13 Pappus/divergence
  formulations over the EXACT B-rep — NEVER the mesh fan oracle
  (sign-only, coned-polyhedron volume; PR 5 review OBS); the
  reviewer's meridian_pappus_volume in review_m2_pr5 tests is a
  starting point; byte-identical STL = mesh bitwise determinism
  (verified incl. debug↔release and across ε rows) + a
  deterministic writer; fine-δ tessellation is ~quadratic
  wall-clock (documented, mesh lib.rs); external
  watertight/manifold checker in CI if available, else
  mesh::validate::check_mesh (combinatorial-only — signed_volume is
  the orientation backstop). K report: unify the K-telemetry
  channels (profile's thread-local funnel vs the name-tagged
  geom-brep/topo/sweep predicates — M2-LOG PR 3 judgment call),
  gather multi-ε margin distributions across the suites, write the
  keep-K=10-or-revise recommendation.
- **M2-exit DESIGN.md ratification sweep** (accumulated list):
  witness = mid-parameter point under D2 (PR 3 S2); prefer-intrinsic
  tier-3 enforcement under D2 + remove from not-checked (Evan
  2026-07-19); sweeps emit single-shell bodies / voids only from
  booleans + M4 sugar note (Evan 2026-07-20); genus-h plan
  correction pointer (already inline in M2-PLAN); chordal-δ ≠
  kernel-ε conventions (PR 6); parameterization/profile-format
  conventions per the M2-PLAN exit list; the
  minimal-sphere-unrepresentable-at-rest note (tier-2 valence-1
  ban, PR 5); K outcome into Q1's residue.
- **Standing process**: one implementer + one adversarial e2e
  reviewer (falsification assignments, real consumer programs) +
  one fix pass per PR; self-merge with full writeups on green CI;
  only genuine design forks wait for Evan; OUTPUT DISCIPLINE header
  in every agent spec (the 64k lesson — orchestration-model M2
  lessons); monitors per the session-start checklist.
