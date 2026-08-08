# LIB-LOG — orchestrator log for the usable-as-a-library program

Program contract: `docs/LIBRARY-DESIGN.md` (RATIFIED, PR #229).
This log is the program's operational record — unit dispatches,
unilateral orchestrator decisions (LB-numbered), and resting state
— in the M*-LOG tradition. This program runs concurrently with the
M6/M7 close-out (its own orchestrator, its own logs); the fence
between the two lanes' footprints is recorded per-spec.

## Rulings absorbed at program start (Evan, in-chat, 2026-08-06)

Recorded in LIBRARY-DESIGN.md §L8; operational consequences here:

1. **U1 + U2 authorized to start now** (LQ5 execution); units past
   that are delegated to orchestrator judgment where footprints
   are independent — Evan: "things past that likely are also
   viable." Genuine design forks still escalate.
2. **Façade placeholder crate name: `pncad`** ("pending-name CAD")
   — greppable, carries the Q9 rename debt visibly. See the
   name-candidates memory for the rename-time grep note.
3. **v2 profiles-as-programs spec timing**: the design-conversation
   draft waits for U2's algebra to be implemented AND the demo
   corpus reworked onto it — the rework is the evidence base for
   what the representation should be. Still ahead of U9 (§L3's
   "Python never ships the opaque-profile state" stands).
4. **A/B**: library-program implementation dispatches draw from
   their own LIB-labeled block series in MODEL-AB-LOG (no
   collision with the M7-N series the other orchestrator draws).
5. **Lane slots**: Evan is building flock-based build-slot locks
   (`cargo-slots.txt` is RETIRED in place); until the script
   lands on main, the 10 GB / two-parallel-cargo-lanes ceiling is
   enforced by this log's slot line.
   **SUPERSEDED same night: PR #230 MERGED (2026-08-07 ~00:00)**
   — `scripts/with-build-slot.sh`, machine-wide flock semaphore,
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

- **2026-08-07 (day): WSL CRASH** (Evan, evening). Symptoms as
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
  RETRACTED the bowtie-forces-raw-seat claim; round 3 Evan
  delegated VQ1 — **RULED (b)-DIRECT** (chain-only schema; the
  additive-vs-subtractive LQ7 asymmetry decides: raw can be added
  later additively, removal has a pre-release deadline).
  **Revised ladder consequence**: vocabulary-growth units precede
  the switch — **G1** (cheap set: circle primitive, arc_via,
  arc_center, far-end anchor, VQ4 exact directors) then **G2**
  (arc-carrier fillet modes; sizing starts by measuring sugar's
  existing arc-leg fillet forms, M5 S2/#137); then the SWITCH
  unit(s) (schema v4 chain-only, replay driver, Expr binding,
  slot addressing); U9 queues behind the switch (Evan: no hurry).
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
away-channel NOT armed (Evan present in-chat; watchlist empty).
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
## elaboration class — Evan retroactive, veto window on #259)

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

- **LB3 AMENDED (2026-08-08, from Evan's factoring question on
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

- **Note (Evan, in-chat 2026-08-08)**: LB7/LB8 confirmed as
  sequencing-class. When the geometric-selector follow-up is
  designed, it should RE-HOME GQ7's selection-filter portion out
  of GUI-DESIGN into the library design docs — Evan: "a bunch of
  general-usefulness stuff got originally mentioned in
  GUI-DESIGN even though it's more broadly applicable." The GUI
  becomes a consumer of the general mechanism, not its owner.

**SWITCH spec drafted (2026-08-08, PR #263 — OPEN, awaiting two
Evan inputs)**: (1) PROFILES-V2 §V3 REVISED — the naming-stability
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

- **LB9 (Evan on #267)**: geom-core's classify-seam `Length<T>`
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
