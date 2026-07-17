# M1 Implementation Log

Orchestrator's running log for M1 (topology + Euler operators). Same
purpose and conventions as `docs/M0-LOG.md`: (1) record design decisions
made *during* implementation that didn't need Evan's input but should be
visible and revisitable; (2) snapshot orchestration state so any session
can resume. Update and commit at every checkpoint.

L-numbering continues from M0 (M0 ended at L7); an L-decision is one the
orchestrator made unilaterally — if contentious, it gets promoted to a
design conversation.

## Process conventions (inherited from M0)

- Orchestrator does central planning, design, and meta-review;
  implementation and first-pass review are delegated to subagents (Opus
  straightforward, Fable medium/hard). One implementer + one adversarial
  e2e reviewer + one fix pass per PR.
- Branches: `ev/m1-<n>-<slug>`, one per M1-PLAN PR, stacked serially.
  PRs target `main`. Merge commits only.
- Design PRs wait for Evan's sign-off; work continues stacked on top.
  Non-design PRs self-merge after subagent review + green CI.
  **Revised after PR #20 (Evan, 2026-07-16)**: high-confidence design
  PRs (dominant-argument conventions, faithful elaborations of the
  ratified plan) now ALSO self-merge with their full writeups; Evan
  reviews the backlog retroactively. Fundamental design forks (changes
  to ratified DESIGN.md decisions; genuinely open multi-answer
  questions) still wait for sign-off. Unsure ⇒ treat as a fork.
- Reviews write and run real consumer programs against the API under
  review (standing rule, `memories/review-and-dependency-policy.md`).
- Reference reading: Mäntylä ch. 9–11 notes live in
  `<main-checkout>/references/notes/mantyla-ch{9,10,11}-*.md`
  (git-ignored, persistent) — implementer/reviewer prompts should point
  there, not at the scan.
