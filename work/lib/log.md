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
family id, each carrying a one-line charter saying what a unit
closing it would deliver. **The ids are NOT listed here** — they are
`FAMILIES` in `crates/pncad-py/tests/test_binding_census.py`, and a
copy of them in this paragraph is a second enumeration that goes
stale the moment a family closes. It had: this sentence still named
`B-READBACK` after LIB-B-READBACK closed it, and would have named
`B-EXPR-READ` after LIB-B-EXPR-READ, while never having gained
`B-NOTATION` or `B-MEASURES`. The census's own guard fails on a
charter nobody cites and on a citation nothing defines, which is
what a list in prose cannot do. The
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

   **SUPERSEDED 2026-09-04 (PR 1850, `ciw/unsample-klint`) — the
   procedure in the paragraph above is now WRONG in both halves, and
   the trailer line above it would RED a run rather than narrow one.**
   The k-lint row stopped being drawn: every code-tier run gates all
   five unifications as five `k-lint (gate, <row>)` legs, so
   `release-budget` runs on every diff and the premise — that a
   baseline-CSV-only diff would not draw the budget row — is gone.
   And `CI-Config:` became additive-only on all three dimensions, so
   `klint=release-budget` no longer pins anything: it fails the
   classify step and names the `workflow_dispatch` inputs, which are
   now the only spelling that narrows. A re-baseline PR needs no pin.
   Left in place rather than deleted because this log is append-only
   and the entry is the record of what was done; annotated because it
   reads as durable procedure rather than as a dated note, and a lane
   following it today gets a red. Announced to LIB in PR 1850 rather
   than edited silently — this is LIB's file.

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

**B-PICKING AT REVIEW (2026-09-03, #1661; mechanical under the 08-29
ruling, no A/B row). Census family B-PICKING CLOSED — the fourth door
onto a name reaches Python, and it answers in the alphabet the other
three speak.** Seven names, all seven unbound at the start: five cross
name-for-name (`Ray`, `PickHit`, `NodePick`, `NodePickError`,
`HitTestError`), `pick_face` becomes `Evaluation.pick_face` beside the
read-back verbs, and `PickTarget` maps to `NodePick` — not a
narrowing but CUR3's construction argument reading out downstream: with
`MeshPick` interior, a raw target has no constructor in EITHER
language, so the value that cannot be mis-paired is the only target
there is and the confidently-wrong-name lane (#1098) has no Python
spelling at all. 39 Python tests, oracled against the doors that
already existed rather than against restated arithmetic: the picked
name IS `select`'s answer for the same face, `patch_names` IS
`all_faces`' set, `boundary_names` IS `all_edges`', and a `NodePick`'s
mesh is triangle-for-triangle `Body.tessellate`'s at the same δ (the
"what is drawn is what is picked" claim, checked rather than asserted).
The tie-break is pinned in both directions — two coincident cubes, the
list reversed — so the winner is the documented rule and not chance.

Two things the closing measured. **The `Mesh` docstring was carrying a
falsehood on both sides of the boundary**: it said a door from a patch
to a `StableName` "does not exist on either side", which stopped being
true when `NodePick::patch_names` shipped kernel-side; binding it is
what makes a patch INDEX a handle rather than a dead end, and both the
stub and `py/mesh.rs` now say so. And **`HitTestError` crosses as a
VALUE, not only as a raise** — `patch_names` is total per patch with
the loud arm in its own slot, and a Python exception being a value is
what makes that kernel shape spellable at all; the stub types the slot
`str | HitTestError` and the fixtures pin it.

Banked, register category A: **`MeshPickError` unmatchable under
`NodePickError`** (`work/lib/mesh-pick-error-is-unmatchable-under-node-
pick-error.md`) — the `DanglingRef` shape one rung along. CUR3's stanza
argues from CONSTRUCTION and is honored verbatim here; what it does not
address is that the type also arrives as a PAYLOAD of a curated
refusal whose other four arms are all matchable, which is exactly the
case the CUR3/CUR4 carrier-projection rule was written for. Joins the
curation micro-unit queue; not relitigated in this unit.
Also banked, and deliberately NOT fixed in the diff: **the census
points at `docs/LIB-LOG.md` three times in the present tense and that
file is gone** — the register it names lives at `work/lib/log.md:439`
now (`work/lib/census-points-at-a-deleted-lib-log.md`). The fix is
three one-line hunks in the most contended file on this track, for a
reason unrelated to any family, so it waits for a pass with no
concurrent B-lane rather than riding a family unit. Two of the three
are LIVE claims about where a reader should go, so the ledger's
append-only-log dispensation does not cover them.

**B-RESOLVE AT REVIEW (2026-09-03, #1664; mechanical under the 08-29
ruling, no A/B row). Census family B-RESOLVE CLOSED — the question a
consumer that STORES names must ask on every run becomes askable in
the language whose whole selection story is store-then-reuse.** Three
names, all three unbound at the start: `Resolution` crosses
name-for-name, `resolve` becomes `Evaluation.resolve` beside the
read-back and picking doors, and `RunCtx` maps to `Evaluation` —
because Rust's `RunCtx` is a (doc, eval) PAIR and Python's
`Evaluation` BECAME that pair, capturing the document at `evaluate`
beside the `ParamEnv` it already captured, for the reason stated
verbatim there and sharper here: Python's `Doc` is mutable and
`accept` swaps it under the handle, so a `resolve(doc, name)` door
would let a caller ask this evaluation about a recipe it is not of and
get a confident answer. NodePick's pairing argument, on a different
door. 20 Python tests, oracled against doors that already existed
rather than against restated verdicts: every materialized name
resolves and resolves AS the kind the materializer that answered is
for (new information in Python, where a name is opaque text nobody may
parse); `denotation` and `resolve` agree where they overlap and differ
exactly where the docstrings say — a transform's pass-through names
pin `resolve` on the UPSTREAM carrier against the evaluation's own
`order()`, and `denotation` refuses `no_such_name` where `resolve`
answers; and the strongest, a resolved verdict's `(node, body)` fed to
`NodePick.build` finds the name again in `patch_names`, so the
verdict's location claim is VERIFIED by an independent door rather
than asserted. The three states are built from documents, not mocks —
a real deleted node, a real vanished side face (square plate vs
triangular, same recipe one argument apart), a real fillet whose
radius will not fit poisoning a real boolean — each pair asserting its
node ids match across the two documents, or the comparison is between
unrelated recipes and proves nothing.

