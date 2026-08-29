# CERT-2 — issue 762 close-out and the chart-speed guard residue

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged pre-draw: **S**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 762 plus SMELL rows D285/D286 (`docs/SMELL-SCAN-2026-08.md`
§Track Q) are the primary specification; the fence seam this unit
crosses is ruled (S-CERT plan, Rulings Q3).

## Situation

Issue 762's headline fix landed on main outside any program
(`geom-brep/src/ssi.rs:991` now guards `!speed.is_finite()`). The
issue stays open because its residue was never swept:

1. **D285** — `ssi/march.rs:423` still spells
   `speed.is_nan() || speed <= 0.0`, so `+∞` passes; `h` collapses
   to `0`, `h_meters` to `NaN`, and the caller gets an
   `ssi_step_progress` escalation instead of the
   `SsiError::StepCollapsed` the guard ten lines up exists to raise
   (its own comment — "nothing downstream can be stated in meters,
   so refuse rather than divide by it" — is the argument for
   catching `+∞` there).
2. **The NaN-dropping fold** — the issue's second ask: `f64::max`
   returns the non-NaN operand, so a single poisoned derivative box
   cannot reach a guard's `is_nan()` arm. Verify whether the landed
   fix replaced the fold at the `ssi.rs` site; apply the
   NaN-propagating fold at whichever of the two sites still drops
   one.
3. **`ssi/exhaust.rs:285`** — the ℝ³ poison arm's message names
   surface kinds its own door excludes; re-word to the cause a
   caller can actually produce (a degenerate instance of a
   supported kind).
4. **D286** — the landed seeding guard made the ℝ⁴ control-net
   poison arm unreachable **by magnitude** (`mag` squares before
   the sqrt, so speed is `+∞` from ~1.3e154 while value poison
   needs ~1e308). Deliverable: a fixture that reaches the arm by a
   route other than magnitude, or a recorded verdict that none
   exists and why — the verdict lands as an issue filed from this
   unit (keyword hygiene below), cited from the arm.

## Verification of the landed fix (before building anything)

Confirm against issue 762's four asks what `91164e3b` actually
carried, with a one-line disposition each in the PR body: the
finiteness guard, the refusal's wording ("derivative bound
overflowed" vs "zero or poison"), the fold, the exhaust message,
and whether `an_infinite_chart_speed_refuses_rather_than_receipting`
moved to the guard arm. What is already done is recorded as done,
not re-done.

## Acceptance

- A row driving `march.rs` with a non-finite chart speed that pins
  `StepCollapsed` (or the honest typed refusal the guard names) —
  red first under the current guard, by the wrong-diagnosis
  signature D285 records.
- A row (or the adopted existing fixture) pinning the
  NaN-propagating fold if a site still drops one.
- D286's fixture red-then-green against the poison arm, or the
  filed-issue verdict with the arm's comment citing it.
- **The SMELL table rows land in this PR**: delete D285 and D286
  from `docs/SMELL-SCAN-2026-08.md`'s Track Q table per §D rule 3
  (a row leaves when it lands; a partly-closed finding leaves
  member by member). If D286 resolves as a filed-issue verdict,
  its row still leaves — the issue is the register that executes.
- ε-three-outcome honesty on new rows; local scope: the touched
  geom-brep ssi suites at default ε plus the interval feature
  (hosted CI proves the rest; a change under `ssi` with `interval`
  in no basename falls back to the lane draw — say in the PR
  whether the gate drew the interval lane).

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: never place a closing keyword immediately before
  a `#`-reference in the PR body or any commit message — write
  "issue 762" spelled out. The orchestrator closes the issue after
  merge; this PR must not auto-close it.
- Scope fence: `geom-brep/src/ssi.rs`, `geom-brep/src/ssi/`
  (march/exhaust), their test suites, and the two SMELL table rows.
  Nothing else — no `pcurve_cache.rs`, `nurbs_iso.rs`,
  `edge_nurbs.rs` (PCURVE-adjacent), no march-stepping behavior
  changes beyond the guard, no `docs/MODEL-AB-LOG.md` or
  `docs/S-CERT-*.md`.
- Any refusal minted or changed is classified against the D2
  addendum in the PR body.
