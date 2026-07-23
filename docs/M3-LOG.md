# M3 Implementation Log

Orchestrator's running log for M3 (splitting, booleans, cross-shell
surgery). Same purpose and conventions as `docs/M2-LOG.md`; the ratified
work order is `docs/M3-PLAN.md` (#42); grounding is
`<main-checkout>/references/notes/m3-grounding-synthesis.md` plus the
ch. 14/15 notes. L-numbering continues (counter at L7, unused since M0).

## Process conventions (inherited from M2 unchanged)

- One implementer + one adversarial e2e reviewer (falsification
  assignments, real consumer programs) + one fix pass per PR;
  overlapped pipeline (fix pass = the only serialization point);
  reviewer suites promote as `review_m3_prN*`; self-merge with full
  writeups on green CI; genuine design forks wait for Evan.
- Branches `ev/m3-<n>-<slug>`, merge commits only; OUTPUT DISCIPLINE
  header in every agent spec (the 64k lesson); push after EVERY commit
  (re-affirmed after the 2026-07-21 WSL crash nearly cost 3 unpushed
  commits).
- All new topology-determining comparisons through Q1 trilean
  predicates, name-tagged into `geom_core::k_stats` (unified in M2
  PR 7).

## PR 1 (Euler-inventory extensions + null-entity scaffolding) — 2026-07-21

Implemented per binding spec (Fable, isolated worktree; launched under
the M2-exit orchestrator in parallel with PR 7's review, stacked on
`origin/ev/m2-7-stl` @ `2be24f2` — pre-fix-pass; NOT yet merged with
current main). Branch `ev/m3-1-surgery`, tip `4f95b5f`, 9 commits,
all gates green (fmt, clippy -D warnings both feature sets, 3 ε rows,
interval lane). Survived the WSL host crash mid-implementation
(transcript resume; no work lost). Full report facts:

- **Null-entity lane (F9)**: `CurveGeom<T> = Certified(EdgeCurve<T>) |
  NullScaffold(NullEdge)` is the curve-arena element type;
  `NullEdge{below_end, above_end}`, `NullFacePair::{Split{above_loop,
  below_loop}, Boolean{in_copy, out_copy}}`. `Body::mev_null(site,
  NewVertexSide)` mints zero-length scaffolding (fan/strut/lone;
  bitwise point copy); mev refactored into shared plan/execute halves.
  Null-face records in a SecondaryMap with kill-op scrubbing. Tier 1
  gained pass 13 (referential-only null hooks — deliberate); tier 2
  refuses NullEdgeAtRest/NullFaceAtRest. `get_curve` REMOVED in favor
  of `get_curve_geom` + `CurveGeom::certified()` — compile-forced
  audit of every consumer; typed refusals in euler ops, mass props,
  tessellation. Fail-loud argument: EdgeCurve stays
  certified-only-constructible (forward-span gate untouched);
  zero-length representable only by type; rejected alternatives
  (EdgeGeometry variant = certification bypass; dangling-key
  sentinel = GWB null style; side table alone = dangling curve slot).
- **Cross-shell kfmrh**: same signature, extended semantics —
  same-shell = M1 genus form unchanged; cross-shell same-solid =
  shell fusion (f2 outer → ring of f1, faces re-homed in list order,
  `KfmrhResult.killed_shell`); cross-SOLID = new typed `CrossSolid`
  error (boolean combine owns that boundary — ratify). E–P: connected
  sum, genera add; postcondition asserts (0,−1,−1,0,0,0,0).
- **Cross-shell mfkrh: NO new op needed** (deviation 1, justified) —
  existing `mfkrh` already performs the lmfkrh motion (ring → face,
  shell surface splits into components; the PR 4 finding); shell-level
  split is deliberately `movefac`'s job (whole components move
  together for pass 10). Surface: `FaceSurface::Inherit` = the
  same-key share the spec asked for.