- Reviewer test suites are promoted into CI as
  `crates/topo/tests/review_m1_prN.rs` after each fix pass (Evan, PR
  #17): independent derivations are regression value; suites hit by
  later API changes (e.g. PR 5's raw-builder demotion) migrate or get
  pruned at that PR like any test.
- The M0 reviewers' demo crates were salvaged from the M0 session
  scratchpad (archived at `references/review-artifacts-m0/`,
  git-ignored, persistent) and promoted the same way as
  `crates/{geom-core,topo}/tests/review_m0_prN.rs`; PR 7's suite was
  ported at the invariant level across the M1 half-edge restructure.

## Carried in from M0 (docs/M0-LOG.md "M0 EXIT")

- K's numeric value (multi-ε experiments; semantics ratified).
- `Body<Interval>` instantiation test (TODO in topo's validate.rs).
- Validator M1 items: arity/emptiness rules, orphan-vertex vs `mvfs`,
  bidirectional D5 provenance check; Euler–Poincaré + watertightness +
  residual certification plug in per validate.rs docs.
- Half-edge/orientation design — grounded in Mäntylä ch. 9–11 (read
  2026-07-16, notes as above).
- L7 allowlist moment (first legitimate `Real +` bound) still pending.

## PR #15 (M1-PLAN) design conversation

Evan's review (2026-07-16), first round:
- **Typed `Loop` state ratified** over Mäntylä's nullable-placeholder
  half-edge ("significantly better") — PR 1 implements the typed
  representation; the placeholder alternative is dead.
- **CCW-from-outside orientation ratified** (no strong opinion; our
  proposal stands). GWB-diagram mirroring hazard remains flagged.
- **kfmrh sequencing ratified** (cross-shell typed error until M3).
- Two-tier validity: elaboration requested; provided on the PR.
  Refinement made while elaborating: **laminae are not topologically
  bannable at tier 2** (two faces glued along their whole boundary ≡ a
  two-hemisphere ball's incidence structure); zero-volume laminae are a
  geometric defect for the M2+ geometric tier. Plan text corrected
  in-branch; tier 2 bans empty loops + valence-1 vertices only.

Second round (chat, 2026-07-16):
- **"Debug builds validate after every op" clarified**: each operator
  debug-asserts its tier-1 postcondition — a per-call check of the
  ch. 9 soundness theorem against our transcription of the surgeries,
  never a semantic gate (tier 1 holds for every legitimate intermediate
  by construction). Wording folded into D1.
- **Body-as-materialized-evaluation ratified into D1** (Evan's framing:
  is the mutability semantically a cache of the immutable
  representation? — yes in authority terms; "cache" kept out of the
  ratified text at Evan's discretion since bodies are long-lived and
  replay is not lazy): Body is never authoritative; coherence =
  bit-identical replay (D9); mutation is evaluator-internal linear
  state only; imported bodies' authority = adopted descriptions +
  import record (D7).
- **No bespoke lamina ban at the geometric tier either** (Evan probed
  why laminae need banning at all): intrinsic edge variants die by
  their D2 margins, but conventional variants (e.g. a height-0
  extrude's true-by-construction `MappedCurve`s) do NOT — the kill is
  the **material wedge-angle predicate** (wedge ∈ (0, 2π) bounded away
  from the ends by θ = ε/r; π = legal smooth seam), enforced at the
  operation (D4 ¶3) and rechecked by the M2+ geometric validator.
  Honest residual gap stated in the plan: coincident faces sharing no
  edge (zero-thickness voids between shells) are edge-locally
  invisible — global self-intersection/clearance, M3 partial / M6 full.

## PR 1 (half-edge restructure) — 2026-07-16

- Implemented per binding spec (Fable, isolated worktree); ratified
  decisions (typed `LoopBoundary`, CCW-from-outside) baked in.
- **Orbit step under our convention**: `next(mate(he))` walks CLOCKWISE
  around a vertex viewed from outside; `mate(prev(he))` is the CCW
  inverse. Derivation in entity.rs, pinned by a hand-computed prism-rim
  test; independently re-derived by the e2e reviewer (abstract sector
  argument + numeric cube corner) — full agreement, no mirrored
  sentence found anywhere.
- **e2e review verdict: mergeable, no blockers.** Sharpest probe: an
  antiparallelism-preserving mate-swap is caught ONLY by vertex-orbit
  closure — pass 6 is the load-bearing watertightness check, and it
  held. Bounded walks held under a 6000-half-edge torn-link attack
  (3 errors, ~104 ms, no hang). Accepted structures = oriented
  2-complexes with single-cycle vertex fans (orientability is automatic
  given antiparallel mates).
- **Deviations to ratify in the PR** (both endorsed by review):
  (1) `pub get_*_mut` patching accessors — the reference structure is
  cyclic in every direction, so pure `add_*` insertion cannot close any
  valid body (without key forging); both halves of the raw builder
  retreat to `pub(crate)` at PR 5. No ratified invariant broken today
  (D5 unrepresentability, lineage determinism intact).
  (2) `outer ∉ rings` — a GWB deviation beyond what PR #15 pre-ratified
  (flout ∈ floops there); listed for explicit sign-off.
- SHOULD fixes applied post-review: shell-partition/edge-adjacency
  coherence gap named in the validator's PR 5 deferral note (an edge's
  two faces in different shells passes tier 1; per-shell E–P closes
  it); fixtures module-doc coordinate claim corrected; "cannot by pure
  insertion" phrasing made honest (slotmap key-forging caveat).
- Process conventions add: implementer agents push branches early and
  often (before review) — Evan follows work-in-progress remotely.

## PR 2 (Euler ops: mvfs/mev/mef + cube) — 2026-07-16

- Implemented per binding spec (Fable, isolated worktree). Site-enum
  addressing (`MevSite::{Fan,Lone}`, `MefSite::{Chords,Lone}`) — the
  typed-`Empty` consequence; atomic ops (preconditions fully resolve,
  mutation phase infallible); typed `Provenance::{Mvfs,Mev,Mef}`
  (Primordial stays on the raw path until PR 5); debug postconditions
  (per-op Euler-vector deltas + full tier-1 validate).
- Conventions fixed: Fan run = CW orbit walk [he1..he2) (pinned by
  asymmetric valence-4/5 tests, incl. a wrapping run); mev `he_plus` =
  old→new (documented deviation from Mäntylä's new→old); mef `he_plus`
  = start(he1)→start(he2) with he1's side becoming the new face's
  outer; `emanating` unconditionally overwritten (branch-free,
  replay-deterministic); mef shares the split face's SurfaceKey and
  mints a Placeholder curve anchored at start(he1).
- **Mäntylä erratum found (Program 11.6)**: the book prints lmev's two
  addhe calls PLUS-half first, which breaks both he1==he2 cases
  (strut: broken v→v half; mvfs-placeholder: `he2->vtx` clobbered
  before read). MINUS-first is coherent; order immaterial when
  he1≠he2. Verified against the scan at 300 dpi; the reading notes
  carry a dated erratum. Our implementation is unaffected — it
  computes final link states functionally instead of sequencing addhe.
- **e2e review verdict: mergeable, no blockers.** Independent cube
  re-derivation (different construction order): all six faces CCW from
  outside by explicit signed-area projection; mates antiparallel in
  coordinates on all 12 edges. **Key-sequence purity under errors
  demonstrated** (the D9 lineage-replay contract's error half): four
  interleaved failing calls consume zero key slots, deep snapshots
  byte-identical. 15 error paths deep-compared body-unchanged;
  release-mode corruption gives typed errors or documented
  garbage-out, never panic/hang (3000-strut torn body: milliseconds).
- Fixes applied post-review: postcondition/D9 doc honesty (debug
  postconditions ARE reachable via public raw-builder corruption until
  PR 5 — the no-panic promise is conditional on tier-1-valid input in
  debug builds; taxonomy fact to ratify at PR 5's demotion);
  no-proptest deviation recorded in-tree (PR 4 owns the sequence
  generator); mef start(he2) liveness check added (symmetry);
  reviewer's key-purity test + deep-snapshot helper added to the
  shipped suite.
- Cost note: debug validate-after-every-op makes construction O(n²) in
  debug (3000-op body ~5 min debug vs ~20 ms release) — fine at M1
  scale; revisit before M2's swept bodies if debug CI builds big
  fixtures.
- Carried to PR 3's spec: deep-snapshot atomicity helper (counts are
  weak for kills); provenance SecondaryMap entries must be removed
  with killed entities (PR 5's bidirectional check will catch leaks);
  ratify the ring-side association convention (GWB's h2-analog);
  `Cycle::first` re-anchoring on survivor loops is the delhe hazard.

## PR 3 (kemr/mekr/kfmrh + ring_move + the holed box) — 2026-07-16

- Implemented per binding spec (Fable, isolated worktree); new
  `euler_ring.rs` module. Ratified conventions: **he1's side becomes
  the ring** (verified identical in content to GWB's h2-side
  description — only the argument keying differs; nothing in PR 3
  needed mirroring, all next-order constructions are
  orientation-neutral); Empty anchors u = start(he1) / w = start(he2)
  (valence-1 derivation); `EmptyAnchorsCollide` defensive error (both
  trigger states verified tier-1-INVALID by the review); kfmrh
  same-shell only (`CrossShell` reachable only via two solids until
  M3 — nothing but mvfs mints shells); f2 must be ring-free
  (`FaceHasRings`); geometry reaping — curve/surface removed iff
  orphaned, deterministic full-arena scans, reported in results;
  `ring_move` = pure reparenting, prominently NOT an Euler op;
  provenance = birth records (kills remove records with entities;
  survivors keep theirs; reparenting/demotion is not a re-birth).
- **Four mekr sites, not three** (`Cycles`, `EmptyRing`, `EmptyTarget`,
  `BothEmpty`): kemr's old-side-empty output forces `EmptyTarget` for
  invertibility. `BothEmpty` deliberately accepts a ring as its
  surviving target — ring-to-ring joins are in-contract.
- **Acceptance**: the §9.3 box-with-through-hole (1 mvfs + 15 mev +
  10 mef + 1 kemr + 1 kfmrh) passed on the first run, ledger
  16−24+10−2 = 0 = 2(1−1). The review added an independently-routed
  triangular side-face hole (genus 1, per-op ledger table) and the
  **first genus-2 body** (double hole: v22 e33 f13 r4,
  22−33+13−4 = −2 = 2(1−2)) — arbitrary genus needs nothing new.
- **Replay-with-kills semantics pinned precisely** (review SHOULD fix
  applied — the first-draft "balanced pairs converge" was overclaimed,
  caught because the in-crate pin used mev, the one make-op minting no
  loop): identical histories replay deep-identically (D9 holds);
  balanced kemr∘mekr pairs converge PER-ARENA — halves/edges/curves
  immediately, the loop arena one loop-mint later (recycled slot,
  bumped generation); unbalanced kill histories diverge per-arena
  permanently (allocation cursor offset). Docs and tests now state
  exactly this.
- e2e review verdict: mergeable, no blockers (1 SHOULD above, 3 NITs —
  Cycles-canonical precondition listing note; two documented
  unreachable-path notes). Kill hygiene held under attack: stale keys
  None across re-mint cycles including provenance lookups; key purity
  with five failing ring-op calls interleaved into a kill-heavy build —
  byte-identical snapshots. Reviewer suite promoted as
  `review_m1_pr3.rs` per convention.
- For PR 4: mfkrh first makes Empty-outer faces operator-reachable;
  the general isomorphism oracle for roundtrip proptests is still
  wanted; `remove_curve_if_orphaned`/`remove_surface_if_orphaned` ready
  for kev/kef.
- For PR 5: add a validator test for two-Empty-loops-on-one-vertex;
  NIT-2's dead StaleGeometry precondition becomes testable when point
  removal exists.

## PR 4 (kill duals: kvfs/kev/kef/mfkrh + roundtrip machinery) — 2026-07-16

- Implemented per binding spec (Fable, isolated worktree); new
  `euler_kill.rs` + test-support `iso.rs` (isomorphism oracle) +
  `seqgen.rs` (random op-sequence generator, pub(crate) — PR 5's fuzz
  source). Non-design PR (inverse-forced semantics), self-merged under
  the PR #20 process update.
- Splice derivations (kev fan-merge = exact undo of mev's four link
  writes via the CW orbit; kef = tail-unswap + remnant reparent)
  independently re-derived by the reviewer from euler.rs's pinned
  surgeries — full agreement; the both-ends asymmetric valence-5 kill
  is the strongest direction pin in the tree (promoted).
- **Single-op re-make taxonomy** (review BLOCKER fixed a
  self-contradiction between seqgen and euler_kill — euler_kill was
  right): kef mate-alone IS re-makeable by `mef(Chords{b,b})` iff the
  surviving singleton is the outer of a ring-free face; irreversible
  iff the survivor is a ring or its face has rings; kev's
  valence-1-side kill has NO single-op re-make (distinct coordinates
  make the strut re-make wrong). Roundtrip skips narrowed to exactly
  the irreversible subcases.
- **mfkrh on a detached (non-handle) ring disconnects the shell's
  surface** while one shell entity remains — naive per-shell h goes
  negative; tier-1-legal and previously undetected by anything.
  Component-aware per-shell E–P derived and ratified into M1-PLAN's
  PR 5 bullet: per component v − e + f − r = 2(1 − g), g ∈ ℤ≥0; per
  shell Σ = 2(c − Σgᵢ); tier 2 adds c = 1 per shell (existing tier-2
  bans do NOT imply it — a promoted detached cycle ring disconnects
  with no empty loops or struts).
- Deviations (review-endorsed): kvfs reaps the face's surface AND the
  vertex's point (exact inverse of mvfs's mints; scan-based so sound
  under future sharing); kef's same-face error path documented with
  the kfmrh attribution + self-loop kill route (mfkrh then kef).
- Review: 21-probe consumer suite (promoted as review_m1_pr4); oracle
  survived all attacks beyond documented blind spots (hexagon-pillow
  automorphisms, coordinate-degenerate determinism); genus-2 teardown
  to empty arenas + empty provenance + zero geometry; kill-heavy key
  purity; 300-strut cross-body-corruption battery in release — typed
  errors, no panic/hang. seqgen covers all 9 entry points / 17 site
  shapes; kvfs randomness thinness noted + weight raised (teardown
  remains the deterministic backstop).
- For PR 5: seqgen is the fuzz source; the component-formula probe is
  the spec seed for the validator's E–P pass; two-Empty-loops-on-one-
  vertex validator test still owed (PR 3 carry).

## PR 5 (validator completion + tiers + raw-builder demotion) — 2026-07-16

- Implemented per binding spec (Fable, isolated worktree). Tier 1
  grows to 12 passes: arity floors (`SolidWithoutShells`,
  `ShellWithoutFaces`), edge-adjacency shell coherence
  (`EdgeAcrossShells`), **component-aware per-shell Euler–Poincaré**
  (`ComponentEulerViolation` with per-component counts; DFS over
  faces in arena order, glue rules per the ratified bullet;
  set-size counts, order-independent), bidirectional D5 provenance
  (`MissingProvenance`/`LeakedProvenance`, all seven arenas).
  `validate_closed()` exported: tier-1 first, then
  `ScaffoldingEmptyLoop`, `ScaffoldingStrutVertex`,
  `ShellDisconnected{components}` in documented order.
- **Raw builder demoted to pub(crate)** — Euler ops + ring_move are
  the only public mutators; debug postconditions are now
  unreachable-by-input through the public API (the PR 2 taxonomy
  hole closes; D9's no-panic claim is unconditional at the public
  boundary). ring_move's tier-1 preservation rests on the
  separating-curve argument (a ring on a genus-0 component is a
  Jordan curve; non-separating rings force g ≥ 1) — named in the
  demotion docs and exercised by seqgen after the review.
- Suite migrations for the demotion: review_m1_pr1/pr3, review_m0_pr7
  → src cfg(test) modules; review_m1_pr2 suite moved whole as a
  directory module; pr1's gap probe INVERTED (the PR 1 review's
  moved-face scenario now yields 4 EdgeAcrossShells + per-shell odd-χ
  ComponentEulerViolations — the gap is closed and pinned).
  cube/box acceptance tests now also assert validate_closed. Mid-PR
  merge of origin/main (merge commit) absorbed the #18/#22 salvage
  suites into the stack — without it the demotion would have broken
  main's build on merge.
- `Body<Interval>` cube test lands (new topo `interval` feature →
  geom-core/interval): both tiers pass at T = Interval — the M0
  carry closes.
- **e2e review verdict: mergeable, zero blockers.** Component pass
  survived exhaustive falsification (all ring_move/kfmrh/mfkrh
  mutations to depth 2 over five adversarial fixtures incl. a
  non-separating-ring pillow-torus and nested detached-genus);
  reviewer's structural insight recorded: passes 3/4/6 force each
  component to be a closed oriented surface, so χ = 2(1 − g) is
  automatic within coherent shells — pass 11 fires only under
  genuine shell-cutting. Multi-detachment counts exact
  (ShellDisconnected{3}); the grown-digon witness for c = 1's
  independence verified necessary (mfkrh on an EMPTY ring leaves an
  empty loop). Demotion attack: no debug panic reachable through
  any public path. Fixes: ring_move named + fuzzed (SHOULD-1),
  provenance coverage 14/14 promoted (SHOULD-2), strut-scan
  doc/gating corrected, reviewer suites promoted as review_m1_pr5.
- For PR 6: DESIGN draft divergences to fold in — "visible solely
  inside operation implementations" → "sequences"; tier-1 checklist
  should name vertex anchoring (the discharged M0 orphan-vertex
  deferral), the ownership/back-pointer partition, and orphan
  geometry; the unreachable-by-input wording gets the ring_move
  caveat.

## Log decisions

(none yet)

## State snapshot

- **Done (2026-07-16)**: Mäntylä ch. 9–11 read by three subagents
  (notes in `<main-checkout>/references/notes/`, persistent); topo
  crate surveyed; `docs/M1-PLAN.md` drafted (6-PR sequence, design PRs
  = 1, 2, 3, 5).
- **RATIFIED (2026-07-16)**: PR #15 signed off by Evan ("lgtm, per
  local discussion") after two conversation rounds (typed `Loop`, CCW,
  kfmrh sequencing, two/three-tier validity, D1 clarifications:
  postcondition asserts + Body-never-authoritative, lamina story).
- **Current**: PRs 1–4 merged (#16, #17, #20, #23); process update
  #21; salvage PRs #18/#22; issue #4 closed via docs PR #19. PR 5
  (`ev/m1-5-validator`) implemented + e2e-reviewed (mergeable, zero
  blockers) + fix pass applied — PR opening for self-merge. M2 reading
  complete (ch. 12/13 notes archived); M2-PLAN drafted and opening as
  a design PR for Evan's ratification. PR 6 finalization
  (orchestrator) after PR 5 merges.

## M1 EXIT (2026-07-16)

All six PRs merged: #16 (half-edge restructure), #17 (mvfs/mev/mef +
the cube), #20 (kemr/mekr/kfmrh + ring_move + the holed box), #23
(kill duals + roundtrip machinery), #25 (validator completion + tiers
+ raw-builder demotion), #26 (exit sweep — DESIGN.md ratifications).
Plan ratified as #15; PRs 1–3 explicitly signed off by Evan; PRs 4–5
self-merged under the PR #20 process update with zero-blocker
adversarial reviews. Adjacent merges during the milestone: #18/#22
(review-suite salvage/promotion, both milestones' corpora now in CI),
#19 (issue #4 LGPL docs, issue closed), #21 (process update), #24 =
M2-PLAN design PR (open, awaiting Evan).

Exit criteria: verified (see M1-PLAN header). Highlights beyond the
plan: an apparent erratum in Mäntylä Program 11.6 found and verified
against the scan (the book's own ch. 12 sweep code depends on the
corrected order); the first genus-2 body; the component-aware per-shell
E–P replacing the naive form the plan guessed; replay-with-kills
semantics pinned per-arena; watertightness demonstrated structural
(orbit closure is the load-bearing check).

**Carried into M2** (all in M2-PLAN):
- K's numeric value — first predicate telemetry arrives with M2's
  geometric predicates; report due at M2 exit.
- Geometric tier (tier 3): D4 ¶2 residual certification starts with
  Newell face equations; the material wedge-angle predicate.
- The L7 allowlist moment (first legitimate `Real +` bound) — likely
  M2 PR 1; `Real` gains floor/rem/copysign there.
- M0 linalg watchlist: project/reject with documented association,
  axis-through-point rotation, orthonormal-basis-from-normal as a
  predicate question.
- Debug-O(n²) per-op validation cost — watch when swept bodies grow.

**Operational lessons (fold into memories):**
- Reference material MUST land in the main checkout's `references/`
  (git-ignored dirs don't propagate between worktrees — the NURBS
  book and Hoffmann were stranded in the original design session's
  worktree until consolidated 2026-07-16).
- The reviewer-falsification pattern (assign the reviewer explicit
  claims to break) caught a real doc self-contradiction (PR 4's
  re-make taxonomy) and produced the PR 5 spec's E–P invariant —
  worth keeping as standard practice for self-merging PRs, where the
  review is the last gate.
