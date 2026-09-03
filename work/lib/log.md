# LIB-LOG — orchestrator log for the usable-as-a-library program

Program contract: `docs/LIBRARY-DESIGN.md` (RATIFIED, PR #229).
This log is the program's operational record — unit dispatches,
unilateral orchestrator decisions (LB-numbered), and resting state
— in the M*-LOG tradition. This program runs concurrently with the
M6/M7 close-out (its own orchestrator, its own logs); the fence
between the two lanes' footprints is recorded per-spec.

## Rulings absorbed at program start (Ev, in-chat, 2026-08-06)

Recorded in LIBRARY-DESIGN.md §L8; operational consequences here:

1. **U1 + U2 authorized to start now** (LQ5 execution); units past
   that are delegated to orchestrator judgment where footprints
   are independent — Ev: "things past that likely are also
   viable." Genuine design forks still escalate.
2. **Façade placeholder crate name: `pncad`** ("pending-name CAD")
   — greppable, carries the Q9 rename debt visibly. See the
   docs/NAME-CANDIDATES.md for the rename-time grep note.
3. **v2 profiles-as-programs spec timing**: the design-conversation
   draft waits for U2's algebra to be implemented AND the demo
   corpus reworked onto it — the rework is the evidence base for
   what the representation should be. Still ahead of U9 (§L3's
   "Python never ships the opaque-profile state" stands).
4. **A/B**: library-program implementation dispatches draw from
   their own LIB-labeled block series in MODEL-AB-LOG (no
   collision with the M7-N series the other orchestrator draws).
5. **Lane slots**: Ev is building flock-based build-slot locks
   (`cargo-slots.txt` is RETIRED in place); until the script
   lands on main, the 10 GB / two-parallel-cargo-lanes ceiling is
   enforced by this log's slot line.
   **SUPERSEDED same night: PR #230 MERGED (2026-08-07 ~00:00)**
   — `local-scripts/with-build-slot.sh`, machine-wide flock semaphore,
   WIDTH 1 (serial builds measured ~40% faster than 2-wide;
   PR #230 has the numbers). Both running implementers were
   messaged to wrap every cargo call in it; all future dispatch
   briefs carry it. Any number of agents may be alive — only
   their builds queue.

## Dispatch record

| Unit | Spec | Model (draw) | Lane | Status |
|---|---|---|---|---|
| U1 façade | docs/LIB-U1-SPEC.md | OPUS (block LIB-1 draw byte 13 = opus,fable; difficulty S logged pre-draw) | lib-u1 | **MERGED #232** (27/27; A/B row recorded at merge). Review APPROVE-WITH-FIXES 0/2/3, rubric 5/3/3; fix pass complete (guard pin proven by executed falsification; Band into prelude; 2 honest closure exceptions). Residue filed: #234 (DuplicateName unnameable), serde_json::Value exception flagged for U9 (see backlog note), #235 (stale .holder cosmetics). Lanes cleaned. |
| U2 PATHS | docs/LIB-U2-SPEC.md | fable (block LIB-1 remainder; difficulty L logged pre-draw) | lib-u2 | **PR-1 MERGED #233** (26/1-skip; A/B row recorded at merge — review 1/1/3, rubric 5/4/5; sign-domain gates landed via the crash-surviving lane diff, adopted by a finisher). **PR-2 MERGED #238** (review APPROVE 0/0/3, rubric 5/4/5, zero silent devs; all seven claims independently re-executed incl. the bracket's exact ulp drift). **UNIT U2 CLOSED.** NOTE-3 pickup banked: the zero-geometry-diff contract has no re-runnable in-repo pin — bank a differential regression row as a U3 rider. |

## Orchestrator decisions (LB-numbered)

- **LB1 (2026-08-06)**: U2 is staged as two PRs — PR-1 the algebra
  + lowering + differential tests (touches `crates/profile` only),
  PR-2 the demo-corpus profile rework (touches `demos/tour`),
  sequenced after U1's façade rework merges. Rationale: the two
  authorized units otherwise collide in `demos/tour`; PR-1 is
  also the natural review boundary (algebra semantics vs
  mechanical rework).
- **LB2 (2026-08-06)**: the U1 façade re-exports the full
  authoring surface as modules + a curated prelude; the
  SurfaceKind-leak closure is specified as a CLOSURE PROPERTY
  (every type reachable through re-exported public error enums is
  importable from the façade) with a compile-level test, not as a
  one-off re-export.

## v2-conversation evidence accumulator

Findings that feed the profiles-as-programs representation draft
(ruling 3 above); source = the U2 PR-1 implementer report:

- **Stadium/slot profiles are UNAUTHORABLE in the ratified
  surface** (finding 7): both-sides-tangent closer + parallel
  carriers — every closure door refuses, and PQ4 blocks mid-carrier
  seams. Real vocabulary gap, not an implementation artifact.
- NURBS legs have no v1 representation to lower to (finding 1) —
  banked for v2 exactly as PATHS-DESIGN anticipated.
- ε_input plumbing for the algebra's junction checks is
  unspecified in the doc (finding 2; run-global Tolerance::get()
  used) — the v2 spec should say where path-authoring tolerance
  comes from.
- Fillet-trim canonicalization is anchor-based (finding 10) —
  defines the bit-identity expectation for PR-2's scene rework
  (anchor-consistent scenes lower bit-identically; others change
  SAID not shape and need per-scene care).

**PR-2's corpus-scale walls (report §6; the richest v2 input yet):**

1. **Directors-as-angles are ulp-dirty** (sin_cos quantization) —
   the corpus's ONE line×line fillet (bracket, the #101 showcase)
   could not move because .angle(PI) carries 1.22e-16 into the
   ray; chord-derived directions are exact. Exactness depends on
   which spelling bound the ray.
2. **Missing arc binding modes**: via-point (4 loops) and
   centre-first (2 loops, one with documented carrier intent).
3. **Closed carriers unauthorable** (4 plain circles — the
   corpus's most common raw shape); finding 7 generalizes to any
   closed-carrier/both-sides-tangent loop.
4. **Arc-carrier fillets** (rocker's 5: arc×line, arc×arc) all
   outside the v1 line×line door.
5. **No far-end-anchor spelling** for a post-fillet side ending
   at a sharp vertex.
6. Polygon/rect sugar is the single most-wanted verb (12 of 26
   loop sites are polygons; slab's extents tuples are already
   dimension expressions the chain re-flattens).
7. "Algebra-authored ≠ validated" (junction checks are local;
   the bowtie authors cleanly) — an honest doc point, not a bug.

## U9 backlog notes (accumulating)

- `MigrationStep`'s `serde_json::Value` payload is the one closure
  exception a Python binding will actually meet (U1 audit) —
  decide at U9 whether pncad re-exports serde_json or the payload
  gets a typed wrapper.
- `BooleanOp` name collision (topo's in the prelude, editor_core's
  by path) — revisit if bindings prefer the document-layer one.

## Incident log

- **2026-08-07 (day): WSL CRASH** (Ev, evening). Symptoms as
  seen from this side: the U2 PR-1 fix pass died TWICE to stream
  stalls ("no progress for 600s"), then the orchestrator session
  itself was down ~10h. Probable trigger: machine load (the U1
  fix agent had measured load 14–19 under three concurrent
  lanes; the U1 fix-pass byte-identity re-run was blocked 4×
  by the same contention). Crash-window battery results were
  already treated as suspect; ~151 lines of partial fix work in
  the lib-u2 lane survived intact. On resume: monitors re-armed,
  state re-verified (#233 OPEN/MERGEABLE, main unmoved at
  e7213f6), one fresh finisher dispatched — ONE build-running
  agent at a time until WSL stability is understood; width-1
  slot locks remain the ceiling for anything heavier.

## Program state at the U2-close seam (2026-08-08)

- **U1 + U2 both MERGED** (#232, #233, #238); A/B rows recorded.
- **PROFILES-V2-DESIGN RATIFIED (#242 MERGED, 2026-08-08)** after
  a three-round conversation: round 1 accepted the drift-proof
  driver construction + serde-as-transport; round 2's probe
  RETRACTED the bowtie-forces-raw-seat claim; round 3 Ev
  delegated VQ1 — **RULED (b)-DIRECT** (chain-only schema; the
  additive-vs-subtractive LQ7 asymmetry decides: raw can be added
  later additively, removal has a pre-release deadline).
  **Revised ladder consequence**: vocabulary-growth units precede
  the switch — **G1** (cheap set: circle primitive, arc_via,
  arc_center, far-end anchor, VQ4 exact directors) then **G2**
  (arc-carrier fillet modes; sizing starts by measuring sugar's
  existing arc-leg fillet forms, M5 S2/#137); then the SWITCH
  unit(s) (schema v4 chain-only, replay driver, Expr binding,
  slot addressing); U9 queues behind the switch (Ev: no hurry).
  U5/U6/U7/U8 remain freely schedulable around them.
- **U3 MERGED #245** (2026-08-08; A/B row at merge — review
  APPROVE 0/0/3, zero silent devs, all claims independently
  re-executed; the base was found to silently BUILD an invalid
  interior section where the branch refuses typed). One
  vocabulary for all four body ops; SectionSegments deleted;
  the census's split-brain door closed structurally. NOTE
  riders banked for the G-series: error-precedence doc line,
  per-call loop clone. Lanes lib-u3/lib-u3-review cleaned.
- **G1 MERGED #254** (2026-08-08; dual-rubric A/B row at merge).
  The bracket moves bit-identically (VQ4 proven); raw census
  down to 3 (boss/rocker/bowtie); ArrivalKind fix red-checked
  both directions. Far-end-anchor design fork ADJUDICATED at
  merge: `.angle(θ).to(p)` accepted — both reviews' conformance
  audits found it doc-faithful (no new DOF, shared resolution
  path, entry refusal parallels U2's `.at` precedent), the
  Start-variant absence stays pinned. G2 remains: rocker's
  arc-carrier fillets. Lanes cleaned. |
- Next in judgment scope after U3: U5 (read-back), U6/U7
  (relations/selection); U4 wants its LQ3 measured spec; U8→U9
  queue behind GQ5 + the v2 ruling.

## Resting state (2026-08-06)

Slots: 1 = lib-u1 (Opus), 2 = lib-u2 (fable). Monitors:
disk-watchdog + hourly-checkin armed in this session;
away-channel NOT armed (Ev present in-chat; watchlist empty).
The v2 representation design conversation is QUEUED behind U2
PR-2's merge (ruling 3 above). Next units in judgment scope after
U1/U2: U3 (SectionSegments retirement) and U5 (read-back) are the
natural nexts; U7 unblocked (M6-5 merged #219/#220).

**Dual-review sample #1 = G1** (2026-08-08): post-amendment
implementation rows are U3 (1), M7-5 (2; the other orchestrator's
KLINT-GATE correctly excluded as CI-infra), G1 (3) → G1 draws the
first dual sample. R1 and R2 both in flight against the same head
(lanes lib-g1-review / lib-g1-review2), identical briefs, R2
blinded to R1's existence and report. Both rubric triples and
finding counts will be recorded R1/R2 in the row; fix pass off
the adjudicated union.

**Dual-review sample #1 result (G1/#254)**: R1 and R2 CONVERGED —
both APPROVE-WITH-FIXES, both independently found the identical
MAJOR (Zero-fit far-end anchor emits an unauthored outgoing
tangency declaration → spurious TangencyContradicted on legal
sharp continuations; §4-item-2 declaration-without-construction).
Complementary residue: R1 got the §3 table ordering + merge
footprint notes; R2 got the t2-vs-anchor verbatim-vertexhood gap,
the §2a PQ4-phrasing clause, and the missing in-band gate rows.
First reviewer-variance data point: HIGH agreement on the
consequential finding, disjoint tails. Fix pass dispatched off
the union.

**G2 dispatched** (2026-08-08, OPUS — first slot of triple LIB-3
(opus,opus,fable), L pre-logged; lane lib-g2, branch
lib/g2-arcfillets, spec docs/LIB-G2-SPEC.md from the executed
census). Risk item named in the brief: derived-corner bitwise
exactness for the rocker differentials. G2 = post-amendment row 4
(single review); remainders (opus, fable) bank for the next two
units. After G2: the SWITCH unit (schema v4 + replay driver +
Expr binding per PROFILES-V2) is the ladder's next rung — its
spec is the orchestrator's next writing task.

## G2 findings-back rulings (LB3–LB6, 2026-08-08; high-confidence
## elaboration class — Ev retroactive, veto window on #259)

- **LB3 (the Bounds blocker)**: the compound-Bounds allowlist
  EXTENDS to one new file — a `path` arc-fillet submodule
  confining the lifted-S8-ladder call — carrying sugar.rs's
  ratified justification verbatim (same rule, same diagnostic
  channel, representation-level selection between
  already-classified constructions; never a re-decision of
  geometry). This PRESERVES the S8 amendment (plain deterministic
  selection, no funnel entry) — the decide-predicate route would
  violate it and refuse-multi-candidate forecloses the eye. The
  G2 fence is amended: the ONE allowlist line for that file is
  permitted, reported.
- **LB4 (anchor lottery)**: NO anchor-fitting, ever. The
  squared-radius circle×circle form is ruled IN as the design
  (structurally exact — the correct closed form, not fitting).
  Sites migrate only where their NATURAL anchors (design-stated
  points, provenance reported per site) land bitwise; others stay
  raw with the wall named — the bracket precedent extended.
  "Line×circle corner derivation is anchor-rounding-dependent" is
  a named wall and v2-accumulator evidence.
- **LB5 (seam-at-fillet)**: the rocker OUTLINE stays raw — its
  mid-arc seam vertex is authored topology (one vertex, one
  lateral face) that the algebra's seam-fillet retrim would eat
  and PQ4/item-4 correctly refuses to reproduce as a mid-carrier
  junction. Wall named; v2 evidence. The EYE migrates (its sharp
  tip is a genuine two-carrier junction that the new Start
  spelling keeps).
- **LB6 (naming fork)**: `.to_on(Start, centre, winding)`
  accepted — the addition §2a item 4 was deliberately left open
  for; distinct from `.to(Start)` (same-carrier retrim) by
  exactly the two-carrier-junction distinction the implementer
  identified.

- **LB3 AMENDED (2026-08-08, from Ev's factoring question on
  #259)**: the selection family (nearest_candidate + the lifted
  joint ladder) moves to its own shared module
  (`profile::fillet_select`-shaped, allowlisted with the S8
  justification, both doors call it) rather than living per-door.
  End state: THREE allowlisted files — sugar.rs boundary, path
  boundary, fillet_select — each with a purpose-matched one-line
  justification. Rationale: the discipline tracks type-level
  bounds, so boundary files can't leave the allowlist, but the
  selection family is a coherent design object with two consumers
  and future S8-family growth lands there. Implementer redirected
  mid-flight; the extraction's bitwise pin guards the move.

- **LB3 correction (implementer finding, accepted)**: TWO
  boundary allowlist entries, not three, and NO CI change —
  `fillet_select.rs` uses sole-bound `T: Bounds`, which the
  ratified rule permits everywhere (the tripwire matches only
  compound bounds). The factoring SHRINKS the discipline
  surface. §3 surface still to land; its design is fully
  recorded (#259/#261/report), so a fresh finisher rebuilds it
  mechanically. G2 session errors self-reported honestly (slot
  held ~2h across a branch switch; ~250 lines lost to an
  over-broad rm) — both recorded for the A/B row's fix-pass
  narrative.

- **LB7 (U7 scope, from the census)**: U7-v1 ships STRUCTURAL
  selectors only — role-path-shape queries over the ratified
  RoleSeg vocabulary (RimEdge(Top,_), Seam{Cap,Band}, …),
  all_faces/all_vertices materializer siblings, and the missing
  pncad doors (NameTable/EntityKind/EntityRef/all_edges/
  edge_name-family exports — StableName is currently write-only
  at the façade). Everything stays a MATERIALIZER (evaluate →
  resolve → store Vec<StableName>) per the M6-5 freeze doctrine;
  no live queries in recipes. GEOMETRIC predicates (carrier
  kind, adjacent-surface pairs, convexity, position) are
  DEFERRED to a designed follow-up: they are decided-predicate
  sites under DESIGN.md's margins discipline and interact with
  unratified GQ7 — shipping them library-first would de-facto
  ratify GUI selection mechanics. Structural-first forecloses
  nothing (geometric selectors add later under their own
  design); the naming collision with profile::fillet_select is
  noted — U7's module gets a distinct name.

- **LB8 (U8 split, from the census)**: U8's display-unit STORAGE
  (per-literal units → ExprKind::Literal/WireExpr/bit_eq/schema
  v4) collides with the SWITCH unit's surface — the schema
  breaks ONCE, so U8b (storage + full 25mm round-trip) FOLDS
  INTO the switch unit's spec. U8a (quantity newtypes at the D6
  boundary, unit table, formatter-with-unit-as-argument, the
  checking text parser against the current AST) ships now,
  schema-free. The geom-core Length<T> name collision is a named
  spec risk. Difficulties pre-logged: U7-v1 = M, U8a = M. Draws:
  U7 → LIB-3 slot 2 (OPUS), U8a → LIB-3 slot 3 (fable).

- **Note (Ev, in-chat 2026-08-08)**: LB7/LB8 confirmed as
  sequencing-class. When the geometric-selector follow-up is
  designed, it should RE-HOME GQ7's selection-filter portion out
  of GUI-DESIGN into the library design docs — Ev: "a bunch of
  general-usefulness stuff got originally mentioned in
  GUI-DESIGN even though it's more broadly applicable." The GUI
  becomes a consumer of the general mechanism, not its owner.

**SWITCH spec drafted (2026-08-08, PR #263 — OPEN, awaiting two
Ev inputs)**: (1) PROFILES-V2 §V3 REVISED — the naming-stability
claim was contradicted by the required measurement
(canonicalization is geometry-dependent; lex-band crossings
renumber; posture = the M6-5 freeze doctrine, Vanished fail-loud);
(2) corpus-authorability dent to VQ1(b) — boss's 3-arc split and
the die half-disc are same-carrier-by-design, chain-unauthorable
forever; recommendation = split-control primitives in the
circle() lowering style; half-disc equator-vertex measurement
ordered first. Spec staging: 3 PRs as TWO A/B units (SWITCH-P
profile L, SWITCH-E editor-core+lift XL); hard-sequenced behind
the G2 finisher (path.rs) and U8a (Unit type). Drafter also
re-verified f64-resolution and found memo hashing already
satisfies V3 via resolved-bits convention.

- **LB9 (Ev on #267)**: geom-core's classify-seam `Length<T>`
  RENAMES to `Margin<T>` (the audit's own vocabulary) — a
  mechanical workspace sweep as its own orchestrator-review PR,
  sequenced after U8a's review concludes; the quantity `Length`
  then owns the name unqualified. And F4's preimage search is
  recorded as the STATELESS STOPGAP: the switch spec's U8b
  section must state that stored per-literal provenance
  supersedes the search on the authored path (search remains
  only for computed values, where the information genuinely
  does not exist).

- **LB10 (G2 finisher's mechanism wall)**: the
  straight-arrival-off-arc-departure refusal is ACCEPTED as a
  recorded wall (§2b) — no corpus consumer exists (rocker's
  outline is raw per LB5), it is unreachable from pre-existing
  programs, and both escape routes (path.rs as a second
  compound-Bounds allowlist entry; capability erasure putting
  .fillet behind Bounds) enlarge exactly the audit surface LB3's
  confinement exists to keep small. Revisit with a concrete use
  case, PATHS §7 posture; the two-route menu is recorded in §2b.
  Also noted from the finisher: the setsid lesson (harness
  timeout kills the process group — slot-queued runs need
  setsid to survive) goes to the lane-ops memory at next seam.

## Seam state (2026-08-09, the parallel batch closes)

- **U8a MERGED #267** (row recorded); **G2 CLOSED, MERGED #268**
  (row recorded; raw census = boss→circle_split-at-switch /
  outline LB5 / bowtie permanent). **Margin rename #270** (LB9)
  merging on green — k-lint margin stream byte-identical, 377
  sites. **Express lane #269 MERGED**, #235 closed; lane-ops
  memory updated (--express + setsid patterns; all future briefs
  carry them).
- **SWITCH-P DISPATCHED** (opus, LIB-4 slot 1, L pre-logged;
  spec §3, lane lib-switchp). SWITCH-E waits on SWITCH-P + the
  U7 merge; its spec §§4-8 already binding.
- **U7 review still in flight** (the last of the batch's
  verdicts; slot-queue dominated).
- Dual-review counter: provisional U8a=4, G2=5 (pending the M7
  orchestrator's #266-class confirmation on the #268 thread);
  row 6 = the next blinded merge (likely U7 or M7-6) — flagged
  at merge time; retroactive R2 is the remedy if the count
  shifts.

- **Dual-review counter CONFIRMED** (M7 orchestrator, #268
  thread): #266 orchestrator-class; U8a=4, G2=5 → **U7 = row 6,
  draws dual review**. R1 was dispatched as single before the
  count resolved; remedy per protocol = R2 dispatched at U7's
  merge against the frozen merge head, independent (no R1
  access). Fix pass consumes the R1∪R2 union — so U7's fix pass
  WAITS for R2, and the merge happens after both (the row
  records both rubrics at merge as usual).

- **LB11 (U6 scope, from the census — U6 DOES NOT DISPATCH as a
  unit now).** The census found: (a) the P9 flush helper is
  value-inferred declaration — C4's forbidden pattern, legal only
  in fixture code; the honest library form is detect(findings) /
  declare(Node::Declare by name) / two-armed refusal menu (no
  absorb arm, per the #256 ban); (b) the DETECTOR is a geometric
  selector — inside LB7's deferred scope; (c) P5's declared-offset
  is authoring/expression-layer (derived table = base + stated
  deltas — Expr-shared-subtree territory), re-homed to
  post-SWITCH-E. Consequence: U6's substance merges into the
  GEOMETRIC-SELECTOR FOLLOW-UP design conversation, which now has
  three consumers (LB7's deferred predicates, GQ7 re-homing,
  U6's finder+declaration+menu) and is ripe to draft. The thin
  declare-by-name sugar ships with it, not before. Fixture twins
  stay put (legal where they are).

- **LB13 (Ev, in-chat 2026-08-09): the LB12 seal deepens, two
  parts.** (a) `pncad` DROPS the whole-crate `editor_core`
  re-export (measured: 2 tour consumers, both curated-servable)
  — the document layer exposes only its curated surface;
  kernel-direct crates KEEP module re-exports (keys are that
  layer's native vocabulary; U1 closure property unaffected).
  Preliminary-no per the VQ1 asymmetry: widening later is
  additive, narrowing post-release is breaking. (b) The boundary
  becomes a TEST, not a soft rule: a rustdoc-JSON public-API
  check that no arena-key type appears in any signature of
  pncad's document-layer surface (source-grep guards miss
  signature leaks — exactly how EntityRef escaped). Lands on
  U5's fix pass (same territory as LB12).

## Ev review-thread inputs (2026-08-09, recorded)

- **Demo byte-identity is a SOFT constraint going forward** (#289):
  "always ok to update demo objects in a way that is not
  byte-identical — they should demonstrate the natural and easy
  way to use the library." Operational: byte-identity remains the
  DEFAULT acceptance for mechanical migrations (it proves
  nothing-changed cheaply), but demo-improvement diffs are
  acceptable when the point IS the better authoring; specs should
  say which contract applies. Claimed byte-identity is still
  verified as claimed.
  **Applied retroactively to #289 itself (2026-08-14).** The ruling
  landed 57 minutes before that PR merged and was recorded as
  forward policy, but #289's own deviation D3 — SEL1 deliverable 4's
  acceptance MOVED off `demos/tour/src/diefillet.rs` to the corpus
  `die_composed`, because the tour's die "cannot be a recipe at
  byte-identity" — was left standing, with the byte-identity
  argument in the demo's own doc comment. It is now discharged at
  its origin site: the die is ONE recipe document and both surgery
  blends are `select_where` calls, geometry unchanged in every
  measured respect. The two library residuals the conversion raised
  (the all-on-axis revolve-emitter refusal, and the missing group
  union) are named in `docs/M8-LOG.md`.
- **Lint-drift check** (#290): the pncad-py hand-restated [lints]
  table needs a drift test vs the workspace set — folded into the
  U9S review as a formal claim.
- **NEW EXPLICIT GOAL** (#290): "make all the demos authorable
  through the python bindings" — recorded as the U9/U10
  acceptance north star (the tour corpus becomes the bindings'
  example set AND its coverage oracle). Feeds the curated-doors
  unit (F1/F2/F3 are exactly what blocks bracket.py today) and
  U10's example strategy.

## LIB residual register (2026-08-10, at the program's close —
## Ev's ask; kernel-functionality-tracking items excluded)

Beyond "docs update as the kernel grows," five categories (a
sixth, F, folded 2026-08-28 — see the re-survey entry at this
file's tail):

**A. Curation-gap residuals (the F1 class — library-side doors):**
- **R1 (the significant one, U10 F1)**: named document parameters
  unreachable from the façade — SetDocParam/ParamName/DocParam not
  curated, so plate_param cannot be authored pncad-only. Pinned by
  a compile_fail doctest + audit gap G10. Fix = one curation pass,
  DOORS-shaped. Highest-value single residual.
- R2 (U10 F2): PathNoCornerReason not re-exported beside PathError
  (the U1 closure-class, mild).
- R3 (SEL2 follow-up): the UndeclaredContact refusal-menu WIRING
  (the finding payload into the boolean's refusal) — shape
  recorded in the SEL2 report. **DISCHARGED (LIB-PYG5)**: the raise
  sites keep their (Operand, FaceKey) pair + decided relation,
  editor-core lifts them through the operands' name tables into
  `NodeErrorKind::UndeclaredContact { finding, diag }` — the
  detector's own FlushFinding shape, no re-detection on the error
  path; the menu crosses to Python as `EvaluationError.finding`.
- **FOLDED 2026-08-28 (the orphan sweep).** Library-shaped issues
  filed by OTHER programs, none of them named anywhere in this log
  before today, so LIB's pickup path could not see them. Each was
  read at fold time; all are OPEN.
  - **#918** — `sweep::chamfer_edges` ships (VERBS-CHAMFER,
    `crates/sweep/src/fillet/build.rs:281`, prelude'd at
    `crates/pncad/src/prelude.rs:100`) with NO `Node::Chamfer`:
    kernel-direct only, so a chamfer cannot appear in a recipe,
    cannot rebuild, and mints no `StableName` for any selector.
    Same class, same shape, NOT separately filed: `topo::shell` /
    `shell_open` (shipped #1048, the teapot built on it) and
    `tube_along_arc` / `tube_along_arc_hollow` — `Node` carries 15
    variants and none of them is a chamfer, a shell or a tube
    (`crates/editor-core/src/node.rs:471`). The audit's row 19
    already prices the tube one (a new node kind is a
    schema-version break).
  - **#757** — `topo::BooleanDeclarations` is exported through the
    prelude with a public CONSUMER and no geometric PRODUCER, so a
    caller holding two flush-built bodies hand-writes ~55 lines
    against the `k_stats` telemetry door; the tree carries two
    copies that declare their twinning in prose.
  - **#758** — no public census/genus query, so the Euler-Poincaré
    identity is hand-written at ~13 sites in several return shapes,
    in exactly the places asserting the kernel is sound.
  - **#759** — the façade `polygon` door was DEMOTED with no
    replacement scheduled (`crates/pncad/src/authoring.rs:115`);
    11 tour call sites route through a demo-hosted fold whose own
    doc comment cites the removed function.
  - **#796** — authoring the lily meant building a shadow vector
    algebra beside `Vec3` (normalize twice byte-identically, rotate
    about an axis, an orthonormal frame from an axis), while the
    same file uses `Vec3` freely when CHECKING results. The
    candidate cause worth confirming is generic-scalar friction —
    if true, the kernel's vector type is awkward in exactly the
    generic code the kernel asks people to write.
  - **#948** — `LoopProgram::polygon` is literals-only, so every
    parametric author writes the five `ProgramStep`s by hand; the
    gap is the whole chain vocabulary's literal/Expr split.
  - **#944** (ASM-filed, library-shaped) — nothing consumes a
    `Pose` into an `Alignment`/`MateFrame`, so "mate THIS face to
    THAT face" has no spelling and the frame stays retyped
    literals. A11 keeps the SOLVE structural either way; what needs
    deciding is freeze-at-authoring vs re-derive.
  - **#743 / #742 / #741** — the export option surface. A plausible
    part name is a hard panic in both demos (`solid-block` sniffs
    as ASCII STL; >80 bytes overruns the header field); the STEP
    writer hardcodes the two Part 21 header fields the standard
    assigns to the USER while already distinguishing the software
    fields; ε has no type of its own, so `Tolerance::init`'s
    finite-and-strictly-positive rule is restated by hand across
    `step-export` and `step-import`. #742 and #741 say on their own
    faces that their plan goes to Ev before implementation.
  - ~~**#1103**~~ — CLOSED. `editor-core::unparse` is the door
    outward: precedence-aware source text pinned by a round trip
    (`parse_expr(unparse(e))` is `bit_eq` to `e`, table plus
    proptest over the grammar's generated span). Two constructible
    shapes the GRAMMAR cannot spell are stated on the function
    rather than approximated — a negative literal (there is no
    negative number token, so `-25 mm` reads back as `Neg(25 mm)`)
    and `Expr::count(i64::MIN)`. `ParseError`'s `Display` half of
    this entry was already closed by the stragglers pass; the
    viewer's debug rendering of it is gone with it.
  - **#1111's editor-core slice** — `HitTestError` is carried on
    the façade (`crates/pncad/src/select.rs:84`) and has no
    `Display`, so its values reach a user surface as a struct dump.
    The rest of that issue's list is `viewer`-owned (category F).

**B. Bindings-parity residuals (the north-star audit, executable):**
G1-G11 ranked in the U10 report — the audit test FAILS as doors
land, so this register self-enforces. The big three: G1 profile
arcs/circles via Python (the PATHS lattice in .pyi — the §L4
typestate stubs, deferred to post-v2, now unblocked), G2
loft/sweep/tube bindings, G3 non-xy sketch planes. Plus G11
(tessellation/STL from Python — completes the ladder's steps 5-6;
the audit page's own G11 row says steps 4 and 5 — the page is the
measurement, this line is the stale copy).

**ENUMERATED BY THE CENSUS (2026-08-28).** This category no longer
carries its list in prose. The enumeration of record is
`crates/pncad-py/tests/test_binding_census.py`: every name the
façade's curated lists introduce is bound in Python, mapped to the
spelling that answers the same question, or listed there with its
family — and every `gap:` entry names ONE id that owns the work. An
audit gap id where `docs/guide/north-star-audit.md` defines one
(cited, never minted, since that page's ids are scene-anchored and
its tallies depend on staying that way); otherwise a census-owned
family id — `B-CHECKS`, `B-PICKING`, `B-RESOLVE`,
`B-EXPR-READ`, `B-CANCEL`, `B-FORMAT`, `B-VALIDATE4` — each carrying
a one-line charter saying what a unit closing it would deliver. A
closed family leaves no charter behind, because the census's own
guard fails on one no entry cites: `B-READBACK` was chartered here
and closed at LIB-B-READBACK, and what records that is the four
read-back verbs sitting in the census's `BOUND_AS` with the unit
named beside them. The

family id — `B-PICKING`, `B-RESOLVE`, `B-READBACK`, `B-EXPR-READ`,
`B-CANCEL`, `B-FORMAT`, `B-VALIDATE4`, `B-DISTRIBUTIONS` — each carrying
a one-line charter saying what a unit closing it would deliver. The
`B-` is this category's own letter: the census owns the surface-debt
id space, this register points AT it, and the census's own test fails
when a pointer stops resolving in either direction (a citation the
audit page no longer defines; a charter no entry cites). What stays
here is what the census does not say: the ranking's lineage above,
and the two items below — neither of which is a missing scene OR a
missing name, which is why neither instrument sees them.

**FOLDED 2026-08-28**: two bindings-parity items the audit test
structurally cannot see, because neither is a missing SCENE —
- **#730**: `step_string` exposes ONE of `StepOptions`' six fields
  (`crates/pncad-py/src/py/value.rs:626`), silently. `uncertainty_m`
  is the one with teeth: a Python caller cannot override the
  ambient tolerance a Rust caller can.
- **#694**: the LOAD path stringifies structured kernel refusals
  (`crates/editor-core/src/persist/wire.rs:155`, `format!("… {e:?}")`),
  contradicting `crates/pncad-py/src/errors.rs`'s "typed exceptions
  carrying the structured error, never strings". Reproduced by
  execution on the issue. Its CLASS is the durable half:
  reachability argued from the AUTHORING doors while ignoring the
  DESERIALIZATION doors, every one of which re-runs a smart
  constructor.

**C. Infrastructure residuals:**
- R4 (U10 F4, strongest available follow-up): the PYTHON TESTS ARE
  NOT IN CI — test_guide.py and test_north_star.py (the docs' and
  audit's no-rot machinery) run only by hand. Needs the wheel-build
  CI job (the U9S fence deferral) — one workflow job, then the
  no-rot property is structural.
- R5: the LQ7 tail as ratified-open: wheel cadence; schema-version
  ↔ package-version coupling post-release.
- R6 (#274): CLOSED BY RULING (Ev, 2026-08-10, on the issue:
  "the cure seems worse than the poison") — no structural
  union-checking machinery. The standing mitigation is the
  process norm in every brief: merge main before opening,
  re-merge on movement, build the union explicitly.

**D. Deferred-by-design (need a use case, not work):**
- LB10's straight-off-arc fillet wall (two-route menu in §2b);
  PATHS §7's banked items (arc-arrival fillets beyond G2's scope,
  NURBS legs — VQ7 says segment-vocabulary work); the geometric
  selector's reserved convexity atom (GS-Q2); the F3 crate
  descriptions (matter only at publication).

**E. The endgame pair (Ev-owned, parked by ruling):**
- Q9: the name (Intension cleanest per the 2026-08-08 re-sweep;
  rename = the pncad grep + the cad audit per the memory).
- The U9 release checklist: reset version numbers (LQ7b), crate
  descriptions (F3), publish gates — a small unit when release
  is actually wanted.

**F. Cross-program library-shaped findings (folded 2026-08-28).**
Recorded so LIB's pickup path SEES them; every one is owned
elsewhere, and LIB taking one silently would be the error:
- **#945** (ASM) — mates x patterns did not compose. **RULED
  2026-08-23** (Ev, at the ASM exit walk's sign-off, on the
  issue): A11 gains the member-vocabulary rider, a mate head may
  be a pattern-placed `Instance(i)`, and the issue CONVERTED from a
  design question into a banked ASM implementation unit (`head_of`'s
  member vocabulary + composing the derived offset into the member
  frame). Not awaiting a ruling and not LIB's.
- **#946** (ASM) — a sub-assembly's MATE-minted declarations are
  lost at the instantiation seam (`product_recorded` runs, `mint`
  does not), so nesting an assembly that has mates reports
  `UndeclaredContact` in the outer document for contacts the inner
  one declared. A semantics call on A2, not a patch.
- **#947** (ASM) — refusal text: `PIN_MISMATCH_RECOURSE` is emitted
  twice (the demo carries an ARMED assertion counting 2, which must
  flip to 1 in the same change); `MateFault::Contradictory` and
  `AssemblyError::NoAtRestRecord` carry no recourse sentence, which
  the ASM ladder's own exit criterion asks for.
- **#917** (VERBS) — the blend refusal vocabulary is shared by
  fillet and chamfer and still speaks as the fillet, down to
  `Display`'s `"fillet assembly: "` on arms a chamfer user reaches.
  The issue's own point is that the ~255-reference rename is the
  EASY half and not the substance.
- **#1120** (GUI) — no persistent `SetPlacement` in the layer-3
  session vocabulary. The document door EXISTS and is curated
  (`DocEdit::SetPlacement`, `crates/editor-core/src/edit.rs:208`,
  carried on the façade), so this is a GUI vocabulary gap, not a
  library curation gap — LIB owes it nothing beyond the Python
  spelling the ASM deposit already claims.
- **#1111's `viewer` half** — CameraError, CameraOpError,
  SceneError, SceneDocError, StartupError, IdMapError,
  PickIndexError, ReplayError, none with a `Display`. GUI-owned;
  the issue asks its taker to re-sweep rather than trust the list.

## PROGRAM COMPLETE — resting state (2026-08-10)

Every LIBRARY-DESIGN §L5 unit is MERGED (the ladder as executed:
U1, U2+G1+G2, U3, U5, U7+SEL1+SEL2, U8a, SWITCH-P/E/PR-C with
U8b folded in, U9S+DOORS, U10). The guide + north-star audit are
hosted-CI-enforced (python-suite, 46 tests). The residual
register (above) is the successor's map; category B is the
opening program. #329 MERGED
(2026-08-10, all green incl. an en-route CI catch: the fixture
pin's embedded ε vs the tolerance sweep, fixed as an honest
exclusion) — G10 CLOSED, R1 DISCHARGED. Its retroactive review
(ordinal 19, frozen merge head) is the successor's first task.
Handoff file: ~/.local/share/cad-work/handoff-prompt-lib-next.md.
Dual-review samples 1-6 all converged on every MAJOR; sample 5
overturned a design disposition; the variance data is in the
rows. LIB-7 remainders banked (opus, opus); next dual = 21.

## Successor session opens — the bindings-parity program (2026-08-10)

Handoff executed: #329 verified MERGED (merge 4230173, frozen
head 9bb1916); its retroactive blinded review dispatched as
ordinal 19 single on the frozen head. Stale SWITCH-E lanes
(lib-swe-r1/r2, lib-switche) verified pushed+clean and removed
(12 GB freed). Monitor suite armed. Opening unit: **LIB-PYG1**
(docs/LIB-PYG1-SPEC.md) — the audit's G1, arcs/circles in
profiles from Python via the §L4 typestate lattice; spec pins
prelude-parity names, distinct state classes, quantities at the
boundary, one-lowering (no Python-side predicates), Node.profile
terminal, and fences out G2/G3/G7/G8/G9, Expr-in-profile, and
NURBS legs. Arm: opus (LIB-7 banked slot 2). The Expr-bearing
profile-steps door (with G9 → plate_param authorable from
Python) is recorded here as a NAMED follow-up of PYG1.

**PYG1 MERGED — audit G1 CLOSED (2026-08-10, #346, 28/28 green,
ordinal-20 APPROVE 0/3/3).** The lattice is bound state-for-state
(both structural rulings proven forced), ty is a live CI gate,
authorable 7→11, suite 48→83. Riders landed in the fix pass:
`LoopProgram::from_recorded` (the door PYG1's finding 1 named —
now a Rust door with a bit-for-bit contract test), prelude
curation (ClosedLoop, circle_split, RecordedProgramError). #347
filed kernel-side (Boolean refuses on carrier-crossing cutter
planes — reproduced by the review at the exact r=4/r=5
crossover; bracket.py rounds at 3 mm citing it). Banked from
findings: the Expr-in-profile door (with G9 → plate_param from
Python), Count still unconsumed, tour scenes without closed-form
oracles (finding 5), the DocParam __eq__ rider (from ordinal
19), straight-run authoring noise (finding 7 — vocabulary
evidence for a future design conversation, not a unit).

**LBRET MERGED — G12 CLOSED, #377's retirement COMPLETE except
RETIRE-TAIL (2026-08-12, #413 at the v8 head, ordinal-32:
NOT-MERGEABLE-AS-IS → re-verified APPROVE — the v4 ladder's
first LIB re-review round, and it caught a real latent defect:
the memo content-key tag collision).** Audit 25/34; 9 NO rows
remain (G2:6, G5:2, G14:1). Schema is v8 (the v7 double-claim
with ASM-2A resolved; the dispatch-time-seam discipline is the
standing fix). The #413 thread also produced the §2c
fillet-family design conversation (PR #419, two rounds folded,
awaiting Ev's 👍): capture-at-fillet, uniform arrival binders,
radius-only arrival spec, ArcSpec staging — the §2b compound
register dissolves at its re-spell unit.

**Register addition (2026-08-12, Ev on #413): the LoopBuilder
test-support shim carries a DELETION HORIZON** — the ~15 legacy
test callers migrate to lattice/raw spellings and the shim
deletes entirely (the twins' verification target becomes recorded
fixtures at that point). Folded into the next housekeeping unit;
this line is the register entry so it cannot silently persist.

**RETTAIL IN FLIGHT (2026-08-12): the demotion + the bowtie
re-home landed; the shim's deletion did NOT, and the reason is
sequencing, not difficulty.** (1) `ProfileLoop::new`/`polygon`
moved off the inherent impl onto `profile::RawLoop`, and
`pncad`'s `pub use profile;` narrowed to a curated `pub mod
profile` that omits the trait (LB13 precedent) — the measurement
that forced it: inherent methods travel with a TYPE, and the
ruling keeps the type nameable, so no amount of module narrowing
alone excludes construction. `pncad::authoring::polygon` deleted;
guide, crate docs, façade tests and every tour scene author
through the lattice; an absence guard in `pncad/tests/all.rs`
holds it. Residue flagged, not glossed: public fields mean a
struct literal still constructs a loop wherever the type is
nameable (private fields + accessors is a plain-data convention
change, out of a housekeeping unit's fence). (2) The bowtie left
the tour for `profile/tests/rejections.rs`, authoring through the
lattice and refusing typed at validate — the K-probe's
`finale_bowtie` refusal-sample row went with it (lily's wall
probes remain that lane's refusal source). (3) The twins now
verify against blessed recorded fixtures and no longer touch the
shim; mutation-sensitivity proven (one ulp into
`sugar::bulge_from_via` reddened two rows, reverted). (4) The
NAMED GAP found BY the demotion (lily's lofted blade,
`demos/tour/src/lily.rs::Section::outline`): at `shoulder = 0`
three consecutive vertices are EXACTLY collinear by design (the
4-tip and 8-corner sections must share one vertex budget for the
loft's segment-to-segment matching), and the PATHS lattice
REFUSES that junction at authoring — `JunctionTangent { margin:
0.0 }` — while `Profile::validate` ACCEPTS it, since collinear
line/line is carrier IDENTITY, legal undeclared. The two junction
rules disagree on same-carrier continuation and the lattice is
the stricter one. With raw construction off the presented
surface, the only spelling left to the tour is the plain-data
struct literal, which is what the scene now uses with the gap
named in place. A same-carrier continuation verb is the fix and
is vocabulary — a design item, not this unit's fence. Worth
noting the demotion is what SURFACED it: the raw constructor had
been swallowing the disagreement. (5) The
shim SURVIVES: its remaining ~42 callers are all arc-leg fillet
chains whose only lattice target is the §2b `at_on`/`to_on`
family — the exact surface PATHS-DESIGN §2c redesigns (RATIFIED
on #419, merged 2026-08-11; the re-spell UNIT has not run).
Migrating them now buys a second migration at that unit, so the
DELETION HORIZON re-points at the §2c unit and is recorded in
`test_support.rs`'s header. Finding worth keeping: the shim's
`fillet_corner` and the lattice's arc fillet both run the one
ratified `sugar::arc_fillet_trims`, so on the fillet family it
was never a second implementation — only a second door.

**RULED (Ev, #413, 2026-08-12): raw ProfileLoop construction
DEMOTES from the presented surface** ("yes we should demote
ProfileLoop"; his framing: kernel vocabulary should be private,
and the broken-on-purpose bowtie cannot justify a public
authoring tier). One housekeeping unit (LIB-RETIRE-TAIL)
combines: the demotion (construction out of prelude/curated
surface; TYPES stay nameable for read-back/error payloads),
bowtie re-homed to a validation-suite fixture, shim stragglers
migrated, shim DELETED, SWITCH-fence sentence amended citing the
ruling. Dispatches after #413 closes.

**PYSEL MERGED — G13 CLOSED (2026-08-11, #393, ordinal-29
APPROVE-WITH-FIXES 0/2/4).** The selector surface crosses to
Python (trilean discipline intact, zero name-text parsing);
diecomposed YES*→YES; audit 24→25 of 34; suite 118→128. Riders:
the SegPat.matches drop upheld-and-stated (RoleSeg is name-text
territory); the reviewer's ε-sliver in_band refusal row adopted;
the pncad-py interval-passthrough CI wall fixed en route (the
first pncad-py-only closure would have gone red without it).
Remaining NO rows: G2 sweep/tube ×6 (U4), G5 ×2 (detect/declare
+ R3), G12 (LBRET in flight), G14 (kernel).

**The #377 design conversation — RATIFIED (2026-08-11, Ev 👍
on #386, after two follow-up rounds that strengthened the §V6
disposition to full test-support banishment + struck V4(c)).** LoopBuilder
retirement per Ev's in-chat ruling requires three dispositions
beyond the §2b route (the lb-diecomposed investigation's
findings): (a) PROFILES-V2 §V6's ratified fail-loud-demo-surface
role — amendment drafted in place; (b) **LB4 disposition
PROPOSED**: rocker migrates under the #289 oracle-equality
contract (derived corners 0–4 ulps off authored anchors are the
natural-authoring outcome; the no-anchor-fitting DOCTRINE is
untouched — nothing fits anchors, the demo simply stops
transcribing them); (c) **LB5 disposition PROPOSED**: the
outline's mid-arc seam vertex re-anchors on migration — the
scene's point is the rocker's shape, not its seam placement, so
authored-topology preservation yields to the demo-purpose rule;
the topology change is stated at the site. The bowtie stays
permanently raw (ProfileLoop data, not LoopBuilder). Sequencing:
ruling first, then ONE S-M unit (the §2b door + rocker migration
+ prelude/guide removal + G12 flip).

**PYBUNDLE MERGED — G4/G6/G7/G9 CLOSED (2026-08-11, #376,
ordinal-28 APPROVE-WITH-FIXES 1/2/3).** Audit 18→24 of 34
authorable (20 YES + 4 YES*); 10 NO remain: G2 sweep/tube ×6
(U4-gated), G5 declared-contact ×2 (the detect/declare slice with
R3), G12 rocker (#377), G14 cutaway split-naming (#380 adjacent).
Suite 95→118. The round's substance: the reviewer authored FULL
diecomposed from Python by parsing name-text provenance,
falsifying the fresh G13 wall — RULED both-arms (row→YES*;
name-text OPAQUE BY CONTRACT — parsing the encoding is
representation-dependence, refused by doctrine; G13 re-scoped to
the unbound Python SELECTOR surface, which Rust already serves,
lib_sel1_geoselect.rs:507-560). Three new gap ids stand: G12
(LoopBuilder/§2b — the #377 design conversation, which the
lb-diecomposed investigation showed also needs LB4/LB5
disposition and a §V6 amendment), G13 (Python selectors — the
natural NEXT unit, small), G14 (kernel split-naming wall).
G8 measured-unbound (pattern Instances cannot feed a boolean —
kernel payload gap, unchanged). Issues: #377, #380.
plate_param-from-Python: ONE door left (Expr-bearing profile
steps).

**PYG23A MERGED — G3 CLOSED, G2's loft half closed (2026-08-11,
#365, ordinal-22 APPROVE 0/1/4).** Authorable 11→18 (14 YES + 4
YES*), 16 NO remain; suite 83→95; the plane vocabulary and
Node.loft are live in Python; LQ3's ratification (#362) landed
mid-unit and the audit's G2 text now names the real blockers.
Banked from findings: the loft READ-BACK residue (wire_loft drops
section_params — needs a Section/Affine3 value surface or a
document-layer read-back door; row 14's recorded residue),
origin-less named planes (finding 4 — the single clumsiest thing
in the new vocabulary; candidate rider on U4b's frame family),
SketchPlane __eq__/accessors (rider, in the PYBUNDLE spec),
elevation= as the odd door (future xy_at retirement candidate),
the §L4 typed-quantities structural-int exception (needs one
written sentence in a future doc pass), YES-with-residue as a
possible third audit mark (finding 7 — vocabulary gap, not
resolved). The billing outage (#366) opened and closed inside
this unit's endgame; its one red shard re-ran green.

**The G2 unit-cut ruling (2026-08-10, orchestrator, from the
substrate survey)**: audit G2 splits. Its LOFT half is mechanical
(Node::Loft has existed since M5 PR 10 with eval + naming) and
ships with G3 as **LIB-PYG23A** (docs/LIB-PYG23A-SPEC.md — two
additive SketchPlane constructors, plane values + Node.loft bound,
7 audit rows flip). Its SWEEP/TUBE half is NOT dispatchable as
mechanical work, three independent walls: (a) wire_sweep
unconditionally refuses — the SWEEP_FRONTIER path-composition
lane is banked past M6 by the PR 10 MAJ ruling (kernel-side, not
this program's to un-bank); (b) 3-D path values + the pose family
are U4, whose landing site LQ3 is RATIFIED-OPEN (needs Ev's
working session — U4 never ran); (c) Node::Tube does not exist,
and a new node kind is a schema break colliding with ASM-1's
in-flight v5 bump (coordination, not code). The sweep/tube tail
is therefore a NAMED DESIGN CONVERSATION (U4/LQ3 + frontier +
version coordination), recorded here as the register's G2
residue; the audit page's rows 15–19 get the honest blocker text
in PYG23A.

**Ordinal 19 closed (2026-08-10)**: the R1-PARAMS retroactive
review returned APPROVE 0/1/4, rubric 5/5/5, zero silent devs —
the row is complete in MODEL-AB-LOG. Fix pass
(orchestrator-applied): the fixture pin's ε filter now asserts
exactly one excluded ε line per side (the MINOR's dup-ε damage
shape goes RED in the Rust pin instead of relying on the Python
load refusal). Banked from the NOTEs: (a) `DocParam` binds no
`__eq__`/`__hash__` while Rust derives PartialEq — undocumented
asymmetry, adopt as a rider on the next bindings unit; (b) the
LB13 guard's blind spot (arena key in a new public FN SIGNATURE
would not trip the pub-use scan) — recorded against register
R-series as a known-scope caveat, exposure zero today.

**RESPELL-TABLE registered (Ev's M2 ruling on #531,
2026-08-16): the full four-projection transition table is the
ratified end state, scheduled as a FOLLOW-UP unit** — the
shipped PR-1 form (enum-side projections mechanical, typed
methods hand-written, drift caught by the differential + smoke
row) merges as the honest interim, with the §2c mechanism text
amended to say so. The follow-up's measured cost: ~8 macro
row-shapes, 500–700 macro lines, ~45 rustdoc-carrying methods
into table syntax. Queues after PR-2 (same files).

**RULED (Ev, in-chat 2026-08-16): ProfileLoop SEALS — private
fields + read accessors.** His lean confirmed after analysis:
sealing makes the PATHS-channel funnel the only compilable route
at every crate boundary (a downstream struct literal becomes
uncompilable; the type stays nameable and readable), and shrinks
#433 to a kernel-internal consistency question. Registered as
**LIB-SEAL** (small): fields private on ProfileLoop/
ProfileVertex, read accessors (or a read-only view) for the
sweep/topo/editor-core consumers, the serde CANNOT-MINT proof
(grep + the wire.rs-style argument — the stored form is the
program, replayed; raw-loop deserialization must be shown
absent), closure-test rows proving the accessor set complete.
Honest boundary stated: privacy seals at the crate boundary;
crates/profile's internals stay on the sealed-verbs discipline.
Sequenced after RESPELL PR-2 (same surfaces). Also settles
#431's open question — noted there.

**DELETION-HORIZON REGISTER ENTRY NARROWED TO ONE NAMED VOCABULARY
GAP (LIB-RESPELL PR-2, 2026-08-16) — it does NOT close, and the
reason is the SURFACE, not the suites.** Of the shim's ~42 callers,
all but one shape class migrated: the plain data fixtures in
`sweep`/`step-export`/`mesh` to raw `RawLoop` vertex chains
(bit-identical by construction, which those pinned fixtures need),
and the line x line, arc x line and lens-shaped arc x arc corners to
the §2c fused family. What does not move is **an arc x arc fillet
corner whose two authored far points differ**, and it cannot: an ARC
arrival always lands on the `OnArc` state, whose only continuations
are the fused verbs — the carrier run from the fillet's second
tangent point to the arrival anchor is emitted by whatever TRIMS it
next, so a verb departing the carrier would silently drop that run.
Exact consequence: an arc arrival may be followed by another fillet
or by nothing (the `p: Start` close), never by a sharp continuation;
so a single-fillet loop with an arc outgoing side must close on that
same carrier, which requires the entry to lie on it — i.e. the far
points to coincide. **REPORTED FOR A RULING** (vocabulary, out of the
re-spell unit's fence): the candidate is an `OnArc` continuation that
ends the carrier run at its anchor and yields an ordinary directed
point — the shape §2b's `at_on` tip had and §2c dissolved. Surviving
callers, all in `crates/profile/tests`: `review_s2.rs` (the S2
blinded-review fuzz, which draws its two far points independently and
carries an arc x arc coverage floor), `review_s8_probe.rs::check`,
and two `arc_fillet.rs` fixtures. The same PR retired the three
arc-leg name doors (`arc_to(p, b)` / `arc_via` / `arc_center`) onto
the one `arc_to(spec)` verb and deleted the §2b compat trio
(`at_on`/`to_on`/`at_toward`) with `PathError::ArcCarrierSpelling`;
what survives of that refusal is not carrier-keyed and is named
`ArcLegOnOpenFillet`. A SECOND instance of the same gap, found in
`sweep`: an all-blended loop (every junction a constructed tangency)
has no lattice entry either — the entry vertex would be a
same-carrier seam, and the seam-fillet escape is closed by
`SeamRetrimsArcFirstSide` when side 1 is an arc. The vesica eye-slot
fixture was re-shaped to one sharp tip, with the finding recorded in
place.

**LIB-SEAL DISPATCHED (2026-08-16, block LIB-11 slot 1)**: spec
docs/LIB-SEAL-SPEC.md cut from a full workspace census (452
literal sites, ~93% test fixtures; 2 production scalar-lift
sites; zero persistence impact — no serde anywhere near the
types, wire.rs's cannot-mint statement re-proven as a unit
deliverable). Lane lib-seal, branch lib/seal. Settles #431's
open question at merge.

**OnArc RE-OPENED as a design conversation (Ev, in-chat
2026-08-16)**: the #576 §3 proposal (an OnArc continuation verb)
is NOT ruled; Ev's pushback — the ratified direction is the §2c
axiom's state vocabulary (everything depends on only the final
directed point), under which OnArc should be IMPOSSIBLE, not
grown. Direction under analysis: dissolve OnArc — arc arrivals
emit to a hard anchor and land on an ordinary directed point,
uniform with line-arrival semantics (emitted legs never
retro-trimmed; corner ahead or refuse). SEQUENCING CONSEQUENCE:
RESPELL-TABLE must NOT run until this is ruled — the table would
bake the OnArc rows into macro form.

**RULED (Ev, in-chat 2026-08-16): OnArc DISSOLVES — and the
ratification is DELEGATED**: "if there's no additional caveats
and we can just go forward with the deletion then no need to wait
for my approval." Operative reading: the §2c revision (arc
arrivals emit to a hard anchor and land on an ordinary directed
point; arc-extension joins ray-extension as the fused incoming
story; OnArc/OnArcIncoming/Radius@OnArc/TipState::OnArc/PathOnArc
all delete; #576 §3's continuation-verb proposal RETIRED — the
state deletes instead) self-merges with its full writeup IF the
in-flight blast-radius census shows every affected spelling gets
an honest refusal-with-recourse or mechanical migration; any
genuine wall (a shipped shape with no honest spelling after
dissolution) re-escalates before merge. The mismatched-r
Radius@OnArc emission hole (bulge_from_center unguarded — found
in-chat) is recorded as a defect the dissolution deletes
structurally; the unit pins it with an executed probe first.
Sequencing: the revision PR rides now; the implementation unit
dispatches AFTER LIB-SEAL merges (same crate, overlapping test
files); RESPELL-TABLE stays gated behind the dissolution landing.

**OnArc DISSOLUTION RATIFIED BY DELEGATION (2026-08-16) — census
clean, amendment merged, LIB-ONARC registered.** The blast-radius
census (full report banked in this entry's PR) found the
structural fact that settles the delegation's condition: the fit
gate ALREADY refuses a trim that would eat a side's authored
anchor (`AnchorOutsideTrimmedExtent`, arc_fillet.rs:353-390 the
live proof), so trim-before-anchor was never a shipped shape —
every constructing OnArc chain in the repo (7 Rust sites: the
family.rs doctest, rocker boss/hub, path_program ×2,
path_property ×3; 3 Python matrix rows; zero on-disk fixtures
with fused steps; zero ty fixtures) is same-carrier with its trim
at/after the anchor and re-emits the IDENTICAL final vertex chain
under dissolution. No caveats → PATHS-DESIGN §2c gains the
dissolution amendment (OnArc retires; arc extension joins ray
extension; the #576 §3 continuation-verb proposal RETIRED; the
mismatched-r emission hole deleted structurally; all-blended
entry explicitly NOT addressed — stays a named gap).
**LIB-ONARC** (docs/LIB-ONARC-SPEC.md, M / STRUCTURAL) executes
it: probe-first on the mismatched-r hole, emission moves to the
arrival verb, arc extension, full surface deletion incl. Python
PathOnArc, bit-identity pinned per census site, shim DELETES
(#377 completes at its merge). Dispatches after LIB-SEAL merges
(same crate, overlapping tests); RESPELL-TABLE stays gated behind
it. Draw at dispatch (LIB-11 slots 2-4 banked: opus, fable,
opus remaining).

**LIB-PYPU DISPATCHED (2026-08-17, block LIB-11 slot 2 = OPUS,
banked draw consumed)**: PlacedUnion's Python/audit slice —
docs/LIB-PYPU-SPEC.md, cut from a full census (Frame/PatternKind
unbound, refusal tags ALREADY crossed via tags.rs, the loft
Expr::count precedent governs the count spelling, the fused-base
wall stays kernel-side). Pre-draw fields logged at spec time:
M / STRUCTURAL. Lane lib-pypu, branch lib/pypu — pncad-py +
audit page only, disjoint from the SEAL fix pass and ONARC by
fence. Ev's load ruling (in-chat, 2026-08-17): LIB runs at
full efficient parallelism while the third orchestrator's
account is down. Also filed: #601 (the SEAL review's MAJOR-2
class made durable — CI compiles no whole-file feature-gated
test lane). SEAL state: review ordinal 55 returned
NOT-MERGEABLE-AS-IS (2 MAJ / 2 MIN / 3 NOTE, 2 silent — both
MAJORs feature-lane compile breaks; MAJOR-1 = the D7 pncad-py
fix, already landed post-freeze); fix pass IMPLEMENTER-INHERITED,
in flight; delta re-review next per the v4 ladder.

**SEAL MERGED (2026-08-17, #596 35/35, merge badbfb1b; ordinal-55
NMAI→delta APPROVE — the row is in MODEL-AB-LOG).** #431's open
question SETTLED (struct-literal sealing executed). The review's
durable finds: the feature-lane CI rot class (#601, both MAJORs'
mechanism — no hosted lane compiles whole-file feature-gated
tests) and D1's honest boundary (the tour names `profile`
directly for lily's #433-gap loop — the gap now lives in the
dependency graph, not an invisible literal). ONARC dispatches
now on LIB-11 slot 3 = FABLE (lane lib-onarc, pre-staged).

**PYPU DELIVERED (2026-08-17, PR #604, suite 140→160; review
ordinal 56 dispatched).** Register fold (the implementer's
deviation-9 flag, orchestrator-applied): category B's G8 text is
NARROWED — replication (Node.placed_union/placed_union_at,
Frame/PatternKind values, U4b trio) and the structural-param
count edit (DocEdit.bind_count_param) are CLOSED from Python;
G8's residual = the kernel's multi-solid boolean operand
(combine's single-solid contract, JoinDesync) + the
memo-observability door (evaluate takes no prior — the banked
third claim). Audit marks unchanged at 25+3+6 by measurement,
not omission. die_tool's Python re-authoring = a banked
candidate behind the Revolve/datum half.

**ONARC DELIVERED (2026-08-18, PR #608; ordinal-57 cross-model
dual dispatched).** Orchestrator disposition of the implementer's
deviation 1: the enclosing-tangency (ρ<0) sub-class is
UNREACHABLE through the §2c door — the other crossing always
carries a strictly-nearer ordinary candidate the gates cannot
exclude; only the retired corner-authoring spelling ever reached
it, so nothing shipped is lost and the delegation's wall clause
is NOT tripped. Recorded as boundary pins (the door must
refuse-or-round, never emit the class) + a NAMED low-priority
design question for Ev: should enclosing tangency ever be
authorable, it needs a corner-authoring-shaped verb — vocabulary,
not a defect. #377 completes at #608's merge.

**PYPU MERGED (2026-08-18, #604 35/35, merge 8d404bd7; ordinal-56
APPROVE 5/5/5 — row in MODEL-AB-LOG).** The en-route CI fix is
the durable part beyond the bindings: interval-only-selection.py
now proves scoped no-ops from SOURCE (the #601-adjacent guard
family; the implementer falsified the orchestrator's diagnosis
and built the correct arm — recorded as the model behavior for
handed-down-diagnosis briefs). LIB-11 slots 1-3 consumed
(SEAL/PYPU/ONARC); slot 4 (opus) banks for RESPELL-TABLE.

**ONARC MERGED (2026-08-18, #608 fully green at de6ff336;
ordinal-57 cross-model dual: R1 NMAI→APPROVE, R2 A-W-F — the row
incl. the divergence calibration is in MODEL-AB-LOG).** #377
CLOSED — the LoopBuilder retirement arc that began at #377/#386
is COMPLETE. The §2c surface now matches the ratified axiom
exactly. RESPELL-TABLE dispatches next on LIB-11 slot 4 (opus) —
the four-projection transition table, now over the
post-dissolution row set.

**LIB-RTABLE DISPATCHED (2026-08-18, block LIB-11 slot 4 = OPUS
— LIB-11 fully consumed: SEAL opus / PYPU opus / ONARC fable /
RTABLE opus).** docs/LIB-RTABLE-SPEC.md: the four-projection
transition table over the post-dissolution row set, closing the
#531 interim's gap; pre-draw fields logged at spec time: M /
STRUCTURAL. Lane lib-rtable, branch lib/rtable. The RESPELL-TABLE
register entry closes at its merge — the register's last
scheduled unit.

**RTABLE MERGED (2026-08-18, #616 34/34, merge db0a4c21;
ordinal-58 A-W-F→fixed, the first v5-instrument LIB row — in
MODEL-AB-LOG). RESPELL-TABLE register entry CLOSED — the
register's SCHEDULED column is EMPTY.** The v5 style lane earned
its keep on row one: the census fix (every table row's replay
coverage pinned) closes a class, not an instance. What remains in
the register is Ev-paced or cross-program: the
enclosing-tangency vocabulary question (#608's named residue),
G8's multi-solid-operand kernel gap + the evaluate-memo door,
the G2 sweep/tube design conversation (U4/frontier), die_tool's
Python re-authoring (banked), Q9, and whatever #614's smell-scan
schedule routes to LIB (the orphaned ProfileError fillet
variants are claimed).

**LIB-PERR DISPATCHED (2026-08-18, block LIB-12 slot 1 = FABLE)**:
the smell-scan finding LIB claimed on #613 — ProfileError's five
fillet variants (validate.rs:411-507) became fully orphaned when
#608 deleted test_support.rs, their only constructor. Brief-as-
spec (S size): delete the five variants + their payload-only
support types IF those go dead too (FilletLeg/FilletLegCarrier/
NoCornerReason are LIVE via NoCornerForFillet — verify, don't
assume); sweep pncad-py's tag mirror, doc references
(PATHS-DESIGN, rustdoc), and any match arms; closure = the
workspace compiles with zero dangling references and the tag
parity tests stay green; zero behavior change (no live path can
mint them — prove by the compiler after deletion). Fence: nothing
else from the smell scan; no other error surface changes.

**PERR MERGED (2026-08-18, #622, merge ecf43ab6; ordinal-59
APPROVE — row in MODEL-AB-LOG). LIB's claimed #614 item is
discharged.** Resting state: NO active lanes; LIB-12 slots 2-4
banked (arms redacted 2026-08-29 — see this log's tail). Everything scheduled is done — remaining
register items need Ev (enclosing-tangency vocabulary, Wave 0
D1-D4, Q9) or another program (G8 kernel gap, G2 sweep/tube),
or a #614 routing.

**Resting-state correction (2026-08-18, prompted by Ev's
pickup-path question)**: the entry above under-enumerates. The
DISPATCHABLE-NOW column is not empty — it has one item:
**G11 (mesh/tessellation door from Python)** — blocks no audit
row but completes the guide's ladder (steps 4-5: tessellate +
mesh-vs-exact cross-check); register category B names it; a
banked LIB-12 slot covers its draw; no design conversation
needed (the Rust mesh door exists — this is a bindings unit of
the PYG-family shape). Also dispatchable when wanted, smaller:
the **evaluate-memo door** (PYPU's banked finding — evaluate
takes no prior, so memoized recompute is unobservable from
Python; a small additive door) and **die_tool's Python
re-authoring** (banked behind its Revolve/datum half). The full
pickup map for a cold successor: this log's tail + the residual
register (category B self-enforcing via the audit test) +
memories/MEMORY.md. Correction recorded rather than edited in
place — the log is append-only by convention.

**Cross-program deposit from ASM (2026-08-23, ASM orchestrator,
recorded here at Ev's direction — LIB inactive)**: ASM-DEMO
(#938, the R2 exit demo) surfaced two LIB-owned items.
(1) **The façade omitted the assembly VALIDATION surface** (the
PR's F1): `pncad::document` exposed the whole assembly authoring
vocabulary (InstantiatePart/Pattern/Mate + payloads, split/
inline, update_references, mixed_pins, solve_document, product)
and not the gate that validates the result — assemble, Assembly,
AssemblyError, AtRestFinding, Attribution, MintedDeclaration,
RefusedRef. A façade-only consumer could author an assembly and
never run its validity gate. The minimal re-export block LANDS
in #938's fix pass (adjudication on the PR: the `profile`
manifest entry documents a DELIBERATE omission, this one was an
accident, and the tour's own LIB-U1 invariant had made the
flagship assembly scene its standing exception). What LIB owes
is only a retroactive curation review of that block at next
activation — if the façade rules want a different shape, say so
on #938's thread.
(2) **The assembly surface is entirely absent from `pncad-py`**
(the demo's Python survey, independently spot-checked by both of
#938's reviewers): no instantiate_part / mate (+ Alignment,
MateFrame, MatePrimitive, AxisSense, ContactClass) / plain
N-bodies pattern / Workspace-DocRef-ContentPin family /
set_placement / set_roots / update_reference / mixed_pins /
solve_document / product / assemble / split / inline — and
structurally FIRST, `evaluate(doc)` takes no resolver, so an
InstantiatePart node cannot evaluate from Python at all. Neither
of the demo's two assembly documents is authorable through the
bindings; the standing demo-purpose goal ("every demo authorable
through the Python bindings") fails for the assembly layer. This
joins the dispatchable column as a PYG-family series with a
stated order: the resolver/workspace door first (small, possibly
wanting a short design conversation on the workspace-from-Python
shape), then the node/edit/refactoring bindings, which are
mechanical once evaluation can resolve. The demo
(`demos/tour/src/assembly.rs`) is the ready-made coverage oracle
for the whole series, per the standing tour-corpus rule.

## LIB re-survey (2026-08-28, Ev's ask — the track after ten days
## at rest). Everything below was measured in-tree or on the tracker
## in the session that wrote it.

**State.** No active LIB lanes since PERR merged (#622, 2026-08-18);
LIB-12 slots 2-4 still banked (arms redacted 2026-08-29). The only movement in this
file since is the ASM cross-program deposit of 2026-08-23. Nothing
LIB dispatched has come back unmerged, and nothing in the register's
SCHEDULED column has re-filled.

**What landed elsewhere that LIB owes a door for.** Three kernel
verbs have shipped with NO recipe-layer door, so each is
kernel-direct only: unreachable from a document, minting no
`StableName`, invisible to Python.

- **chamfer** — `sweep::chamfer_edges`
  (`crates/sweep/src/fillet/build.rs:281`, re-exported
  `crates/sweep/src/chamfer.rs:66`, prelude'd at
  `crates/pncad/src/prelude.rs:100`). Filed as **#918**.
- **shell / shell_open** — `crates/topo/src/shell.rs:484` and `:507`,
  recorded SHIPPED at #1048 in `docs/KERNEL-VERBS.md`, with the
  teapot (`demos/tour/src/teapot.rs`) built on it. NOT filed.
- **tube** — `tube_along_arc` (`crates/sweep/src/revolve/tube.rs:265`)
  and, since VERBS-TUBEWALL, `tube_along_arc_hollow` (`:306`). Known
  only as the audit's row 19, which correctly prices it: a new node
  kind is a schema-version break, and the missing node now has to
  carry the wall too — one node kind, not two.

The measurement behind all three: `Node`
(`crates/editor-core/src/node.rs:471`) carries fifteen variants —
Datum, Profile, Extrude, Revolve, Loft, Sweep, Fillet, Split,
Boolean, Transform, Pattern, PlacedUnion, Declare, InstantiatePart,
Mate — and none of them is a chamfer, a shell or a tube.

**Why it accumulated: two structural holes, one per side of the
façade.** Neither is a lapse by any unit; both are the absence of a
test that would have made the drift loud.

1. **The north-star audit page's ROSTER is unguarded.** Its test
   (`crates/pncad-py/tests/test_north_star.py`) rebuilds every YES
   row and asserts every named gap is still a gap — so the page
   fails the day a DOOR lands, which is the property category B
   leans on. Nothing compares the page's row set against the tour's
   stop set. Measured at this entry's writing: the table holds 34
   numbered rows (`docs/guide/north-star-audit.md:197-230`) and
   thirteen of the tour's named stops appear nowhere on it —
   `bench`, `benchlayout`, `budfillet`, `diechamfer`,
   `diechamferblank`, `hollowelbow`, `hollowring`, `hollowtorus`,
   `klein`, `spacer`, `teapot`, `twopeg`, `twopeg_apart`. (This
   entry first said twelve, off by `spacer`: the hand grep behind
   it reads `name: "…"` literals, and `bodies.rs` builds its six
   stops through a helper whose name is a parameter. The roster
   guard below extracts all three spellings, which is why it can
   be trusted where the grep could not.) Row 10's `lily
   (8 bodies)` is stale by the same mechanism: `plant()` returns
   fifteen pieces (nine in the literal, plus three bud and three
   sepal; `demos/tour/src/lily.rs:1395-1545`), and the scene's own
   doc comment still says eight (`lily.rs:1275`). **The same PR re-cuts the page and
   adds the guards** (`the_north_star_audit_has_a_row_for_every_tour_stop`
   and `the_north_star_audits_tallies_are_derived_from_its_rows`,
   `crates/pncad/tests/all.rs`): the roster is 47, the thirteen rows
   are added, `lily` reads fifteen, and every headline number and
   per-gap stops column is re-derived off the rows rather than
   carried forward — the tally guard caught G2's stale `6` on its
   first run. What is recorded above is the state as measured BEFORE
   that landed, deliberately, so the reason the guards exist stays
   legible. Four gap ids were minted for the new NO rows: **G16**
   (chamfer node), **G17** (shell node), **G18** (the Python assembly
   series), **G19** (declared contact beyond the plane — whose
   diagnosis was refuted then refined: `Declare` is carrier-agnostic
   and `topo::carrier_pair_relation` exists, so the blocker is
   narrower than "the detector is plane-only" — `FlushFinding` is the
   declare arm's sole input and is unconstructible from Python).
2. **The Python bindings have no coverage guard at all.**
   `crates/pncad-py/tests/test_stubs.py` checks `.pyi` <-> module
   drift at NAME level and nothing else; no test compares the Rust
   façade's curated surface against the Python one. The Rust side is
   self-enforcing — `crates/pncad/tests/all.rs:2899`
   (`every_document_layer_root_export_is_carried_or_listed`) fails
   when the document layer exports a name the façade neither carries
   nor lists as interior, which is exactly why the GUI program's
   façade additions got carried correctly and the bindings' did not.
   What has accumulated on the far side of that missing guard, all
   verified absent from `crates/pncad-py/pncad.pyi` and present on
   the façade: the whole assembly block (`document.rs:178`), the
   checks door `run_checks`/`ChecksReport`/`enforce_checks`
   (`document.rs:219`), the picking family
   `NodePick`/`pick_face`/`Ray`/`PickHit`/`PickTarget`/`HitTestError`
   (`select.rs:84`), the expression read side
   `eval`/`eval_count`/`EvalError` (`document.rs:61`), and
   `ClassAdmission`/`class_admission`/`CLASS_DEFERRAL`
   (`document.rs:163`). **The same PR adds the Python-side
   guard** (`crates/pncad-py/tests/test_binding_census.py`): every
   name a `pub use` introduces in the façade's document/select/prelude
   lists is bound top-level, mapped through `BOUND_AS` to a Python
   spelling the stub is verified to declare, or listed in `NOT_BOUND`
   with its family — 323 curated names, of which **112 are `gap:`
   entries carrying the pointer that owns them**. Both rosters decay
   in the other direction, as the Rust guard's stale check does. The
   census reads source text only, so it runs with no compiled module.
   Its own finding, which this register should carry: the audit's gap
   list is SCENE-driven, so debt no tour scene exercises can go
   unnamed — **checks, picking, name resolution, the read-back doors
   and the expression READ side** reach the record for the first time,
   as the census-owned `B-*` families. The assembly block, mates,
   split/inline and product roots turned out to be named already, by
   the **G18** row this same PR minted, and are cited to it rather
   than given census ids: where an audit gap id exists the census
   CITES it, and where none does the family tag IS the id. That
   division is itself guarded — a census entry citing `G##` must name
   a gap the audit page actually defines, so the two instruments
   cannot drift apart silently.

**The register fold (done in this entry's change).** Nineteen
library-shaped issues filed by other programs were recorded NOWHERE
in this log — zero mentions of any of them before today — so they
were invisible to LIB's pickup path even though the register is
supposed to be the successor's map. Each was read, not skimmed, and
placed: eleven into **category A** (the F1 curation-gap class:
#918, #757, #758, #759, #796, #948, #944, #743/#742/#741, #1103),
two into **category B** (#730, #694 — bindings-parity items the
audit test structurally cannot see, because neither is a missing
SCENE), five into a **new category F** for cross-program findings
LIB must SEE but must not silently take (#945, #946, #947, #917,
#1120), and one — **#1111** — SPLIT, because it is a class spanning
two owners: its editor-core slice (`HitTestError`, carried on the
façade) into A, its `viewer` list into F. F
is a new grouping rather than a stretch of A-E because A-E are all
LIB-OWNED work; filing another program's item under them would make
the register lie about who picks it up. The register's body is
inside the historical part of an append-only log, so the fold is the
smallest edit that puts each item under the category a successor
reads: bullets appended in place, existing text untouched except the
"five categories" line, which now points here.

**Two corrections the fold produced.** (i) **#945 is not open for a
ruling.** It was RULED 2026-08-23 on the issue (Ev, at the ASM exit
walk's sign-off): A11 gains the member-vocabulary rider, and the
issue converted from a design question into a banked ASM
implementation unit. (ii) **#1120 is not a library curation gap.**
`DocEdit::SetPlacement` exists and is curated
(`crates/editor-core/src/edit.rs:208`, carried on the façade); the
gap is the viewer's session vocabulary, GUI-owned. Both are recorded
under F with that ownership stated, so neither is picked up here by
mistake.

**R2, checked live and STILL OPEN — with a narrower honest
statement.** `PathNoCornerReason` (`crates/profile/src/path.rs:487`)
is the payload of `PathError::NoCornerForFillet`'s `reason` field
(`path.rs:576-579`). Neither the `profile` crate root's `pub use
path::{…}` (`crates/profile/src/lib.rs:131-134`) nor the façade's
two lists (`crates/pncad/src/profile.rs:55`,
`crates/pncad/src/prelude.rs:91`) carries it, though all of them
carry `PathError`. It is NOT unreachable: `path` is a public module
(`profile/src/lib.rs:120`) re-exported wholesale by the façade
(`crates/pncad/src/profile.rs:51`), so `pncad::profile::path::
PathNoCornerReason` names the type. So R2's real shape is "not
carried beside its carrier", not "not re-exported" — one line in
each of two files, which nobody has spent in the eighteen days since
the register named it. Not fixed here: this unit's fence is this
file.

**The map, for a cold successor.**

- **Dispatchable now, no design conversation.** G11 (the mesh /
  tessellation door from Python — the Rust door exists,
  `pncad::prelude` re-exports `mesh::{Mesh, TessellateError,
  tessellate}` at `prelude.rs:133`; completes the guide ladder's
  steps 4-5 per the audit's own G11 row). G15 (workspace store /
  `DocRef` / `ContentPin` — the audit's G15 row states every door is
  curated in Rust already, so it is a binding unit). The
  **evaluate-memo door** (PYPU's banked finding: `evaluate` takes no
  prior, so memoized recompute is unobservable from Python). The
  **Python assembly series** from the ASM deposit, in its stated
  order — resolver/workspace door first, then the node/edit
  bindings, with `demos/tour/src/assembly.rs` as the ready-made
  oracle. **die_tool's Python re-authoring** (banked). And the
  retroactive curation review of the A5 assembly-gate re-export
  block that landed on the façade in #938 (`document.rs:178`) —
  small, and LIB's to do, per the deposit.
- **Wants a design conversation, LIB-shaped.** The three
  door-less kernel verbs above, of which only chamfer is filed
  (#918): what a recipe-layer shell or tube node costs is a
  schema-version question, and #918's own text says the emitter is
  where the care is (do not replicate `emit_fillet`'s #708 tie
  defect). Also #741/#742 by their own faces.
- **Needs Ev.** The enclosing-tangency vocabulary question
  (#608's named residue), Wave 0 D1-D4, Q9.
- **Needs another program.** G8's kernel gap (multi-solid boolean
  operand), the G2 sweep/tube frontier, and all of category F.

The pickup path is unchanged in shape and now actually complete:
this log's tail + the residual register (categories A-F) +
`memories/MEMORY.md`.

## LIB reactivated (2026-08-29) — new orchestrator, remote host

**Session opening (Ev, in-chat).** LIB resumes after eleven days at
rest, with a new orchestrator on a remote preemptible container
(4 CPUs / 15G RAM / ~29G disk) rather than the tmux host the standing
memories assume. Host adaptations, stated once: lanes are full clones
via `new-lane.sh` (3–4 in parallel per Ev), heavy cargo stays behind
the build-slot mutex, hosted CI remains the verification of record, no
monitor scripts or away-channel (Ev is present in-session; the
tracker is read at check-ins), GitHub via MCP tools rather than `gh`.
Program prefix `lib/` reconfirmed; orchestrator branch
`lib/orchestrator`.

**Rulings recorded (Ev, in-chat, this session):**
1. **Mechanical units run outside the model A/B** — opus implementer,
   no review lane, merged on green hosted CI + the orchestrator's own
   read of the diff, logged here per unit. Full text and the readout
   population note: MODEL-AB-LOG's 2026-08-29 entry.
2. **Banked LIB-12 slots are untouched by mechanical units** — they
   remain the draws for LIB's next A/B (substantive) rows.
3. The proposed mechanical/substantive split of the re-survey's map was
   approved as proposed: mechanical = R2, #1111's editor-core slice,
   G11, G15, the evaluate-memo door, die_tool's re-authoring, the
   assembly node/edit bindings (G18's second half); substantive (full
   protocol) = the resolver/workspace door, and any chamfer/shell/tube
   node unit its design conversation produces.

**Blinding repair, done before any A/B dispatch:** the LIB-12 block
record on main named the banked slots' arms by arithmetic (the exact
class of the PCURVE 2026-08-28 redaction). Redacted in place in
MODEL-AB-LOG and at this log's two restatements; exposure and the
contamination flag for consuming duals recorded in MODEL-AB-LOG's
entry. This log stays append-only for entries; blinding redactions
edit in place by the standing precedent.

**Register correction (measured on the tracker and in
`docs/SMELL-SCAN-2026-08.md` this session): the "needs Ev" column's
"Wave 0 D1–D4" is STALE.** It was written 2026-08-18 and carried
forward unre-checked by the 08-28 re-survey. As of 08-19/08-20: D1
RULED (a `Dual` may not certify but may have `Bounds`), D2 RATIFIED
into DESIGN.md (#628), D4 DECIDED (delete), and D3 resolved as
deliberate-frontier via the closed `Surface` enum. What actually
remains Ev-paced from LIB's map: the enclosing-tangency vocabulary
(#608's residue, explicitly low-priority) and Q9 + the U9 release
checklist (urgent only when release is wanted). #741/#742/#944 wait on
LIB drafting plans, not on Ev. (The smell-scan §D's open-decision
table — D6, S14(b), S65/S70/S82/S90/S107/S116(p) — is that program's
queue, not LIB's; noted so no successor re-mistakes it for ours.)

**Wave 1 DISPATCHED (2026-08-29, all mechanical under ruling 1,
implementer arm opus by that ruling's own text):**
- **G11** (`lib/g11-mesh`, lane lib-g11): the mesh/tessellation door
  from Python — completes the guide ladder's steps 4–5; audit G11 row
  + census `gap: G11` entries are the oracles.
- **G15** (`lib/g15-workspace`, lane lib-g15): the Workspace/DocRef/
  ContentPin bindings — the assembly series' first half; hard-fenced
  off `evaluate`'s signature (the resolver door stays a design
  conversation) and off all G18 vocabulary.
- **CUR** (`lib/curation-r2`, lane lib-cur): R2 (PathNoCornerReason
  carried beside PathError at its three sites) + #1111's editor-core
  slice (`HitTestError` Display), with the Display-gap class swept
  and listed, not fixed.

All three briefs point at `docs/prompts/implementer-discipline.md` by
path, carry the foreground-polling rule, and expect census/pyi/audit
merge contention (merge main before opening; re-merge on movement).
Orchestrator holds: the workspace-from-Python resolver-door design
conversation (opens after G15's dependency report), the retroactive
curation review of #938's A5 re-export block (orchestrator's own,
next), and the #742/#741/#944 plans (drafted when their turns come).

**#938 A5 re-export block curation review CLOSED (2026-08-29,
orchestrator's own — the deposit's owed retroactive review).** The
block (`crates/pncad/src/document.rs:165-179`: Assembly, AssemblyError,
AtRestFinding, Attribution, MintedDeclaration, RefusedRef, assemble)
stands AS LANDED. Checked: payload closure — every type the carried
names expose in fields/arms (RecipeNodeId, ContactClass, FaceKey,
NameTable, ContactRecords, ValidationError, EntityKind, MateSide,
StableName, ProductError) is carried on the façade's curated lists
(document.rs, select.rs, or prelude), so the block introduces no
R2-class "payload not carried beside carrier" gap; the rationale
comment states the gate's why in the file's house style; placement
beside the authoring vocabulary matches the deposit's own argument
(the gate the vocabulary can construct-and-not-check). No comment owed
on #938's thread — the deposit asked for one only if a different shape
was wanted.

**CUR MERGED (2026-08-29, #1161 — Wave 1's first return; mechanical
under the 08-29 ruling, no A/B row).** `HitTestError` gains the
LIB-DOORS-F6-shape `Display` + `Error` impl (arena key deliberately
not printed — kind + body index instead) with a contract test whose
negative half pins against a future struct-dump regression. #1111's
editor-core slice DISCHARGED; the issue stays open for its GUI half.
**R2 was found ALREADY DISCHARGED** — carried at all three sites by
09dbd562 (2026-08-28, an adjacent lane's cleanup), one day after the
re-survey's "checked live and STILL OPEN" paragraph; that paragraph is
hereby corrected rather than edited (append-only). Register effects:
R2 CLOSED; #1111's A-slice closed. Two lane findings with homes:
(1) **GUARD-SIBS joins the dispatchable column** (mechanical-shaped):
`every_document_layer_root_export_is_carried_or_listed` reads
`editor-core/src/lib.rs` ONLY — no sibling guard for the profile/
topo/mesh/quantity layers, so an R2-shaped gap outside the document
layer is mechanically invisible (exactly how R2 sat 18 days); the
unit is the sibling guards. (2) Process rule for curation briefs:
grep the target symbols on a FRESH clone of main before spending the
lane — a register item can be discharged by an adjacent commit
without the register moving. The lane's façade Display-gap re-sweep
(12 real gaps, 4 new to #1111) is recorded on #1111 itself.

**GUARD-SIBS MERGED (2026-08-29, #1166; mechanical under the 08-29
ruling, no A/B row).** The carried-or-listed guard family completes —
with the unit's own measurement CORRECTING the dispatch premise: only
TWO façade layers are curated per-name (`editor_core`, already
guarded, and `profile`, the hole R2 sat in); the other ten are
whole-crate re-exports whose surfaces cannot drift by construction, so
the brief's "at minimum profile, topo, mesh, quantity" would have
minted three vacuous guards. What landed instead: the assertion tail
extracted (one function, not four copies), the profile-layer guard
(root declarations scanned too — that layer DECLARES types at root,
closing blind spot #2 for it), and the classification guard
(`every_facade_layer_is_whole_re_exported_or_per_name_guarded`) that
buckets every manifest path-dep exactly once — so a layer narrowing
out of whole-re-export into curation FAILS on that commit instead of
silently joining the unwatched case. Negative checks reproduced the
real R2 hole (deleting PathNoCornerReason's carriage reds the guard;
the historical gap was 20 days, df89aff8→09dbd562, longer than the
register knew). **Banked, register category A (curation candidate):
`profile::BlendArc`** — the one unargued entry in either list; return
type of a carried type's method, cross-referenced from carried docs,
matching neither withholding family. Fold into the next curation
pass.

**G11 MERGED (2026-08-29, #1165; mechanical under the 08-29 ruling,
no A/B row). Audit G11 CLOSED — the guide ladder's steps 4–5 are
sayable from Python.** `Body.tessellate(chordal: Length)` → `Mesh`
(shared position buffer + per-face patches both cross, so
watertightness is index-checkable and the mesh-vs-exact cross-check
is the CALLER's computation — argued as the better step-5 shape, a
genuinely independent second measure, since `mesh::validate`'s
re-derivations are not curated and binding them would have reached
past the façade); `to_stl_ascii`/`to_stl_binary`; `TessellateError` +
`StlError` typed with exhaustive tag maps (kernel-side arm additions
arrive as compile errors). Census: all eleven `gap: G11` entries
dispositioned; audit page re-cut (G11 → closed gaps, no mark moved —
its anchor was the ladder, not a stop); new guide page `meshing.md`
under test_guide. Register effects and banked findings, each with a
home: (1) **`StlError` uncurated** (the R2 shape — prelude carries
the writers and their option error types, not the writers' own
refusal): joins BlendArc as the pending **curation micro-unit**,
category A. (2) **Kernel Display gaps** (`mesh::TessellateError`,
plus #1111's re-sweep quartet: `ContactRefusal`, `ReadbackError`,
`FmtQuantityError`): a dispatchable mechanical unit
(**DISPLAY-KERNEL**), recorded on #1111. (3) **No patch→StableName
door on either side of the boundary** — the next picking/rendering
consumer's ask, adjacent to census `B-PICKING`/`B-READBACK`;
design-shaped, recorded in the audit page's G11 residue. The
picking-chain keys stay unnameable by curation, correctly.

**G15 MERGED (2026-08-29, #1164; mechanical under the 08-29 ruling,
no A/B row). Audit G15 CLOSED — the workspace store crosses whole.**
`Workspace(path)` (scan-by-header, `documents()`, `create`/`resave`,
`resolve` with A4's Cargo.lock pin semantics un-softened —
`pin_mismatch` refuses typed with `wanted`/`found` +
`PIN_MISMATCH_RECOURSE`), `ContentPin`/`DocRef`/`content_pin`/
`canonical_bytes`/`header_document_id`/`random_document_id`, one
typed `WorkspaceError` with every attribute present on every arm.
The unit's argued deviation ADOPTED at adjudication: the pin-UPDATE
family (`update_to_store`, `update_references`, `mixed_pins`,
`UpdateError`, `PinMultiplicity`, `PinSites`) is NOT bound and
re-cited to G18 — a site is an `InstantiatePart` node's `DocRef`, so
on any Python-authorable document those doors answer only
"referenced nowhere"; the audit page's own test already grouped them
with G18, and the page was cut in the test's favor. Its measured
finding kept as a test: a pin says which VERSION, never which PART
(`canonical_bytes` strips `id` by design). **The re-survey map's
"G15 dispatchable" bullet retires at this row** (the lane flagged it
as the stale line it would become). Sequencing note: merged after
G11 and GUARD-SIBS with a union conflict resolution in the census
docstring, the audit page's further-gaps section (now empty — both
its rows closed in one wave) and the closed-gaps table; census green
on the union pre-push, hosted CI the gate as always. The
resolver-door design conversation this unit's report sharpened is
RATIFIED and specced: `docs/LIB-G18A-SPEC.md` (Ev, in-chat,
2026-08-29) — the next unit, full A/B protocol.

**CUR2 MERGED (2026-08-29, #1173; mechanical under the 08-29 ruling,
no A/B row).** The two banked curation carriages land: `StlError`
beside its writers in the prelude's section 7 (G11's banked R2-shape
finding closed), `BlendArc` on the curated `profile` module — and NOT
in the prelude, by measurement (the prelude carries the validate
family's gate/refusal/output tier only, and no consumer outside
`crates/profile` names `BlendArc`; the corpus rule agrees). The
profile guard's interior list is down to `RawLoop`, its one argued
name. The R2-class re-sweep at the merge base: 57 prelude doors
origin-resolved, two hits, one fixed here, one a non-instance
(`HitTestError` IS carried beside its carrier in select.rs; prelude
lift would be a minimality question, not R2). Banked findings kept:
(1) the census scans document/select/prelude ONLY — a name carried
onto `profile.rs` alone incurs no census obligation (worth knowing
before assuming a Python consequence); (2) symbol-name greps mislead
in this workspace (five collision examples recorded in the PR) —
façade sweeps should origin-resolve through the pub-use graph first.
Stale-comment follow-up for the next pncad-py sweep: the reach-past
note at mesh.rs's StlError import.

**G18A MERGED (2026-08-29, #1176 — LIB's first full-protocol unit
since reactivation; ordinal 300, sample #41, row in MODEL-AB-LOG).**
`evaluate(doc, *, resolver=, prior=)` per the ratified spec: the
assembly seam opens from Python (a document carrying InstantiatePart
nodes, loaded from a Workspace, evaluates; refusal family typed), the
memo becomes a measurement (PYPU's banked finding CLOSED), and the
tour bench corpus rides as committed bytes with a three-axis honesty
header. The dual's headline (bilateral): a `prior=` serves memo hits
WITHOUT re-running the seam's gates — contract now stated at the door
in both reviewers' framings, pinned on both availability arms, and
the kernel design question filed as **#1185** (the class: "an
argument that silently voids another argument's gate"; two sibling
sites named for future sweeps). **#1186** schedules the corpus
structural hole. Register effects: G18's first half SPENT — the
node/edit bindings (**G18b**) are now dispatchable-mechanical per the
standing split; rows 46/47 stay NO until they land. The audit's G8
row prose corrected (counter invariant). The delta re-verification
round (R1 resumed, narrow scope) is the shape to reuse: cheap (~40k
tokens), executes rather than reads, and the resumed reviewer's
context made it 12 minutes.

**DISPLAY-KERNEL MERGED (2026-08-29, #1175; mechanical under the
08-29 ruling, no A/B row).** All ten façade-carried refusal types
the #1111 re-sweep measured without a `Display` now render
F6-shape prose (`TessellateError`, `ContactRefusal`, `ReadbackError`,
`FmtQuantityError`, `DeclareError`, `InterrogateError`,
`SelectRefusal`, `ResolveFault`, `ParseError`, `MigrationError`),
each with a struct-dump-fingerprint contract test; four arms listed
recourse-less for a design pass rather than minting recourse prose
(#947's lesson applied prospectively). The unit's ARC is the record:
its 1e-12 draw exposed a main red no main head had drawn
(`r2_m10_di_probes` — filed as #1178, fixed by M10 as #1193 per
ratified DL3), the fix was PORTED onto this branch per the
drive-to-green rule, and the port's CI-Config pin re-drew the exact
failing point GREEN (run 33265053740) — the red closed honestly, not
re-drawn away. Lane process finding kept: the lane pushed a tree it
had not re-verified once (one red run, disclosed, fixed next
commit). Python `{:?}` message sites now flippable to the new
Displays: listed in the PR for the next pncad-py sweep.

**G18B MERGED (2026-08-29, #1192; mechanical under the 08-29 ruling,
no A/B row). Audit G18 CLOSED — the assembly series is COMPLETE, and
the north star reads 34 of 47 (30 + 4 YES*).** The whole authoring/
edit/refactoring vocabulary crosses: `Node.instantiate_part`/`mate`
(+ payload and solve read side), `set_placement`/`set_roots`/
`update_reference` + the pin-update family (`update_to_store` with
its snapshot contract STATED at the door per #1185's class — and
executed as contract tests on three doors), `product`/`solve_document`
/`assemble` + the A5 gate family typed, `split`/`inline`. **Row 46
(`bench`) flips YES outright** — TestBenchStand authors the scene
from nothing against the scene's own expectations, gate CERTIFIES.
**Row 47 (`benchlayout`) flips YES\*** on the honest mark: authorable
end to end via `placed_union` where the scene says `Node::Pattern`
(G8's deliberately-unbound plural payload — a THIRD caveat flavor,
added to the headline gloss rather than overclaimed; G8's stops
3→4). All 43 census `gap: G18` entries dispositioned; the
PlacementRuleFault census reason corrected by execution. The ASM
deposit of 2026-08-23 is fully discharged. Banked with homes:
(1) **#1185's second live site** — `product`/`assemble`/
`SolvedPoses.placement` take a document plus something that must be
OF it, uncheckable because an evaluation carries no document
identity; kernel-shape question, stated at each door, recorded on
#1185's thread by this entry's merge. (2) #947's doubled recourse now
asserted from Python too (goes red in two places when fixed).
(3) Three RefusedRef arms measured unreachable-one-door-earlier from
Python authoring; negatives recorded in the test file. Two scene
claims not reproducible (`shells().count()`, `face_frame`) — both
census-owned (`B-READBACK`), named in row 47.

**PYDISP MERGED (2026-08-29, #1196; mechanical under the 08-29
ruling, no A/B row).** Nine pncad-py sites flipped from `{:?}` to the
DISPLAY-KERNEL prose; four stale "no Display" comments truthed — one
of them a `create_exception!` docstring whose false Debug-rendering
sentence was shipping in Python `help()`. Two same-class sites beyond
#1175's list found by the re-sweep and flipped with disclosure. The
deliberate asymmetry recorded: `select_refusal`'s per-arm prose KEPT
(a candidate is spelled through `name_text`, the StableName alphabet
Python speaks — flipping would regress the boundary's naming
contract), while `declare_err`'s hand prose retired (no such reason).
The CUR2-flagged reach-past comment located at tags.rs (not mesh.rs
as the register said) and truthed. Banked: prelude-curation vs
pncad-py-import-comment drift is a recurring class (the StlError
comment survived three units) — a periodic grep of "is NOT
prelude-curated" claims against the actual prelude is cheap and
worth a future polish unit's line item.

**GUIDE-ASM MERGED (2026-08-29, #1198; mechanical under the 08-29
ruling, no A/B row).** `docs/guide/assembly.md` — the assembly
surface's user story, nine executed Python blocks under test_guide's
no-rot machinery: the three vocabularies (identity/pin/reference with
the pin_mismatch refusal executed, recourse asserted), authoring
(instantiate/mate/cluster-gauge), the seam (`resolver=`, and the
memo's seam-gate contract QUOTED from the door, unsoftened, #1185
named), solve + A5 gate, four refusals each reached by authoring the
mistake, split/inline/pin-door with the three-door what-reads-when
table. G18a's "a user learns resolver= from the stub alone" banked
finding CLOSED. Two pre-existing registration gaps fixed in passing
(GUIDE.md §4 missed meshing.md; examples.md missed assembly.rs) —
adjacent-list repairs, kept. **Usability finding banked per
demo-purpose (real friction, stated in the page, not smoothed): a
MATE NODE IS A PRODUCT ROOT** — `Doc.roots` answers instances AND
mates (roots = live nodes nothing consumes), so `set_roots` on three
solids names five nodes and omitting the mates refuses
`root_uncovered`, while the door's prose says "ordered PRODUCT
ROOTS" and `product` gathers only body-denoting ones. Vocabulary
evidence for a future design conversation, not a unit. Orchestrator
process note, honestly: the pre-merge "lane stalled" read was WRONG —
liveness was judged from processes and branch pushes without checking
the PR list; the lane had been done and gated for an hour. Check
open PRs before declaring a lane stalled.

**B-READBACK MERGED (2026-08-29, #1216; mechanical under the 08-29
ruling, no A/B row). Census family B-READBACK CLOSED — "a name
answers with VALUES, never keys" gets its first Python face.**
`Evaluation.face_frame`/`edge_frame`/`vertex_position`/`denotation`
+ `Pose`/`Denotation`/typed `ReadbackError` (Pose deliberately
carries no `==`, mirroring geom_core's absent PartialEq; directions
cross dimensionless per the place.rs rule). The census learned HOW A
FAMILY CLOSES (no precedent existed; the decay guard forces the
charter out, the closure paragraph records the unit). Row 47's
face_frame ask answered against the placement arithmetic, red at a
1-in-10^4 perturbation. **MERGED RED on two main-inherited legs, on
Ev's explicit in-chat authorization ("you can merge those PRs red
if you've already determined it wasn't your fault")**: the
default-lane clippy red (#1174's — since fixed on main by M10's
#1226, so moot at this merge) and the teapot k-lint (#1223 — the
re-baseline is in flight on lib/tess-rebaseline, reading done per
the gate's own recourse ladder: face genuinely replaced
cylinder→sphere, growth is #1180's documented curvature, slack table
untouched). Banked, register category A: **DanglingRef uncurated**
(ReadbackError::Dangling's payload type absent from the façade's
lists, so the two dangling lanes share one tag) — joins the curation
micro-unit queue. Process lesson kept honest: two lanes were
reported reclaimed in prose without the command having run —
reclaims are now verified by listing, the GUIDE-ASM lesson's
sibling.

**B-CHECKS MERGED (2026-08-29, #1215; mechanical under the 08-29
ruling, no A/B row). Census family B-CHECKS CLOSED — the DS6
advisory registry crosses whole.** 13/13 names bound name-for-name
(`run_checks` answers a value, `enforce_checks` refuses typed — two
exception classes that cannot be confused; the waiver rule is a TYPE:
`separation` takes `Advisory`, so waiving `Error` is unspellable,
pinned in the ty illegal fixture); 19 tests, every document authored
through public doors, both residents exercised. **MERGED RED on the
main-inherited teapot k-lint leg, on Ev's explicit in-chat
authorization** — every row the unit owns green; the re-baseline
(#1223's reading done, geometry-change arm) is in flight on
lib/tess-rebaseline. Union note: merged after B-READBACK with a
clean textual merge; the source-level census guard passed 7/7 on the
union, the compiled-module stub check rides the next gated PR.
Banked: a charter is written when a family is NAMED, not closed
(B-CHECKS' said "the connectedness check" and there were two
residents by closing time) — recorded in the census's closure
paragraph. The register's census-id sentence corrected in passing
(B-DISTRIBUTIONS was chartered but unlisted).

**TESS-REBASELINE MERGED (2026-08-29, #1243; orchestrator-direct —
the #1223 repair, the gate's own recourse ladder executed).** The
teapot budget baseline re-cut for #1180's sphere-zone belly: reading
done first (face genuinely replaced cylinder→sphere; growth is
authored curvature; slack table untouched), the re-cut made with the
sweep script's own invocation, and the diff verified row-by-row
before committing (13 changed + 12 vanished rows, all teapot;
non-teapot byte-identical; cross-machine byte-consistency with CI's
own generation). Two instrument lessons paid for en route and kept:
a sweep without `--deviation` NaN'd 77 cells and was caught by the
pre-commit diff, not by any gate; and the FIRST gate run of this PR
was green with the tess steps SKIPPED — the klint pin vocabulary is
row-specific (`release-budget` is the budget row; `release-default`
is the tour suite) and a baseline-CSV-only diff does not trigger the
gate that reads the baseline, so a re-baseline PR must pin the
budget row explicitly or its green verifies nothing — and the pin
must ride the HEAD commit, because a CI-Config trailer voids on any
later commit: the post-conflict merge of main voided the first pin
and this addendum re-carries it (the lane-ops note, met live).

CI-Config: klint=release-budget

**G16 MERGED (2026-08-29, #1224 — ordinal 301, sample #47, row in
MODEL-AB-LOG). Audit G16 CLOSED; RECIPE-DOORS unit 1 of 3 complete;
schema v16.** `Node::Chamfer` lands as the fillet's twin with the
#708 tie-deferral debt paid to ZERO sites (both emitters on
`names/defer.rs`; the shared `name_blend` makes the twins unable to
drift), rows 2/11/12 flip YES with derived closed-form oracles, and
the census/audit re-cuts hold (47 = 33+4*+10). The dual converged
A-W-F/A-W-F with no MAJOR either arm and produced permanent gates:
the corpus NAME-TABLE digests (19 documents — a surface nothing
committed covered), the blend message gates, the v15→v16
demonstration row, and the D3-discrimination probe. Banked with
homes: (1) register the tour composed-die as a CORPUS document (the
argued decline's right shape — a reviewer's transcription measured
it byte-identical but a copy would drift; the registration is a
small mechanical unit); (2) the #917 vocabulary exemption at the
chamfer op message (carries FilletError per D2, argued in place —
folds into #917's rename when taken). Next per ratified D1
sequencing: tube (HELD behind #1205's mode-flag ruling), then shell
(HELD behind #1202's kernel birth channel).

**CORPUS-DIE MERGED (2026-08-29, #1266; mechanical under the 08-29
ruling, no A/B row).** The G16 argued-decline's right shape lands:
the demo tour's composed die is corpus document `die_composed_tour`
— committed BYTES regenerated by the tour's own new `die-corpus`
mode, authored ONLY at `diefillet.rs::build`, replacing the
transcription that would have drifted. Two argued deviations, both
adjudicated sound at merge: (1) the exported document is
`gallery_document`'s (blank deleted, #1162's ruling holding for a
corpus consumer — measured: the three-root form refuses `assemble`
with vertex-vertex `UndeclaredContact`, the coincident-roots row
`r2_m10_di_probes` pins); (2) the file rides the EDIT LOG, not the
snapshot — the snapshot records its ε and `persist::load` refuses
across the ε matrix; this is the reusable half of the G18a pattern
and is now written at the module door. The regeneration diff gate
landed in ci.yml on the tour job's SAMPLED row — disclosed, not
papered over: a die-scene change drawing another row lands
unchecked and reds on the next sampled run (the lane's banked
finding generalizes: ANY gate added to the tour job is a sampled
gate). Digests: name-table `die_composed_tour` pinned; m10-p fence
re-blessed by the roster procedure (removal-alone returned all four
prior constants). Cost stated: +14.1s (+47%) on the editor-core
aggregated suite, the registry's heaviest row at ~18× die_composed;
416 KB asset read at run time rather than include_str'd ×30. One
orchestrator fix pre-merge: the `die-corpus` usage comment in
main.rs contradicted the code beside it (claimed the corpus keeps
the blank's three fillet sites; `corpus_text` asserts the
blank-deleted document) — the stale-comment class again, corrected
at review.
**CUR3 MERGED (2026-08-29, #1262; mechanical under the 08-29 ruling,
no A/B row).** The G16-cycle banked curation finding lands:
`DanglingRef` rides beside `ReadbackError` in select.rs and the
prelude's group 9 — the refusal's MATCHABLE payload, the same
convention `SurfaceKind` follows for `BooleanError` — and pncad-py's
shared `dangling` tag splits into `dangling_entity` /
`dangling_geometry`, matched exhaustively over `DanglingRef`'s arms
so a third kernel lane stops the crate compiling. The
previously-unconstructible pin arm is now constructed and both texts
pinned ("ONE ARM IS ABSENT" retired); census carries
`DanglingRef: ReadbackError.variant` on the `RootFault` precedent
(7/7); no layer-guard motion (topo is whole-re-exported; LB13
untouched). Sweep delta vs #1173/#1216: door rung unchanged (three
raw hits, all previously disposed); the NEW payload rung (53 carried
error enums, 54 raw hits) narrowed to the DanglingRef shape leaves 8,
tabled in the PR, with the stated blind spot that struct payloads are
invisible to the enum-indexed scan. Banked with homes: (1)
`BandField` ← `BandError::InvalidValue` — the semantic twin (two-arm
pure discriminant of a prelude-carried refusal, uncurated); (2) the
`FilletError` submodule trio (`FilletSite`/`CornerConfig`/
`RunOutPolicy`) — the structural twin of the ReadbackError lift; (3)
the `ValidationError` key-bearing trio; `MeshPickError` is DECIDED
absent, not a gap. #1173's StlError reach-past residue verified
already correct on main — closed, no longer owed. The lane also
FOUND MAIN RED (display_budget's `include_str!` naming the
v15 fixture #1224 renamed — a union break gating every PR's Rust
shards) — routed orchestrator-direct as #1264, per the
red-goes-straight-to-a-fix social rule.

**LIB-DIETOOL DELIVERED (2026-09-03, mechanical, brief-as-spec).**
The banked "die_tool's Python re-authoring (banked behind its
Revolve/datum half)" is CLOSED, and the verdict is CLEARED — by
construction, not by argument. The record first: the blocker was
`die_pips`' deviation (b), the equator workaround — the revolve NAME
EMITTER refused an all-on-axis two-pole loop ("revolve vertex
resolution exceeded elimination"), so no sphere reached a
`Node::Revolve` and the ball was charted as two quarter arcs meeting at
an off-axis equator vertex, the second's bulge derived from `tan(π/8)`.
`7581fb65d` (2026-08-15) deleted it from `die_pips`, `die_composed` and
the tour once the emitter grew its pole export. **The Rust corpus
document never carried it**: `die_tool` (`54f44ac90`) postdates that
deletion by one commit and reuses `half_disc_program`'s natural
bulge-1 semicircle, so there was nothing to re-author on that side and
the name-table digest could not move (measured: `die_tool` still
`0x9e24_4be7_b06b_9a40`, and `m10_p_fence`'s three scalars unmoved —
the registry is untouched, so neither gate's re-bless procedure was
entered). What was genuinely banked was the CROSSING: `heat_sink_fins`
(Linear, extrude-only) had a Python twin and `die_tool` (Explicit,
Revolve about a `Datum::Axis`) had none.
It authors clean. `test_placed_union.py::TestTheDieTool` says the
document's seven nodes through the bound doors — `Node.polygon`,
`extrude`, `datum_axis`, `profile` on a `from_frame` plane, `revolve`,
`placed_union_at` of six `Frame.rotate_then_translate`s, `boolean`
Subtract — green, valid, one solid, 18 faces, volume on the six-cap
oracle at 1e-12 relative. The sameness is BYTES, not eyeballs:
`lib_dietool_crossing.rs` pins the registered document's `persist::save`
text as `corpus/die_tool.pncad` (with a `PNCAD_BLESS` door), and the
Python row asserts its own `Doc.save()` against that file line for line
— the whole 468 lines, identity included (`Doc("mod")` derives the id
`fixture::Recorder` does), bar the one swept `"epsilon"` line on the
`plate_param` precedent. A recipe change on either side is now a red
run.
Two things swept en route. The Python die scene
(`test_north_star.py::DieScene.ball`) still carried the equator
workaround with a docstring asserting a refusal that had been retired
eighteen days earlier — the stale-comment class, and this one was
load-bearing prose. Replaced by the scene's own meridian; all
seventeen die rows (`diepips`, `diecomposed` incl. its 42-rim
`select_where`, `diechamfer`) green unchanged, so the re-chart was dead
weight. And ONE finding filed:
`work/lib/pncad-py-doc-has-no-node-kind-read-door.md` — `Doc` answers
`order`/`node_count`/`placement`/`reference`/`interface` and no node
KIND, and `Value.kind` is the value's ("body" for both a group and a
union), so the Rust row's `(groups, unions, transforms) == (1, 0, 0)`
has no Python spelling; the mirror asserts the node COUNT (7 against
the pairwise chain's 18) and leans on the byte pin's JSON for the rest.
No audit row flips: G8's residual is the Union-into-a-BASE fusion
asserted by `TestHeatsink`, and this row is a Subtract — though it does
execute "a group feeds a boolean from Python", which is the half of
that sentence that was in doubt.

## Tracker migration (2026-09-03)

This log moved here from `docs/LIB-LOG.md`; the program's contract stays
`docs/LIBRARY-DESIGN.md` (no plan file). The slate now lives in this
directory's item files and in `work/STATUS.md` (generated); this log
stays the narrative. Items created at migration: LIB-TUBE (spec),
LIB-G17 (parked on issue 1202).

**CUR4 MERGED (2026-09-03, #1633; mechanical under the 08-29 ruling,
no A/B row).** CUR3's three banked twins come back measured, and two
of the three answers are not the ones the bank assumed. The
`FilletError` structural twin is CARRIED — but every coordinate in
CUR3's row had moved: `6cedf722b` renamed `sweep::fillet` to
`sweep::blend`, so it is `BlendSite`/`CornerConfig`/`RunOutPolicy` off
`sweep::blend`, and `FilletError`/`FilletSite` have zero references in
the tree. The case is STRONGER than the one CUR3 fixed: `DanglingRef`
at least sat at `topo`'s root, while `sweep` re-exports nothing from
`blend`, so the only spelling was `pncad::sweep::blend::CornerConfig`.
A fourth name rides with them — `Convexity`
(`BlendError::ConvexitySignFlip`), which CUR3's scan could not see
because its narrowing rule was "same module as the carrier" and
`Convexity` lives one deeper in `blend::battery`. The
`ValidationError` trio is CARRIED too, one rung and no further:
`CensusContact`/`RingContact`/`StaleDeclaration` sit at `topo`'s root
exactly as `DanglingRef` did, which is what settles that
root-reachability does NOT discharge a curated list; the list was
already half-persuaded, since `DeclaredContact` (the
`ContactContradicted` payload) has been curated through select.rs all
along. `BandField` is the one that flips: ARGUED NON-CARRIAGE, because
every verb derives its band through `Band::linear` = `Band::new(ε,
K·ε)` and `Tol`'s invariant makes the `zero` check unfirable, so the
only `InvalidValue` a prelude caller can receive is `field: Escalate`
— a discriminant that is CONSTANT at the curated boundary has nothing
to branch on. `Band::angular_at` has no live call site anywhere, and
the argument is written into prelude.rs with its own falsifier (a
caller for `angular_at` makes `Zero` reachable and flips it). NO
PYTHON TAG MOVES either way, measured not skipped: `BlendError`
projects no arms (`node_error_tag` reads the VERB) and the validate
doors cross as joined `Display` prose with no per-arm tag at all — so
the rule the two units together settle, now written into the census,
is that **a payload's category follows what its CARRIER does at the
crossing** (`DanglingRef` is `BOUND_AS` because `ReadbackError`
projects; these seven are `INTERIOR` beside `BandError` and
`DeclaredContact`). New pin
`carried_refusal_payloads_are_matchable_through_the_prelude` reaches
every carried name by BARE prelude name with no module path, which is
the failure mode all.rs's nameability sinks cannot see. CUR3's blind
spot (a) closed one rung: the struct-payload sweep over 425 curated
names and 51 carried refusals found 77 uncurated payloads at rung 1
(20 of them structs) and 24 at the new rung 2, tabled in the PR. Three
findings banked with homes rather than swept up: `Indeterminate` +
`MarginDiag` (the escalation payload, uncurated under THIRTEEN
refusals — far the largest in the tree, and its answer may well be
`BandField`'s); `EntityId`/`GeomRef`/`ContactFinding` (the rung both
curation units stopped at, now named once instead of twice);
`LoopKey`, curated out of step with `VertexKey`/`EdgeKey`/`FaceKey`
and invisible to both scans because `slotmap::new_key_type!` mints it
— a FIFTH blind spot (e) beside CUR3's four, filed with the
methodology note that a flat name index also silently takes the wrong
definition across crates (`viewer::blend::BlendError` shadowed
`sweep::blend::BlendError` and hid the whole fillet quartet on the
first run). `MeshPickError` re-verified DECIDED absent, unchanged. No
kernel edits; the diff is `pub use` lines, prelude arguments, one test
and census rows.
**LIB-TUBE DELIVERED (branch `lib/tube`; spec docs/LIB-TUBE-SPEC.md
under RECIPE-DOORS D4 AS REVISED by the #1205 split ruling, AMENDED
2026-09-03). RECIPE-DOORS unit 2 of 3; audit G2's TUBE HALF CLOSED.**
`Node::Tube` and `Node::HollowTube` land as two node kinds over the
kernel's two public doors — the wall REQUIRED on the hollow kind,
`Option` nowhere in the recipe vocabulary — with content-key tags 28
and 29 appended and a `TubeWindow` recipe enum whose variant is
structural payload. `Node.tube` / `Node.hollow_tube` bind them in
pncad-py, with `TubeWindow` crossing as a VALUE (`full()` /
`arc(t0, t1)`) rather than an optional pair of angles.

**Persistence is ADDITIVE GROWTH, not a break.** The unit was built
against the pre-BOOL-13 spec and carried a v17 SCHEMA_VERSION bump,
its ledger entry, a prose tripwire and a v16→v17 demonstration row;
#1553 demolished the version machinery underneath it, and all of that
came OUT at the re-merge rather than being carried. What the unit owes
under the amended deliverable 3 and delivers instead: the golden
fixture regenerated by its own recipe (`M4_PR6_BLESS_GOLDEN=1`) with
the two kinds and both window spellings appended to it, the corpus
digests re-blessed by the roster procedure, and the round-trip row
extended so a document carrying both kinds survives save/load/replay
bit-identically. An older document naming neither kind loads, from
the same one door, unchanged.

**The two measurements the spec demanded, answered by execution.**
(1) The revolve emitter template applies WHOLESALE: `name_revolve`
reads only `Revolved<T>`'s own maps and never the profile, both tube
doors return one built by the same `full`/`partial` machinery, so
zero new `RoleSeg` variants and zero changes to `emit_sweep.rs`. The
one tube-specific step is a step NOT taken — no `anchored` rewrite,
because there is no profile node to anchor to. (2) The storage
contract is metered at the STORED BITS, not the volumes: rows read
`Surface::Torus`'s `minor_radius` off the built body and compare
`to_bits()`, which is the only oracle that can see the claim rows
25/26 actually make (volumes agree to 1e-12 either way).

Audit re-cut, honest — at the table's CURRENT numbering, which the
montage-v3 curation renumbered 47 rows to 43 while this lane was
dead: rows 23 (`tube_along_arc`), 25 (`hollowelbow`) and 26
(`hollowtorus`) flip YES; rows 13 (`lily`) and 27 (`teapot`) KEEP
their NO on blockers that were never the tube (sweep + placement;
shell) and their rows now say which half lifted; rows 19-22 unmoved
— no `wire_sweep` motion, U4/LQ3 untouched. G2 re-counted 8 -> 5,
tallies re-derived from the sheet.

Banked, with homes: (1) per-ARM Python error tags — every op on
`node_error_tag` gets ONE tag (`revolve` covers ten `RevolveError`
arms), so a Python caller distinguishes a wall refusal from a frame
refusal only by prose; worth doing for every op at once, not for the
one whose unit was written last. FILED at the fix pass as
`lib-per-arm-error-tags` (a banked finding with no item is a finding
nobody can pick up — R1's Q6). (2) `reader_census`'s dot-component
filter tests the ABSOLUTE path, so any checkout under a dotted
directory (`~/.local/...`) reports all 34 ledger entries stale and
reds for an environmental reason; the filter wants the path relative
to the repo root. Neither is this unit's.

**Interruption, recorded.** The implementer lane finished the unit
locally on 2026-08-29 and was killed by a session usage limit before
it could push; four idle days later main had landed BOOL-13's schema
demolition, the work/ tracker migration, the code-quality migration
and the Evan→Ev rename, and the spec was amended (#1623) to match. A
first resume died to a container restart mid-merge (that half-merge
was aborted, nothing lost). This row is written at the adaptation,
which re-merged the eight original commits onto that main, removed
the void version machinery, moved this row here from
`docs/LIB-LOG.md`, and re-verified the deliverables.

**FIX PASS (both reviews A-W-F, no MAJOR; union of 10, all taken).**
The corrections worth naming: the stale pre-migration row numbers were
a CLASS, not a typo — five citations across the log, `lib_tube_node`,
the Python suite and the audit's own self-referencing `hollowtorus`
cell, all re-pointed at the 43-row table; and TWO caller-facing doc
surfaces claimed a non-unit AXIS refuses, which is FALSE and was
demonstrated so — a datum axis normalizes its direction on the way
through (`(0, 0, 2)` builds silently), so only `u_ref`, which passes
through no datum, is refused for length. Both now say what executes
and name the asymmetry. Also: `tube_ring`'s header no longer advertises
an exact mass pin over `pin: None`; the golden's last "the bump" phrase
is gone; the spec's "canonicalizing construction doors" is recorded as
VACUOUSLY satisfied at `TubeWindow` (no set-shaped payload exists to
canonicalize, and the one candidate — the window's angle pair — must
not be); the audit's four dead `docs/LIB-LOG.md` pointers now resolve;
and both reviewers' probe branches land as merge parents,
authorship-preserving, wired in as permanent rows.

Next per ratified D1 sequencing: shell, still HELD behind #1202's
kernel birth channel.

**LIB-TUBE MERGED (2026-09-03, #1628 — ordinal 302, sample #114, row
in MODEL-AB-LOG). RECIPE-DOORS unit 2 of 3 complete; block LIB-12
consumed and OPENED.** `Node::Tube` and `Node::HollowTube` land per
the #1205 split ruling — wall required, `Option` nowhere in the
vocabulary, kernel untouched, revolve emitter wholesale with zero new
RoleSegs. The dual converged A-W-F/A-W-F with no MAJOR; the fix pass
took the full 10-item union and adopted both probe branches as merge
parents. The unit survived the most interrupted history in the log
(usage-limit death unpushed, a four-day gap in which BOOL-13
demolished the schema version mid-unit, a container-restart mid-merge
discard, a pre-review spec amendment) with the seam proven clean by
both reviewers' interdiffs. The sampled matrix earned its keep twice
in the fix pass alone: the rustdoc gate and a 1e-6 draw that
falsified a unit-written ε-dependent fixture no other point could
see. Next per D1: shell stays parked on #1202; the census B-families
resume the mechanical track; a LIB-13 block draw precedes any next
full-protocol unit.
