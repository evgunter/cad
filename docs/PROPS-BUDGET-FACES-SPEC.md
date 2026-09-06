# PROPS budget-faces — `BudgetExhausted` splits into its four terminations

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Lanes,
the offset_fit lane's first unit; the item is
`work/props/budgetexhausted-conflates-three-terminations.md` — read it
in full, including CERT-7's review comment that adds the fourth face;
difficulty logged at spec: **E**, an E rider — single style review,
outside the A/B experiment). Read `docs/prompts/implementer-discipline.md`
in full. Branch `props/budget-faces`, cut from `main`.

## The ruling (the item's "D2-addendum classification owed", answered)

Splitting a refusal by cause is a **row-1 refinement**: the admission
set does not change, the caller learns which knob it was refused on.
`OffsetFitError::BudgetExhausted` (`crates/geom-brep/src/offset_fit.rs:~298`)
fires today for four different terminations and names one knob
(`budget`) for all of them. After this unit each termination has its
own face, and each face names the lever that would have changed it:

1. **Rounds out, still converging** — keeps the name
   `BudgetExhausted { budget, grid, achieved, tolerance }`; its doc says
   the lever is `OFFSET_FIT_BUDGET` and that `achieved` is FINITE (face
   4 takes the other case).
2. **The per-direction sample cap reached** — `SampleCapReached
   { cap, rounds, grid, achieved, tolerance }` (the loop's `break` at
   `next.us.len() > OFFSET_FIT_SAMPLE_CAP || …`): the lever is
   `OFFSET_FIT_SAMPLE_CAP`, and `rounds` says how many of the budget's
   rounds actually ran, so "budget 6 after 5 rounds" can no longer be
   read as "raise the round budget". The measured instance (`d = 1e-7`
   on the quarter cylinder) is the red-first row.
3. **Nothing marked** — at this head the schedule-exhaustion path
   (`!next.grew(&us, &vs)` after the both-directions fallback) already
   returns `RefinementStalled`. MEASURE whether any path still reaches
   the budget refusal with an unmarked round (instrument the loop on
   CERT-7's fixtures and the `offb_r2_probes` corpus); if none does,
   the unit records that the third face is `RefinementStalled`'s
   schedule-exhaustion arm and says so at both docs; if one does, it
   gets its own face named for the lever (none — the schedule cannot
   grow), and the row that reaches it is the red-first row.
4. **No finite bound ever produced** — `achieved: inf` at expiry
   (measured: quarter cylinder at `d = 1e-8`): a refusal whose promise
   "carries the achieved bound" is broken exactly where the caller
   needs the number. Its own face, `BoundNeverFinite { rounds, grid,
   tolerance }` (no `achieved` field — the type says there is none),
   whichever of the budget or the cap stopped the loop; the doc says
   what a non-finite bound means (the certificate limb never answered
   on this grid) and names the lever honestly: not a knob, the base's
   own poison at the sampled cells, so the caller's move is the
   meters, not the budget.

`RefinementStalled` is untouched. No new admission: every input that
was refused is refused; the D2 row-1 classification is stated once at
the enum with the four faces listed.

## Deliverables

- The enum, `Display` for every face (each message names its lever;
  the cap face prints `rounds` and the cap; the never-finite face
  prints no number where there is none), the loop's four exits
  returning the face that names them.
- Red-first: the `d = 1e-7` instance asserting `SampleCapReached
  { rounds: 5, .. }` (red today: it reads `BudgetExhausted { budget: 6 }`)
  and the `d = 1e-8` instance asserting `BoundNeverFinite` (red today:
  `BudgetExhausted { achieved: inf }`); quote both reds. The face-1 row
  (rounds out with a finite bound) stays green through the split —
  name it. Face 3 per its measurement.
- Every consumer of the variant re-spelled: `crates/geom-brep/tests/
  {offset_fit,approx_surface,offb_r2_probes,cert7_r1_probes,cert7_r2_probes}.rs`,
  `crates/topo/tests/census_g2_carrier.rs`, and `grep -rn BudgetExhausted
  crates/ demos/ tools/` for the rest; `crates/pncad-py/src/prose_census.rs`
  names `OffsetFitError` — read whether the Python surface mirrors the
  variants (`cargo check -p pncad-py --features python`, the `.pyi`) and
  say so.
- Module docs (`offset_fit.rs:~46-50`, `~217-240`, the `OFFSET_FIT_BUDGET`
  and `OFFSET_FIT_SAMPLE_CAP` docs) say which face each constant is the
  lever of; discipline §4 — present tense, no history.
- Sweep obligation (discipline §5): the shape is *one refusal variant
  fired from more than one termination, naming one lever* — every
  `Budget*`/`*Exhausted` variant in `crates/*/src` (`geom::curves::fit::FitError::BudgetExhausted`,
  `topo::chart_region::{WitnessBudgetExhausted, WitnessOutcome::BudgetExhausted}`,
  `mesh/budget.rs`, and whatever the grep finds): per hit, how many
  exits raise it and whether they share a lever; hit list with
  dispositions (fix only in the fence; file the rest as ONE issue with
  `work.py new`, in `work/issues/` unless the owner is obvious); what the
  pattern cannot match.
- Territory: `python3 scripts/work.py territory --base origin/main` in
  the body with every owner (`offset_fit.rs` is PROPS' since #1924;
  the `geom-brep/tests` and `topo/tests` files are tcost's).

## Posture

- ε posture: none — no tolerance read moves; say so. No `CI-Config:`.
- Bit identity: the split changes no arithmetic; the `achieved` values
  the faces carry are the same numbers the old variant carried. State
  it; the existing certificate rows are the receipt.
- Review: single style review, outside the experiment.
- **Landing: the item gets `pr:` and `status: review`. DO NOT MERGE —
  the orchestrator lands after the review and its fix pass; the fix
  pass closes the item, deletes this spec at merge with its
  `## Per-merge deletion` section in `docs/DOC-LEDGER.md`.** No
  `Co-Authored-By`; push early to `props/budget-faces`.

## Acceptance

Four faces, each naming its lever; the two measured instances red
first and green under their faces; face 3 measured and recorded;
every consumer re-spelled; the sweep's hit list filed; hosted CI green
on the full matrix.
