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
- Reviews write and run real consumer programs against the API under
  review (standing rule, `memories/review-and-dependency-policy.md`).
- Reference reading: Mäntylä ch. 9–11 notes live in
  `<main-checkout>/references/notes/mantyla-ch{9,10,11}-*.md`
  (git-ignored, persistent) — implementer/reviewer prompts should point
  there, not at the scan.

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

## Log decisions

(none yet)

## State snapshot

- **Done (2026-07-16)**: Mäntylä ch. 9–11 read by three subagents
  (notes in `<main-checkout>/references/notes/`, persistent); topo
  crate surveyed; `docs/M1-PLAN.md` drafted (6-PR sequence, design PRs
  = 1, 2, 3, 5).
- **Current**: M1-PLAN open as a design PR, awaiting Evan's
  ratification. No implementation until sign-off.
