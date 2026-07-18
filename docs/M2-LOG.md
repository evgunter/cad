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

## State snapshot

- **Done**: M2-PLAN ratified & merged (#24, all forks resolved in
  conversation). PR 1 implemented + reviewed (zero blockers) + fix
  pass; PR opening for self-merge. PR 2 (profiles) implementing on
  the stacked branch (overlapped pipeline). M3 reading (Mäntylä
  ch. 14–15) dispatched.
- **Next**: PR 2 review ∥ PR 3 (EdgeGeometry) implementation.