Two things the closing measured. **The tag pin could not be written
the way every other pin in that file is written.** `Resolution` cannot
be ASSEMBLED through the façade at all — only obtained — so
`resolution_status_tags_are_stable` builds a document and lets the
three states happen to it (resolved, minting node deleted, canceled
run's suffix). That is a stronger test than the literal pin it
replaces, because it asserts each state is reachable BY THE ROUTE a
caller reaches it; it is also a workaround, and the thing it works
around is the finding. And **the three states carry the RECOURSE**,
which is why folding `indeterminate` into `failed` would be the one
substantive error available here: rebinding a name whose minting node
merely failed repairs the wrong end of the document.

Banked, register category A: **`ResolveError`'s three arms are
unmatchable under `Resolution`** (`work/lib/resolution-failure-arms-
are-unmatchable-under-resolution.md`) — the carrier-projection rung a
THIRD time, and the first where the carrier is a VALUE rather than a
refusal. `DanglingRef` (curated, CUR3), `MeshPickError` (one arm of
five, open), this (every payload of the carrier). The issue states
GUI-2's counter-argument fairly — it carried these payloads briefly
and put them back because nothing consumed them, and that stands on
its own terms — and notes only what changed since: a Rust panel
rendering `Display` has the payload one field away, a Python caller
holds a string and has nothing else, so the disposition was taken when
only one side of the boundary existed. Curation queue, not a binding
unit's call.

Also banked, and deliberately NOT fixed: **the pncad-py
python-feature clippy lane is red on main and no CI row runs it**
(`work/lib/pncad-py-python-feature-clippy-lane-is-red.md`). Verified
on pristine origin/main by checking out its `crates/pncad-py/` and
re-running — a pre-existing `type_complexity` at `py/value.rs:319`.
CI's clippy row runs at DEFAULT features (`ci.yml:1523`), which is
exactly what the manifest's interpreter-free gating is for; the
consequence nobody had written down is that every `#[pyclass]` in the
crate sits outside every clippy row CI runs, so the lane has never
been green because it has never been run. Filed so the next unit that
runs the command does not re-derive that it is not theirs.
**B-EXPR-READ IN REVIEW (2026-09-03, #1662; mechanical under the
08-29 ruling, no A/B row). Census family B-EXPR-READ CLOSED — the
expression READ side crosses, and the family cost three times the
names it owned.** `Doc.parse_expr` / `Doc.eval` / `Doc.eval_count`
plus `Expr` (`dimension`, `text`, `literal_value`, `params`) and two
typed refusal classes, `ParseError` and `EvalError`. 30 Python rows in
`tests/test_expressions.py` (352 suite rows to 382), both tag maps pinned arm-by-arm in
`src/tests.rs` against source strings rather than hand-built values,
ty fixtures on both sides.

**The measurement worth keeping: the entries an id owns are not the
entries a unit must move.** B-EXPR-READ chartered three names —
`eval`, `eval_count`, `EvalError` — and none of them is reachable
without a value to evaluate. The four that make one (`Expr`,
`parse_expr`, `unparse`, `ParseError`) were filed under `G1`, because
the census had split this family on WHICH HALF THE AUDIT REACHED
rather than on what a unit would have to build. Nine entries moved,
`ParamEnv` included. The census's own docstring predicted this
("not that a cited id is the RIGHT owner"); this is the first case
that executed it, and it is recorded in the closure paragraph rather
than smoothed.

**G1 is NOT closed by that**, and the re-cut says so twice: the
residue this unit touched is the AUTHORING half, which is now a
SIGNATURE rather than a missing name — no door takes an `Expr` INTO a
document — so the census structurally cannot watch it and
`tests/test_north_star.py` does, executing an `Expr` against `Radius`
and `DocParam.length` and watching both refuse. G1 also keeps a
census citation outright, which the pre-merge draft of this row got
wrong: B-PICKING's merge brought `ArrivesTangent` in under the same
id, for a residue of the same row that has nothing to do with
expressions. `ParamEnv` moved to `INTERIOR` rather than
anywhere else, correcting an entry that had been a `gap` on the RUST
door's shape: Python's `select_where` never took one, `Evaluation`
captures the environment at `evaluate`, and `Doc.eval` now builds one
from the document it is a method on — two doors, neither handing it
out.

**One routing decision, made and recorded on all four surfaces.** The
expression layer's `DimensionError` had two Python routes and neither
was branchable: `LiteralError` on the literal door, and `load`'s
`PersistError`/`unreadable` misrouting (#694, untouched here). The
text door is a third, and it lands as `ParseError` with `variant ==
"dimension"` and the mismatch's own tag as `kind` — because what
refused is the PARSE and the byte offset is the recourse, which a
`LiteralError` has nowhere to put. `test_north_star.py`'s old entry
asked whoever bound this to decide; that comment now records the
answer instead of the question.

**Banked, boundary-wide: `args` is unusable as a typed-exception
attribute.** It is `BaseException`'s own and CPython requires a tuple,
so the crate's "every field on every arm, `None` where absent" shape
raises `TypeError: 'NoneType' object is not iterable` from inside the
raise. Found by 14 red rows, not by reading. `WrongArity`'s count is
`given`; the constraint is written at `py::typed_err`, the single
construction site, so the next door does not rediscover it.

Two stale claims corrected in passing, both in files this unit
touched: `pncad.pyi`'s header still listed the geometry read-back
doors as deliberately absent (LIB-B-READBACK bound them a week
earlier) and still paired chamfer with shell (G16 closed); and this
register's own paragraph carried a second enumeration of the census
families — which named `B-READBACK` after it closed and never gained
`B-NOTATION` or `B-MEASURES` — now replaced by the pointer the
sentence already claimed to be. The census's gap-id counts were
stale the same way ("two … the other seven" against nine cited
families). They are now GONE rather than corrected: this unit first
re-cut them to "one … the other eight", and B-PICKING closing in the
same window falsified that before either landed. A number two
concurrent units can invalidate is not a measurement, it is a merge
conflict with a plausible face, and the guard that IS checked says
nothing about how many there are.

Measured and not filed: `ParseError::MalformedNumber` is unreachable
from text. The lexer hands `f64::from_str` only a run of digits with
at most one dot, so every malformed shape refuses earlier and under
another arm. Defensive rather than dead — the lexer's rule need not
stay a subset of `f64`'s — so it keeps its tag and the pin says why
it has no fixture. Not a kernel change and not an item.

**B-FORMAT IN REVIEW (2026-09-03, #1668; mechanical under the 08-29
ruling, no A/B row). Census family B-FORMAT CLOSED — the D6 display
formatter crosses, and the family cost exactly the names it owned.**
`Length.format` / `Angle.format` plus `FmtQuantityError`. 13 Python
rows in `tests/test_quantities.py` (421 suite rows to 434), the tag
map pinned in `src/tests.rs` through the formatter itself rather than
against a hand-built arm, ty fixtures on both sides.

**The measurement, and it is the negative of B-EXPR-READ's.** That
family chartered three names and moved nine, because the census had
split it on which half the audit reached rather than on what a unit
would have to build. The same check ran first here and came back
negative: `fmt_length` needs a canonical value and a `LengthUnit`,
Python has had `Length`, `Angle` and the seven unit constants since
§L4, so the pins were already constructible and there was nothing to
reclaim from another id. Three chartered, three moved. **A family is
cheap exactly when its arguments already cross** — which is the only
general thing the two closings say together, and it is worth writing
down because the audit-reach check is now a step rather than a
discovery.

**One mapping decision, on a door whose Rust signature does not name
its carrier.** `quantity::fmt_length` takes canonical metres as a
bare `f64` because Rust reaches that module from BELOW, where the
newtype is already unwrapped; Python has no such caller. What a
Python consumer holds is a `Length`, and it holds one precisely so a
length and an angle cannot be interchanged — so binding the free
functions AS free functions would hand the interchange back
(`fmt_length((90 * deg).radians, mm)` type-checks and prints
plausible nonsense). The door lands on the value it needs instead.
That is #1661's `PickTarget` argument one door over: the value that
cannot be mis-paired is the only receiver there is.

**The oracle is the parser, and Rust structurally cannot use it.**
`crates/quantity/src/fmt.rs` claims `parse(fmt(x, unit))` recovers
`x`'s exact bits, where `parse` is the expression text parser's
literal semantics. `quantity` sits below `editor-core`, so the Rust
round-trip test transliterates that rule by hand
(`tests.rs::parse_back`). Python is above both, so the pin is checked
against the door it names — every unit, `pi rad` included, on values
authored in the unit and on values arrived at by arithmetic, bit
equality rather than a tolerance. That became possible one merge ago:
without #1662's `Doc.parse_expr` / `Doc.eval` there is no Python-side
parser to be an oracle, and this unit would have had to restate the
arithmetic instead. The charter's own claim is an assertion too —
`1.0240000000000047 m` has no preimage in millimetres, the hand
form `f"{x.in_unit(mm)} {mm.symbol}"` reads back to different bits and
`format` does not, so the family closes against a measured difference.

**Banked, one item, two defects with one root:
`work/lib/the-quantity-boundary-compares-and-hashes-as-if-poison-and-
signed-zero-cannot-arrive.md`.** The formatter is the first Python
door that must have an opinion about a non-finite quantity, and asking
that of `Length` found two its neighbours answer wrongly: `==` goes
through the same `partial_cmp` the orderings do, so two NaN lengths
RAISE an untyped `ValueError` where a bare float answers `False`; and
`-0.0 * m == 0.0 * m` is true while their hashes differ, the
data-model violation `DocParam`'s fold already avoids one layer up.
Neither is a kernel need — `crates/quantity/src/lib.rs:69` says the
newtypes refuse no float, deliberately — and neither is fixed here,
because both are semantics calls on doors this unit does not bind.
Both are PINNED as they stand, so a fix goes red rather than silent.

**A false claim corrected in place**, in a file this unit touches and
falsifies: `py/quantity.rs`'s comparison arm said "NaN cannot arise
from the constructors (the boundary refuses non-finite input)". Both
halves are false. The kernel is deliberate about it; the BINDING was
what assumed otherwise, and binding the door that has to render
poison is what made the assumption visible.

**A spliced paragraph in this register, removed rather than banked.**
The bindings-parity section carried the same paragraph twice — an old
half ending mid-sentence on a dangling "The", a blank line, then
#1662's replacement resuming the clause. The stale half carried a
LIVE enumeration naming seven family ids as open charters, two of them
already closed and a third closed by this PR. The duplication predates
#1662, whose merge replaced only the second half. #1661 banked the
`docs/LIB-LOG.md` pointer fix because it was "for a reason unrelated
to any family"; this one is not unrelated — leaving it ships a
register saying B-FORMAT is open on the PR that closes it. What
survives is #1662's text, which already makes the argument the deleted
half violated.

Measured and not filed: the canonical-unit fallback rate. `fmt.rs`
records mm ~2.8% over uniform random finite f64; a Python sample over
`uniform(-1e3, 1e3)` metres measured 2.3%, and over lengths AUTHORED
in millimetres, zero in 20k — which is the module's own honest
statement ("a value authored in the unit lies in the parse map's
image and always keeps its suffix") reproduced from the other side.
Not a new fact and not a pin: a sampled rate two lanes could
re-measure differently is not a guard. What the suite checks instead
is that ONE named fallback value falls back, and that its text still
reads back exactly.
**#917 residue DISCHARGED (2026-09-03, orchestrator note).** The
G16-banked "#917 vocabulary exemption at the chamfer op message"
found its home while LIB slept: the conversation is ratified as
docs/BLEND-VOCAB-DESIGN.md (Ev, 2026-08-30) and executes as BLEND-6
(V1/V2 wording, V3 rename, Filleted/Chamfered collapse with
aliases). The G16-era crate markers are already gone (verified:
only STEP-fixture #917 false positives remain in crates/). Nothing
LIB holds; the recipe layer's NodeErrorKind::Blend shape is cited
in that design as ratified precedent.

**B-CANCEL IN REVIEW (2026-09-03, #1676; mechanical under the 08-29
ruling, no A/B row). Census family B-CANCEL CLOSED — a Python caller
can stop an evaluation, and the family found that binding the STOP
was the small half.** `CancelToken` (top-level, spelled identically,
no `BOUND_AS`), `evaluate(..., cancel=)`, and `Evaluation.canceled`.
19 Python rows in `tests/test_cancellation.py` (454 suite rows to
473), a cross-door tag pin in `src/tests.rs`, ty fixtures both
directions.

**The audit-reach check, run first as it now is on every family, and
negative in the strongest available form:**
`docs/guide/north-star-audit.md` does not contain the string "cancel"
at all. No tour scene interrupts an evaluation, so no audit gap id
reaches this door, nothing sat filed under another id to reclaim, and
the family moved exactly the one name it chartered. By B-FORMAT's
arithmetic that is the cheap case. It was not, and the reason is
worth the register.

**A charter that names a door's ARGUMENT undercounts by whatever the
door ANSWERS with.** B-FORMAT was cheap because `fmt_length`'s
arguments already crossed. B-CANCEL's argument needed no building
either — `CancelToken` is an `Arc<AtomicBool>` with two methods, and
threading it through `evaluate` is four lines. What was missing was
the other end: a canceled run answers by BEING a partial `Evaluation`,
and Python could not ask an `Evaluation` whether it had completed. So
the unit bound its one chartered name and then had to bind the answer
that name makes reachable before the door meant anything. That is the
general form of what B-EXPR-READ found by splitting a family on audit
reach and B-FORMAT found by not needing to.

**`EvalOutcome` is the entry this re-cut IN PLACE rather than moved,
and it is the sharper half of the same finding.** It sat as
`different-shape` — "the surface is a different shape in Python, not
a debt" — and that was true only VACUOUSLY: Python's `evaluate`
minted a token nobody could reach, so the outcome of every Python
evaluation was the constant `Completed`, and a constant needs no
accessor. `cancel=` made the second variant reachable and the entry
therefore owed a real Python shape, which is now `Evaluation.canceled`.
**A disposition can be honest about a surface and stop being honest
when a NEIGHBOURING door opens, with nothing mechanical to say so** —
the roster guards check spelling and decay, not whether a `SHAPE`
claim still has a shape behind it. Recorded at the entry, because
reading the entry beside the door is the only thing that catches it.

**The change that was not on anyone's list: the GIL.** Threading the
token needed no kernel and no editor-core change — but `evaluate` held
the GIL across the whole kernel call, so no other Python thread could
run, so `token.cancel()` could not be called during the only window in
which it matters. Binding the token without releasing the GIL would
have left the charter's own sentence ("a Python caller cannot stop a
long evaluation") TRUE on the PR that closes it. So the kernel run is
now inside `py.detach`, measured both ways on the suite's own
document (31 nodes, ~300 ms debug): with the GIL released a helper
thread cancelled it 20/20 at a 20 ms delay and 20/20 at no delay; the
same code against a build differing only in the `detach` cancelled
**0 of 20 at either delay**. Safe because `doc` stays borrowed for the
call, so a concurrent MUTATION of the same document raises pyo3's own
`RuntimeError("Already borrowed")` rather than editing a recipe out
from under a running run — measured, and pinned, rather than reasoned:
the first draft of this sentence named `Doc.accept`, which is a Rust
`&mut self` helper Python does not have. The doors that DO cross are
`Doc.insert` and `Doc.apply`, and a test now executes one of them.

**What is pinned and what is only measured.** Cancelling a run already
under way is a race by construction — the kernel checks between nodes,
and which node it stops at is the scheduler's answer. So the contract
is pinned on the DETERMINISTIC arm: a token canceled BEFORE the run
starts fires the check before the first node, every time, giving the
full `order()`, zero results, `recomputed == 0`. The concurrent arm is
pinned only on invariants that hold whichever way the race falls
(canceled ⟹ the results are a PREFIX; never a hole, never a node
failed by the stop) plus ONE categorical yes/no: a helper thread
stamping the clock every millisecond lands 139 stamps in the middle
half of the call's window with the GIL released and exactly 0 with it
held. 0-versus-many is a fact about the binding; a hit rate is a fact
about the box, and the hit rates are quoted in the module docstring
rather than asserted anywhere.

**A false claim corrected in place, in the door this unit made
reachable.** `Evaluation.value` answered `unknown_node` — "no such
node in the evaluated document" — for every node with no entry. With
no way to stop a run, a live node always had one, so the sentence was
true because its false case could not arise; a canceled run's prefix
is exactly that false case. The door now splits them, and the second
arm speaks the STANDING LADDER's own word (`node_not_evaluated`, as
`ReadbackError` and `HitTestError` spell it) rather than minting a
third. The read-back and pick doors reach that word through a `match`
on a kernel arm and this one cannot — `Evaluation::result` answers a
bare `None` — so the word is now `tags::NODE_NOT_EVALUATED` and
`tests::the_evaluation_door_speaks_the_standing_ladder` pins the copy
against both matching doors, in both directions.

**One north-star pin re-cut, not silenced.**
`test_the_named_gaps_are_still_gaps` asserts `evaluate`'s whole
keyword-only shape as G18's structural evidence; `cancel` joining it
made that row red, which is the guard working. It is re-cut to the new
list with the reading recorded at the line — `cancel` belongs to no
audit gap, which is why the census owned it — rather than loosened.

**Banked, one item:
`work/lib/the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row.md`.**
`crates/pncad-py/src/py/` is 12621 lines behind `#[cfg(feature =
"python")]`; CI's clippy row runs default features and the
python-suite row runs no clippy, so none of it is linted on the merge
gate. Found because this unit ran clippy WITH the feature and hit one
standing `type_complexity` at `py/value.rs:319` — pre-existing on
main, and the only one, so the debt is one lint rather than a backlog.
Not fixed here: it touches the merge gate and a field this unit does
not bind.

**B-VALIDATE4 IN REVIEW (2026-09-03, #1677; mechanical under the 08-29
ruling, no A/B row). Census family B-VALIDATE4 CLOSED — the ladder's
fourth rung crosses, and the family's whole content is HOW its second
argument does.** `validate_pseudomanifold` becomes
`Body.validate_pseudomanifold`. 13 Python rows in the new
`tests/test_validate.py`, one Rust row in `src/tests.rs`, ty fixtures
both directions. The suite count is re-cut at the merge and stated
once: the lane measured 454 to 467, B-CANCEL landed its 19 in the
same window, and the merged tree runs 486. The DELTA is what this
unit owns and it is 13 either way — which is the reason the count is
given as a delta here rather than as a total two concurrent lanes can
each invalidate (#1662's lesson, one register entry later).

**The derived scope, and the audit-reach check.** One chartered name,
one `gap:` roster entry, and the #1662 check came back NEGATIVE, as
it did at #1668 and for the same reason: the door needs a `Body<f64>`
and a `ContactRecords`, Python has had `Body` since §L4, and no other
id owned an entry this unit had to build. One chartered, one moved.
Three families in a row now say the audit-reach check is a step
rather than a discovery — and this one says something the other two
could not, because its arguments did NOT both already cross and it
still cost nothing: the second one crossed by being CAPTURED.

**The measurement the family exists for.** `ContactRecords` has no
Python constructor and never will — it is minted by the ops that
certify geometry — so a `validate_pseudomanifold(contacts)` door
would be a door nobody can call, and one taking ANOTHER body's
records would spell exactly the mis-pairing F1 refuses ("the
validator never blesses discovered contacts"). So a `Body` carries
the declarations its own producer minted for it and the Python door
is a bare method like the three rungs below it. That is #1668's
carrier-projection rule (`fmt_length` landing on the `Length` it
needs) at a door whose Rust signature takes two things, and #1664's
pairing argument (`RunCtx` becoming `Evaluation`) at a door that
would otherwise let a caller ask one body about another's intent.
`ContactRecords` accordingly STAYS `INTERIOR` — same disposition,
second reason under it. The demo tour states the same rule from the
other side and was the brief's pointer to it: its `SceneBody` carries
`contacts` beside the body and runs 3′ "with the op's OWN declared
contacts".

**The capture reconciles the kernel's two homes, where the kernel
does.** `NodeValue::contacts` is `instantiate`'s carried D-1 set; a
boolean's records ride `BooleanValue::Body`; `product::sources_of` is
where they meet, and `Value.body`/`Value.bodies` now make the same
reconciliation so a body read off a value and the same body read by
the gather cannot disagree about what was declared over it.
`assemble` is the second source (D-1 plus the mates' minted D-2).
`product` is deliberately the third case: it gathers and declares
NOTHING. Empty is the honest default rather than a hole — with no
declarations the kernel's own contract is 3′ ≡ tier 3 plus the census
actually run, the STRICTEST rung — so a door that drops records can
only make this gate refuse, never falsely pass. That property is what
made the two filed findings safe to file rather than guess at.

**The oracle is two doors over one geometry, and nothing else would
have done.** A claim that records are captured is worth what a test
can show, and every single-door test passes whether the capture works
or not. The mated bench stand is the separating pair: `assemble(doc,
ev).body.validate_pseudomanifold()` PASSES over its two seats while
`product(doc, ev)` — same document, same evaluation, volume asserted
equal — reports 16 undeclared contacts. The mate-less `layout` is the
control that stops that reading as "assemble answers more kindly":
zero minted, both doors pass. Corpus loaded through
`test_assembly_eval.opened`, so the geometry is the tour's own scene
and the Rust side already asserts about it. Second oracle, on the
rung itself: two touching slabs gathered by `product` pass tiers 1, 2
and 3 and refuse at 3′ — the census IS what the fourth rung adds,
asserted rather than restated.

**Measured and NOT pinned, because there is no door to pin it with.**
A DECLARED glue — two slabs resting face to face, unioned through
`Node.boolean(declare=…)` — comes out with an EMPTY record set: the
union welds the declared faces, so no coincidence survives for a
record to back. Its 3′ pass is the empty-record case, not the
certified-seam case. No Python door reads a body's record count, so
the distinction cannot be asserted from Python at all; it is written
into the test file's header instead, because a reader would otherwise
take that row for evidence of the capture it does not demonstrate.

**Banked, and the sharper of the two: `work/lib/tier-3-prime-
findings-render-through-debug.md`.** Only tier 3′ runs the census, so
the other three rungs cannot reach its arms — and the kernel words
three of them out of `Debug` (`UndeclaredContact`'s `CensusContact`,
`StaleContactDeclaration`'s payload, and `census::witness`, which is
`format!("{p:?}")`). The first honest call of the new door PANICKED
inside this crate's own `reads_as_prose` assertion, on the ordinary
path: two touching solids gathered by `product`. The assertion's own
docstring predicted it — "what the check cannot see is a door no test
reaches" — and the kernel has already made this exact fix one arm
over (`validate.rs:1587`, the S6 sweep). Not fixed here: the
rendering is `crates/topo`'s, the `witness` is an opaque `String` no
consumer can re-derive, and re-wording at the boundary would fork a
diagnosis the kernel owns while DROPPING the coordinate that makes a
finding actionable. `run_validator` raises through a new
`typed_err_kernel_authored` instead — one caller, message is
`ValidationError::to_string()` by construction, the whole argument in
its doc comment — and the current text is pinned in BOTH directions,
`src/tests.rs` (no-interpreter row) and `test_validate.py`, so the
fix goes red rather than silent and takes the exemption with it.
Note what is NOT the complaint: `edge {edge:?}` reads that way in two
dozen tier-1/2/3 arms and crosses today. The fingerprint is the
STRUCT brace, not the arena key.

**Also banked: `work/lib/subject-body-drops-the-declared-
contacts.md`.** `editor_core::checks::subject_body` calls
`product::sources_of`, which returns `(ix, body, contacts)`, and
discards the third element one line later. So `pncad.subject_body`
answers with a plain body, and a subject that IS a declared boolean
result reports its own certified seam as undeclared where the same
body through `Value.body` passes. Filed rather than worked around,
for the reason the whole family turns on: a binding-side guess about
which records belong to a subject would be the invention F1 forbids,
and the failure is loud either way. The narrowing is stated at the
door.

**A false claim corrected in a file this unit touches**: `docs/
GUIDE.md`'s validator-ladder section documented tier 3′ as
`validate_pseudomanifold(body, contacts)` with a "3-versus-3′ choice"
paragraph addressed to the caller, and said nothing about Python —
which after this merge would read as a promise that the Python door
takes contacts and that the choice is the caller's. It now says all
four rungs are `Body` methods, that the fourth takes none, and which
door puts a body on which side of the choice.

**Not this unit's, and re-measured rather than assumed:** the
python-feature clippy lane is still red on `Datum.axes`
(`type_complexity`, `py/value.rs:395` — the same field at
B-RESOLVE's `:319`, shifted by this diff), exactly as
`work/lib/pncad-py-python-feature-clippy-lane-is-red.md` records, and
no CI row runs it. One error, unchanged, inherited.

**THE CENSUS B-FAMILY SLATE IS COMPLETE (2026-09-03).** All six
queued families closed in one day, each mechanical under the 08-29
ruling: B-PICKING (#1661), B-EXPR-READ (#1662), B-RESOLVE (#1664),
B-FORMAT (#1668), B-CANCEL (#1676), B-VALIDATE4 (#1677) — joining
B-READBACK and B-CHECKS from 08-29. The census's FAMILIES table is
now empty of open charters; what remains in the gap roster is G1's
authoring-signature residue (watched by execution, not by name) and
G2's sweep half (kernel-owned, U4/LQ3). The day's method notes,
earned: the derived-scope-first discipline caught one family split
on audit-reach rather than build-shape (#1662 moved nine for a
charter of three); the carrier-projection rule read out five more
times (PickTarget, RunCtx, the format receivers, the captured
ContactRecords); and the closures found three latent defects worth
the price of the sweep alone — the GIL held across evaluate (Python
cancellation could never fire), tier-3′ findings rendered through
Debug (the first honest call panicked), and the quantity boundary's
poison comparison raising untyped. Program standing after the
slate: shell (LIB-G17) parked on kernel #1202; a LIB-13 block draw
owed before any full-protocol unit; the banked-findings pile is the
remaining mechanical feedstock, headlined by the unlinted
python-feature CI row (no clippy row on the merge gate covers
12.6k lines of binding surface — routing decision pending).

**LIB-MECH1 MERGED (#1696, 2026-09-03), and the banked pile is now the
program's whole open surface.** One PR over seven banked issues, chosen
by one rule — done-state MACHINE-checked rather than judgment-checked —
with the twenty-one it did not take each given a reason in the unit
file, so the selection is checkable rather than asserted. Four
implementer lanes ran it, two reviewers (style, correctness) read it,
and the round is worth recording for what the reviewers found rather
than for what shipped.

**The headline is the CI row nobody had.** `crates/pncad-py/src/py/` is
~12.6k lines behind a non-default feature, and no clippy row in either
CI half compiled it — the merge gate's `clippy` runs at default
features, where every `#[pyclass]` is `#[cfg]`-ed away. The lane had
never been green because it had never been run. Routing was the open
question the two duplicate issues both stopped at; the answer is that
`python-suite` (hosted) and the nightly ungated re-take already install
an interpreter, already cache that feature graph, and are off the Rust
critical path, so the row costs the gate nothing. Measured 37.0 s cold.
The seed-key skip it inherits is stated at the step rather than
discovered later.

**Three claims this program wrote turned out false, and the reviewers
found all three.** The demo's roster still advertised the doubled
recourse as an open gap twenty lines above the site comment that had
just closed it. "The three armed pins" was two — a `println!` is not a
pin — in both the unit record and the issue's close. And the argument
for deleting the `PartResolver` arm rested on the recourse coupling
being unguarded, when `crates/viewer/tests/instance_authoring.rs`
guards it, in this workspace, with no interpreter; the reviewer proved
it by deleting the recourse from `Display` and watching that test red.
That last one is the instructive failure: the safety argument was not
wrong about the code, it was wrong about the tree, and it was written
with more confidence than the sweep behind it.

**The tree caught the fourth by itself, which is the better story.**
The tag-value guard's first draft hand-rolled a Rust reader —
comment stripper, string lexer, brace counter — and `reader_census`
reds on exactly that, by construction, because the population of such
readers "is not a list" and a new one arriving is what that row
detects. Converted to `test_utils::source`: structure in the
`code_only` view, literals read at the same offsets, `balanced_end` for
every bracket walk. The conversion is the adoption that crate's docs
ask of `pncad-py` BY NAME. It came out +51 lines rather than the ~120
fewer expected, and that is recorded rather than fixed by trimming
documentation to a number. It also turned up a latent bug in passing:
the guard resolved its crate through a bare `CARGO_MANIFEST_DIR`, which
a replayed nextest archive need not have — `crate_dir` handles both,
and the sibling manifest read went with it.

**Two lessons about our own green.** The local box's ruff is 0.15.8
where CI pins 0.16.1, so `check-python-lint.py` SKIPS locally by design
— four lanes and the orchestrator all read that skip as a pass, and CI
found the `RUF059` none of them could see. The pinned binary is
cheap to fetch and is now what this lane runs. And no lane ran
`cargo nextest run --workspace` before the first push, which is where
`reader_census` lived; both CI reds this round were findable locally
and neither was found. The rule the round earns: a bundle that touches
a guard runs the WHOLE suite before it pushes, and a linter that
reports "skipped" is not a linter that reported.

**Three findings banked rather than swept**, each with its own file
because a finding that dies with the issue that held it is a finding
nobody can pick up: `select-refusal-predicate-names-are-unpinned`
(four of five reachable predicate names pinned nowhere, and neither
carrying arm is constructible from `pncad-py`, so it is a K-name-space
job and not a rider); `two-refusals-carry-no-recourse-sentence`
(finding 2 of #947 — recourse prose authored into a kernel crate);
`datum-in-plane-reads-back-a-length-pair-bare` (the write door takes
`tuple[Length, Length]` where the read door answers bare floats —
found because this unit ADDED the stub entry that made it visible).

**Program standing after the merge.** Shell (LIB-G17) still parked on
kernel #1202; a LIB-13 block draw still owed before any full-protocol
unit. Twenty-four issues open, and the mechanical feedstock is now
spent: what remains is design questions, Ev rulings, kernel-crate prose
and multi-unit surfaces. The next LIB unit is a substantive one.

## Hand-off from DOCM (2026-09-04)

`no-door-mints-mate-frame-from-face` re-homed here by header-preserving
`git mv`: the frozen-at-authoring answer is ratified as the mate side's
(`docs/DOCM-REFERENCES-DESIGN.md` DM1, the asymmetry paragraph), the
viewer's mate tool already derives its frames that way, and what is
left is the headless door — a `Pose` into a `MateFrame` from the façade
— which is LIB's surface. Signed (DOCM orchestrator).

## LIB reactivated again (2026-09-06) — new orchestrator, remote host

**Session opening (Ev, in-chat).** Ev asked for a read of the LIB
track and whether it was worth orchestrating; the read said yes and
Ev said go. Host is the same shape as 08-29's: a remote container,
4 CPUs / 15G / ~29G free, full clones via `new-lane.sh`, heavy cargo
behind the build-slot mutex, hosted CI the verification of record,
no monitor scripts, GitHub through MCP. **Orchestrator branch is the
session's designated `claude/lib-work-track-review-06m46w`**, not
`lib/orchestrator` — the host pins the branch it may push, so the
`lib/` prefix convention holds for unit branches only this session
(the LIB-MECH1 precedent, `claude/lib-mechanical-clippy-ci-tadd42`).

**State as found, measured on main at 2de5a1f6.** No LIB PR open, no
`lib/orchestrator` on origin, main green, lint green. 25 open rows:
two queued mechanical census families (B-FACE-FRAME, B-PART, DOCM's
09-04 filings), LIB-G17 `parked` on a trigger that fired 09-04 (SEAT's
courtesy issue of 09-05; the int in `blocked_on` is why lint did not
red — filed on META's slate as
`parked-on-an-int-is-invisible-to-the-fired-trigger-rule`), and 22
issues. Three more census families are chartered in
`test_binding_census.py`'s `FAMILIES` with no row anywhere
(B-NOTATION, B-DISTRIBUTIONS, B-MEASURES; B-FACE-FRAME's file called
them "unscheduled alongside") — filed today as `LIB-B-NOTATION`,
`LIB-B-DISTRIBUTIONS`, `LIB-B-MEASURES`.

**Protocol standing.** v6 duals continue (Ev, 2026-09-06, recorded on
`work/meta/ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated`);
the 08-29 mechanical ruling stands; the next full-protocol unit needs
a LIB-13 block draw (LIB-12 closed 09-03).

**Orchestrator decisions (continuing the LB numbering):**

- **LB14 — the slate is worked in four lanes, in this order.**
  (1) Mechanical, under the 08-29 ruling: B-FACE-FRAME and B-PART
  staggered (census contention), then a LIB-MECH2 bundle of
  `lb13-guards-are-line-local` (fix fully specified by CIW, same-file
  scanner exists) and `pncad-py-doc-has-no-node-kind-read-door`
  (shape specified in the issue). (2) LIB-G17 on the full protocol
  after the LIB-13 draw — the design is ratified (D5), the kernel and
  seat enablers exist, nothing is Ev-paced. (3) A LIB-CUR5 curation
  unit over the five payload rows that share one question
  (`escalation-payload…`, `loop-key…`, `next-payload-rung…`,
  `mesh-pick-error…`, `resolution-failure-arms…`), the carriage
  decided and logged by the orchestrator first. (4) The four real
  design forks go to Ev as FOUR `[ev]` PRs, one per item, each
  self-contained (Ev's ask: separate PRs when unrelated, full
  context in the description): `save-a-copy-duplicate-id-bricks-store`
  (identity fork on save), `facade-polygon-door-demoted-without-replacement`
  (build the lattice door or say no at the site),
  `bench-corpus-staleness-hole` (a mechanism or a ratified narrower
  claim), `python-check-and-assembly-doors-gather-twice` (what a
  Python `Product` is). None gates (1)–(3).
- **LB15 — two rows are hand-offs, not dispatches.**
  `load-path-stringifies-structured-refusals` is a fix in
  `crates/editor-core/src/persist/` and `two-refusals-carry-no-recourse-sentence`
  is prose in `crates/editor-core/src/mate.rs`; neither path is in
  LIB's `paths:`. Both stay on LIB's slate as the party that wants
  them and get routed to DOCM at the next natural seam rather than
  dispatched across the fence.

**LIB-G17 SPEC WRITTEN AND BLOCK LIB-13 DRAWN (2026-09-06).**
`docs/LIB-G17-SPEC.md` binds the unit as D5's elaboration. Three
decisions the spec makes that D5 did not spell out, each argued at the
clause: `open` is ORDERED (the kernel's `RimNaming::rim` is
`sources[0]`, so sorting would move the rim's identity — the one place
the blend precedent does not transfer); the refusal crosses as
`NodeErrorKind::Shell(Box<ShellError<f64>>)` by a total fold, since
`NodeErrorKind` is scalar-free and `ShellError<T>` is the first generic
kernel refusal to reach the document layer (`verb_refused`'s own comment
anticipated this arm); and three additive roles (`Inner`, `Rim`,
`HoleRim`) translate `ShellNaming`'s rows one-to-one. LB16 — those are
faithful elaborations of a ratified record shape, self-merged with the
writeup and offered for Ev's retroactive read in the unit's PR, not an
`[ev]` fork. The teapot's conversion is fenced out (render lane,
tess-budget rows) and the spec's `vessel.rs` corpus document is the
document-door spelling of the same mouth. Sequencing deviation stated
rather than hidden: the draw byte was read in the same tool call as
the last spec read, minutes before the difficulty sentence was written
down; the difficulty is the G16 precedent's and was not moved.

**B-FACE-FRAME MERGED (2026-09-08, #2074; mechanical under the 08-29
ruling, no A/B row). Census family B-FACE-FRAME CLOSED — sketch on a
face, from Python.** `Node.datum_face_frame(at, face, spin)` (the
`datum_*` prefix convention, spin a typed `Angle` with no default),
`Evaluation.face_carrier_kind` (the fifth read-back door and the first
that is not a frame; `SurfaceKind` crosses OUT for the first time, and
the crossing retires the tripwire's dead twin), `Pose.sense`. 26 tests,
every number an oracle against the read door's pose. The unit's real
yield is a census blind spot measured at the closure: a three-door
charter had a one-row roster because `Pose.sense` is a FIELD of a
rule-1-accounted type and `Datum` matches name-for-name across two
different types (the authoring enum and the read-side value), so a
whole authoring arm hid behind the census's own rule. Banked with
files: `datum-crosses-name-for-name-as-two-types`,
`pncad-py-comparable-enums-do-not-hash` (23 of 23). Fixed in passing:
`run-python-tests.sh` read a hardcoded `$root/target`, so every lane
that set `CARGO_TARGET_DIR` as the discipline requires saw it exit 1 on
a successful build. Outside the fence, reported: `local-scripts/
bt-testbin.sh` and `bt-symbols.sh` carry the same defect. Relaunched
lane (the 07:29 kill); the dead tree's derived scope was read and
corrected, not applied. Orchestrator merged main into the branch after
the report (415 commits, clean) and landed on the re-run.
**LIB-MECH2 MERGED (2026-09-08, #2072; mechanical under the 08-29
ruling, no A/B row).** Two banked findings closed. (1) Both LB13
boundary guards in `crates/pncad/tests/all.rs` read `pub use`
STATEMENTS now — accumulated to the `;`, whitespace collapsed, the line
reported being the one the statement opens on — and the RawLoop guard's
minting patterns match a whitespace-squashed view of the whole file with
offsets mapped back to true lines. Planted red both ways: a key added
inside an existing multi-line brace list reds the new guard by name and
PASSES the merge-base guard. Numbers re-derived at the merge base (75
statements, 33 multi-line, 17 into `editor_core::`; the root's `pub mod`
count is 33 and the doc now carries no number). The inherited commit
from the killed lane had a defect of its own — a doc block edited in
place on the wrong function — repaired. Three remaining line-local
readers filed as `facade-guard-file-keeps-two-line-local-readers` (its
id says two; ids are stable). (2) `Doc.node_kind(node) -> str` from one
exhaustive match with no wildcard (proved by deleting an arm), the
vocabulary pinned whole on `TAG_INVENTORY`'s discipline, snake_case and
deliberately NOT the wire's variant identifiers; an unknown id refuses
`unknown_node`. The die-tool Python row now asserts the group by kind,
mirroring the Rust row, instead of leaning on the byte pin. One red
owned and fixed: an intra-doc link to a `cfg(test)`-gated module. Local
`nextest --workspace` 6378 passed before the first push — the MECH1
lesson, applied. Orchestrator merged main after the report (436
commits, clean) and landed on the re-run.
**LB17 — the CUR5 carriage, decided (2026-09-06).** One rule settles
all five rows, and it is the one the CUR3/CUR4 pair already wrote at
the census's `BlendError` entry: **a payload's category follows what
its carrier does at the crossing.** Applied:
- `MeshPickError` under `NodePickError` (carrier projects tags): CARRY
  `MeshPickError` alone into `crates/pncad/src/select.rs`, `MeshPick`
  stays interior, `mesh_pick_error_tag` with `position_out_of_range`.
  CUR3's construction argument is untouched.
- `ResolveError` / `ResolutionFailure` / `ResolveIndeterminate` under
  `Resolution` (carrier projects `resolution_status_tag`): CARRY the
  three beside `Resolution`, WITHOUT `Diagnosis`, `Tombstone`,
  `TieWitness`, `RecipeEditRef`, `Resolved` (the key-bearing and
  telemetry half the stanza is really about); `resolve_error_tag`,
  `resolve_indeterminate_tag`, and `variant` on `Resolution`'s arms.
  The stanza's own argument — "a door carried for a consumer that does
  not exist" — no longer holds: the consumer is Python, which holds a
  string and nothing else. A strictly smaller carriage than GUI-2's.
- `Indeterminate` (thirteen prelude carriers): CARRY on the prelude
  (contract clause 1 is met already at `pncad::geom_core::Indeterminate`;
  the curated-list half is what is owed). `MarginDiag`: MEASURE first —
  carry iff some curated refusal's Python projection exposes its
  discriminant or the unit adds one; else INTERIOR with the reason
  written at the entry, the `BandField` precedent.
- `LoopKey`: CARRY beside `VertexKey`/`EdgeKey`/`FaceKey` in the
  prelude's group 4 (the same `topo` re-export); record blind spot (e)
  — macro-minted types are invisible to a declaration-level index —
  where CUR3's (a)–(d) are recorded.
- `EntityId` / `GeomRef` / `ContactFinding`: CARRY through the groups
  their siblings already sit in (`topo` root beside the keys; the
  contact vocabulary's missing quarter through `crate::select`). The
  LB13 guard names `EntityRef`/`EntityKey`/`Entry` — editor-core's
  document-layer keys — and the unit READS the guard before touching
  anything: if `EntityId` is a document-layer key rather than `topo`
  vocabulary, that row STOPS and reports.
Mechanical under the 08-29 ruling (the CUR/CUR2/CUR3/CUR4 precedent).
**Held until Wave 1 lands**: CUR5 edits the census, `tags.rs`,
`pncad.pyi` and `all.rs`, the exact files B-FACE-FRAME and MECH2 are
in, and a fourth build target on this box is past the disk budget.

**RULED — save-a-copy is two acts (Ev, PR 2016, 2026-09-06: "A sounds
good!").** Recorded on the item and as one sentence at ASSEMBLY-DESIGN
A4 (`crates/editor-core/ASSEMBLY.md`). The library half — the save-door
refusal when the directory already holds the id under another filename,
and the fork act minting a fresh id — is a LIB unit, mechanical under
the 08-29 ruling (the shape is fully specified by the ruling and the
store's own `DuplicateId`); the viewer `SessionOp` spelling is a rider
handed to the GUI programs after it lands. Queued behind Wave 1 for the
box, not for any decision.

**RULED — the façade polygon door gets built (Ev, PR 2017, 2026-09-06:
"A is good here too!").** A LIB unit, mechanical under the 08-29 ruling:
the fallible lattice-backed `pncad::authoring::polygon`, one `PathError`
arm for the sub-three-vertex case (with its Python tag and inventory
row), the tour's `path_polygon` helper deleted and its thirteen call
sites moved onto the door (`demos/tour` is a render-lane touch; frames
should not move since the spelling is the same lattice — a moved frame
is a finding, not a re-baseline). Queued behind Wave 1 for the box.

**RULED — a Python `Product` is a plain value the doors clone (Ev, PR
2020, 2026-09-06: "plain value doors clone sounds good"), with the
combined door as the measured fallback.** Tradeoffs stated on the
thread. A LIB unit, mechanical under the 08-29 ruling, whose first act
is the clone-cost measurement. Queued behind Wave 1 for the box.

**Correction owed on the bench-corpus question (PR 2019).** Ev asked
why the corpus bytes are committed, and the honest answer exposed a
stale premise in my own `[ev]` PR: the corpus exists because Python
could not author assembly nodes at LIB-G18a, and G18B closed that the
same day — `test_assembly_author.py` already authors the whole scene
from nothing into a temp store. The header of `test_assembly_eval.py`
still says "Python cannot AUTHOR an instantiate node", a false live
claim. Recommendation revised on the thread to (E): delete the
committed bytes and build the eval test's store from the authored
scene, which closes the hole rather than narrowing it. Lesson for the
orchestrator: an `[ev]` question is a claim about the tree and owes
the same re-measurement as any other before it is asked.

**RULED — the bench corpus's committed bytes go (Ev, PR 2019,
2026-09-06: "E is great!").** A LIB unit, mechanical under the 08-29
ruling: `test_assembly_eval.py` builds its store from the Python-authored
scene, the four `.pncad` files and the MANIFEST are deleted, the false
header claim with them. All four `[ev]` questions of the day are now
ruled; the four units they produce (SAVEFORK, POLYGON, CORPUS, PRODUCT)
queue behind Wave 1 for the box, each brief-as-spec off its item's
"Ruled" section.

**RULING REVISED — a Python `Product` is a memo on `Evaluation`, not a
value (Ev, in chat, 2026-09-06: "sounds great!").** Ev asked on PR 2020
whether an option with the good qualities of both the combined door and
the clone-value door exists; it does: the gathered product memoized on
the immutable Python `Evaluation`, keyed by tolerance — no new surface,
no staleness, every consumer served. Recorded on the item as (5); the
LIB-PRODUCT unit runs on it with the clone measurement kept.

**Incident — three lanes killed by a session interruption (2026-09-06,
~07:29Z).** B-FACE-FRAME, MECH2 and G17 all died within a minute of
each other; found at the 08:16 check-in (no processes, a stale
`slot-1.holder`, no reports). The harness refused to resume them and Ev
authorised a relaunch (in chat, 09:20). Fresh lanes from main on the
same briefs, the dead trees moved aside as UNTRUSTED reference
material (a derived-scope patch and an uncommitted diff for
B-FACE-FRAME, an uncommitted diff for MECH2); MECH2's one pushed
commit is inherited by its fresh lane. G17 had not built anything, so
its A/B row will carry no interruption annotation — the arm and slot
are unchanged and the lane started from nothing both times. Lesson for
the orchestrator: a lane that has not pushed in thirty minutes is
suspect regardless of the notification channel; the hourly check-in
now reads the lock holders and process table, not just the branches.

**LIB-G17 MERGED (2026-09-08, #2150 — ordinal 303, sample #158, row in
MODEL-AB-LOG; block LIB-13 slot 1 consumed). RECIPE-DOORS is COMPLETE:
chamfer, tube and shell all have their recipe doors.** `Node::Shell`
through the verb seat, named from `ShellNaming` under three additive
roles, spelled in Python, proved on an opened box with exact closed
forms and on the teapot's own mouth. Three decisions the spec made
survived the dual: `open` ordered (both reviewers executed the
order-only-moves-the-rim claim and it held to the bit), the refusal
folded to f64 (reshaped onto the lane by the dual's convergent
finding), the three roles. The teapot's conversion, the two-half
mouth designation and the registry hold-out are on the slate as
their own files. Standing after the merge: Wave 1 mechanical
(B-FACE-FRAME, MECH2) landed; B-PART in CI; CUR5 and the four ruled
units (SAVEFORK, POLYGON, CORPUS, PRODUCT) queued on the box's disk;
LIB-13 has three OPUS slots open for the next full-protocol units.
**LIB-G17 DUAL ADJUDICATED (2026-09-08, PR 2150, ordinal 303, frozen
`4a093c5c`; R1 opus, R2 fable per byte 218).** R1 APPROVE-WITH-FIXES
0/6/8 (+9 style), rubric 4/4/3; R2 APPROVE-WITH-FIXES 0/5/6 (+style),
rubric 4/3/3. **Convergent, and the headline is bilateral**: the fold
of `ShellError<T>` to its f64 witness takes `lo()` on every field with
one argument (the thickness gate's) covering one field, and NO row
distinguishes the bracket ends — R1's `lo→hi` mutant went green
against the whole suite; and the `T: Bounds` widening on the three
scalar-free refusal doors (with the `bounds-allowlist.sh` 15→16 bump
and seven `Payload` roster rows outside LIB's fence) was the one
disclosed deviation with no schedule, whose alternative — the witness
declared per lane on `ShellLane` — neither the spec nor the PR weighed.
**Ruled**: the fold moves onto `ShellLane` with a per-field argument at
each impl and a non-degenerate-bracket pin; the gate count and the
roster rows revert. Also ruled: the repeat-`open` check moves to
`Node::input_fault` so the insert door and the load door refuse alike
(R2's probe: the public variant passes insert and `save` refuses — the
blends' identical asymmetry filed as its own issue); the order row
asserts both orders' rims (R2's `sort_unstable` mutant stayed green
against the row named for the claim); the `(slot, param)` join and
`feed_*` are shared rather than copied, the lane-name ladder written
once, and the whole-correspondence generic filed for the third
instance; `attach_shell`'s comment made true; every prose site the PR
falsified fixed (four payload-name enumerations, the "wrapped
UNALTERED" header, the 41-variant census floor, the verbs crate's
"no shell node" premises, the audit cell rewritten, the naming README's
N4 table). Adopted rows: R1's order-swap set-difference, rebind, thick
wall, thickness-only key/memo; R2's P1/P2/P3/P4/P6 and **P7, the only
row anywhere for `HoleRim`**. Zero unilateral MAJORs, so the v6 tally
does not move. **Pair FLAGGED (v6 item 5)**: R1 disclosed an accidental
glimpse of R2's command line through a process listing — three mutant
literals and one probe name, all after R1's own findings were written;
disclosed in full, nothing used. Item 3(e) applied conservatively: the
pair is recorded and excluded from the tally (which it could not have
moved). Both reviewers' isolation otherwise clean. Method lesson for
the next dual on a shared box: `pgrep`/`ps` over the box is a glimpse
channel; the brief should say "never list processes you do not own."

**LIB-CUR5 MERGED (2026-09-08, #2169; mechanical under the 08-29
ruling, no A/B row). The five banked payload rows closed under LB17's
carrier rule; nine names curated, `MarginDiag` measured interior.**
Row 1: `MeshPickError` carried on `crate::select` with `MeshPick` still
interior (an index is BUILT, a refusal RECEIVED — CUR3's construction
argument untouched); the tag does not forward: `variant` stays
`mesh_index` and the payload's discriminant arrives beside it at
`NodePickError.index_variant`, one arm matched exhaustively so a second
indexing invariant breaks the build. Row 2: `ResolveError`,
`ResolutionFailure`, `ResolveIndeterminate` carried beside `Resolution`;
`all.rs`'s "a door carried for a consumer that does not exist" answered
where it was made (a `Display`-rendering Rust panel was one consumer;
a Python caller holding a string is the other); Python's `Resolution`
gains `variant` — ONE attribute for both vocabularies, `status` already
saying which it is drawn from, `None` on resolved. Five of six arms
pinned from Python, `ambiguous` reached by no test on either side and
both docstrings say so (an N2 tie has no authoring door). Row 3:
`Indeterminate` on the prelude (thirteen prelude refusals carry it);
`MarginDiag` NOT carried, measured: `pncad-py` names neither type
anywhere, every escalation crosses as one tag plus prose, and the
type's own doc forbids branching on the margin — `BandField`'s
disposition with a different reason, written beside the carriage with
its falsifier (a door that projects the escalation's shape). Row 4:
`LoopKey` joins its three siblings (`RingMeetsOuter` names all four;
pinned by signature). Row 5: `EntityId`, `GeomRef` (group 4, the sums
over the keys) and `ContactFinding` (`crate::select`, the contact
vocabulary's fourth quarter); the arena-key stop clause did NOT fire —
the sealed names are `editor-core`'s `EntityRef`/`EntityKey`/`Entry`,
these are `topo::entity`'s, and the distinction is now written in the
prelude group. `NOT_CARRIED` 94→90. Re-sweep at the merge base: 22
enum-shaped hits, five new and filed
(`payload-rung-re-sweep-finds-five-more-uncurated-discriminants`:
`CensusSubject`, `MappedCurve`, `RevolvedKind`, `PromotedKind`,
`ImportContact`), blind spot (f) new — a private field of a curated
struct reads as a payload. Residue filed:
`mesh-index-numbers-cross-as-prose-under-a-projecting-door`. No kernel
crate touched. Orchestrator note: main gained SHELL-5 (`topo/src/shell.rs`
and the shell tracker) between the green run on `7239cf48` and the
merge; no file overlaps this diff and nothing it touches reads shell
geometry, so it landed on that run rather than chasing main a third
time.

**LIB-B-PART MERGED (2026-09-08, #2163; mechanical under the 08-29
ruling, no A/B row). Census family B-PART CLOSED — one body out of a
multi-body value, from Python.** `PartSelect.split_half` /
`PartSelect.instance` (a frozen constructor pair, `PatternKind`'s
shape, so the one roster row leaves under rule 1 rather than moving to
`BOUND_AS`), `Node.part(of, select)`, `DocEdit.bind_instance_param`
(`SlotId::Instance`'s own door beside the count's — a door per slot
rather than a `slot=` argument, keeping the slot vocabulary off the
crossing), and `Node.pattern`, which the charter never named. 20
tests, every number an oracle against the split's or the pattern's own
value; both chartered refusals (`empty_half`, `instance_out_of_range`)
reached from Python for the first time; a half is not an instance at
the type level (ty fixtures both ways). **LB18 — `Node.pattern` is
bound outside the charter, and the orchestrator accepts it as a
faithful consequence of DM3 rather than a widening.** The sweep found,
before any code moved, that `PartSelect::Instance` selects out of a
`ValuePayload::Instances` and exactly one node emits one
(`Node::Pattern`), deliberately unbound under G8's reason that a
plural payload fed no downstream door. `Node::Part` IS that door, so
binding the projection without its only source would have shipped an
unreachable half and an unconstructible refusal tag — which the census
could not have reported, both names being arms behind `Node`. G8's
conclusion is untouched: no audit row flips, no tour scene changes,
`PlacedUnion` still authors the heat sink and a boolean still refuses a
plural payload (`test_a_plural_payload_cannot_feed_a_boolean` now
executes all three states). What moved is the rationale, recorded in
the item file, the census closure paragraph and the G8/G18 cells; row
43's star is now a JOB (`bench-flat-pack-star-is-now-a-pattern-job`)
rather than a gap. Also banked: `structural-slots-without-a-binding-door`
(`VDegree`, `Stations`). The census lesson lands a third time: an
enum VARIANT behind a rule-1 name is as invisible as a field. Lane
merged main twice (G17, then EVAL-2) under instruction, resolving the
`pncad.pyi` absent-doors paragraph and the north-star rosters keeping
both sides; landed on the re-run. Orchestrator note: main gained EVAL-5
(`editor-core` verbs and eval), the tess tools and tour files, and a
tracker regrouping between the green run on `4b33617e` and the merge;
no file overlaps this diff, so it landed on that run rather than
chasing main a fourth time.

**LIB-PRODUCT MERGED (2026-09-08, #2181; mechanical under the 08-29
ruling, no A/B row). The gathered product memoized on the Python
`Evaluation` — Ev's option (5), the memo keyed by tolerance.** Measured
first, as the brief required: at the heat sink's 160-fin point (161
solids / 991 faces) a whole-`Product` clone is ~7.5 ms against a
~372 ms gather — 2%, an order of magnitude inside the "under a tenth"
line — so the memo keeps the product and hands `assemble_gathered` a
COPY; the take was not needed, no DOCM hand-off, no kernel file
touched. Shape: `crates/pncad-py/src/product_memo.rs`, Python-
independent so the default build tests it; a `OnceLock` on
`py::value::Evaluation` (the `NodePick` mesh-handle precedent, no new
locking discipline) holding the product and the tolerance it was
gathered at; a refusing gather is not memoized; four doors joined
(`run_checks`, `assemble`, `product`, `product_named`), signatures
unchanged, no new Python surface. Two decisions worth the record: the
memo gathers from the document the evaluation CAPTURED, not the `doc`
argument (a `Doc` is mutable and its id survives edits — the one
behaviour change, disclosed: an old evaluation asked about an edited
document now answers about the pair it is of, not a hybrid), and the
DI3 pairing is asked BEFORE the memo is consulted at every door, each
wrapping `Mispaired` in its own refusal under the unchanged tag, pinned
from Python in the order that would catch a check sited behind the
memo. Gather count is not a Python observable, so the brief's fallback
shipped: eight Rust pins on the default build path reading
`gathers_on_this_thread` as a difference (1 in either order, 1 across
all four doors, 0 after the consuming gate, 0 for a subject-free
config, 2 for two refusing asks, 2 for a different tolerance through
the keyed seam, and the mispaired refusal), plus nine Python rows for
everything a caller can see. One small deviation from the brief's
"one entry (the last)": the slot holds the FIRST tolerance's product
and answers a different tolerance by gathering without replacing — 
unreachable while a process commits one tolerance (`Tol` is the
witness), and answered rather than assumed away. Census: the six
`behind-a-door` entries carry the true reason (the memo calls them);
`gathers_on_this_thread` stays `INTERIOR` with the argument that a
debug-only counter is not a public door. Findings outside the fence,
reported not filed: `Subject`'s derivation in `editor-core`'s
`run_checks` has to be re-spelled by any caller holding a product, and
`Subject::not_needed` is private (DOCM's). Lane was cut off by an API
session limit mid-unit and resumed in place once it reset.
**LIB-CORPUS MERGED (2026-09-08, #2182; mechanical under the 08-29
ruling, no A/B row). The bench corpus is authored, not committed —
Ev's ruling (E), PR 2019.** `crates/pncad-py/tests/corpus/bench/` (four
`.pncad` documents and the `MANIFEST`) deleted, and with it the whole
staleness surface; `crates/pncad-py/tests/bench_scene.py` is the ONE
Python definition of the tour's bench (six constants, three derived
seats, two part shapes, the flat-pack layout, the mated stand), which
`test_assembly_author.py` authors from as before and
`test_assembly_eval.py` now writes into a temp `Workspace` and resolves
back out through a `DocRef` on every call, so the load path is
exercised over documents nothing keeps on disk. The header's false
claim that Python could not author an instantiate node is gone with
the bytes. The tour guard stays and widens: `TestTheSceneIsTheToursOwn`
reads `demos/tour/src/assembly.rs`'s source and compares the six base
constants by value, the three seats BY FORMULA (parsed and computed, so
a changed derivation reds though the bases did not move), the flat-
pack's placement literals and the stand's gauge offset and mate seats,
each arm mutation-proven red (ten mutations of the tour, ten reds); what it cannot see (structure, the two
deliberate Python/tour differences — literal prisms for parametric
ones, `placed_union` for `Node::Pattern` — anything outside the
constant block and the two authoring fns, and a rename or reformat,
which reds as a false alarm) is stated in the test's header rather than
banked. Every oracle the eval test carried is still asserted; the
placement row was re-cut as a family outline plus one cap frame per
placement, and three rows that moved the shelf's pin through a
parameter edit now re-author the part under the same label, since the
Python-authored parts hold no parameters. `demo-tour asm-corpus` and
`assembly::corpus` RETIRED (its only consumer was the deleted corpus;
`gallery` remains the door that saves documents), the render lanes
reporting no moved frame. `ci.yml`'s die-corpus note re-pointed. Now
that `Node.pattern` is bound (B-PART), authoring the posts as a pattern
is the separately filed job
(`bench-flat-pack-star-is-now-a-pattern-job`), not this unit's. Orchestrator
note: main gained only LIB-PRODUCT between the green run on `dfb5e776`
and the merge, no file overlapping this diff, so it landed on that run.

**LIB-POLYGON MERGED (2026-09-08, #2183; mechanical under the 08-29
ruling, no A/B row). The lattice-backed façade polygon door — Ev's
ruling (A), PR 2017.** `pncad::authoring::polygon(&[(f64, f64)], tol)
-> Result<ProfileLoop<T>, PathError<T>>` (`T: Decide`, the bound the
lattice's junction decisions need), curated into the prelude, spelled
exactly as the tour's helper spelled it — `Open.at(p0)`, a `line_to`
per vertex, `line_to(Start)` as the seam — so a within-band-tangent or
cusped corner refuses AT AUTHORING and the emitted loop is the raw
vertex table (bulge 0, no declared joints; pinned position-for-
position and against the hand-spelled chain, since `RawLoop` is off
the façade and `ProfileLoop` has no `PartialEq` — the brief's `==`
against the raw constructor was unwritable at that seat, disclosed).
Two ruled cross-fence touches, both the only edits in their trees:
`crates/profile` gains ONE arm, `PathError::PolygonTooFewVertices {
given }` with its `PathErrorKind` mirror, `kind()` row and `Display`
recourse, crossing as `polygon_too_few_vertices` with its
`TAG_INVENTORY` row and a construction pin through the door; and
`demos/tour/src/paths.rs` is deleted with its eleven call sites moved
onto the door (the ruling's thirteen counted references, not
invocations — the true count is recorded on the item), no scene
changed, all three render lanes green with no re-baseline. The
authoring-seam roster guard in `all.rs` names `polygon` beside
`validated` as the two fallible seams. Census: `polygon` is a curated
name now and rule 1 does not account it (`pncad.pyi` spells it only as
`Node.polygon`), so `BOUND_AS` maps it there with the reason; no roster
row moved. `docs/guide/examples.md` lost the helper's row and a false
sentence beside it (the tour's `Cargo.toml` names no kernel crate but
`pncad`). Not taken, on the lane's argument and the orchestrator's
agreement: `docs/PATHS-DESIGN.md` §3's refusal register enumerates the
lattice VERBS' refusals and no verb produces this arm — PATHS's page,
not LIB's, if that register is ever meant to list every `PathError`
arm. Filed: `tag-inventory-prose-counts-are-stale` (the tag-table doc
comment narrates 37 functions / 361 literals against a measured
42 / 402; the unit corrected only the count its own change moves).
Orchestrator note: main gained the gate scripts and TESS-BUDGET
between the green run on `c45c6785` and the merge, no file overlapping
this diff, so it landed on that run.

**LIB-SAVEFORK MERGED (2026-09-08, #2184; mechanical under the 08-29
ruling, no A/B row). A save is two acts — Ev's ruling (A), PR 2016,
recorded at ASSEMBLY.md A4.** Two doors on `Workspace`
(`crates/pncad/src/workspace.rs`), bound on Python's `Workspace` under
the same names: `save_at(doc, target, tol)` keeps the identity and
reads the scan BEFORE any write — the id claimed at a different path
refuses typed, claimed at `target` it is a resave, unclaimed it is a
create at the caller's name (which `create` cannot spell, forcing
`{id}.pncad`); `save_as_new_document(doc, tol) -> (DocumentId,
PathBuf)` mints a fresh random id and writes `{newid}.pncad` through
`create`'s validator, the original untouched so every inbound `DocRef`
pinning the old id still resolves to it. The arm decision, argued
both ways and settled on a NEW arm: `WorkspaceError::
SaveWouldDuplicateId { id, existing, requested }` rather than
`DuplicateId` reused, because the recourse differs (the scan's
duplicate is two files that exist, fixed by deleting one; the save
door's is a write that has not happened, fixed by choosing an act);
a second new arm `SaveTargetNotInStore { path }` refuses a target that
is not a `*.pncad` file directly in the store root, since a different
root is a different store. Both matched exhaustively at
`resolve_fault`, `workspace_err` and the tag map; Python tags
`save_would_duplicate_id` / `save_target_not_in_store` with inventory
rows and construction pins, the two paths riding `first`/`second` so
handling reads any duplicate-id refusal without branching on the door.
The pin question answered: the fork's content pin EQUALS the
original's, because `canonical_bytes` removes the `id` key (A4's own
sentence), and the two save files differ in the `id:` header and the
snapshot's id — pinned in both languages, with the fork shown to
differ from the original in identity and nothing else. Five facts
pinned on each side (refuses before the file exists and the store
still opens; resave in place; create at a chosen name; the fork's
identity, resolution and coexistence; equal pins). **One announced
fence crossing**: `crates/editor-core/src/doc.rs` gains
`Doc::under_identity(self, id) -> Self` — the fork constructor, one
field, no minting, DOCM's file — because `Doc::id` is crate-private,
no `DocEdit` moves it, and the alternative was a serde round-trip in
the façade; the CHROME `member_of` shape, disclosed in the PR and the
unit file, and DOCM may re-home it. Hand-off filed on VIEW
(`work/view/session-save-is-two-acts.md`): `SessionOp::Save` writes
around the store through `docio::save_path`, the tree's only
production write that does, so the ruled refusal cannot fire from the
viewer until that routes through `save_at`, and a second op spells the
fork. Guide and audit sentences that said "`create` and `resave` are
the two write doors" corrected. The lane installed the pinned ruff and
ran `check-python-lint.py` for real. Orchestrator note: main gained one
SHELL commit (`shell/ab-claims-2303`) between the green run on `1ec85d61`
and the merge, overlapping nothing in this diff, so it landed on that
run.

**THE RULED QUEUE IS EMPTY (2026-09-08, LIB orchestrator).** LB14's four
lanes have all landed: wave 1 mechanical (B-FACE-FRAME, MECH2), LIB-G17
on the full protocol, LIB-CUR5, and the four `[ev]` forks ruled and
built (SAVEFORK, POLYGON, CORPUS, PRODUCT), plus B-PART. Nine LIB
merges in one day, every one on a green head with the state-sync
commit last. Two method notes worth keeping: a green head whose only
drift from main is non-overlapping files lands without another chase,
with the drift named in the entry (four of today's merges did — the
alternative was a ~20-minute CI cycle per tracker commit on a main
that moved every twenty minutes); and a lane's target and clone are
reclaimed only AFTER the merge call succeeds (CORPUS's clone was
removed a step early and had to be re-cloned to resolve a tail-of-log
conflict — kept both entries in landing order).

- **LB19 — wave 3 is the three chartered census families, then the
  curation residue.** Mechanical under the 08-29 ruling, the B-PART
  shape (derived scope stated before code; the census's field- and
  variant-level blind spots measured at each closure): B-NOTATION and
  B-DISTRIBUTIONS staggered (they share the census, the stub, the tag
  files and the parameter doors), B-MEASURES after B-DISTRIBUTIONS as
  its own file sequences it. B-DISTRIBUTIONS establishes the
  `interval` gate on the analysis read doors FIRST and binds only what
  the default build compiles, reporting the rest as the measured
  limit. Then a LIB-CUR6 over `payload-rung-re-sweep-finds-five-more-
  uncurated-discriminants` (the CUR3 test — is it a discriminant a
  caller branches on — applied to each of the five, `ImportContact`
  judged as a reach defect instead), and the bench pattern job
  (`bench-flat-pack-star-is-now-a-pattern-job`, a Python row that
  MEASURES whether a plural payload answers the mates and the A5 gate
  the same way). The next full-protocol candidate is the teapot
  conversion (`teapot-scene-through-node-shell`: render lane, tess-
  budget rows, three audit rows flip on a Python row) — it needs a spec
  and LIB-13 slot 2 (OPUS by the block's arithmetic) before dispatch.
  LB15's two DOCM hand-offs and the tag-projection issues
  (`lib-per-arm-error-tags`, `pncad-py-seven-doors-lack-field-projection`,
  `census-findings-cross-without-a-per-arm-tag`) stay queued behind
  them: the last three are one design question about per-arm
  projection and go to Ev as one `[ev]` PR when the wave clears.

**LIB-B-NOTATION MERGED (2026-09-08, #2189; mechanical under the 08-29
ruling, no A/B row). Census family B-NOTATION CLOSED — a parameter
authored in millimetres remembers the millimetres.** Measured first,
with the bytes: at the merge base `DocParam.length(25 * mm)` saved
`"display_unit": "m"`, the canonical row, because a Python `Length`
wraps the arithmetic type and erases at the multiply — and it cannot
be taught the unit, since `quantity::written` deliberately defines no
arithmetic on an authored value (no notation for the sum of a
millimetre and an inch). So the notation crosses as the SECOND type,
the pair the census's two rows always named: `WrittenLength` /
`WrittenAngle` bound at their own spelling (`in_unit` multiplies and
remembers, `canonical_in` records a notation for a value already
computed, `length`/`angle`/`meters`/`radians`/`unit`, equality on both
halves, no arithmetic), `DocParam.written_length` / `written_angle`
(total — a mis-dimensioned written value is UNREPRESENTABLE at the
type, so the row is a ty fixture, and the persist walk's `display_unit`
refusal is pinned as the one reachable from Python), and
`DocParam.unit` answering the symbol (`""` for the dimensionless row,
`None` for a Count). Both roster rows leave under rule 1; the charter
leaves `FAMILIES`. The unit's real yield is the third census lesson:
this was the first family whose roster was honest about its own
entries, and closing it still required two doors the roster could not
see — `Doc.params`, a METHOD of a rule-1 type (Python had NO door
answering a parameter back, so the memory would have been observable
only by parsing saved JSON), and `LengthUnit.__eq__`/`__hash__` (with
`AngleUnit`'s), a MISSING DUNDER invisible to both rosters because
`test_stubs.py` checks only stub-declared operators (`mm == mm` held by
identity alone). Not taken, argued: `Expr::written_length` needs no
door because `Doc.parse_expr("25 mm")` already records the notation and
`Expr.text` reads it back; `DocParamValue` gets no written door because
the notation rides with the declaration (pinned both ways). 30 rows
reading the saved bytes wherever the claim is about what a document
records. Filed: `node-slot-literals-erase-the-authored-notation` (52
node-slot literal sites in 18 doors record the canonical row — the same
erasure one vocabulary over, and the typed doors disagree with the text
door about one authoring). The lane ran the pinned ruff for real.
Orchestrator merged main (SAVEFORK, overlapping `pncad.pyi` and the
legal fixture — additive hunks, clean merge) and landed on the re-run.

**LIB-B-DISTRIBUTIONS MERGED (2026-09-08, #2192; mechanical under the
08-29 ruling, no A/B row). Census family B-DISTRIBUTIONS CLOSED —
parameter uncertainty and the analysis lane (E1/E2).** The gate
measured before scope, as the brief required: `crates/pncad/src/
analysis.rs` splits at `:49` (ungated) and `:55`–`:104` (`interval`),
all three chartered doors are on the ungated line, so the family
closes on the default build the wheel is made from and nothing is
deferred behind a feature; there is no precedent for a feature-gated
Python door and none was minted. Bound: `Distribution.band / uniform /
normal / truncated_normal` as a frozen constructor class whose offsets
are TYPED quantities in the parameter's own dimension (the annotation
carries none of its own — E2 — so it borrows the declaration's, which
makes the read direction free and two disagreements checkable as
`DimensionError` with `op` naming the door) and whose construction
runs the kernel's own `Distribution::check`, so a document that would
refuse to load cannot be authored and the refusal lands where the
sigma is written (`DistributionFault`, every arm's payload present on
every arm); `DocParam.length/angle/scalar(value, distribution=None)`
and `DocParam.distribution`/`dimension`; `analyzed_box(doc, policy)`
with `AnalysisPolicy`, `AnalyzedBox`, `AnalyzedParam`,
`DEFAULT_QUANTILE_MASS`; the tail and leaf columns as METHODS ON THE
BOX (`tail_mass`, `box_mass`) rather than the kernel's free functions,
on the kernel's own argument that loose triples let a caller pair one
parameter's distribution with another's box; `MeasureUnavailable` (a
band prices nothing shape-dependent, and names the parameter) and
`AnalysisPolicyError`. The sharp edge pinned in all three states
(the rebuilt `DocParam` still deletes the annotation; the value door
carries it forward; a redeclaration can now restate it) — there is no
distribution-only edit arm and none was invented, so the closure is
the constructor and the prose on `set_doc_param` says what the door
does. Census: three rows left three ways (`Distribution`,
`DistributionFault` under rule 1; `DistributionField` to `BOUND_AS` as
`DistributionFault.field`); the charter leaves `FAMILIES`. The census
lesson in its widest form yet: three of the charter's four things
(`analyzed_box`, tail mass, leaf mass) were never rows and could not
be, because `crates/pncad/src/analysis.rs` is outside the census's
alphabet in both directions — which will do the same to B-MEASURES's
read half. 36 Python rows mirroring `m10_1_analysis.rs` and the
façade's end-to-end row, two Rust pins on the default path, five
fixtures each way, a GUIDE §3.3 block executed by `test_guide.py`.
Deviations with homes: a keyed `Doc.doc_param` was written and then
deleted at the B-NOTATION merge in favour of `doc.params.get(name)`
(one door per question); `written_length/angle` take no
`distribution=` because the kernel's notation doors write none
(reported as the kernel-side gap). Filed:
`advisory-monte-carlo-lane-has-no-python-door` (the E11.1 estimator is
ungated on the façade precisely for the caller with no certified
scalar, and a Python caller is that caller). Reported outside the
fence: `std_deviation`'s `NOT_CARRIED` row withholds a name whose
answer is one method call away on a carried type (M10's call).
Orchestrator note: main gained EVAL/RESOLVE internals and tests between
the green run on `528d9274` and the merge, overlapping nothing in this
diff, so it landed on that run.

**LIB-CUR6 MERGED (2026-09-08, #2193; mechanical under the 08-29
ruling, no A/B row). The five re-sweep hits settled under the CUR3
test, four carried and one argued at the entry.** `CensusSubject`
carried into prelude group 5 (two arms, two recourses — an `Entity` is
a carrier outside the certifiable inventory, a `FacePair` is a
candidate contact; both payload types CUR5 curated, so the
discriminant was the last thing in the way; the unordered-pair half
deliberately unpinned since no curated name mints two distinct
`FaceKey`s, said at the guard); `RevolvedKind` carried into group 3
and CONSTRUCTED rather than fabricated — the row calls `revolve` twice
and reaches both arms, whose fields are two disjoint sets of handles
rather than a label; `PromotedKind` carried into group 7 with the
item's carrier attribution CORRECTED (`SurfacePromotion` sits on an
uncurated chain; the curated carrier is
`StepImportError::RecognitionAmbiguous::kind`), the tag arriving
beside `recognition_ambiguous` rather than forwarded
(`StepImportError.promoted_kind`, `promoted_kind_tag` exhaustive,
inventory row, Rust pin) and a stale runtime docstring ("`refused` or
`wireframe`") fixed in passing; `ImportContact` carried as a REACH
defect — `ImportOptions::declared_contacts` is a `pub Vec` whose
element type no curated list spelled, so the import-side declaration
channel was callable and not fillable, and the façade row now fills
it; `MappedCurve` NOT carried, the third entry of the
`BandField`/`MarginDiag` family and the first whose reason is the ARM:
`Scaffold` is fenced to construction and refused at rest, its
authoring form is on no list, every kernel consumer is a re-mint, and
the rung is uncarried whole (`ChartCurve`, `Pcurve`), so carrying one
arm would be the `Convexity` inconsistency in reverse — falsifier
stated. Census: `PromotedKind` to `BOUND_AS`, `ImportContact`
`different-shape` beside its absent argument, `CensusSubject` and
`RevolvedKind` `INTERIOR` by the carrier rule with the measurement
(the validate doors cross as prose; `Revolved` does not cross at all).
Re-sweep at the merge base with blind spot (f) closed (bare-`pub`
fields only), (g) new and closed (variant names read as payloads),
(e) narrowed (a `pub enum` inside a `macro_rules!` body IS indexed),
(h) new and open (crate-aware, not module-aware): six new hits, all in
`profile`, filed as
`payload-rung-re-sweep-finds-six-uncurated-profile-discriminants`;
the lane's counts do not reconcile with CUR5's and it says so — the
pattern is prose re-implemented each run, every row hand-verified to
a `file:line`. Also filed:
`a-successful-step-imports-own-report-is-uncurated` (`import_step`'s
refusal half is curated and its success half is not). No kernel crate
touched; the lane ran the pinned ruff for real. Branch level with
main at the merge.

**LIB-BENCH-PATTERN MERGED (2026-09-08, #2199; mechanical under the
08-29 ruling, no A/B row). The flat-pack posts measured as
`Node.pattern` against `placed_union` through every layout door, and
the pattern shipped.** `bench_scene.layout` grew a `posts=` switch
over the two spellings (one count, one rule, one set of constants) and
`TestBenchLayout` runs its whole battery under `subTest` against
BOTH: over the flat-pack the plural `Instances` value answers exactly
as the fused body does — `product` gathers `PATTERN_COUNT ×
POST_VOLUME + SHELF_VOLUME`, `select` answers `PATTERN_COUNT` distinct
instance-qualified cap names, each `face_frame` lands on the placement
ladder's rung to 1e-12, every `denotation` is untied at one candidate,
the tessellated outline is the same box, and `assemble` passes the A5
gate outright with `minted == []`. The two spellings part at exactly
ONE door and the pattern is the better side of it: on a document
authored for the question, a mate head on a `Node.pattern` copy
resolves, solves `Determining`, joins one cluster and mints — the
`SlotId::Instance` walk (`editor-core/src/mate/member.rs`) reached
from Python for the first time — while the same head on a
`placed_union` copy refuses ("does not resolve to a live member") and
the mate node's failure reds the product. The issue's premise that the
LAYOUT carries mates over the posts was FALSE and its `## Closed` says
so: the pattern is in the layout, whose expectations are the material,
the outline, the per-placement frames, the names and a gate that
passes with nothing minted; the cluster, gauge, solved translation,
minted declarations and CERTIFYING gate are `TestBenchStand`'s, which
carries no pattern and is untouched. `posts=` keeps `placed_union`
reachable so the comparison stays executed rather than remembered.
Audit: row 43 `**YES**`/`—` with the substitution sentence replaced by
the measurement, headline "34 outright, and 3 more" (37 of 45
unchanged — a row moved between the YES columns, not across the NO
line), G8 `degrades 3`, G18's residue sentence records the row as
written, and the gap-list arithmetic prose (the one sentence the tally
guard does not check, already stale at `32 + 4 = 36`) corrected to
`34 + 3 = 37, and 37 + 8 = 45`. Python tree: the scene's second
"deliberate difference" deleted (parametric prisms stay), the eval
seam row reads `[POST_VOLUME] * PATTERN_COUNT`, the outline row
tessellates every body (`.body()` refuses typed on `instances`), the
tour guard's blind spot (2) is one item, a docstring's "four ways"
now names the constant (2). No kernel change; `demos/tour` untouched;
the lane ran the pinned ruff for real. One inherited red on the first
run (`render lanes / freecad montages` hung at its upload step with
both lanes reporting `matches this render`; `rerun-failed-jobs` is
403 for this integration) cleared by the main re-merge — green on the
landed head, recorded in a PR comment. Orchestrator note: main gained
EVAL's per-node nominal-environment work (`editor-core/src/eval/`,
its tests) and an AB-LOG row after this head's last merge, none of
which overlaps the diff, so it landed on that run's green without
another chase.

**LIB-B-MEASURES MERGED (2026-09-08, #2198; mechanical under the
08-29 ruling, no A/B row). The authoring half of measurement bound,
and `FAMILIES` is EMPTY.** Two of the kernel's twenty-three recipe
node kinds — `Measure` and `Assertion` — had been unconstructible from
Python for the life of the binding while `Value.measure`/`assertion`
read them; now `MeasurePrimitive` (`.distance/.angle/.min_clearance/
.gap`, frozen static constructors reading `verb`/`dimension`/an
argument-ordered `refs` pair off the kernel's own doors),
`AssertionDir` (`AtLeast`/`AtMost` with `symbol` — the `BooleanOp`
MIRROR rather than the brief's constructor-class shape, argued: every
fieldless kernel enum on this surface is an `eq_int` mirror and the
spelling buys the exhaustiveness witness
`_binds_every_kernel_direction`), `MeasureExpr` (the WHOLE arithmetic
— `primitive/value/add/sub/neg/mul/div/min/max`, `dimension`, a
pre-order `primitives`; `value` takes an `Expr` from `Doc.parse_expr`,
the one text door), `Node.measure(expr, refs: list[tuple[NodeId,
str]])` through the kernel's own `Node::measure` so an index past the
end refuses where it is written (`MeasureNodeFault`:
`variant/verb/index/refs`), and `Node.assertion(measure, dir, bound:
Expr)` — the bound an `Expr` because its dimension is fixed by the
node it points at, not by a slot address, and so a bound can BE a
document parameter (re-decidable by an edit; pinned Holds → Violated
on one `set_doc_param_value`). The gate measured and it is the LANE,
not a feature: every curated name sits in `pncad/src/document.rs`
under no `cfg`; `min_clearance` is answered by `MinClearanceLane`,
whose only `Some` impl is `Interval`, and the binding evaluates at
`f64` alone — so `MeasureUnavailableAt` (`variant/verb/scalar/door`,
tag `needs_enclosure`, raised by `Value.measure` in place of
`EvaluationError`) IS reachable on the default build and
`MinClearanceRefusal` is reachable at NO feature set; it stays on the
roster retagged SHAPE with that sentence (a third way off a charter,
after B-DISTRIBUTIONS's two), and `SitedRef` retagged SHAPE (never
handed across — both authoring doors take a node and a name). The
name collision with B-DISTRIBUTIONS's `MeasureUnavailable` resolved by
keeping the Rust type's own name: neither subclasses the other, the
two tag functions deliberately not one, the non-subclassing pinned.
`MeasureExpr`'s arithmetic raises the expression layer's own
`DimensionError` through `LiteralError` (a shared class WIDENED:
`LiteralError.value` now `Optional[float]`, the door inventory at four
doors, the "every kind on this class is a literal-value refusal"
sentence rewritten because the unit made it false; routing to
`ErrorClass::Dimension` rejected to keep "never intercepts an
expression-layer mismatch" true). Five rows leave `NOT_BOUND` under
rule 1; every remaining `gap:` cites an AUDIT id (`G2`), none the
census owns. Closed forms re-derived, not transcribed from
`m10_2_measure.rs`: ±0.30 cylinders measure 0.60, slabs gap +2 in
both role orders, opposed caps subtend exactly pi, bore/pin gap
`r_bore - r_pin` across all three C5 regimes, the same vertex name at
an extrude and its transform exactly the translation apart; a
`min_clearance` measure evaluates with no value and its assertion
reports `Unevaluated` naming verb and door; a document carrying both
saves, loads and `bit_eq`s. Sweeps: every `Node` arm's constructor
(two absent: `Sweep`, not-this-unit, and the n-ary `Union` with
`SetMembers` beside it, FILED as
`n-ary-union-and-set-members-have-no-python-door`); every measurement
refusal arm with ten reached and three not, each with its reason
(`measure_malformed` at the EDIT door unreachable by construction,
its LOAD twin reached; `measure_ref_unreadable` needs the state
`test_face_frame.py` records as unreachable). `test_north_star.py`'s
G1-residue paragraph ("an expression goes in through no door at all")
corrected — narrower, still there. A second site of #694
(`load-path-stringifies-structured-refusals`, at `Snapshot`) named
and pinned. Outside the fence, PR body only: a measure authored over
the product ROOTS takes them (D-3's tip transfer) and `product` then
refuses `no_body_roots` with no `set_roots` repair — pinned as a row
whose docstring says it exists to go red with the argument in hand.
42 new Python tests; the lane ran the pinned ruff for real; no kernel
crate touched. Orchestrator note: main gained BENCH-PATTERN after
this head's last merge (`bench_scene.py`, the assembly tests, the
audit page, tracker files), none overlapping the diff; merged in for
the log tail alone and landed on run 34225477952's green without
another chase.

**LIB-TEAPOT MERGED (2026-09-08, #2206; full protocol — block LIB-13
slot 2, v6 dual at ordinal 304, sample #163). The tour's teapot is
ONE recipe document and the numeric scans are gone.** The pot's
revolve is hollowed twice by `Node::Shell` over one operand and wall,
parted only by the open list (empty for the sealed body whose census,
capacity and Void class §3's pins read; the mouth's `Band`/`BandPi`
half-discs BY NAME for the cup, which comes back as ONE rim face); the
lid's revolve rolls through `Node::Fillet` on its `BandRim` names —
THREE names, not the spec's six, because an annular profile takes the
emitter's lamina branch (one closed rim per meridian vertex, no
`BandRimPi`) — and in TWO requests where `fillet_edges` takes one,
because the one-request output cannot be NAMED: the flange's rim and
the dome's foot both slit the flange cone's seam meridian and
`RoleSeg::BandSlit` is keyed on the source edge alone, so the emitter
refuses `Naming(Duplicate)` before any geometry is doubted (§8's first
stop clause, fired one step over from where it was written; filed as
`blend-slit-name-collides-when-two-rims-share-a-meridian` — editor-
core's code, on LIB's slate by the spec's routing, RE-HOMED below —
with the true shape after the dual: two rims whose bands slit the SAME
seam meridian, adjacency necessary not sufficient); the spout's
revolve is placed by `Node::Transform` in axis-angle, whose cos/sin of
`atan2(0.8, 0.6)` come back as the 3-4-5 components and whose placed
root annulus sits on the authored root at exactly 0.0 — MEASURED off
the body only at the fix pass, the delivered assertion having been an
arithmetic identity (the dispatcher's seeded question, found by both
reviewers); the handle is `Datum::Axis` + `Node::Tube`; the two joins
the operand gate has no arm for are `Node::Boolean` nodes that refuse
at `evaluate` under the kernel's `CurvedPairUnsupported` (same face,
operand and kinds as the kernel-direct union; wall 3's `other_face`
KEY moved 1v1→2v1 with the arena numbering, disclosed at the fix).
`plane_chart_at` and `rim_at` deleted — the role names were asserted
equal to the scan's keys in the branch before the scan went, and what
ships reads each rim's circle back THROUGH its name. `gallery_document`
exposes the recipe (four roots; three sinks deleted, which is the
document layer saying a recipe cannot hold a narration body — filed).
Tess budget: the gate clean against BOTH the pre-PR and the re-cut
baseline; the baseline re-cut anyway because 27 teapot rows gained the
durable face `name` and three `teapotlid` rows permuted their
triangle counts with the two-request roll's face order — a file that
moved with no geometry moved, §8's third clause contradicted and said
so, the reverse re-cut scheduled by the live probe that goes red the
day `BandSlit` gains its discriminator. Render lanes: wild and FreeCAD
"matches this render"; uv NEUTRAL on the lid's re-slotted cells — the
one moved cell, a §9 finding by the spec's letter, cause disclosed.
Audit: rows 27 and 44 → YES on `TestTeapot` and `TestTorusvessel`
(the torus-walled vessel's sealed hollow against its own closed form
at two thicknesses — a row the spec did not ask for and the page's
discipline required), row 45 stays NO on `Body::merge_coplanar_faces`,
G17 `stops` 3→1, headline 39 of 45 (36 + 3 YES\*, 6 blocked). Dual:
CONVERGED (A-W-F 1/6/7 / A-W-F 1/6/7, rubric 4/3/3 both), severity
labels swapped on both headlines, zero unilateral MAJORs, no tally
candidate, pair flagged under item 5 (the build-slot banner leak,
filed). Fix pass IMPLEMENTER-INHERITED: all seventeen items taken, none declined, six commits: the spout's placement MEASURED off the placed body — root and tip annuli named at the revolve and read at the transform against the exact image under the direction's own matrix, ≤1e-15, axis cross 0e0, both subjects guarded off the turn's fixed axis, the bitwise libm pin retired for the `erf` precedent and the observed cos/sin REPORTED (the Python row pins the same; deleting `Node.transform` now fails at 0.0078125 vs −0.1266); the Python lid row pins each selected rim's station before the roll and the three band spine stations after it (the vertex-3 mutant red at 0.21198 vs 0.21875), `RIMS` as `(vertex, radius, station)`, `SEG_MOUTH` read off the built revolve; `rim_circle` from the NAME to the one edge that carries it and the circle off that edge's own carrier (R2's same-station mutant unreachable); finding 6 ATTEMPTED live in `per_rim_answers` with the `Naming(Duplicate)` pinned and the reverse re-cut scheduled in the assertion; `demos/tour/tests/teapot_document.rs` with the rim-pair table ({1,2} and the triple refuse; five pairs incl. adjacent {2,3},{3,4} build 8/16/8) and the two-requests-equal-one-request row (census, the three bands' stored `(station, major, minor)` bit for bit, mass to 1e-14, face ORDER asserted to differ); the mouth-chart pin restored both ways; `join_outcome` with a reachable `Ok` arm; the germ-pair sentence matched whole; R1's mouth mutants as a Python row with V/A EXACTLY equal across the two orders; the issue corrected to the shared-meridian invariant with the vertex→segment table (0→5, 1→1, 2→1, 3→2, 4→3, 5→5); wall 3's key disclosed at four sites; stale prose at six sites; the gallery `why` truthful and `concat!`-built; the re-cut disclosed as UNFORCED (lint clean against both baselines, §8's clause contradicted and said so); the uv cell a §9 finding with f005's loop-start change named; the lid STL measurement recorded; two duplications disclosed and FILED (`no-facade-door-mints-a-revolves-role-names`, `a-recipe-cannot-hold-a-narration-body-without-it-becoming-a-root`, gap-commented at `gallery_document`); the note renders the germ pair not `Debug` and drops 11,261→9,764 chars. nextest 6662/6662, Python 698 OK, pinned ruff, tess-lint 0 findings and 0 rows moved by the fix pass; the finalize 403 hit one intermediate head once and never recurred. Orchestrator
notes: the implementer lane was killed by a container restart mid-unit
with the branch unpushed (the clone survived; resumed; "push early" now
means before the first build); the reviewers' scratch must live in
their own lane directories, not the shared root (`memories/` lesson
candidate, not written — Ev's call). Spec deleted at merge, ledger
row written. RE-HOMING: `blend-slit-name-collides-…` and
`recipe-cannot-hold-a-narration-body` are editor-core findings filed
on LIB's slate; they stay here as LIB-owned CROSS-FENCE items until
the owning program claims them (STATUS shows them under lib).

**LIB-ARMS MERGED (2026-09-08, #2217; mechanical under the 08-29
ruling, no A/B row). The first unit under the per-arm rule Ev licensed
on `[ev]` PR 2196: every refusal whose carrier already projects a word
gains its inner arm as a second attribute.** `EvaluationError` gains
`inner_kind` and `EditError` gains `inner_variant` — the kernel
refusal's own arm as a snake_case word, `None` where the refusal has
no arms — and `kind`/`variant` are untouched, so a caller branching on
the op ladder sees no change (the ruling's own argument for the shape,
measured: the diff deletes no match arm and no literal, and no
`kind ==`/`variant ==` fixture moved). Ev's recorded reservation is
answered at the door — the `EvaluationError` docstring, `pncad.pyi`
and `docs/guide/fail-loud.md` (an executed block: `("revolve",
"degenerate_angle")` / `("revolve", "vertex_crosses_axis")`) — as two
enums' discriminants projected where each lives, not one division
stored twice. The survey: 68 `NodeErrorKind` arms, 29 project an
inner enum through twenty new exhaustive maps (profile, replay,
structure-refusal, extrude, revolve, tube, split-op, blend, boolean,
transform, skin, loft, band, naming, param-attach, shell,
program-refusal, param-box, seed; 206 literals), 39 answer `None` in
four stated groups (three already project the payload's word under
the carrier's name; eight carry a VALUE enum, the payload half's
question; five carry a struct with no matchable discriminant, one of
them `WitnessBifurcation` — filed
`witness-bifurcation-arm-has-no-inner-word`, an expiry not an
omission; 23 carry no enum). `EditError`: 58 arms, five project.
Nested enums one level only. The compile-time alarm fires both ways,
RUN not asserted (a planted `RevolveError::ProbeArm` and a deleted
map row both red `revolve_error_tag` with E0004). Six inner words
reached from real documents, the `None` case pinned at both carriers,
the poisoned path carries the root cause's pair; `EditError` reaches
one of its five inner arms and the other four have pinned reasons
(the doors pre-check). Disclosed deviation: the FAÇADE moved —
`NamingError`, `ProgramRefusal`, `SeedError` out of `NOT_CARRIED` and
`ParamBoxError` out of the `interval` block (`NOT_CARRIED` 90→87), by
the façade's own payload rule, argued in `crates/pncad/tests/all.rs`;
`BifurcationKind` deliberately stayed. Eleven census rows to
`BOUND_AS` at the two carriers' second words with the measurement
stated; four types that cross the same way are not rows because none
is a leaf of the three curated lists. Also closed:
`tag-inventory-prose-counts-are-stale` (option 1 — aggregates and
rosters deleted, a roster found already false; the two floors reset
to 60/500 under a table at 71/623). The tag-table reader learned
`Option<&'static str>`, `Some(..)` and bare `None`. Mid-unit main
moved under it (SHELL-8 renamed a shell arm and added one; the
exhaustive match caught it — the alarm's first live firing). Not
taken, per the brief: the payload half and the `findings` sequence,
next. No kernel crate touched; the lane ran the pinned ruff for real.

**LIB-FINDINGS MERGED (2026-09-08, #2225; mechanical under the 08-29
ruling, no A/B row). Ev's ruling (A) on
`census-findings-cross-without-a-per-arm-tag` shipped: `ValidationError`
keeps `door` and `failure_count` and gains `findings`, a list of frozen
`ValidationFinding`s — one per failure, `len(findings) ==
failure_count`, in the kernel's own report order — the single exception
to "`variant` is a scalar", argued by the door's own shape (the one door
that reports MANY refusals in one raise) at the three places a reader
meets it (the class docstring, `pncad.pyi`, the README's taxonomy
paragraph) so it does not become a second convention.** Each finding
carries `variant` (the arm, from `validation_error_tag`, exhaustive
over all 71 `ValidationError` arms, no wildcard), `contact_kind`
(`UndeclaredContact`'s `CensusContact`, 8 arms) and `subject_kind` +
`entity_kind` (the two census-unsupported arms' `CensusSubject`: 2
arms, and the `EntityId` kind beside it, 7 — CUR6's own shape at the
façade, the ruling's "plus the entity kind or the pair"), every
attribute present on every finding, `None` where the arm carries none;
no arena key crosses (a `Body` is an opaque handle), so WHICH face or
vertex stays in the kernel's prose on the joined message, which is
byte-identical; the door still raises once. Python-independent
`validation.rs` assembles the words (its two payload EXTRACTORS carry
a `_` arm under the kernel enum's own extract licence — a question,
not a classification; the classifying map is the exhaustive one).
Rows: the old absence pin REWRITTEN into its positive form (the two
scalar words still absent — one raise carries N findings — and the
sequence is where the arms live); `len == failure_count`; each word a
phrase the message spells; structural `==`/hash and frozenness; every
attribute present; two distinct arms off ONE raise (a cylinder resting
on a slab: `undeclared_contact` with `vertex_on_face` beside
`census_undecidable`); the arms Python cannot produce NAMED with the
reason (the structural/geometric arms want a corrupt arena; the two
subject-carrying arms want an uncertifiable carrier, and every public
product certifies) and pinned in Rust where the refusal constructs;
ty fixtures both ways. Census: `CensusContact` and `CensusSubject` left
`INTERIOR` for `BOUND_AS` together, as their own rows' falsifier
predicted; `EntityId`, `ContactFinding`, `RingContact`,
`StaleDeclaration` re-argued where they named the old measurement;
the prelude's group-5 note corrected (comment only, no façade name
moved). Residue filed:
`two-validation-payload-discriminants-still-uncrossed`
(`StaleDeclaration`, `RingContact` — the ruling named two types and
these are not them). Not taken: the six doors' payload attributes
(LIB-DOORS-1 ran beside this unit). No kernel crate touched; the lane
ran the pinned ruff for real.

## Announced seam from SHELL (2026-09-08): `ShellError::Pcurve` arm in editor-core's fold

SHELL-9 (PR #2223) adds `ShellError::Pcurve { source: PcurveMintError }`
(the closing pcurve mint's typed refusal — unreachable by any
committed fixture, stated so) and extends the one exhaustive fold,
`crates/editor-core/src/verbs/shell.rs`'s `fold_shell_error`, with
the arm; `pncad-py`'s census and tags need nothing (the field is not
`Real`-typed; every `ShellError` folds to the `"shell"` tag). Same
shape as SHELL-5's `ShellError` change. No action asked. Signed
(SHELL orchestrator).

**LIB-DOORS-1 MERGED (2026-09-08, #2227; mechanical under the 08-29
ruling, no A/B row). The first door under Ev's ruling (A) on
`pncad-py-seven-doors-lack-field-projection`: `EditError`'s 58 arms
flatten into one record of 21 payload fields, published beside
`variant`/`inner_variant` as 23 attributes on every raise site of the
class — the document layer's, the declare sugar's `Edit` arm, and the
three the boundary builds itself — present on every arm, `None` where
the arm carries none.** One exhaustive match with no wildcard in a new
Python-independent `edit_payload.rs` (a stated deviation from the two
worked examples, which sit inside the `python`-gated module: the drift
alarm has to ring on the interpreter-less CI row and the construction
pins have to be able to run there), a struct with `..Self::NONE` rather
than a 1218-entry positional tuple with exhaustiveness kept twice (the
match, and `presence`'s `..`-less destructuring). One attribute per
CONCEPT: `node`/`input`/`referenced_by` under the kernel's three
spellings of a subject node, `expected`/`found` carrying
`declared`/`referenced` and `measured`/`bound` as dimension words, a
short list's `found` spelled apart as `count`, `from_kind`/`to_kind`
(a Python keyword), `path` vs `value_path` (two trees). `slot` is a
WORD not an index — `SlotId` is a named per-node-type enum (D5: never
an index) — so `slot_id_tag` (40 literals, the axis spelled into the
word, `Profile{..}` stopping at `profile`) and `attr_kind_tag` (3)
join the table. Façade moved twice under its own payload rule
(`AttrKind`, `ExprPath` out of `NOT_CARRIED`, 87→85; `MetaVersionError`
deliberately NOT — a nested refusal whose arm has no inner word, the
per-arm question, filed `meta-unversioned-arm-has-no-inner-word`, the
one arm of 58 the construction pin cannot build). Nine arms reached
from Python by real edits, the rest pinned in Rust with the reason per
arm; a deleted match arm fails to compile (run, reverted); no shipped
`variant`/`inner_variant` moved. Census: `SlotId`, `AttrKind`,
`ExprPath` to `BOUND_AS` with the measurement (addressing a slot and
reading one off a refusal are two questions). The item stays open with
a `## Progress` line: the edit door done, `persist`/`frame`/`stl`
running as LIB-DOORS-2, `path` behind the kernel `PathError`
discriminant. Merged main twice mid-unit incl. LIB-FINDINGS (two ty
fixture conflicts, both hunks kept). No kernel crate touched; the lane
ran the pinned ruff for real.

**LB20 — wave 4 planned from a full slate survey (2026-09-08, after
LIB-FINDINGS and LIB-DOORS-1 landed).** The ruled queue emptied twice
today; a read-only survey of every open LIB item (33) sorted them by
class and by the files a fix touches. MECHANICAL, grouped into lanes
that do not collide (two builds live at a time): LIB-PROJ (the ten
`MateFault` accessors exhaustive + the pick indices as attributes;
owns `py/mate.rs`, `py/pick.rs`) DISPATCHED beside LIB-DOORS-2;
LIB-CUR7 (`meta-unversioned-arm-has-no-inner-word` + the six profile
discriminants; owns the façade lists and `tags.rs`) and LIB-DOORS-3
(the n-ary union / `SetMembers` doors + the `VDegree` structural slot;
owns `py/doc.rs`'s constructors) after DOORS-2 lands; LIB-SMALL (the
split/inline maintenance getter, `Datum.in_plane`'s length pair, the
guide's chamfer and tube steps, the audit's `arc_continue` line once
BOOL-10 merges); LIB-HASH (the 23 comparable enums hash) ALONE in a
gap. DESIGN, asked as four unrelated `[ev]` PRs per Ev's standing ask,
each recommending (A): #2230 (does per-arm projection generalise past
the two named types; the census alphabet declares same-spelled types),
#2231 (the authoring seat: a probe body the product skips; builders
for a revolve's role names), #2232 (what a curated door owes on its
SUCCESS side: the import report, the gate's enclosure, the advisory
lane), #2233 (dimensioned seats: written notation at node slots;
equality/hash on a poisoned or signed-zero quantity). KERNEL-SIDE,
RE-HOMED by territory with a `## Re-homed` note each: the blend-slit
collision, the blend selection's load-only check and the persist
path's stringification to DOCM; the mouth chart and the shell corpus
hold to SHELL; the predicate-name pins to BOOL. STAYING in LIB by
assignment or by expiry: the mate-frame door and the two recourse
sentences (S-MATE's hand-off gives refusal prose to LIB), the
witness-bifurcation arm (an expiry on M6), the façade guard's readers
(Track E), `correspondence-structs-coincide` (a verb-seat hold with no
owner). `path`'s door waits on the kernel `PathError` discriminant.
Orchestrator note for the record: the per-arm maps are a standing
tripwire — main's own kernel PRs tripped them three times today (a
shell arm renamed, an extrude arm retired, a shell arm added), each
fixed by one map row; and reviewer scratch must live in each lane's
own directory (the item-5 lesson, `build-slot-banner-leaks…`).

**LIB-DOORS-2 MERGED (2026-09-08, #2228; mechanical under the 08-29
ruling, no A/B row). The second unit under Ev's ruling (A) on
`pncad-py-seven-doors-lack-field-projection`: the `persist`, `frame`
and `stl` doors project every arm's payload as attributes, present on
every arm and `None` where the arm carries none, each from ONE
exhaustive match with no wildcard, on the single door function every
raise site of the class already went through.** `persist_err` (thirteen
arms, fifteen fields beside `variant`; `py/store.rs`'s pin doors ride
the same door): the four NESTED arms — a profile-program fault, a
distribution fault, a snapshot invariant, a replayed edit's `EditError`
— cross as their own word on `inner_variant` with the nested payload
left as the inner door's surface, which minted `program_fault_tag`
(2) and `snapshot_error_tag` (19, delegating the product-root arm to
`root_fault_tag`); `detail` and `document` each carry one concept
under the kernel's several spellings; the recursive `NonFiniteSite`
crosses as one sentence on `site`, not a field per rung. `frame_err`
(two arms, nine fields): the degenerate arm's `input` is NOT a second
attribute — it IS the per-input `variant` — while the classifier's
own shape crosses when the margin landed in band (`margin` or
`margin_low`/`margin_high`, `zero`/`escalate`, `predicate`; a poisoned
margin carries the band and no number) and nothing when it was a
definite zero; the `band` arm carries `inner_variant`/`field`/`value`
via the new `band_field_tag` (2). `stl_err` (six fields): the three
kernel enums plus the boundary's own `not_utf8` unified as a private
four-arm `StlRefusal` so the projection is one match — the stated
deviation, crossing nothing. Reachability: five persist arms, three
frame arms and three STL arms driven from Python with the payload
asserted; the arms no Python door can reach (the persist door's four
nested arms, the frame door's `band` — every constructor derives its
band from the tolerance witness, so `Band::linear` cannot fail there —
the STL writers' four) pinned in Rust with the reason per arm. Census:
six rows to `BOUND_AS` with the measurement (`ProgramFault`,
`SnapshotError`, `NonFiniteSite`, `SolidNameError`,
`BinaryHeaderError`, `Indeterminate`). **The curation consequence:**
`prelude.rs` argued `MarginDiag`'s and `BandField`'s non-carriage on
"no attribute on any bound exception carries a margin, an enclosure
bound or a band", and the frame door made that sentence false — the
count of thirteen prelude refusals is intact (`FrameError` is a
`geom_core` refusal, not one of them), the sentence is not. Both
prose notes corrected in the PR; the re-measure the prelude asks for
is filed as
`margin-diag-non-curation-was-measured-on-a-count-that-moved` (a
curation question, not a binding one; nothing unreachable). No shipped
`variant` moved; 23 new tag literals, all on `inner_variant` or
`field`. The carrying item stays open with a second `## Progress`
line: what remains is `path` (behind the kernel `PathError`
discriminant) and `step_import` (argued at its site). No kernel crate
touched beyond the prelude's prose. A deleted arm fails to compile
(three `E0004`s, one per door; run, reverted). Merged main twice
mid-unit; the lane ran the pinned ruff for real.

**LIB-PROJ MERGED (2026-09-08, #2236; mechanical under the 08-29
ruling, no A/B row). Two one-file projection repairs under Ev's
ruling (A): the `MateFault` accessors are exhaustive, and
`NodePickError` projects the index arm's three numbers.** The
seventeen mate accessors now read off ONE record,
`crates/pncad-py/src/mate_payload.rs` (the `edit_payload.rs` shape —
`MateFaultPayload`, `presence()` without `..`, `NONE`, one exhaustive
match over thirteen arms, sited outside `py/` so the drift alarm
rings on the no-Python row): the lane's stated choice, since
seventeen per-accessor matches would name thirteen arms seventeen
times and charge a new kernel arm seventeen edits. `Unleverable`
answers `mate` and nothing else — its `LeverRefusal` is a nested
refusal of a type the façade does not re-export, filed rather than
guessed. The lane swept its own file past the charter, with the
reason: `MatePrimitive::offset`, `Subgroup`'s three and
`ClusterMaintenance`'s seven became exhaustive IN PLACE (few arms
over few accessors — a record there adds a layer without removing a
match); `py/mate.rs` has no `_ =>` at all. Measured: a `MateFault`
arm added kernel-side fails `cargo check -p pncad-py` with no
features at the record's match, where before only the tag map fired
and the seventeen accessors compiled unchanged. `NodePickError`
gains `patch`/`triangle`/`index` from `pick_payload.rs::index_payload`
(exhaustive over both `NodePickError` and the `MeshPickError` inside
it); `node_pick_err` is the class's one raise site, so the three are
on every instance by construction. **The brief's claim the lane
could not make**: "the three pick numbers reachable from Python" —
the arm reports a mesh violating its own invariant, unauthorable and
unconstructible from Python, exactly as the item said; pinned in
Rust at three distinct numbers (`3`/`11`/`47`, so a swapped slot
shows as a moved value), with Python owning the present-and-`None`
half. Orchestrator note: that claim was the brief's error, not the
lane's, and the lane's refusal to assert it is the discipline working.
Census: `MeshPickError`'s `BOUND_AS` mapping unchanged, prose moved.
Two residue items filed: `mate-fault-arms-carry-payload-that-does-not-cross`
(six kernel fields over six arms no attribute crosses — two
`DocumentId`s, `Contradictory.lever`, and the four nested refusals
whose types the façade does not re-export, which is why the Rust arm
table pins nine of thirteen; the projection is still total, the
executable table is not, and the two guarantees differ — the `Frame`
row's wait ended when DOORS-2 landed, noted on the file) and
`payload-accessor-wildcards-remain-in-checks-and-assembly` (the
sweep's hit list: `py/checks.rs`'s five `CheckEvidence` accessors and
`py/assembly.rs`'s two `RefusedRef` accessors are live siblings of
the shape; `validation.rs`'s extract licence argued at its site;
`py/doc.rs`'s `Node` extract not this class; blind spot stated for a
named catch-all). Both closed items carry `## Closed`. No kernel
crate touched; the lane ran the pinned ruff for real; ~1h15m,
~221k tokens.

**LIB-DOORS-3 MERGED (2026-09-08, #2238; mechanical under the 08-29
ruling, no A/B row). Three doors, all arms behind a bound name and
so invisible to the census's rule 1: `Node.union(members,
declare=None)`, `DocEdit.set_members(node, members)`, and
`DocEdit.bind_v_degree_param(node, name)`.** `union` is the n-ary
fold over a member LIST in the list's order (D9), with `boolean`'s
optional `declare` slot fed at the fold step its pair meets at; 22 of
the kernel's 23 `Node` variants now have a constructor, `Sweep` the
stated exception (`wire_sweep` refuses unconditionally). `set_members`
is what makes the membership DATA rather than a boolean chain's
shape: the three tags minted at LIB-DOORS-1 with no Python caller
(`set_members_on_non_list`, `too_few_members`, `duplicate_input`) are
each provoked from Python with the payload asserted (`count`,
`input`, `node`). `bind_v_degree_param` is `VDegree`'s door beside
its two siblings, each naming its own slot; `Stations` stays
undoored (its only node is the sweep) and `bind_count_param`'s prose
now accounts for all four structural slots. The scene the item asked
for: `TestLoftPrism`'s three sections skinned at a BOUND degree
enclose 9 m³ at degree 2 and 8.75 at degree 1, one `set_doc_param`
apart, with the kernel's value rule still refusing degree 3 bound
exactly as literal. `test_union.py` pins the fold against
`boolean(boolean(a,b),c)` and against the three boxes'
inclusion-exclusion closed form (16.40625 m³), and one row per
refusal at both doors. The roster in `test_north_star.py` gains the
three names (the positive list that sees missing arms); the guide
gains one sentence at the loft rung, no step (LIB-SMALL's). One
stated deviation: a duplicated inert `@staticmethod` above
`set_roots` in the stub deleted inside the stanza the unit edits. No
façade list moved (the argument types were already the loft's and
the boolean's), no projection changed, no `variant`/`kind` value
moved; the lane ran the pinned ruff for real. ~40 min, ~195k tokens.
**LIB-CUR7 MERGED (2026-09-08, #2237; mechanical under the 08-29
ruling, no A/B row). The `meta_unversioned` arm has its inner word,
and the six profile discriminants LIB-CUR6's re-sweep filed are
decided.** `MetaVersionError` carried at `pncad::document` under the
payload rule (`NOT_CARRIED` 85→84; the family paragraph that held it
— "a nested refusal whose arm has no inner word" — replaced by the
one saying both halves of that reading are spent), `meta_version_error_tag`
(three literals, the kernel's own arms) with its inventory row,
`edit_inner_variant_tag`'s `MetaUnversioned` arm pointed at it, and
the construction pin now covers **58 of 58** `EditError` arms; the
census row moves to `EditError.inner_variant` with the measurement
(no Python door mints `SetAppearanceMeta`, so the reach is the Rust
pin). No shipped word moved: the arm's inner word was `None`. The
six: FIVE carried onto the prelude as one stanza beside the refusals
that name them — `ContactKind` (three self-intersections, three
repairs), `EscalationSite` (which stage escalated; the recourse forks
on it before it reads the band) with `SegmentRef` under it (a seventh
name, the lane's stated deviation: the rung under `EscalationSite`
and the direct payload of four other `ProfileError` arms, so
carrying one without the other stops one short), `FilletLeg` (the
one thing a caller shortens), `FilletLegCarrier` (decides the UNITS
of the setback beside it), and `NoCornerReason`, decided beside
`PathNoCornerReason` as the item asked — the pair closes. `Step` NOT
carried, argued in `prelude.rs` with its falsifier: the RECORDING half
of the profile layer (program, replay door, its refusal, the
structure record) is uncarried whole, and the element type is the
last thing a replay caller needs; flips if a prelude door ever takes
or returns a recorded program. `carried_refusal_payloads_are_matchable_through_the_prelude`
gains the profile rung by bare prelude name. Census: five `INTERIOR`
by the carrier rule with the measurement (`ProfileError` crosses one
word per arm at `EvaluationError.inner_kind`; `PathError`'s `corners`
rows flatten the anchor arm to a word), `NoCornerReason`
`different-shape` with the corner family whose rows already carry
its two words. **The re-sweep** at the merge base: banked set EMPTY,
no new hit — and the methodology finding the item asked to be
recorded is now its own file,
`payload-rung-sweep-is-prose-and-a-third-run-disagrees`: three runs,
three sets of numbers (raw hits 115/137/208 with no tree change
accounting for the last jump; CUR6's own table and prose disagreed by
one row), plus the blind spot this run adds — every implementation
reads three of the façade's FOUR curated lists, and all six hits were
already on the fourth (`pncad/src/profile.rs`), which did not make
them false positives (matchability THROUGH the prelude is what a
curated list owes) but means the scan cannot tell "uncurated" from
"curated on a list I do not read". What closes it is the sweep as a
committed script reporting WHICH list. Orchestrator note: that item
is the right next CUR unit, and a mechanical one. No kernel crate
touched beyond `pncad`'s lists; the lane ran the pinned ruff for
real. ~77 min, ~244k tokens.

**LIB-SMALL MERGED (2026-09-08, #2240; mechanical under the 08-29
ruling, no A/B row). Three small closes, one file each.** (1) The
refactoring doors carry their maintenance: `SplitOutcome` gains
`remainder_maintenance`/`part_maintenance` and `InlineOutcome` gains
`maintenance`, each straight off the kernel outcome, and the three
`Doc`-minting getters hand the record across with the document
instead of `Vec::new()` — ALL THREE sites (the brief miscounted: the
"third site" was `InlineOutcome.doc`, the inline wrapper's own, the
same defect on the same ground). No getter added: the record reads
off `Doc.last_maintenance`, the door that already answers the
question; its doc and `Doc::accept`'s funnel note say why the
refactoring wrappers are the one family that does not pass through
the funnel. The funnel test's new row cuts the bench stand's whole
cluster out: part `["join","join"]`, remainder `["split","split"]`,
inline back `["join","join","drop"]` — non-empty on every door,
because an empty record is indistinguishable from "nothing moved";
the input document's own reading asserted UNCHANGED (a pure door).
(2) `Datum.in_plane` reads its origin back DIMENSIONED — decided for
`Length`, the bare-shape argument weighed and rejected AT the field:
being written in a frame's coordinates changes the datum a position
is measured from, never its dimension, and the write door had already
settled it; the direction pair stays bare under `py/place.rs`'s rule.
Stub, ty fixture, `TestDatumReadback` (`0.25 * m` in, a `Length` equal
to it out, the sibling `origin` asserted beside it); no census row
moves (no curated Rust name maps to the field). (3) Two guide steps
before `### Hollowing a body`, where G17's spec said the shell step
sits "beside" them: chamfer as fillet's twin (same frozen selection;
setback not radius; planar supports only; the closed form metered;
the chamfer takes more than the fillet at the same number;
`chamfer_selection_empty` caught) and tubes (the five intent
parameters, then the hollow tube's REQUIRED wall with `minor_radius`
as the outer radius, an arc window, and the solid-minus-hollow bore
differential); `test_guide.py` executes 38 blocks, up from 36. The
audit's `arc_continue` line stays open until BOOL-10 (#2135) merges.
No kernel crate touched; the lane ran the pinned ruff for real.
~65 min, ~182k tokens.

**LIB-WILDCARDS MERGED (2026-09-09, #2243; mechanical under the
08-29 ruling, no A/B row). The last two payload-accessor wildcards in
`pncad-py`: `CheckEvidence`'s five accessors and `RefusedRef`'s two
are exhaustive.** `CheckEvidence` gets a Python-independent record
(`check_payload.rs`, the `mate_payload` shape; `reason` a `Cow` —
borrowed from the separation arm's own sentence, owned where the
shell arms render one) and `RefusedRef` stays in place, matching the
`at` accessor it sat beside — and the deciding reason is
`pick_payload`'s second one, not arm arithmetic: THREE of
`CheckEvidence`'s six arms are unreachable from Python (`escalated`,
`unsupported`, `separation_unavailable`), and the `py/` accessors
compile only under the `python` feature, so an in-place projection
would have pinned those arms nowhere. The lane named this as a
deviation from the brief, which offered in-place matches as possibly
cheaper AND asked for a construction pin — the two cannot both hold;
the record is what the pin costs. Orchestrator note: the brief's
error, correctly resolved. The added-arm alarm run and reverted: a
seventh `CheckEvidence` arm fails `cargo check -p pncad-py` with no
features at the record's match and at `tags.rs`. Pin covers four of
six arms (the shell arms hold a `ShellClassifyError` the façade does
not re-export), with the separation arm's sentence itself asserted.
The sweep re-run at the merge base with the item's stated blind spot
closed (named-binding and `Some`-of-default spellings): the added
spellings found nothing the original pattern missed; remaining hits
are the argued extracts (`validation.rs`, `py/doc.rs`'s `Node`
extracts, `prose_census.rs`) and exhaustive named arms under `use`
aliases; the pattern's own blind spot (line-local; a rustfmt-split
arm or a helper-returning catch-all) stated. `py/checks.rs` and
`py/assembly.rs` have no `_ =>` left. Residue:
`check-evidence-shell-refusal-crosses-as-prose-only` (the shell
door's typed refusal under two arms crosses as prose only; needs the
curation half first) — its sibling `SeparationUnavailable { kind }` is
already on FIX's slate (`boolean-kind-not-published-at-the-python-door`),
reported rather than re-filed. No attribute added, renamed or
removed; stub, census and Python suite untouched; the lane ran the
pinned ruff for real. ~47 min, ~159k tokens.

**`[ev]` PR 2230, item 1 RULED (2026-09-09, Ev): (A).** "A works":
every payload DISCRIMINANT of a projected refusal crosses as an
attribute of its own, named per type, `None` on every other arm;
arena-key fields still do not cross. Recorded on
`two-validation-payload-discriminants-still-uncrossed` and split onto
its own docs-only PR (#2244) so LIB-DISCRIMINANTS (`stale_kind`,
`ring_contact_kind` on `ValidationFinding`, two exhaustive maps) can
dispatch without waiting on item 2, where Ev asked why (A) over (B)
for `datum-crosses-name-for-name-as-two-types`; answered on the PR
(B fixes the instance, A the shape; B renames shipped surface; A's
cost is a stated hand-maintained list). #2230 merges when item 2 is
ruled. The witness-bifurcation arm falls under the same rule and
waits on the M6 solver constructing it.

**LIB-HASH MERGED (2026-09-08, #2242; mechanical under the 08-29
ruling, no A/B row). Every comparable enum mirror hashes.** All 24
fieldless mirrors (not the item's 23: `AssertionDir` arrived between
filing and fix, the argument for a guard over a roster) carry `eq,
eq_int, frozen, hash` and derive `Eq, Hash`; `frozen` IS required —
established from the pinned pyo3 0.29.0's `pyclass_hash`, not assumed
— and costs a fieldless mirror nothing, said once at the first
mirror. The item's question answered by reading every mirror's doc:
none is deliberately unhashable. `Denotation` — the class the item's
probe named as invisible to it — hashes by hand over the same
`(tied, candidates)` its hand-written `__eq__` reads, and the stub
declares the pair (the stub's own convention: hand-written dunders
declared, pyo3-derived ones not). `test_hashability.py` is enumerated
from the compiled module, never a written list: every member hashes,
the whole surface goes into one set and reads back out of one dict,
hash agrees with equality over every ordered pair, a door-minted tag
keys the same as the class attribute, two door-minted denotations are
one key — and `TestNothingComparesWithoutHashing` reads `__hash__`
off every class in `vars(pncad)` and requires an unhashable one to be
on the `UNHASHABLE` roster with a reason, so a 25th mirror without
`hash` or a new value class that compares without hashing fails.
Falsified for real: `frozen, hash` removed from `SurfaceKind` alone →
two failures and three errors naming it; restored. The face-frame
tally is the set it wanted to be. No kernel change (`editor_core::
Denotation` does not derive `Hash`; the binding hashes the projection
its `__eq__` reads). Two residue items filed inside the fence:
`pncad-py-value-classes-compare-without-hashing` (eleven classes
compare without hashing — `Expr`/`MeasureExpr` by design, stated on
the stub; NINE undecided in three shapes: small value records,
findings, reports/configs — each owing the `-0.0` fold `DocParam`
already does) and `pncad-py-stub-omits-eq-on-three-mate-classes`
(`MateFrame`/`MatePrimitive`/`Alignment` define `__eq__` the stub
omits, unverifiable by `test_stubs.py`'s `hasattr` guard by
construction). No `variant`/`kind` value or ordering moved; the lane
ran the pinned ruff for real. One red the lane owned: the roster spelled a filed item as a `.md` string literal under `crates/`, which `ci-filter` fails closed on (it guards a consumed page dropping into the docs tier) — fixed by naming the item without the suffix, the selftest added to the lane's local run. ~3h20m (mostly CI polling and three merges of main), ~190k tokens.

**`[ev]` PR 2230, item 2 RULED (2026-09-09, Ev): (D) — the name
match accounts MEMBERS, not the type.** Asked as (A) a hand-kept
same-spelling-different-type list, (B) rename the read-side class,
(C) leave it; Ev asked for a structural check that keeps the name
match, and (D) is that: a same-spelled Python namesake accounts only
the arms (or pub fields) it spells, every other arm needing its own
`BOUND_AS`/`NOT_BOUND` row. It would have caught `Datum::FaceFrame`
and the `Node::Union`/`DocEdit::SetMembers` gap alike, and it reuses
LIB-SWEEP's declaration resolver. Recorded on
`datum-crosses-name-for-name-as-two-types`; the unit (LIB-MEMBERS)
dispatches after LIB-SWEEP lands. Both items of #2230 are now ruled
and the PR merges.

**`[ev]` PR 2231 RULED (2026-09-09, Ev).** Item 1
(`a-recipe-cannot-hold-a-narration-body-without-it-becoming-a-root`):
Ev suspected every option changed kernel code for a demo's benefit,
and the tree agreed — roots are exactly the sink set, and a
`Node::Measure` consumes its bodies as edges while denoting no body,
so "measured, not modelled" already exists in-graph; the tour's cost
is self-inflicted (it measures out of graph). Ruled **(D)**: no node
or root change, closed as the demo's own cost; the residual E3
question (mass properties as a measure primitive) explicitly NOT
filed. Item 2 (`no-facade-door-mints-a-revolves-role-names`): Ev asked
whether (A) over (B) was just less machinery — yes (an extension
trait on an editor-core id vs free functions beside `SegPat::tag`) —
and ruled **(A)**; the unit (LIB-NAMES) dispatches when a slot frees.
Orchestrator note: item 1 is the second time today a filed design
question dissolved on a closer read of the tree (the seat existed);
the lesson for the filer is to grep for the seat before asking.

**`[ev]` PR 2232 RULED (2026-09-09, Ev): (A), (A), (A) — what a
curated door owes on its SUCCESS side.** Item 1
(`a-successful-step-imports-own-report-is-uncurated`): carry
`StepImport` and its record vocabulary at the façade under the reach
clause, and give Python an `ImportReport` value class beside the
body. Item 2 (`pncad-py-import-step-drops-the-gates-enclosure`): the
report carries `.body` and `.enclosure`; `Body` stays a pure handle —
ruled after Ev asked whether (A) was test-only and (B) not, and the
premise was corrected (both change binding code; (A) at the door's
return shape, (B) on the handle). Item 3
(`advisory-monte-carlo-lane-has-no-python-door`): `monte_carlo` and
its value classes bound as LIB's build, `analysis.rs` joining the
census's read list — "no real hurry", so it queues behind the
mechanical units. Items 1+2 dispatch as one unit (LIB-IMPORT-REPORT).

**LIB-SWEEP MERGED (2026-09-09, #2245; mechanical under the 08-29
ruling, no A/B row). The payload-rung sweep is a committed script,
`scripts/payload-rung-sweep.py`, run by both halves of CI rather than
re-derived from prose.** It reads ALL FOUR curated façade lists
(`document`, `select`, `prelude`, `profile`) and reports which list
each side of a row is on, which splits what a three-list run
flattened: UNCURATED (on no list) from CROSS-LIST (curated, on no
list that carries its carrier). Four counts, a deterministic narrowed
table with `file:line`, `--json`, `--lists` to reproduce an earlier
run's definition, `--check` pinning the narrowed NAMES against two
disposition tables held as data with each argument's home
(`DISPOSITIONS`, `CROSS_LIST_DISPOSITIONS`) in both directions, and a
`--selftest` fixture battery; blind spots (a)–(i) indexed in the
docstring and stated beside the code that has each ((a), (f), (g)
CLOSED; (e) narrowed to macro-minted names; (b), (c), (d), (h) open;
(i) new — registry dependencies are outside the path closure). The
CI row sits in `discipline` on both halves under a `HOSTED MIRROR`
marker. **The drift, bounded**: `curated` settles INCLUDING CUR5's
unexplained 525 (CUR5 read the façade's whole source directory, not
the three lists — 536 there today vs 413 over three lists, the same
gap); `declared` is monotone with the tree; `narrowed` reconciles
EXACTLY (CUR7's 16 names minus the six it settled = the ten the
four-list run reports; the three-list run adds only `Step`, which is
curated BESIDE its carrier on `profile.rs` and so is not a rung there
— CUR7's prelude non-carriage stands); `raw` does NOT settle and is
bounded instead (121 as specified; 146/160/185/188 with each closed
blind spot re-opened; the reported 115 and 208 lie outside that
interval, so the difference is what an implementation counted as one
hit, which no artefact records). The refusal filter is a NAME test
(`*Error`/`*Refusal`/`*Fault`), measured against a `Display` test
that does not separate refusals from five carried discriminants —
the lane's stated deviation from the brief's argued list
(`LeverRefusal`, `MintRefusal` dropped by the filter, not by a row).
Uncurated column at the merge base: no new row. Cross-list column:
four rows over two names, filed as
`cross-list-payload-rungs-under-document-only-carriers` (`EntityKind`
and `SplitHalf`, on `select`+`prelude`, ride document-only carriers;
the `NamingError` precedent on `select.rs:57` argues one way, the
document list's own payload rule the other — a curation act either
way, and it settles whether a payload owes its carrier's list or the
vocabulary's), pinned in the script so a third such name reds CI.
Orchestrator note: this is the resolver option (D) on `[ev]` #2230
item 2 would reuse. Also measured by the lane: the selftest discriminates — nine mutants of the scan each red the battery; two runs byte-identical. ~92 min, ~208k tokens.

**LIB-DISCRIMINANTS MERGED (2026-09-09, #2246; mechanical under the
08-29 ruling, no A/B row). The first unit under Ev's (A) on
`two-validation-payload-discriminants-still-uncrossed` (ruled this
morning on `[ev]` #2230): `ValidationFinding` carries `stale_kind`
(`StaleDeclaration`, four arms — which declared record lost its
witness, which is which record to withdraw or re-seat) and
`ring_contact_kind` (`RingContact`, three arms — how a ring meets its
face's outer loop, which is where the ring has to move), each from an
exhaustive tag map with no wildcard, `None` on every other arm; the
arena-key FIELDS do not cross, so the projection stops at the
discriminant as everywhere else.** `ring_contact_tag` reuses the
census vocabulary's spellings where the shape is the same
(`vertex_vertex`, `vertex_on_edge`, `edge_along_edge`), so a caller
reading two contact words off one finding learns one spelling; the
escalated sibling (`RingContactEscalated`) answers `None` — an
undecidable separation is a margin, not a shape. Reachability, the
part worth recording: NO arm of either enum is reachable from
Python — a stale record needs a declaration parted from its witness
and every door mints declarations from the geometry it looks at or
gates them; a ring on its own outer loop needs raw Euler surgery the
binding does not expose — so all seven words are pinned per arm in
Rust with the reason, and `test_validate.py` says the gap is the
DOORS' rather than the projection's; crossing them is still the move,
because the attribute is the contract a caller reads the day a door
produces one. Census: both rows from `INTERIOR` to `BOUND_AS` with
the measurement, and the door's four payload discriminants are now
the RULE rather than a pair. `witness-bifurcation-arm-has-no-inner-word`
gains its `## Progress` line: the rule applies, the arm waits on the
M6 solver constructing it. Stub, class docstring, `__repr__`/`__eq__`/
`__hash__` over six words, ty fixture. No shipped `variant`/
`contact_kind`/`subject_kind` value moved; the lane ran the pinned
ruff for real. The lane's stated deviation: the two extractors keep `census_contact`'s extract-licence `_ => None` (the brief asked for that AND for arms named — over 71 arms the worked example wins; the classifying site `validation_error_tag` is the exhaustive one). ~72 min, ~207k tokens.

**LIB-MEMBERS MERGED (2026-09-09, #2247; mechanical under the 08-29
ruling, no A/B row). Ev's (D) on `datum-crosses-name-for-name-as-two-types`
executed: the binding census's rule 1 keeps its name match, but a
match accounts MEMBERS, not the type.** A curated name that resolves
— through `scripts/payload-rung-sweep.py`'s resolver, SHARED by path
rather than re-implemented (the script gains `declared_members`, the
second reader on the same declarations, with selftest rows and two
fixture types; its own report unchanged) — to a `pub enum` or `pub
struct` accounts only the members its Python namesake spells (an arm
as a class attribute, or snake-cased as a constructor/property/
method; a bare-`pub` field as a same-named attribute); every other
member owes a row in `MEMBERS_BOUND_AS` or `MEMBERS_NOT_BOUND`, and a
member with neither fails the census naming itself. A second pair of
tables, not `Type::Member` keys in `BOUND_AS`, because every check
over that roster reads its keys as curated names. **The count, taken
first**: 104 matched declarations, 616 members, 420 over 60 types
needing rows — and `Node`/`DocEdit`/`Datum`, the expected bulk, are 18
of the 420; the refusal enums whose arms cross as tag WORDS dominate
(`ValidationError` 71, `EditError` 58, `PathError` 30, …), and the
rule admits no exception, so they got rows (385 `BOUND_AS`, 35
`NOT_BOUND`) — recorded as the measurement the ruling's cost estimate
lacked, not used to narrow the rule. What the first run FOUND, which
is the whole point: a SECOND same-spelled-different-type pair
(`pncad.pyi`'s `DimensionError` is the quantity boundary's refusal;
the curated one is editor-core's document-layer refusal, crossing at
`ParseError.kind`); and three real gaps chartered as census families
and filed — `two-datum-arms-have-no-node-constructor`
(`Datum::Point`/`Frame`: a Python author builds four of six datum
kinds), `five-doc-edit-arms-have-no-python-door` (`SetParam`,
`SetExpression`, `Rebind`, `ReWitness`, `ReWitnessBulk`),
`mesh-boundary-polylines-have-no-python-door` (`Mesh::boundaries`
unread; a wireframe cannot be drawn). Blind spots stated at rule 1: a
coincidental snake-case match (four of `Evaluation`'s ten fields are
accounted that way against a different Python type); the resolver's
alias/generic arms (b)/(d) and crate-not-module (h); a tuple struct's
unnamed fields. Falsifier: `Node.extrude` removed from a stub surface
in-file fails the rule; `Datum::FaceFrame`'s row removed fails naming
it. No stub change, no new binding; the lane ran the pinned ruff for
real. Orchestrator note: the three gap families are the next
mechanical wave's obvious members. ~66 min, ~296k tokens.

**LIB-IMPORT-REPORT MERGED (2026-09-09, #2249; mechanical under the
08-29 ruling, no A/B row). Ev's (A)+(A) on `[ev]` #2232 items 1 and 2
executed: `import_step`'s SUCCESS side crosses.** Rust half: the
prelude's `step_import` group gains seven names — `StepImport` and
the record vocabulary its `Solid` arm names (`StructureNormalization`,
`NormalizationKind`, `CurvePromotion`, `PromotedCurveKind`,
`PlacedInstance`, `FaceCensus`) — under the reach clause, with the
prose saying which clause each half closes (the refusal was a
MATCHABILITY gap, the answer a REACH one); the `Wireframe` arm needs
no new name (`Curve3` is already on the list); a curation guard in
`all.rs` binds every field of the arm by name from the prelude alone
on a REAL import (the round-trip oracle's own exported text) and
asserts the enclosure against a second `mass_properties` call bit for
bit — "not a second computation" as an equality rather than a claim;
`NormalizationKind` matched exhaustively there. Python half:
`import_step` answers a frozen `ImportReport` with `body`,
`enclosure` (the gate's own certified `MassProperties`), `eps_in`,
and the three record lists as frozen rows (`StructureNormalization`,
`CurvePromotion`, `PlacedInstance`, with `FaceCensus` a class of its
own because a normalization carries TWO and their difference is the
record); `Body` gains nothing, `mass_properties` keeps one meaning —
the ruled shape. Decisions stated: `surface_promotion` keeps one word
with `promoted_to`/`residual` beside it (five words, not six);
`PlacedInstance.placement` is `Optional[Frame]`, `None` left as `None`
(an identity `Frame` would read as a map the file chose). Deviation:
`MassProperties` gains `Clone, Copy` + `skip_from_py_object` so the
Python surface is unchanged. Census: `StepImport` → `ImportReport`
with the double-quadrature measurement, the two discriminants to
their `kind` words; the four row classes are same-spelled and
accounted by rule 1 (a `BOUND_AS` row for them is stale by the decay
check — found that way). Callers moved onto `.body`/`.enclosure`
(`test_document.py`, `examples/bracket.py`, the guide's Python
journey). The payload-rung sweep stays green with seven more curated
names (narrowed 10, cross-list 4 unchanged). `ImportOptions`'
non-crossing stands. Orchestrator note: this lane was killed by the
container restart with six files uncommitted and resumed with "push
first"; it committed the Python half, closed both items and opened
the PR within forty minutes — the discipline's push-after-every-commit
rule is what bounded the loss to one turn. The lane ran the pinned
ruff for real. The measurement, taken: import-then-measure on a lofted rational-walled body 8.26 s + 8.24 s — the second quadrature is 49.9 % of the old journey and `.enclosure` is bit-identical; on the box 0.3 %, because a box has no quadrature to repeat. ~85 min including ~11 min of restart downtime, ~230k tokens.

**LIB-NAMES MERGED (2026-09-09, #2248; mechanical under the 08-29
ruling, no A/B row). Ev's (A) on `no-facade-door-mints-a-revolves-role-names`
executed: `band`, `band_pi`, `band_rim`, `meridian_vertex` and
`carried` are free functions returning a `StableName`, reached at
`pncad::select`.** DEFINED one crate down, in
`crates/editor-core/src/names/role.rs` beside the `RoleSeg` arms they
mint, re-exported through `names::mod`, `editor_core`'s root and
`pncad::select` — the lane's stated decision, because
`crates/editor-core/tests/corpus/` cannot depend on `pncad` and
defining them there is what lets the corpus share them
(`names/README.md`'s own module table puts the vocabulary in
`role.rs`). Signatures the ruling's verbatim, `meridian_vertex(end,
node, vertex)` with the end first as the arm carries it; each fixes
the `EntityKind` its role denotes (`Face`/`Face`/`Edge`/`Vertex`),
`carried` takes the inner name's kind (a survivor is the same entity
one op later) — the field a hand-spelled name got wrong silently
until emission refused it. Consumers converted: the tour (five
private helpers deleted), its test, the corpus `vessel.rs` (its two
`pub` helpers deleted and their G17 callers moved), and the
hand-spelled `BandRim`/`FromTarget` sites in six more editor-core
test files. Five pins, one per builder, each asserting the exact
`StableName` against the hand-spelled form it replaces. Carried at
`select` only, not the prelude (its group 9 is hand-curated with its
own argument). Two residues filed:
`pncad-py-has-no-door-that-mints-a-revolves-role-names` (Python
speaks names as text and hand-writes the `StableName` JSON at the
fillet and shell doors — the same gap one alphabet over, worse) and
`the-role-name-builders-reach-only-the-outer-profile-loop`
(`loop_index: 0` fixed by the `u32` signature; a hole's band is
still hand-spelled at two test sites; three shapes weighed). **Two
orchestrator notes.** (1) The lane's first CI head was RED on the
python suite: re-exporting five names at `pncad::select` made them
curated names the binding census must account for, and no rows were
written — fixed with a new census family `B-NAME-BUILDERS` (a
`FAMILIES` charter for the five Python doors the filed item asks
for) and five `NOT_BOUND` gap rows citing it. My brief's verification list omitted
`run-python-tests.sh` because the unit "touched no Python"; a
curated-list change is a census change, and the brief should have
said so — recorded as the brief's gap. (2) A container restart at
~03:10Z killed this lane mid-CI-poll (clean, pushed) and
LIB-IMPORT-REPORT mid-edit (six files uncommitted); both resumed via
SendMessage with "push first", nothing lost — the "push after every
commit" rule held, and the restart cost was one re-poll. No kernel
behaviour change; tour renders and die corpus byte-identical per the
lane; the lane ran the pinned ruff for real. Thirteen sites in eight files converted; the sites NOT converted are listed with reasons (the emitter, the kind pins, a `BandRimPi` pairing with no builder). ~2h (incl. the restart), ~232k tokens.

**LIB-GAPS-1 MERGED (2026-09-09, #2250; mechanical under the 08-29
ruling, no A/B row). Two of the three census families LIB-MEMBERS'
member rule chartered, closed: `Node.datum_point(position)` and
`Node.datum_frame(origin, u, v)` beside the four datum constructors —
a Python author now builds six of six datum kinds — and
`Mesh.boundaries`, the tessellation's boundary polylines.** The
datum doors take the sibling shapes (`Length` triples for positions,
bare direction triples matching the slots' `Scalar`); `datum_frame`'s
`u`/`v` are authored freely and orthonormalized at evaluation with
`u` kept, so a merely non-perpendicular pair is legal and a parallel
one refuses `degenerate_direction` naming the axis; a point refuses
nothing at evaluation (a non-finite coordinate refuses at the door,
as every literal does). `Node.sketch_frame` still mints the same
`Frame` arm from a `SketchPlane` value; `datum_frame` is the arm's
own spelling, the two documented against each other. The read side
needed no extension — `Value.datum()` already answered `"point"` and
`"frame"`; the arms were readable and only unauthorable, exactly as
filed. `Mesh.boundaries` answers `list[list[int]]`, one polyline of
position indices per model edge in the kernel's edge order — the
same opaque alphabet `Mesh.triangles` speaks — with two stated
decisions: no value class (the polyline's other three fields are
arena keys the curation keeps unnameable, so a class would hold the
indices and nothing else; the pairing that makes an index a handle
already exists as `NodePick.boundary_names`, entry for entry), and
answered whole rather than by index (a patch is indexed because
`triangles` concatenates them and separability must be recoverable;
polylines have no concatenated spelling to be separable from — the
list IS the door). Closure decided on indices, never coordinates, as
watertightness is. Stated deviation from the brief: the
`Mesh::boundaries` census row is DELETED rather than moved — the
member rule accounts a same-named attribute on the namesake, and the
roster-decay check refuses a row for a member the namesake now
spells; the two `Datum` rows move to `MEMBERS_BOUND_AS` (the
read-side `Datum` spells no arm); `B-DATUM-DOORS` and
`B-MESH-BOUNDARIES` leave `FAMILIES`. Rows: read-back, both
downstream doors (a sketch plane on the frame, a distance to the
point) and both refusal shapes for the datums; count, index
validity, segments-are-triangle-edges, closure and name pairing for
the polylines; stub, fixtures, the north-star roster (the one the
census cannot see). Three stale sentences saying the polylines were
unbound corrected. No kernel change; the lane ran the pinned ruff
for real. Five stale sentences saying the polylines were unbound corrected (module header, the pick door, the stub, the meshing guide page, the audit's G11 row) — the guide page also wrongly said no index-to-name door existed on either side; `NodePick.patch_names`/`boundary_names` are that door and the page names them now. ~64 min, ~240k tokens.

**LB21 — wave 5 planned from the slate after wave 4 (2026-09-09,
after LIB-NAMES landed).** Wave 4 (LB20) closed thirteen units in
one day — DOORS-2, PROJ, DOORS-3, CUR7, SMALL, WILDCARDS, HASH, SWEEP,
DISCRIMINANTS, MEMBERS, IMPORT-REPORT, NAMES, plus three `[ev]` PRs
ruled and merged (#2230, #2231, #2232) with #2233 still open — and
the slate is 21 open issues, five of them filed by wave 4's own
units. Sorted by what they wait on. RUNNING: LIB-GAPS-1 (the two
datum constructors and the mesh boundary polylines) and LIB-EDITS
(the five `DocEdit` constructors) — both MEMBERS' findings. QUEUED,
mechanical, briefs written: LIB-PYNAMES (the five role-name builders
in Python, answering the name text the fillet and shell doors take —
NAMES' residue and the `B-NAME-BUILDERS` family); LIB-CUR8 (three
curation decisions under LB17: `MarginDiag`/`BandField` re-measured
on the count DOORS-2 moved, `ShellClassifyError`'s carriage under
`CheckEvidence`, and the cross-list rule SWEEP surfaced — whether a
payload owes its carrier's list or the vocabulary's); LIB-SMALL-2
(the two recourse sentences S-MATE handed to LIB, the stub's missing
`__eq__` on three mate classes with the guard that would have caught
it, the façade guard's last three line-local readers through
`code_only`; the audit's `arc_continue` line rides along only if
BOOL-10 #2135 has merged); LIB-MC (the advisory Monte Carlo door,
ruled (A) and unhurried per Ev). AFTER CUR8: LIB-MATE-PAYLOAD (the
`MateFault` arms whose payload does not cross — the two `DocumentId`s
mechanical now, `Frame`'s in the frame door's vocabulary, `Band`/
`Indeterminate` per CUR8's decision, `Unleverable` still behind a type
editor-core does not re-export). AFTER `[ev]` #2233: item 1 (A) — the
fifteen slot doors accept `Length | WrittenLength` — and item 2 (B′)
— the quantity newtypes compare as Rust's `PartialOrd` and stop
hashing — then LIB-HASH-2 applying the same mirror rule to the nine
value classes `pncad-py-value-classes-compare-without-hashing` lists.
WAITING on other programs or on Ev: `path`'s door (the kernel
`PathError` discriminant, SMELL D37/D39); the witness-bifurcation arm
(the M6 solver); `correspondence-structs-coincide` (the verb seat);
`no-door-mints-mate-frame-from-face` (S-MATE's by number, a
freeze-at-authoring design question — the next `[ev]` batch);
`the-role-name-builders-reach-only-the-outer-profile-loop` (a
signature choice with three shapes weighed, low stakes — the same
batch). Orchestrator notes for the record: (1) a curated-list change
IS a census change — every brief that touches a `pub use` list now
carries `run-python-tests.sh` in its verification list, after NAMES'
red; (2) the container restart at ~03:10Z cost one re-poll and zero
work, because every lane pushes after every commit — the rule earned
its place today; (3) merging the second of two lanes that touched the
census file has needed a main merge and a CI re-run every time this
wave; the file is the shared seam of the binding, and two lanes on it
at once is a cost to weigh at dispatch, not a defect.

**LIB-EDITS MERGED (2026-09-09, #2251; mechanical under the 08-29
ruling, no A/B row). The third of LIB-MEMBERS' chartered families,
`B-DOC-EDITS`: TWO doors of the five, and the three that could not
be built each stopped for a reason the lane verified by trying.**
`DocEdit.set_param(node, slot, expr)` — a continuous slot's
expression on a live node, the slot named by its WORD (the same word
`EditError.slot` answers in, so a refusal is an address a caller
retries at unchanged; D5: a name, never an index); the alphabet is
read inward by a new `slot_word.rs`, sited apart from `tags.rs`
because the tag-value guard reads that module as a table of
`-> &'static str` functions and would stop at one answering a
`SlotId`; the round trip is pinned against the guard's own inventory
(every word reads back to the slot that spells it, `profile` being
the one word with no reading — a program expression's address is two
integers and an argument role the word does not carry, refused in its
own sentence), and junk words are boundary `ValueError`s. One door
for every continuous slot, against one door per structural slot: the
decisions differ because the vocabularies do. `DocEdit.rebind(from_name,
to_name)` — THE name repair, halves suffixed as `EditError.from_kind`/
`to_kind` are (`from` is a keyword), one test row per tag it can raise
and a success row on the live shell document `test_shell.py` said the
Python surface could not rebuild through. The three that stopped:
`ReWitness`/`ReWitnessBulk` carry `WitnessDatum` and
`BranchCertification`, which the façade does not curate at all (filed:
`the-witness-edits-need-a-facade-type`); `SetExpression` is mechanical
and its payload curated, but its own refusal `path_off_tree` renders
the address through `Debug`, and the binding's prose gate — a
`debug_assert` the workspace keeps live under release — panics on the
brace fingerprint, so the door would PANIC exactly where it must
refuse. The lane wrote the door, provoked the refusal, saw the panic,
and parked the item on DOCM's `debug-in-prose-residue-after-finding-sink`
(`the-expression-path-edit-cannot-refuse-as-prose`; the
`KNOWN_BRACED` row now says it blocks a door). Census: `SetParam` and
`Rebind` leave `MEMBERS_NOT_BOUND` entirely (namesake for namesake,
rule 1 accounts them); `B-DOC-EDITS` stays in `FAMILIES` with three
rows and a charter re-written to say what each needs first.
`DocEdit` has fourteen static constructors; five node constructors
name the door that moves the literal they mint. No shipped
`variant`/`kind`/`slot` value moved; the lane ran the pinned ruff for
real. Orchestrator note: the brief said "STOP on that arm, file it,
and say so" for a type the façade does not carry, and the lane did
that for two arms and found a third stop the brief did not
anticipate — a refusal that cannot be rendered — which is the better
outcome than a door that panics. Seven tags reachable only through these arms are now provoked from Python; two (`rebind_appearance_collision`, `rebind_metadata_collision`) stay unreachable behind the uncurated appearance doors, stated. ~74 min, ~324k tokens.

**LIB-PYNAMES MERGED (2026-09-09, #2252; mechanical under the 08-29
ruling, no A/B row). NAMES' residue closed: the five role-name doors
exist in Python — `band`, `band_pi`, `band_rim`, `meridian_vertex`,
`carried` — as module-level functions beside `select`, each minting
the kernel's own `StableName` through `pncad::select`'s builder and
answering `name_text`'s output, so the text a door answers is
BYTE-IDENTICAL to what a materializer answers for the same entity:
one alphabet, minted on either side of the boundary.** That
byte-equality is the whole claim and the pin: the left side of every
row in `test_role_names.py` is a name Rust minted and serialized (a
materialized selection), the right side the Python door's answer on
the same arguments — a door agreeing on shape and not bytes would
author a selection that resolves to nothing, which is exactly the
failure a hand-written name has. Two scenes, the two shapes a full
revolve takes: a profile clearing the axis (one face per segment,
whole-circle rims) and one touching it (the pole splits every band
into its `[0, π)` and `[π, 2π)` halves — why `band_pi` exists).
`MeridianEnd`'s mirror gained ONE kernel mapping (`to_kernel`) used
by both the selector side and the minting door — a second copy would
be a second answer to "which end is Seam". The text stays opaque: a
caller composes by naming a ROLE, never by assembling the
serialization. The outer-loop limit is inherited and restated (a
hole's band is reachable from neither alphabet). Census: all five
names left `NOT_BOUND` and needed no `BOUND_AS` — the stub declares
them at those exact spellings, so rule 1 accounts them — and
`B-NAME-BUILDERS` left `FAMILIES`; the `StableName` `NOT_BOUND` row's
reason sharpened (a name is `str` on this side; the doors mint by
role and answer opaque text, which is why they are module doors and
not methods on a name class). No conversion sites existed — the
suite and the guide selected off evaluations, never hand-writing a
name — so the doors' positive form is the new test file, and the
guide gained a STEP ("Naming a role before the body exists",
executed like every block) because the section teaching selections
said the opposite of what is now true; prose that had gone false
("there is no name-building vocabulary in Python", "never composed")
was fixed, the surviving invariant being "a name is never READ or
assembled". `meridian_vertex` takes the bound `MeridianEnd` mirror,
not a word. The two volume oracles are asserted to a relative 1e-12
(π summed in a different order than the kernel accumulates), stated. Two `legal.py` fixture variables were renamed because they shadowed the new module-level `carried`. No shipped word moved; the lane ran
the pinned ruff for real. Orchestrator note: the pre-read of the first commit matched the report line for line; the mid-flight merge of EDITS (census, stub, both ty fixtures) was clean and every row was re-run on the merged tree. ~59 min (~9 of them waiting on the build slot), ~253k tokens.

**LIB-CUR8 MERGED (2026-09-09, #2253; mechanical curation under
the 08-29 ruling, no A/B row). Three curation decisions, each with
its falsifier written where the next sweep reads it, and the sweep
itself is now the re-sweep.** (1) `MarginDiag` CARRIED at the prelude
beside `Indeterminate`/`Band`/`BandError`: the old non-carriage's own
trigger fired — a door projects the escalation's SHAPE (the frame
constructors fork on the three margin arms: a value, an enclosure's
two bounds, or nothing for a poisoned margin), so the discriminant is
read at a boundary AND varies; the argument distinguishes reading the
arm (whether there was a number at all — three different next moves)
from branching on the margin (recovering the sign the classifier
refused, which the escalation contract forbids). Falsifier: no door
projecting the shape anywhere makes the type telemetry with no
consumer. `all.rs`'s `escalation_is_readable` now matches the three
arms exhaustively by bare prelude name and BUILDS the struct it
reads. (2) `BandField` NOT carried, argued anew on the new count: the
word is read at a boundary (`band_field_tag` crosses it on the frame
refusal) but still not VARIABLE — every producer reaching the
crossing is `Band::linear`, whose `zero` check cannot fire under
`Tol`'s invariant, so the two arms are an exhaustive match's drift
alarm; falsifier restated (a kernel caller of `Band::angular_at`, or
a door taking a band's thresholds from its caller). (3)
`ShellClassifyError` CARRIED at `pncad::document` beside
`CheckEvidence` under that list's payload rule, and projected as an
inner word: `shell_classify_error_tag` (four literals, exhaustive:
`band`/`props`/`escalated`/`zero_volume`), `inner_variant` on the
payload record (the exhaustive destructure grows to six) and on the
Python `CheckEvidence`, the stub, an inventory row; the construction
pin covers 6/6 arms where it covered four. `SeparationUnavailable
{ kind }` left to FIX, stated. (4) The cross-list rule decided for
the GENERAL case: a payload whose vocabulary one of the four curated
lists owns lives on THAT list, spelled once, and the carrier's list
points at it; a payload whose only home is the refusal holding it
rides its carrier (the `VerbKind` rule) — written beside the
`NamingError` precedent in `select.rs` and as the general sentence at
`document.rs`'s payload-rule header; `EntityKind` and `SplitHalf` stay
on `select`+`prelude`, and the sweep's `CROSS_LIST_DISPOSITIONS` rows
move from `filed` to `argued`. Sweep: curated 471, declared 821, raw
111, narrowed 9, cross-list 94 raw / 4 narrowed, `--check` green.
Three items closed (`margin-diag-non-curation-…`,
`check-evidence-shell-refusal-crosses-as-prose-only`,
`cross-list-payload-rungs-under-document-only-carriers`). No shipped
`variant`/`inner_variant` value moved; the lane ran the pinned ruff
for real. Orchestrator note: the lane found the closed item's central reach claim false (`pncad::topo` is a whole-crate re-export, so the binding could always NAME the type; what was missing was the curated placement and the tag) and repaired two doc comments carrying the same false-reach shape; `MateFault::Indeterminate` becomes buildable in the binding as a side effect, which is `mate-fault-arms-carry-payload-that-does-not-cross`'s to spend. The 6/6 pin was proved by blanking `inner_variant` on the two arms and watching the pin go red. ~74 min, ~233k tokens.

**LIB-MC MERGED (2026-09-09, #2255; mechanical under Ev's (A) on
`advisory-monte-carlo-lane-has-no-python-door`, no A/B row). The
advisory Monte Carlo lane has a Python door.** `monte_carlo(doc,
analyzed, config=None) -> McReport`, a module function on the analysis
surface; `analyzed` is REQUIRED (the box is the analysis's knob, E2,
and hiding a default inside the run would move that choice); the GIL
is released for the run. `McConfig(samples, seed, parallel)` frozen,
comparable and hashable, each field defaulting to the kernel's own
(`DEFAULT_SAMPLES`/`DEFAULT_SEED` on the module); a zero sample count
is refused by the RUN, not pre-checked at the constructor. `McReport`
/ `McMeasure` / `McAssertion` project every kernel field,
`violation_fraction` as `Optional[float]`; `McReport.render()` crosses
too (deviation, argued: the label discipline E11.1 requires — count and
seed on every line — is structural in the kernel, and stopping it at
the boundary would leave a caller printing unlabeled numbers).
`McMeasure`/`McAssertion` hash folding `-0.0` through `fold_zero`.
`McRefusal` typed under `PncadError` with `variant` from an exhaustive
`mc_refusal_tag` and `param`/`node`/`cause` on every arm; the band arm
DELEGATES to `measure_unavailable_tag` so one fault has one word.
`sample_offset(param, dist, u)` crosses free, answering in the
distribution's borrowed dimension. The bit-stability pin:
`parallel=True` and `False` answer the same bits (`.hex()` on every
field). Census: `analysis.rs` joined `FACADE_FILES` — curated 427→488,
63 introduced, all accounted (16 by rule 1, 6 `BOUND_AS`, 41
`NOT_BOUND` as `different-shape`: `OffsetInterval` and the 40 behind
`#[cfg(feature = "interval")]`, absent from the wheel's build) and the
census's `cfg` blind spot is stated at `FACADE_FILES`. The sweep took
the same file (`ALL_LISTS` is five; the two readers share one
resolver, so four-vs-five would be the "no two runs agree" defect one
layer up), with a selftest witness for blind spot (j) — a `cfg`-gated
`pub use` read as curated unconditionally. Six new rungs dispositioned:
`PairingViolation` argued, `KProbe` filed
(`kprobe-is-a-rung-under-drive-config-on-the-analysis-list`), and
`Dimension`/`Distribution`/`MeasureUnavailableAt` ARGUED under the
cross-list rule CUR8 ratified while this branch was open — the same
rule with the two lists swapped, and the clearest case it has; the
ratified prose in `document.rs`/`select.rs` now states the rule
without a list count. GUIDE's distributions rung gained the fourth
door. No shipped `variant` value moved; the lane ran the pinned ruff
for real. Orchestrator note: the lane reported "~5 h 45 m" of wall clock, which is not what the clock shows — launched 06:45Z, reported 08:10Z, so ~85 min, most of it the hosted CI run; the figure below is the measured one. ~85 min, ~629k tokens.

**LIB-SMALL-2 MERGED (2026-09-09, #2254; mechanical under the 08-29
ruling, no A/B row). Three small closes.** (1) The two refusals that
carried no recourse end on one: `CONTRADICTORY_RECOURSE` and
`NO_AT_REST_RECORD_RECOURSE` beside `UNDER_RECOURSE` in `mate.rs`,
carried through `pncad::document` and bound top-level in Python on
`UNDER_RECOURSE`'s precedent, so no test re-types the prose. One
sentence covers both shapes the contradiction arm renders (a pair of
mates, and a mate contradicting itself through its own rider — "delete
one of the two" is false of the second); the arm's four exits (empty
set, levered, finite, non-finite) fall through to one recourse write,
and `display_contract.rs` asserts it on each. The rung is named in v1
vocabulary rather than by a roadmap id (`Rest` is the one class v1
mints and verifies at rest; a curved contact verified at rest is
outside v1 and not built). The tour's refusal walk asserts all four
recourses against the library's constants and its output was
regenerated through its own door. (2) The stub declares `__eq__`
where the compiled class carries one, and a guard reads it off the
class's OWN `__dict__` — the direction `hasattr` cannot reach. The
`module_class_names` underscore filter stays, argued at its site
(`__eq__` is the one comparison dunder whose presence in the own dict
is evidence). The fieldless mirrors are exempt and the exemption is
CHECKED per member (two accesses answer the same object), with reach
floors so two empty lists are not a pass. It found SEVEN, not the
item's three: `Length`, `Angle`, `Count` compare through
`__richcmp__` and `FaceCensus` had a `fn __eq__` the item's sweep
missed — the item's own declared blind spot. (3) `crates/pncad/tests/
all.rs` reads Rust through `test_utils::source`: `code_without_comments`
deleted, the U1 guard's check 1 reads `use` STATEMENTS off `code_only`
(check 2 stays on the literal-keeping view — a kernel path in a literal
is the false-alarm direction), `root_declared_pub_names` and
`code_without_cfg_gated` read tokens and balanced extents across line
breaks; `test-utils` becomes the façade's one dev-dependency, admitted
as a `use` root at the allow-list and argued in `Cargo.toml`. The item's
stated blind spot for reader 3 was not the real one: a `#[cfg]`
sharing its line with the item it gates made the line unit swallow the
NEXT declaration — a false GREEN — and that is the selftest's fixture.
`reader_census.rs`: `Shared`, `UNCONVERTED_TODAY` 5→4. Not taken:
`north-star-audit-verb-list-names-arc-continue` (BOOL-10 #2135 still
in review). Three items closed. No shipped `variant`/`kind` value
moved; the lane ran the pinned ruff for real. Orchestrator note: the stub now declares `__eq__` on `Length` and `Angle`; the pending [ev] ruling on #2233 (B′) would remove their `__hash__`, which this guard does not touch, so the two do not collide. ~80 min, ~387k tokens.

**LIB-MATE-PAYLOAD MERGED (2026-09-09, #2258; mechanical under (A)
on `pncad-py-seven-doors-lack-field-projection`, no A/B row). The
six `MateFault` fields that did not cross now do, and the arm table
pins 13/13.** Fourteen attributes, 17→31, in the record's order:
`expected_document`/`found_document` (the `str` a `Doc.id` answers),
`inner_variant`, `margin`/`margin_low`/`margin_high`, `zero`/
`escalate`, `field`/`value`, `lever_tilt`/`lever_arm`, `extent`/
`floor`; `presence()` stays exhaustive with no `..`. The frame door's
vocabulary is REUSED, not re-spelled: the classifier's fork (a value,
an enclosure's two bounds, or nothing for a poisoned margin) is one
helper, `escalation.rs`, sited outside `py` so it compiles and is
tested under every feature, and both `frame_err` and the mate record
call it — two doors forking separately would be two spellings of one
fact. `predicate` is SHARED between `Contradictory` and
`Indeterminate` ("the predicate that decided" and "the one that could
not" are one concept asked of two outcomes; the arm says which), said
in the rustdoc and the stub. `inner_variant` is ONE level in
(`frame_error_tag` for `Frame`, `band_error_tag` for `Band`,
`lever_refusal_tag` for `Unleverable`; `Indeterminate` carries a
struct and has no inner word — its shape is which margin attribute is
set, CUR8's `different-shape` measurement). Quantities: margins,
`lever_arm`, `extent`, `floor` are `Length`, `lever_tilt` an `Angle`;
`zero`/`escalate`/`value` stay plain reals because that is what the
frame door already answers for them (a band's thresholds are in
whatever its predicate measures, and dimensioning them here would
decide a question that door left open). The one curation:
`LeverRefusal` re-exported from `editor-core` and curated at
`pncad::document` beside `MateFault` under the cross-list rule (its
only home is the refusal holding it, so it rides its carrier), with
`lever_refusal_tag` (one literal, exhaustive so a second arm is a
compile error) and its inventory row; census `BOUND_AS`
`MateFault.inner_variant`. The arm table builds sixteen values over
the thirteen arms (each `MarginDiag` and `BandError` arm executed)
and reads every field of each; the "nine of thirteen" docstring is
gone. Python rows reach each attribute by authoring the mistake (the
levered and unlevered clash, the mispaired solve, the too-small
datum, the in-band frame); four attributes have no Python row that
CARRIES them because no f64 solve produces an enclosure and the
tolerance witness cannot fail to form a band — stated, the Rust
table owns that half. No new door, no kernel behaviour change beyond
the re-export and the curation; no shipped `variant`/`inner_variant`
value moved; the lane ran the pinned ruff for real. Orchestrator note: the falsification was run the way the brief asked — `floor` blanked on the `Unleverable` arm turned the pin red, reverted green — and the mid-flight merge of SMALL-2 conflicted in the two re-export stanzas (both lanes added names to them), resolved by union and re-verified. ~73 min, ~284k tokens.

**LIB-ZERO MERGED (2026-09-09, #2259; mechanical under Ev's (B′) on
`the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive`,
`[ev]` #2233 item 2, no A/B row). The Python `Length` and `Angle`
mirror the newtypes' derives.** `continuous_quantity!`'s
`__richcmp__` answers on the canonical floats exactly as the derived
`PartialEq`/`PartialOrd` do — IEEE: a NaN operand answers `False` to
every relation but `!=`, `±inf` orders normally, `-0.0 * m == 0.0 * m`
— and the bare `ValueError` arm and its history-telling comment are
gone, replaced by the invariant (the newtypes refuse no float; the
funnel refuses non-finite where a value enters recipe data, so the
boundary type does not re-decide it). `__hash__` is removed from the
macro with no replacement: PyO3 puts `__hash__ = None` in the class
dict for a class that defines the comparisons and no hash, verified on
the compiled module rather than assumed, so `{1 * m}` raises Python's
own `TypeError`. The invariant is stated on the two `#[pyclass]` docs
and the stub stanzas (the two stub `__hash__` lines go; `__eq__` stays
for the drift guard); neither GUIDE nor the README taught the subject.
`Length` and `Angle` join `test_hashability.py`'s `UNHASHABLE` with the
ruling's reason; the nine `FILED` entries (LIB-HASH-2) are untouched.
The mirror table over all seven classes in the file: `Count` matches
its `Eq/Ord/Hash` derives exactly (left); `WrittenLength`/`WrittenAngle`
hash over `PartialEq`-only newtypes and are LEFT because the ruling as
posed names them as keeping their hashes (recipe data past the funnel,
folding the zero) — a deviation from the brief, which had read the
rule as reaching them, stated; `LengthUnit`/`AngleUnit` have the same
more-than-the-derives shape and were FILED rather than decided
(`the-unit-classes-hash-over-a-partialeq-only-newtype`: a unit is a
table row, the seal makes the symbol determine the row, and
`test_notation.py` pins a unit as a dict key on purpose — if the rule
reaches them the repair is upward on `crates/quantity`). Pins moved and
listed: the "raises today" row becomes an eighteen-answer truth table
on a NaN operand plus `±inf` rows; the signed-zero row keeps the
`format`/`==` relationship and loses its hash line; three rows added.
Kernel untouched (`crates/quantity` and every other crate: zero diff);
no `variant`/`kind` value moved; the lane ran the pinned ruff for real.
The ITEM stays open until `[ev]` #2233 merges (it is on that branch);
this unit closes with its ruling recorded there. Orchestrator note: the
brief over-read the ruling on the Written pair; the lane read the
ruling's own words and was right to. ~53 min, ~167k tokens.
## `[ev]` #2233 ruled (2026-09-09): dimensioned seats at the Python boundary

Two items, one theme — what a `Length`/`Angle` seat at the Python
boundary owes — and both rulings came out of Ev pushing on the
mirror. **Item 2 (`the-quantity-boundary-compares-and-hashes-as-if-
poison-and-signed-zero-cannot-arrive`): (B′)** — the Python `Length`
and `Angle` mirror the Rust newtypes' derives (`PartialEq, PartialOrd`,
no `Hash`): IEEE comparisons with no raise on poison, no `__hash__`;
the kernel omits hash and total order on values in favour of the
funnel, so the boundary type does not re-decide it. Landed as LIB-ZERO
(#2259) before this PR merged; the item is closed with it. **Item 1
(`node-slot-literals-erase-the-authored-notation`): (H)**, reached
through five revisions — (A) a `Length | WrittenLength` union at each
slot; (D) the Python `Length` carrying its written unit, withdrawn
because it broke the mirror; (E) `Length | Expr` with `Expr` gaining
Rust's constructors, after Ev asked whether Rust has the union (it
does not: the slot's type IS `Expr`) and whether Python has an `Expr`
(it does, minted only by `parse_expr`); (G) a named union alias at the
seat, after Ev asked whether a union could replace `Expr` (at the seat
yes; the class no — Rust's tree enum is crate-private, and a Python
union of node kinds would be wider than the kernel); and (H) when Ev
asked why a union at all rather than `Expr.from_length`: the union
existed only to keep the bare-`Length` spelling at ~200 sites. **(H):
each dimensioned slot door takes an `Expr` and nothing else, as the
Rust slot does, and `Expr` gains `literal`/`written_length`/
`written_angle`**; helpers for ergonomics, if wanted, go on BOTH sides
mirrored; and a site that spells a unit (`25 * mm`) converts to the
WRITTEN form, keeping the notation, with `Expr.literal` only for a
computed quantity. Mechanical unit LIB-SEATS. Orchestrator note: the
lesson of this thread is the one `memories/` already states — measure
the Rust shape first and propose its mirror; every revision here was a
step back toward that.

## `[ev]` #2257 ruled (2026-09-09): the role-name builders take the loop — (A)

Ev asked why the four builders fix `loop_index: 0`; the answer was
scope, not design — the ruled two-argument shape came from the
thirteen hand-spelling sites, all outer-loop, and LIB-NAMES kept a
ruled signature rather than widen it. With nothing about the outer
loop earning the shortcut on its merits, **(A): the symmetric
signature — `band(node, loop, seg)` and its three siblings take the
loop index as an argument beside the segment, the outer loop `0` as
`ProfileEdgeRef` spells it**; every current call passes `0`; the
thirteen Rust sites and the five Python doors move to the new arity
under the same byte-equality pins. Mechanical unit LIB-LOOPS.
Orchestrator note: I recommended parking (D); Ev chose symmetry over
the convenience, which is the surface's own rule — a builder that
privileges one loop is a second way to spell the vocabulary.
## `[ev]` #2256 ruled (2026-09-09): a mate frame from a face — (F), now

Ev asked, in order: whether materializing a face's frame stores
logically duplicate numbers (yes — derived data stored as authored,
though the author types those numbers by hand today), then whether the
solve can avoid reading geometry forever. Measured: `mate.rs` carries
no `Expr` and no `ParamName` — a mate is plain `f64`s — so a document
parameter change in a part stales its mates silently; the solve cannot
even read a parameter today, so "never" is not available and the
design moves now. **(F): `MateFrame` gains a `FromFace { face,
reference }` arm resolved at evaluation through the exact `face_pose`
readback (NURBS refuses typed and keeps authored vectors); the solve
runs over resolved frames and is itself unchanged; nothing is stored
twice; A11's inputs sentence is revised while its algorithm claim
stays.** The kernel design is MSOLVE's, filed as
`work/msolve/mate-frames-resolve-from-a-face-at-evaluation.md`; the
LIB item is parked on it for the façade and Python half; (A) is not
built. Orchestrator note: the first recommendation here was (A) with
(F) as "presumably not wanted" on the item's own word; the measurement
that settled it (no parameter reaches a mate) was one grep away and
should have been in the first draft.