- **`split_edge`**: parent survives as [t₀,t] (he keys unchanged),
  new edge [t,t₁] with he_plus from the new vertex — both children
  forward on the unchanged carrier, no reversal. Geometry restricted
  per description kind (arc bulge′ = tan(atan(b)·(s₁−s₀));
  MappedCurve place-composition; Intersection witnesses re-minted
  bitwise as each child's schedule mid-sample; Seam unchanged); BOTH
  children fully re-certified pre-mutation. Interiority: both
  sub-spans metered in meters through the new K-funnel predicate
  `split_edge_param_interior` (typed SplitParamNotInterior /
  SplitParamEscalated).
- **`revert`**: functional new body value (operand untouched —
  Problem 15.7 both-results-free). Map: start↔end, next↔prev,
  he_plus↔he_minus (keeps every curve bitwise-unchanged and
  forward), emanating ← mate(emanating), Plane normals negated.
  Bitwise involution + determinism pinned. Planar-only (F5): curved
  ⇒ typed UnsupportedSurface (M5). Posture: reverted bodies are
  tier-2 currency; tier 3 = exactly NegativeVolume (the complement,
  by design — pinned; deviation 4, sensible).
- **`laringmv`: NO new op** (deviation 2) — existing `ring_move` IS
  it; docs now carry the ratified division of labor (containment is
  the caller's, arriving with PR 2's machinery).
- **`movefac`**: worklist component labeling over the validator's
  pass-11 glue relation, D9-deterministic (list-order seeds), comp 0
  keeps the shell, others minted (`Provenance::Movefac`).
- **`merge_coplanar_faces` (F7)**: structural (same-key) or declared
  (bit-identical Plane description) rungs ONLY — numeric coplanarity
  NEVER merges (test proves same-geometry/different-description stays
  unmerged); kef absorption + kemr for duplicate edges; staged on a
  clone with tier-2 gates both sides; typed refusals; curved same-key
  excluded (M5). Bit-equality originally via the f64 Debug
  (shortest-roundtrip) dump channel because interval deliberately has
  no PartialEq — deviation 3; the review REJECTED it with a working
  NaN-payload exploit and the fix pass replaced it (see below); a
  `Real`-level eq_bits door lands with PR 4's oriented-plane-equality.
- Integration exemplar `crates/topo/tests/m3_pr1_surgery.rs`
  (cube_with_inner_box: strut→kemr→grow→mfkrh detached-component
  recipe; multi-shell lifecycle mev_null → mfkrh → movefac →
  cross-shell kfmrh fuse-back → tier 2).
- **Judgment calls needing ratification** (fold into the PR writeup):
  CrossSolid boundary; kemr plus-side ring designation in merge (re-
  homed once containment exists); tier-1 null hooks minimal-by-design;
  the Debug-dump bit-equality channel.
### PR 1 adversarial review + fix pass (2026-07-21, session 2)

- Rebase-free re-baseline first: origin/main merged into the branch
  (clean, `ba12c82`), full gate matrix re-verified green locally.
- Review (Fable, warm worktree, suite on `review/m3-1`, 21 tests
  promoted): every falsification target HELD with executed witnesses
  except one — **F1 MAJOR: NaN-payload Debug collision**. The declared-
  equality rung's Debug-string comparison is bit-injective EXCEPT NaN
  (all payloads print "NaN"); bit-different NaN-payload Planes
  (insertable via FaceSurface::New, nothing gates finiteness pre-tier-3)
  merged as "declared-equal". Also flagged: the interval lane's channel
  was inari's Debug impl — an external crate's formatting choice as
  coincidence semantics. HELD highlights: connected-sum E–P derived
  independently and asserted exactly (genus ladder); no null-scaffold
  laundering path (set_edge_curve door closed by the forward-span gate);
  bulge′ re-derived from bulge = tan(θ/4) and bit-compared; revert
  involution bitwise on three body classes; movefac partition ==
  pass-11 components (plus code-read of why); both no-new-op deviations
  exercised e2e and sufficient. MINORs (doc'd in fix pass): circle-rim
  split_edge knocks a body out of the props inventory (typed
  NotIsoRectangle, now documented); bulge′ lane unreachable e2e until
  PR 2 (coverage note added). NITs pinned as current behavior:
  mev_null→kev roundtrip not byte-neutral (replay determinism
  unaffected); face-island merge refuses coarsely (recheck in PR 5).
- Fix pass (tip `6e6c576`): F1 fixed via per-component to_bits — new
  `Interval::repr_bits` (bound-pair bits + decoration; identity channel,
  not a comparison door) + module-private scalar_repr_bits dispatch;
  no Debug strings in any decision path; bit-identical NaN planes still
  compare equal (declared garbage; tier 3 refuses downstream —
  documented). Unknown scalar types conservatively never declared-equal
  (workspace panic-lint policy; PR 4's Real-level door is the extension
  point). Witness flipped to pin unmerged-as-required. Pass 13 gained
  the reviewer-proposed referential loop-key resolution check (typed
  `StaleNullFaceLoop`; referential-only posture preserved). kemr ring
  designation documented as "provisional designation, not truth".
- Ratification flags with reviewer assessments (final form, in the PR
  writeup): (a) CrossSolid boolean-combine boundary — RATIFY, PR 5's
  spec must name the combine door; (b) kemr plus-side ring — ACCEPT
  with the provisional-designation doc; (c) tier-1 minimal null hooks —
  ACCEPT plus the pass-13 referential check (adopted); (d) Debug-dump
  channel — REJECTED by exploit, replaced with repr_bits; Real-level
  eq_bits lands with PR 4.
- Gates at `6e6c576`: all rows green (discipline, fmt, clippy ×2,
  default + 3 ε rows, interval ×2); review_m3_pr1 15/15.

## PR 2 (split part 1: reduction + neighborhood classification) — 2026-07-21

Branch `ev/m3-2-reduce` (stacked on `ev/m3-1-surgery` @ ba12c82).
Implemented per binding spec; facts in the PR writeup. This section is
the **rule-(b) adjudication record** (F4 — a named review deliverable)
plus the wide-sector decision, both derivation-backed.

### Rule (b) adjudication: the BOOK's table is correct; TOG §3 is the erratum

Contradiction (synthesis §C): on the two symmetric ON-edge contexts the
witnesses swap verdicts — book (Program 14.6 + its table): AOA→BELOW,
BOB→ABOVE; TOG 1986 §3: AOA→ABOVE, BOB→BELOW; both agree AOB→BELOW,
BOA→BELOW. Adjudicated from the rule's stated purpose (nonmanifold
configurations must come out as DISCONNECTED pieces; no dangling
faces/edges), on the two discriminating fixtures, executed at f64 (all
ε rows) and Interval (`crates/topo/tests/m3_pr2_reduce.rs`):

- **Tangent-edge** (V-notch cut into a block's top, tip edge in SP,
  material below — Fig. 14.8's embedded form): tip-vertex entries are
  cyclically [slantL: A, tip: ON, slantR: A, cap-bisector: B] (the cap
  corner is reflex; its convex-subdivision duplicate classifies below).
  Above's material is two wedges whose face fans at the tip vertex are
  DISJOINT, and a half-edge vertex admits exactly one cyclic orbit — a
  representability fact AT THE VERTEX: one merged run/one vertex copy
  cannot host both fans, regardless of how PR 3's joining later
  completes the section. That is why TOG's AOA→ABOVE is wrong (one run
  {slantL, tip, slantR} ⇒ one copy pinned to both fans); the "4-face
  tip edge" is only one possible completion of that copy, not the
  forced consequence — with distinct section faces the completion has
  coincident distinct edges but still the two-fan vertex.
  **AOA→BELOW** yields two ABOVE runs ⇒ two null edges ⇒ two vertex
  copies, one fan each; the tip edge survives inside Below's coplanar
  top as an artifact edge (Fig. 14.2's own artifact-face story). Book
  right. [Argument sharpened at the PR 2 fix pass — the pinned verdict
  is unchanged.]
- **Touching-wedge** (notch from below, material above): entries
  [slantL: B, tip: ON, slantR: B, cap-bisector: A]. NOT settled by
  mirroring the tangent-edge argument (the originally logged "mirror
  argument" was false: copies are minted only for ABOVE runs, so at
  PR 2 exit both below wedge fans stay on the single old vertex under
  either verdict, and the fan TOG's BOB→BELOW leaves there is
  contiguous and structurally buildable — no 4-face edge follows).
  Replaced at the fix pass by two independent arguments, same verdict
  **BOB→ABOVE**: (i) ±n EQUIVARIANCE — splitting by (o,−n) reads the
  same physical configuration as the tangent-edge AOA case, and
  physical piece-assignment cannot depend on plane orientation, so the
  table must pair the verdicts: the book's BOB→ABOVE is the unique
  companion of AOA→BELOW (executed witness:
  `review_m3_pr2.rs::r1b_orientation_equivariance_pins_bob_from_aoa`).
  (ii) distinct-entity 3′ representability — BOB→ABOVE gives the
  groove fin its own vertex copies, so the below piece's tip contact
  is through distinct entities (legal 3′ touching); TOG's BOB→BELOW
  leaves the fin sharing the old vertex with both material wedges — a
  shared-entity pinch, unrepresentable per F2. Book right.
- Mixed cases: either verdict is manifold (the pieces don't touch);
  BELOW kept (both witnesses agree; consistent with rule (a)'s
  coplanar-edge-goes-below choice).

Pinned table: **BELOW-ON-BELOW → ABOVE; every other context → BELOW**
(`splitting/rules.rs::apply_rule_b`, unit-tested per row, fixtures
tested at classification AND surgery level both lanes). Honest residue:
a solid tangent to SP from one side only at an edge (no below material
anywhere in the neighborhood) still gets surgery under AOA→BELOW and
its Below piece degenerates to the bare tangent edge — PR 3's joining
must detect/refuse the degenerate section polygon (the TOG table would
skip surgery there but corrupts the embedded cases; representability
outranks the cosmetic).

Forward liabilities (recorded at the fix pass — NAMED PR 3 REVIEW
TARGETS):
- Post-reduction the tangent-tip EDGE is still shared by both slant
  faces: reduction never duplicates edges, only vertices. "The wedges
  disconnect" is a PR 3 joining outcome — untested until PR 3.
- Mirrored fact for BOB→ABOVE: the tip edge rides the above-side
  vertex copies while its two faces span below material.
- The one-sided-tangency residue carries NO machine-readable flag in
  `SplitReduction` (its null edges have `dangling == false`); PR 3
  must detect "the below side has no real material" itself.

### Wide/reflex sectors: convex subdivision (book), by derivation

A planar sector's interior is the positive cone of its bounding
directions iff its angle < 180° — endpoint verdicts then decide the
whole sector; at ≥ 180° the cone argument fails, so split at an interior
direction into two sub-180° virtual sectors (= the book's
store-twice-with-bisector). TOG's alternative (complement-and-negate at
180°, interior-vector sign beyond) answers point-membership, not
side-classification, and carries no cone argument. Subdivision
direction: definite reflex ⇒ −normalize(a+b) (true bisector); near-180°
band ⇒ n×b (90° into the interior — valid across the band, since any
interior direction with sub-angles < 180° is a legal subdivision). The
wideness trilean has no escalation cliff (duplication is sound at every
angle — documented posture); sin≈0 ambiguity (0 vs π vs 2π) is
disambiguated by a cosine predicate, with the spike-corner case (θ≈0,
distinct edges) escalating as a sliver.

New K-tagged predicates: `enters_material`, `enters_material_arm`
(geom-brep); `split_vertex_side`, `split_sector_coplanar`,
`split_sector_reflex`, `split_sector_straight`, `split_bisector_side`,
`split_sector_arm` (chord arm, neighborhood.rs), `split_sector_extent`
(face extent, rules.rs — renamed from a `split_sector_arm` name
collision at the fix pass) (topo).

### Adversarial review + fix pass (2026-07-21)

Review (branch `review/m3-2`, `crates/topo/tests/review_m3_pr2.rs`,
10 tests): **CONCUR on the pinned rule-(b) table** — BOB→ABOVE, all
other ON contexts →BELOW STANDS; the justification was amended (no
code blocker). All falsification targets HELD. Specifics:
- MAJOR-1 (doc): the tangent-edge argument sharpened to the two-fan
  vertex representability fact (the "4-face edge" was one completion,
  not forced), and the touching-wedge "mirror argument" replaced by
  ±n equivariance + distinct-entity 3′ representability — amended
  above in place; verdicts unchanged.
- MINOR-1 (code): the two distinct margins sharing the K name
  `split_sector_arm` split into `split_sector_arm` (chord arm) and
  `split_sector_extent` (face extent).
- MINOR-2 (code): the strict-row certification refusal of a
  large-coordinate crossing now surfaces as
  `SplitReduceError::CrossingInsertion { edge, endpoints, source }` —
  crossing site attached, typed Euler error nested whole.
- NIT-1 (doc): noted in rules.rs that a WideBisector duplicate can
  carry On into rule (b) (bisector exactly in-plane) and is
  table-reclassified as if an edge — book-consistent, harmless.
- Accepted deviations: dev 3 (`ScaffoldingOperand` refusal of
  mid-surgery operands) and dev 4 (rule (a) deterministic last-wins on
  a shared entry) — both accepted WITH derivations; last-wins is safe
  because genuine disagreement between adjacent coplanar sectors
  requires an invalid pinched operand.
- PR 3 carry-forwards: the three forward liabilities listed above
  (shared tip edge post-reduction; its BOB mirror; no machine-readable
  one-sided-tangency flag).

## PR 3 (split part 2: joining, finish, sectioning) — 2026-07-21

Implementation writeup: the PR description (per process conventions).
This section records the adversarial review + fix pass.

### Adversarial review + fix pass (2026-07-21)

Review (branch `review/m3-3`, 5 suites / 21 tests promoted:
`review_m3_pr3_{bob,consumer,order,pil,rings}.rs`): **MERGEABLE, no
code blocker**. One MAJOR (wording), four MINOR/NIT doc items, one
genuine pre-existing bug unearthed en route (issue #60, main-side).

**MAJOR-1 — the corrected BOB statement (executed fact, was misstated
as ±n equivariance)**: the `DegenerateSection` refusal is
orientation-DEPENDENT. It fires iff the pinched pieces lie on the
NEGATIVE side of the given normal; MIRRORED under (o, −n) SUCCEEDS;
`split(S, n)` refuses exactly where `swap(split(S, −n))` returns the
same physical decomposition (flip-and-swap workaround, now on the
error variant's docs). PR 2's equivariance principle — physical
piece-ASSIGNMENT is orientation-invariant — still holds; op SUCCESS is
not orientation-invariant. `bob_mirror_pinch_refuses_typed` pins two
BOB presentations (MIRRORED, +n) and (NOTCHED, −n), both with the
pinch on the negative side — its docs no longer claim equivariance.

Held falsification targets (all green): carve referential integrity
(no orphans/dangling keys post-carve); tiny-real-sliver not wrongly
refused vs in-band tangency escalating typed; vertex-only contact =
typed empty; single-solid gate `split` vs `plane_section` asymmetry
witnessed; `plane_section` winding consistency; two-hole box ring
re-homing both ways (+ interval lane); split through a hole = two
section polygons; MIRRORED-flipped-plane success + NOTCHED orientation
table (the BOB asymmetry, executed); concave-loop containment in/out,
ray graze deterministic retries, ray exhaustion reachable, boundary
pre-pass edges; tilted-plane f64/replay + interval agrees-or-refuses
typed; orientation flip swaps sides only.

Judgment calls (b)–(f), reviewer outcomes and fix-pass disposition:
- (b) interval-lane order coverage: **accept** — inexact-arithmetic
  shared-u crossings refuse typed (`split_join_order_u` hairline); in
  practice the interval lane splits axis planes over dyadic geometry,
  tilted planes refuse. Documented contract (order.rs), not a bug.
- (c) `plane_section` single-solid-gate bypass: **accept** — slices
  every solid into one polygon set; deliberate for a query; asymmetry
  vs `split`'s `NotSingleSolid` documented (section.rs).
- (d) winding/tangency/frame contract: **accept** — pinned in docs:
  polygons consistently CCW in (u, v) (+1 signed area); tangency
  refuses typed rather than reporting a degenerate trace; u_ref =
  normalized first chord, v_ref = n × u_ref, None iff empty.
- (e) reassembly oracle scope: **accept with doc narrowing** —
  crossing-mode census reference is the only implemented mode
  (pristine-operand mode for through-vertex sections marked future);
  `carve` is OUTSIDE the oracle's net, stated plainly; compensating
  coverage: the review's referential-integrity audit + acceptance
  censuses.
- (f) tier-3 consumers: **accept for the PR 3 gate, at PR 6** —
  split's output cannot currently feed tier-3 (no Intersection
  descriptions / no public upgrade op); recorded as a PR 6 obligation
  below.

Also: join.rs's second-chord guard (the laringmv skip window) is
book-faithful (Program 14.10 placement) but no legal fixture reached
it in this review — commented as a WATCH ITEM for PR 4/5's join reuse.

**BOB fork status**: resolved — Evan adopted (B) on #61 (2026-07-21),
with below-copy minting a COMMITTED PR 6 obligation (not a revisit;
see the accumulating list). The reviewer's two conditions were met:
(1) this MAJOR-1 wording fix (done), (2) the PR 4 charter check on
boolean BOB-routing (executed in the PR 4 review — verdict there).

**Issue #60 (pre-existing on main, fixed here)**: presented as the kef
make/kill roundtrip swapping ring ownership between the two re-made
faces. Diagnosis DEVIATED from that presumption: key-level dumps show
the kef/mef surgery restores raw ring ownership exactly; the real bug
was the iso ORACLE — coordinate-identical automorphic ring-twin
components tie in candidate encoding, the tie broke by scan order, and
a kef+mef roundtrip changes the shell face-list order, so isomorphic
bodies emitted canonical forms with the face section's outer/ring
pairing flipped (false negative). Fixed in iso.rs: `dart_attachment`
now references the committed minimal dart labels of the face's loops,
pinning later components to the committed labeling. Regression vector
re-enabled (gating); fast deterministic counterpart
`issue_60_kef_roundtrip_on_coincident_ring_twins` added; full seqgen
suite green.

## PR 4 (booleans part 1: reduction + classification) — 2026-07-21

Implementation summary (condensed; full writeup in the PR
description per process conventions): `boolean_reduce` — ch. 15
§§15.4–15.6 re-derived, TOG 1986 as second witness for the unprinted
on-edge machinery. Pipeline: gates (planar-only F5; no scaffolding
operands; F7 maximal-faces via the coincidence ladder — structural or
declared coincidence only, numeric coplanarity never triggers the
precondition), all-pairs reduction sweep (both directions, contacts
emitted as declared-contact records — the future 3′ declarations,
never scanned-for after the fact), v-v sector classification
(15.7–15.9 re-derived; 15.10 in full; TOG Tables II/III as typed
decision tables; the derived edge-edge membership rule subsuming the
angular sort + Table I ties), v-on-f classification via the ch. 14
deltas + pierced-face ring insertion, and paired null-edge insertion
with F9 attributes as data and explicit A↔B correspondence keys
(F12; `ssortnulledges` engineered out, the 15.11 consecutive-pairing
invariant guarded at runtime). The 15.7 sign INVERSION derived and
mirror-pinned (printed `IN = +1` is coherent only for inward normals;
under outward normals Enters ⇒ IN is the opposite sign).
`oriented_plane_eq` (plane_eq.rs) replaces `vecequal`: declared rung
through the one sanctioned `Real`-level bit door
(`geom_core::bit_identity`), geometric trilean for definite-different,
near-without-declaration ⇒ typed `Undeclared` (F6).

### Adversarial review + fix pass (2026-07-21)

Review (branch `review/m3-4`, suite promoted as `review_m3_pr4.rs`,
15 witnesses): **every core derivation independently CONCURS** —
Table III all 12 cells re-derived, the two corrected ∖ cells
geometrically witnessed (resting + embedded-floor fixtures); the 15.7
inversion; the edge-edge membership rule (with the R-1 carve-out,
below — now fixed); the kemr insertion sequence. Independent censuses
(two-brick, post-through-slab, notch-fill dense ties, mirrored
resting sign chain, 4-crossing stress) all green, f64 + interval
lanes.

**MAJOR-1 (process) — consumer-level bit-identity fence restored**:
the PR's ci.yml rewrite had quietly relaxed Evan's #53 invariant
(EVERY new consumer of the bit-identity channel is allowlisted +
retirement-noted) to a punning-only check ("one punning seam").
Restored as TWO steps: (a) consumer grep
(`bit_identity::|repr_bits|eq_bits`, comment lines excluded) with
allowlist {bit_identity.rs, interval.rs, merge_faces.rs,
plane_eq.rs}, step comment restating the rule (new consumer ⇒
allowlist entry + retirement-scheduled doc note in that file); (b)
the punning grep kept as a complementary check
(`downcast_ref|downcast_mut|TypeId|core::any|std::any`, single seam =
bit_identity.rs; `downcast_mut` added, stale comment fixed).
plane_eq.rs got its missing retirement note; DESIGN.md's M4 entry
restored to the every-consumer-acknowledged wording. Both directions
witnessed by scratch files (an `eq_bits` caller in topo caught by
(a); a `downcast_ref` caught by (b)).

**R-1 (behavioral) — mixed-order collinear edge overlap: FIXED**
(path taken: principled fix, well inside the time box). The
reviewer's fixture (TOG Fig. 19-left analogue: brick corner edge
collinear with a mid-span of a prism edge, dihedral wedges
interleaved) refused with `ClassificationInvariant("odd number of
surviving crossing records…")`. Diagnosis: the vertex pair hosts TWO
on-direction events — the collinear-edge overlap along the shared
line AND a transverse crossing whose direction coincides with the
wide-sector subdivision bisector. `resolve_bisector_graze`'s (and
`resolve_edge_sector`'s) fallback reference-sector search matched ANY
On record on the wide-sector twins, so the graze event keyed against
the OTHER event's face (keys (On,On) ⇒ "no crossing"), its true
crossing records were cancelled, and one germ went missing — odd
count. Fix (`find_ref_sector`, recl.rs): the reference search
requires the candidate record's On bound to BE the event's ray (its
start-holder equals the event's holder). The review test now asserts
the derived classified census: per z-cap the section polyline
(1,1)→(0.7,0)→(1,0.075) crosses A's boundary at 3 sites ⇒ 6 seam
pairs (4 v-v + 2 v-f), op-independent, seam expected for ∪ per TOG's
mixed-order criterion. Full workspace corpus green after the fix (no
regressions).

**Judgment calls 1–7**: 1–6 accepted as implemented; #7 (the F5
curved gate's acceptance witnessed only through the scaffolding arm)
needs-fix, folded — the reviewer's direct curved-face witness
(cylinder-swapped face ⇒ `CurvedBooleanUnsupported` naming operand +
face) lives in the promoted review suite (kept there rather than
duplicated into the shipped acceptance, matching the review-suite
precedent of PRs 1–3).

**BOB-routing verdict (PR 3's charter check)**: HELD on the executed
corpus — no below-copy minting anomaly on any routed fixture. The
original caveat (the R-1 class refused before reaching routing) is
now discharged in part: post-fix, the along-edge-seam fixture routes
and validates in both lanes. A dedicated below-copy audit of
along-edge seams under joining remains a PR 5 watch item (the joining
PR inherits the seam machinery and must re-witness F9 sides there).

**Small items** (fix pass): `PlaneDesc` re-exported at the boolean
module root and crate root alongside `oriented_plane_eq` /
`PlaneRelation`; `BooleanReduction::null_edges_of(operand)`
per-operand accessor added (PR 5 ergonomics), exercised in the review
census.

## PR 5 (booleans part 2: public ops) — review + fix pass — 2026-07-22

**Adversarial review** (branch `review/m3-5`, suite promoted into the
PR branch as `crates/topo/tests/review_m3_pr5.rs`: 18 f64 + 19
interval-lane tests): **no code blocker** — nowhere a silent wrong
body; every Ok the review probed carries an exact volume/area oracle,
every failure is a typed, deterministic, operand-preserving refusal.
Findings:

- **R1 (orientation halves)**: the cookie-cutter lane is
  ORIENTATION-DEPENDENT — the identical blind pocket succeeds with the
  exact volume on a brick's {+z, −x, −y} faces and refuses
  `SeamOrientation` on {−z, +x, +y} (pinned by
  `pocket_orientation_matrix`), a handedness-correlated HALF of face
  orientations. The PR's "fixed except for double-ring configs" claim
  was an overclaim.
- **R2**: multi-collinear-site seams (four crossing sites on one
  line) refuse `JoinDesync`.
- **R3**: crossing-polygon face disconnection refuses
  `SeamOrientation`.
- **R5 (contact-remap key lineage)**: recorded as a PR 6 obligation
  below.

**Honest limitation statement (reviewer's, for Evan's retroactive
review; now ratified into the `ops` module's "Known limitations"
docs)**: single-ring pockets/bosses refuse `SeamOrientation` on a
handedness-correlated half of face orientations (works {+z, −x, −y},
refuses {−z, +x, +y}); also refusing: multi-collinear-site seams
(`JoinDesync`), crossing-polygon disconnection (`SeamOrientation`),
coplanar-overlap ∩ (Fig 15.1), corner-flush. Working envelope:
transversal crossings + interior-rest coplanar unions, all with exact
oracles. All refusals typed/deterministic/operand-preserving. Root
cause (one sentence): the cross-solid null-edge ordering/orientation
discipline — `choose_roles`' prefer-mirror heuristic has no
consistency theorem — which is PR 5.5's charter.

**Merge rationale**: nothing silent-wrong anywhere probed; the working
envelope is exact-oracle-verified; the refusal surface is honest and
typed; the Fig 15.1 coplanar-overlap deferral is Evan-approved →
PR 5.5. Mergeable under the pause instruction (small fix pass only).

**Fix pass (usage-constrained, four items)**: (1) limitation statement
corrected per R1 — `ops` module "Known limitations" section, the
narrowed KNOWN LIMITATION test comment, a `choose_roles` KNOWN GAP
note; (2) volume-inequality backstop at the op gate for `Seamed`
results — vol(∩) ≤ min(vol A, vol B), vol(∪) ≥ max(vol A, vol B),
vol(∖) ≤ vol A via the exact planar mass properties, new typed
`BooleanError::ResultVolumeImplausible { which, got, bound }`, never a
panic; comparison posture: each bound margin classified through the
certified trilean (`sign_within` against the op's linear band — Q1's
only legal comparison), refusing only on a CERTIFIED violating sign
(in-band ties pass: dyadic-corpus sums are exact and the guarded bug
class violates bounds macroscopically; a poisoned margin refuses
`Escalated` — poison never passes a gate); each bound applies only
against a certified-BOUNDED operand (positive flux volume) —
complement operands (revert(B) in the A∖B ≡ A∩revert(B) oracle
route) carry negative flux volume and an infinite true volume, so
their bound is vacuous and skipped (caught live: the review suite's
revert corpus tripped the naive bound); wiring unit-tested by
construction (`volume_backstop_wiring`, mismatched `quad_prism`
bodies + the complement skip/apply pair), and the full acceptance
corpus runs green through the backstop; (3) this record; (4) review-suite promotion verified green
in the merged tree.

## Demo PR (boolean tour) — 2026-07-22

The held-back `ev/demo-boolean-tour` branch opened + merged per the
resumption order, after merging post-fix-pass main (clean, zero
conflicts) and reconciling the flagged call sites. Notable: the die's
self-promoting assert FIRED on merge — its pip targets +z, which is in
R1's WORKING set, so the pocket subtract now succeeds exact
(V=7.979192, Seamed) — and the die was promoted per the self-promoting
contract: live 6-face R1 orientation matrix (+z/−x/−y exact, −z/+x/+y
typed `SeamOrientation`), 3-pocket compose (V=7.937576 exact),
through-pillar double-ring refusal kept, guard retargeted to a −z pip
(fails loud when PR 5.5 lands the refusing half). open_box's pure
interior-top cutter now succeeds (V=2.368 exact; narration updated);
table narrations auto-track typed refusals; voidbox_cutaway's refusal
shifted RingHoming → `JoinDesync` (narration-only). NO
`ResultVolumeImplausible` sightings anywhere in the demo corpus.
Renders: openbox + montage regenerated (fix pass retessellated the
scoop seam; volume unchanged); all others byte-identical. Gates green
(fmt, clippy -D warnings kernel+demo, test-fast 997 passed, demo run
EXIT=0). `memories/boolean-consumer-findings.md` updated: finding 1
(silent wrong component) RESOLVED with evidence; finding 2's
chord-normalization workaround still in place UNTESTED against PR 5's
extrude-remap — re-check when PR 5.5 unblocks the pure variants.

Follow-up #68 (same day, prompted by Evan noticing the openbox render
looked unchanged): the pure open box — which #66 had left
narration-only — promoted to the rendered `openbox` stop (V=2.368
exact, verified open-top-only in the render); the scoop variant moved
to its own `scoopbox` stop; README boolean-leg section reconciled
(die row + limitation bullets were still pre-fix-pass); ci.yml gained
the KEEP IN SYNC back-reference to scripts/ci-local.sh (#67 comment).

**Post-5.5 demo refresh charter (Evan, #68 comment, 2026-07-22)** — for
the demo update that lands with/after PR 5.5, low effort explicitly
requested ("don't spend too much effort on this early demo"):
- cut the `scoopbox` stop;
- add whatever of the die WITH PIPS is buildable (the full pipped die
  keeps its self-promoting trigger for whenever full support lands, if
  5.5 doesn't get there);
- voidbox: either cut it from the montage (keep it in the walkthrough)
  or render with opacity < 1 — only do the opacity route if easy.
Plus the standing PR 5.5 unblock items: table/open-box pure variants,
F7 declared-rung verification, and the finding-2 extrude-remap
workaround re-test.

## Accumulating PR 6 (M3-exit sweep) obligations

Beyond M3-PLAN's own PR 6 list (F1/F2/F5/F6/F7/F8 ratifications, tier
table, voids documentation), the sweep has picked up:

- **PERF-PLAN ratifications (Evan's Q-P1 sign-off, #49, 2026-07-21)**:
  fold the §3.3 GPU boundary table and §2.2 deterministic-parallelism
  idioms into DESIGN.md as a D9 addendum. PERF-PLAN itself is merged
  and advisory; DESIGN.md stays the single contract.
- **Tier-3 upgrade path for split output (PR 3 review, judgment call
  (f))**: split's results cannot feed tier-3 validation today — by
  PR 6, split must either emit Intersection descriptions on the
  entities it mints or a public upgrade op must exist; the PR 3 gate
  accepted this deferral explicitly.
- **Sweeps-vs-voids invariant (chat, 2026-07-21, from the demo-tour
  hole discussion)**: ratify explicitly — "sweeps produce genus, never
  voids; voids are boolean-born; the extrude/full-revolve hole
  asymmetry is an instance of the invariant, not an inconsistency"
  (extruded holes = cap-to-cap tunnels, one shell; full-revolve holes
  = closed inner shells = voids; partial revolve is extrude-shaped and
  already supports holes). FullRevolveHoles' pointer to the boolean
  route lands in PR 5 per F8.
- **Below-copy minting at BOB pinch vertices (committed, Evan #61
  2026-07-21)**: PR 6 implements it as part of the tier-3′ work so the
  split-lane negative-side pinch refusal surface disappears; end state
  identical to the immediate-rework option, only sequencing differed.
  Until then `DegenerateSection` + flip-and-swap is the documented
  interim.
- **Saddle fixture for the 15.11 pairing guard (PR 4 review)**: the
  F12 runtime guard (B-cyclic adjacency + run-side agreement) is
  stressed only by planar 4-crossing fixtures; build a saddle-vertex
  fixture (non-convex neighborhood where A-consecutive ≠ B-consecutive
  pairing is geometrically realizable) that either witnesses the guard
  firing or proves the configuration unreachable for tier-2 operands.
- **Contact records are vertex-granularity (PR 4 review)**: the three
  ON-sets record vertex pairs / vertex-on-face points only; the
  tier-3′ certification must RECONSTRUCT edge-on-face and
  coincident-edge SEGMENTS from their bounding vertex records — the
  reconstruction rule (and its failure mode when a bounding vertex
  record is missing) must be designed and pinned at the 3′ gate, not
  assumed.
- **Contact-remap key lineage (PR 5 review, R5)**: boolean results
  drop contact records whose exact keys do not survive into the
  result, but the geometric coincidence can PERSIST via minted copies
  (seam-zip / merge-stage descendants of the recorded entities) — the
  remap keeps only direct key lineage. The tier-3′ certification must
  either carry records across the mint (a descendant map) or prove
  the dropped-record configurations are re-derivable at the 3′ gate;
  silent record loss over live coincidence is not acceptable there.

## PR 5.5 (cross-solid seam discipline) — implement + review + fix pass — 2026-07-22/23

Merged as **#70** (`fd90a54`), full writeup in the PR body. The short
record; the PR body is the long one.

**Derivation (F3 style)**: on the germ line of (fA,fB), region
boundary directions tA(in)=nA×nB, tA(out)=nB×nA, tB(x)=−tA(x) ⇒
antiparallel zip iff each solid attaches section loops to its own
regions CCW-consistently — op-independent, no cross-solid coupling.
**Sense theorem**: half faces germ g UP ⟺ σ·det[n_own,d,n_other]>0;
corollary sense_A(g)=¬sense_B(g) at every germ — the he1↔he2
crossover as derived data, minted as F9 attributes at insertion.
**Mechanism refinement (flagged deviation, orchestrator ruled
refinement-not-fork)**: role order does NOT carry chord orientation
(senses do); it moves face identity only, load-bearing solely for
ring splits (residual-side probe). Root cause as recorded — the
unenforced ssortnulledges discipline — confirmed; enforcement point
moved. prefer-mirror deleted; wild/bind/relabel machinery deleted;
slot-locked matching; clean_dir corrected record-sibling →
match-partner (R2's actual source); `bool_strut_order` angular spike
order added (Fig 15.1's second mechanism).

**Review** (suite `review_m3_pr55.rs` promoted, 23 f64 + interval
mod + stl e2e): NO blockers. Sense census: sense_A=¬sense_B at every
reachable germ, all ops, incl. det-2/skew/reflected operands.
`bool_strut_order` weak spot resolved: the dot rule is FORCED for
sector widths ≤ π = the whole reachable crossing-minted class;
reflex corners W>3π/2 open the unforced window ⇒ the reflex-corner
tilted-crossing class refuses SeamOrientation, typed (documented in
the envelope with the width bound). THE DIE: 21 pips, all six faces,
exact volume each op, watertight admesh STL, raw extrude operands —
`normalize_edges_to_chords` is obsolete (PR 5's remap suffices).
**Fix pass**: point_in_loop degenerate-segment poison fix closed the
mixed collinear+transversal refusal (flipped to exact success);
envelope amendments; defensive-guard notes (SectionLoopMixed +
both-arcs-dirty: unwitnessed defensive guards). Final envelope:
boundary-on-boundary seams (corner-flush UnpairedLooseEnds{4},
stacked-full {8}; germ-dump-verified reason: seams along existing
edges need an on-edge-run mechanism) + reflex-corner tilted
crossings. Gates: full ci-local 11/11 on the merged tree (main
#66–#69 + review suite merged in); regression 1022 f64 + 1152
interval at review time.

**Retrospective (with Evan, #69)**: ~7h wall — theorem complexity,
not code volume; chunking judged correct, task irreducibly hard; NO
convention change (scout/spike agents only when scouting is cheap).
Operational: reviewer AND fix-pass agent both hit the
stop-after-long-task pattern (fix pass twice; orchestrator finished
its matrix + push + PR directly). For future fix passes: point
CARGO_TARGET_DIR at a warm cache before running the matrix — the
interval lane is the cold-cost dominator (Evan commissioned a
caching investigation; findings recorded in the snapshot below).

## State snapshot (session 3 pause point, 2026-07-23)

Supersedes the session-2 snapshot below (kept as historical record).
Wind-down on Evan's instruction (#69, 2026-07-22): finish in-flight
work, record state, stop; PR 6 and the demo refresh NOT launched.

- **Merged to main** (`fd90a54`): everything through **M3 PR 5.5**
  (#70) — M3 PRs 1–5 + 5.5 all complete through the full
  implement → adversarial-review → fix-pass cycle. Today's session
  also merged: #66 (demo boolean tour; die promoted to the live R1
  matrix), #67 (`scripts/ci-local.sh` — the CI mirror), #68 (pure
  open box rendered stop + scoopbox split + ci.yml↔ci-local
  back-references), #69 (docs: post-5.5 demo charter).
- **Hosted CI is DOWN** (GitHub free-plan Actions minutes exhausted,
  all jobs fail at start with the billing annotation; resets at
  Evan's billing-month rollover). THE MERGE GATE IS
  `./scripts/ci-local.sh` on the merged tree (11-row mirror of
  ci.yml; keep the two in sync — cross-references in both files).
  admesh 0.98.5 is source-built at `~/.local/bin/admesh`. Cold-cache
  cost is dominated by the interval lane; see the caching-findings
  paragraph appended below when the commissioned investigation
  reports (in flight at snapshot time).
- **Auto-merge caveat**: the repo has no branch protection, so
  `gh pr merge --auto` merges IMMEDIATELY — do not rely on it to
  wait for anything while Actions is down.
- **Branch hygiene (one-off, Evan #69)**: remote pruned to exactly
  main + live work branches (73 merged branches deleted, 134 local
  refs pruned). Not a standing convention — Evan explicitly said
  one-off.
- **Post-5.5 demo refresh: DONE (#71, in this wind-down)** — the
  full 21-pip die is a rendered stop (exact dyadic oracle after
  every one of the 21 subtracts, final V = 7.8359375; die_blocked
  and its self-promoting guard retired — the promotion happened);
  scoopbox cut; voidbox rendered translucent (per-body alpha in
  render.py, backface culling off for translucent bodies — the
  internal void shell is visible in the render and montage);
  `normalize_edges_to_chords` deleted (raw extrude operands, per
  the review's e2e proof). Demo gates green; kernel untouched.
- **Next orchestrator's first moves**: (1) session-start checklist
  (arm away-channel + usage monitors per
  memories/orchestration-model.md; usage events.jsonl goes stale —
  advisory only). (2) **PR 6, split in three** (retrospective
  agreement with Evan): (a) tier-3′ validator + touching corpus
  (substantive; the Accumulating PR 6 obligations section above is
  the charter), (b) M3-exit DESIGN.md ratification sweep + K
  snapshot + tier table (docs), (c) any caching implementation the
  investigation below recommends (small, process). Then the M3 exit
  walk against M3-PLAN's exit criteria.
- **Worktrees at pause**: implementer worktree freed (branch merged);
  ae88 (`ev/m3-5-bool-finish` stale checkout) used for gate runs +
  caching experiments — clean both at the next seam per
  memories/worktree-disk-hygiene.md; disk was 83–84% after this
  session's cleanup.

## State snapshot (session 2 pause point, 2026-07-22)

Superseded by the session-3 snapshot above (kept as historical
record).

- **Merged to main** (`318bdf7` + docs PRs after): M3 PRs 1–4 — #53
  (surgery), #55 (split reduce), #61 (split join/finish + `split` +
  `plane_section`), #62 (boolean reduce/classify) — each through the
  full implement → adversarial-review → fix-pass cycle; process PRs
  #46–#48, #50 (worktree hygiene), #52+#54 (CI test-speed, final
  state: spade+mesh package overrides + scripts/test-fast.sh), #56
  (PR 6 obligations), #57+#58 (bit-identity retirement + tripwire),
  #59; PERF-PLAN #49 (ratified, Q-P1 answered: DESIGN.md folds at
  PR 6); demo tour #51 (`demos/`). BOB fork RESOLVED on #61: option
  (B), below-copy minting a COMMITTED PR 6 obligation.
- **In flight at pause**: (1) PR 5 implementer (booleans part 2) on
  `ev/m3-5-bool-finish` — public union/intersect/subtract + two-brick
  bitwise acceptance landed; TWO BINDING ADDITIONS delivered mid-run
  and absorbed per commit messages (`4672cd0`): the cookie-cutter
  silent-wrong-component fix + fixture family with volume oracles,
  and the extrude-operand description remap (extrude outputs
  previously refused with Merge(InputNotClosed{DanglingDescription}));
  final report pending at gates. (2) Demo agent DONE: boolean scenes
  on `ev/demo-boolean-tour` @ `d5b6511` (based on the PR 5 branch; PR
  deliberately NOT opened until PR 5 merges) — table/openbox/voidbox
  work with renders; die honestly blocked on cookie-cutter with a
  self-promoting assert; findings in memories/
  boolean-consumer-findings.md on that branch.
- **Next actions (Evan's pause instruction, 2026-07-22, final form:
  usage limits near. OK to run: PR 5's adversarial reviewer, a SMALL
  fix pass, and trivial completions — opening/merging PR 5 and the
  demo PR on green. NOT OK: PR 6 (any part), and major PR 5 rework —
  if the review finds major changes, record them in this log as the
  resumption plan instead of executing)**. The reviewer charter,
  accumulated: cookie-cutter family verification (exact-volume
  oracles; suspects were Eq 15.1 seeds / lmfkrh ring-side / carve
  classification); the extrude-operand path e2e (real profile→extrude
  operands, no caller normalization); the reworked join internals
  ("germ-keyed lockstep joining, facing+nearest, wild-strut binding,
  geometric role resolution at quiescence" — new machinery, attack
  hard); the below-copy audit under joining; the voided-split cutaway
  refusal diagnosis (Join(RingHoming) — principled refusal of the
  unbuilt cross-shell annulus assembly, or a masked wrong-loop bug?);
  touching-union refusals honesty (UnpairedLooseEnds{8},
  RingHoming — honest classification or the cookie-cutter bug in a
  typed mask?); combine-door contract vs the CrossSolid boundary;
  Eq 15.1 selection + revert lane; A∖B ≡ A∩revert(B) oracle scope;
  voids/disjoint/Empty landings; F7 output stage on declared pairs;
  D9 replay; interval lane; poison grep.
- **After the pause** (resumption order): PR 5 fix pass (+ M3-LOG
  record) → open PR 5 with full writeup, self-merge on green → open +
  merge the demo PR (`ev/demo-boolean-tour`; check its listed call
  sites against the fix pass: booleans.rs normalize/upgrade helpers,
  die_blocked's assert, table/open_box narrations) → **PR 5.5
  (Evan-approved in chat, 2026-07-22): the collinear-seam family as
  its own PR landing before PR 6** — Fig 15.1 coplanar-overlap ∩
  (the deferred PR 5 acceptance item), double-ring single-face seams
  (SeamOrientation), flush-stacked/corner-flush unions, PLUS the
  PR 5 review's additions: R1 the orientation halves (single-ring
  pockets/bosses refuse SeamOrientation on {−z, +x, +y}; matrix
  pinned in review_m3_pr5.rs — flip `pocket_orientation_matrix`'s
  refusal assertion when fixed), R2 multi-collinear-site JoinDesync,
  R3 crossing-polygon disconnection; diagnosis to start from:
  `choose_roles`' prefer-mirror heuristic carries no consistency
  theorem relating the two solids' role orders to a derived
  cross-solid orientation relation — the processing-order/orientation
  discipline the book buries in ssortnulledges, to be enforced as
  data through the germ-keyed join (validate against the PR 5
  review's assessment); full implement→review→fix cycle like any PR;
  the demo's die/open-box/table pure variants and the F7
  declared-rung verification unblock here → spec PR 6 (the
  Accumulating PR 6 obligations section above is the charter) → M3
  exit sweep.
- **Operational notes**: the usage events.jsonl
  (~/.mngr/agents/<id>/events/claude/usage/) went STALE mid-session
  (last entry 14:26Z while the session ran 13+ h more) — the usage
  monitor silently watched a dead file; successors should verify the
  file's mtime before trusting it and prefer Evan's own meter. Evan
  also notes (2026-07-22) the events file may not reflect FABLE-tier
  limits at all even when fresh — treat it as advisory only; Evan's
  in-chat warnings are the authoritative signal.
  Monitors at pause: away-channel comments+issues + watchlisted
  reactions (watchlist file in the session scratchpad), usage watch
  (stale-file caveat). Sign-off watchlist mechanism: register comment
  IDs when explicitly requesting a 👍.

## State snapshot (handoff point, 2026-07-21)

- **Merged to main**: everything through M2 exit — M2 PRs 1–7 (#27,
  #28, #31, #33, #37, #39, #43), the exit sweep (#44), M3-PLAN
  ratified (#42), GUI/usability #32, docs/memories #30/#34–36/#38/
  #40/#41. main = `98c406c`. **M2 is COMPLETE** (exit-criteria walk in
  M2-LOG's "M2 EXIT" section). K = 10 kept, run-configured
  (docs/K-REPORT.md FINAL).
- **Implemented, review pending**: M3 PR 1 on `ev/m3-1-surgery`
  (tip `4f95b5f`, pushed, gates green) — see the PR 1 section above.
  NOTE: stacked on pre-fix-pass `2be24f2`; it has NOT been merged
  with current main (PR 7 fix pass touched geom-brep props +
  tolerance docs; exit sweep was docs-only — expect small or no
  conflicts, but the merge + full gate re-run is step one of the
  review cycle).
- **Nothing else in flight**: no live background agents; the M2-exit
  orchestrator's monitors die with its session (kill any stragglers
  per the session-start checklist).
- **Next orchestrator's first moves**: (1) session-start checklist
  (kill stale pollers; arm the away-channel + usage monitors — see
  orchestration-model). (2) Merge `origin/main` into `ev/m3-1-surgery`,
  run the full gate matrix. (3) Spec + launch PR 1's adversarial
  reviewer — falsification targets: the E–P delta derivations (esp.
  cross-shell kfmrh's connected-sum genus bookkeeping); the
  null-scaffold fail-loud audit (try to make ANY consumer treat
  scaffolding as geometry through public APIs); split_edge
  certification honesty (children re-certified claim, bitwise witness
  re-mint, the arc bulge′ = tan(atan(b)·(s₁−s₀)) restriction formula,
  interiority band behavior at all ε rows); revert (bitwise
  involution, exactly-NegativeVolume posture, UnsupportedSurface);
  movefac partition vs the pass-11 glue relation + determinism;
  merge_coplanar_faces (numeric-coincidence must NOT merge — the
  round-8 teeth; staged-clone atomicity; kemr ring designation); the
  two no-new-op deviations (verify existing ops truly suffice); the
  Debug-dump bit-equality channel (f64 Debug injectivity; interval
  behavior). (4) Overlapped pipeline: spec + launch PR 2 (split part
  1 — reduction + neighborhood classification, the sign-chain PR per
  M3-PLAN; fixture-driven rule-(b) adjudication is IN PR 2's review)
  stacked on PR 1 once its report exists. (5) After PR 1's fix pass:
  open PR with full writeup incl. the four ratification flags,
  self-merge on green.
- **Standing process**: per the conventions section above; Evan's
  away-channel is GitHub comments (running thread: #41) [REFINED
  2026-07-21, session 2: watch for Evan's inbound comments; outbound
  status posts are likely missed unless he asked or is active in the
  thread; questions for Evan go out as design-doc-editing PRs (or
  issues) — see memories/orchestration-model.md]; only genuine
  design forks wait for Evan (M3-PLAN's forks are all resolved; F4's
  rule-(b) adjudication is a method commitment executed in PR 2, not
  a fork).
