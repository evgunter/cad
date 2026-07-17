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
- **Current**: PRs 1–2 merged (#16, #17); review-suite promotion #18
  merged; issue #4 closed via docs PR #19. PR 3 (`ev/m1-3-rings`,
  stacked on PR 2) implemented + e2e-reviewed + fix pass applied —
  design PR opening next. M0 demo salvage/promotion in flight
  separately. PR 4 spec is the orchestrator's next action.
