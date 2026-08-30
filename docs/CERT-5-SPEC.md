# CERT-5 — the rational-patch-flux lane, native and import sides (issues 453 + 390 route 1)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec, PRE-DRAW for block CERT-B2: **M/L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issues 453 and 390 are the primary specifications (390 by its
**route 1 only** — see Fences); this document fixes scope and
acceptance under the plan's Q4 ruling.

## The defect

`geom-brep/src/props/quad.rs`'s rational composite pays a Θ(1/p)
straddle floor: composite cells cut without regard to interior knots
straddle them, and a straddling cell's enclosure widens by the jump
`g` may genuinely take there (the file says this of itself at the
straddle branches). Consequence: rational patch flux cannot certify
through interior off-grid knots at realistic budgets, on the native
door (issue 453) and the import door (issue 390), and the lily leaf
demos carry flip-when-fixed paragraphs against exactly this.

## The deliverable (Q4 ruling, ratified 2026-08-29)

1. **Knot-aligned composite cells** — the primary deliverable: cell
   boundaries land ON the interior knots (union of both directions'
   interior knots with the requested subdivision), retiring the
   straddle floor for the composite rounds. The precision the ruling
   rests on, restated so it is not fudged: knot-aligned cells restore
   **certified convergence to target** (composite, refined) — they
   are not claimed exact.
2. **The `w`-uniform-in-v exact arm**, taken if it falls out cleanly:
   `rational_patch_face`'s own doc names it as a lever (weights from
   the profile direction only make the v integral exactly
   polynomial — loft walls and dm1 satisfy it). It is the true
   analogue of `patch_flux_exact` and strictly better where it
   applies. If it does NOT fall out cleanly, say so and ship the
   knot-aligned cells alone — the ruling makes the exact arm
   conditional, not owed.
3. **A regression row at 6+ stations with off-grid knots** (the
   plan's own acceptance shape): red-first at the straddle floor,
   green at certified convergence after. ε-three-outcome honest.
4. **Flip conditions carried, executed if reached**: dm1 first-class
   (`WILD_IMPORTS` 9→10) and the lily leaf demos certifying with
   their flip-when-fixed paragraphs retired. If the fix genuinely
   flips them, the flips land in this PR with the demo/render
   re-baselines owned and argued (the render-lane conventions:
   PRs REPORT, main COMMITS; say what moved and why). If a flip
   condition does NOT flip, that is a finding — report it with the
   measured residual, do not force it.
5. **C-m's recorded questions, this unit's half**: the PR
   description answers, for the sites this unit edits, which flux
   engine is authoritative and what the change implies for the other
   `props/quad.rs` copies (CERT-1's PR answered its part; C-m/D30
   consolidation itself is Track R's — answer the questions, do NOT
   consolidate).

## Fences and keep-outs

- **Route 2 of issue 390** (the algebraic CYLINDER certificate) is
  UNCLAIMED and stays so — not this unit's, do not start it. Route 1
  (this unit) also serves issue 453, which route 2 cannot.
- **`props/quad.rs` consolidation (C3/C-m, D30) is Track R's**: this
  unit fixes enclosure quality in place. If the knot-aligned cells
  force a shared helper, home it minimally and note it for Track R.
- **CERT-10 later edits this file** (patch-hull consolidation): keep
  the diff scoped so its seam stays clean — no opportunistic
  restructuring beyond what the cells need.
- **CERT-4 is in flight concurrently** on `cert/4-period-folds`
  (profile/topo period folds; disjoint files). Merge origin/main
  before opening the PR and again if main moves; if you somehow
  collide with it, stop and report rather than resolving another
  lane's ground.
- Track M trait ground: H-R3/#867 governs (`CertifiedBounds`
  tightening works "at least for now") — consume the trait as it
  stands, no trait surgery here.
- `geom-core/src/linalg/vec.rs`: check whether PCURVE PR 1177 has
  landed at your merge base; if not, read-only.

## Order of work

1. Red-first: the 6+-station off-grid-knot row reproducing the
   straddle floor with digits (native door), and an import-door
   twin (a STEP fixture whose rational wall carries off-grid
   interior knots — dm1 itself if suitable).
2. Knot-aligned composite cells; re-measure both rows.
3. The `w`-uniform-in-v arm decision (take or decline with the
   measured reason).
4. Flip-condition check: dm1 (`WILD_IMPORTS`), the lily leaf demos;
   execute or report per item 4 above.
5. Blast radius: touched-crate suites at default + interval; any
   margin population that moves is re-derived with the argument in
   the PR body; k-lint per the K-REPORT runbook if it fires; the
   tessellation-budget memory's anisotropic-sliver hazard is nearby
   ground — if your cells change any mesh sizing input, MEASURE per
   `memories/tessellation-budget.md`.

## Acceptance

- Certified convergence through interior off-grid knots on both
  doors, red-first evidence with digits in the PR body.
- The straddle-floor retirement argued at the code (the composite
  rounds' doc updated to the new cell rule — no stale self-diagnosis
  left standing).
- Flip conditions executed or their non-flip measured and reported.
- Sweep obligation: the same straddling-cell shape elsewhere in the
  flux/area lanes of this file (the sibling straddle branches at the
  area rounds) — hit list with dispositions; state what the pattern
  cannot match.
- Hosted CI green on the head; say which point the sampler drew and
  whether the interval lane ran (use `CI-Config: lane=both` if your
  claims need it — a `props/` change is not basename-matched).
- ε-three-outcome honesty on new rows; deviations stated; any
  refusal minted or changed classified per the D2 addendum (this
  lane's refusals are load-bearing: a narrowed refusal set must be
  argued row by row).
- No `Co-Authored-By` in lane commits; write "issue 453" /
  "issue 390" spelled out (the orchestrator closes them after
  merge — 390 closes only if route 1 alone discharges it; expect it
  to stay open annotated instead).
