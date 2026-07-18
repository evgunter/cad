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

## State snapshot

- **Done**: M2-PLAN ratified & merged (#24, all forks resolved in
  conversation). PR 1 merged (zero blockers). PR 2 implemented +
  reviewed (zero blockers) + fix pass applied; PR opening for
  self-merge. PR 3 (EdgeGeometry) implementing on the stacked branch
  (overlapped pipeline). M3 reading (Mäntylä ch. 14–15) nearly done.
- **Next**: wind-down to orchestrator handoff after PR 3's
  implementation report; next session picks up at PR 3 review ∥ PR 4
  (extrude) implementation.
