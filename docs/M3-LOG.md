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
  excluded (M5). Bit-equality via the f64 Debug (shortest-roundtrip)
  dump channel because interval deliberately has no PartialEq —
  deviation 3, needs a review nod; a `Real::eq_bits` door is the
  cleaner long-term shape if PR 4's oriented-plane-equality wants it.
- Integration exemplar `crates/topo/tests/m3_pr1_surgery.rs`
  (cube_with_inner_box: strut→kemr→grow→mfkrh detached-component
  recipe; multi-shell lifecycle mev_null → mfkrh → movefac →
  cross-shell kfmrh fuse-back → tier 2).
- **Judgment calls needing ratification** (fold into the PR writeup):
  CrossSolid boundary; kemr plus-side ring designation in merge (re-
  homed once containment exists); tier-1 null hooks minimal-by-design;
  the Debug-dump bit-equality channel.
- Adversarial review + fix pass: NOT yet run — the next orchestrator's
  first move.

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
  Above's material is two wedges meeting only along the tip edge; each
  wedge's section face must meet its slant face in its own edge, so a
  shared tip edge would carry FOUR faces (slantL/R + sectionL/R) —
  non-manifold, unrepresentable in the half-edge structure.
  **AOA→BELOW** yields two ABOVE runs ⇒ two null edges ⇒ two vertex
  copies: the wedges disconnect; the tip edge survives inside Below's
  coplanar top as an artifact edge (manifold — Fig. 14.2's own
  artifact-face story). TOG's AOA→ABOVE merges one run
  {slantL, tip, slantR} ⇒ one copy pinning both wedges: the 4-face
  edge. Book right.
- **Touching-wedge** (the mirror: notch from below, material above):
  entries [slantL: B, tip: ON, slantR: B, cap-bisector: A].
  **BOB→ABOVE** isolates the tip edge into its own run (Above copies;
  the Below wedges separate; the cap-bisector run becomes the dangling
  null edge); TOG's BOB→BELOW leaves the wedges pinned. Book right by
  the mirror argument.
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
`split_sector_arm` (topo). Adversarial review + fix pass: pending.

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
  away-channel is GitHub comments (running thread: #41); only genuine
  design forks wait for Evan (M3-PLAN's forks are all resolved; F4's
  rule-(b) adjudication is a method commitment executed in PR 2, not
  a fork).
