# PROPS recourse grammar — the eleven rows PROPS closes on

**Binding at dispatch** (PROPS program). Difficulty logged at spec:
**M / SWEEP**. **Review tier: SINGLE review, FULL**
(`memories/orchestration-model.md`). Reason, recorded here so the call is
not invisible: no tricky logic and no architectural decision, so not a
dual — but the correctness question is whether each refusal offers a
lever that *actually exists* and carries a value where it has one, and
getting that wrong ships misleading guidance to a user, which reading the
diff will not catch. So the reviewer carries claims to falsify, not only
the style questions. Read `docs/prompts/implementer-discipline.md` in
full. Branch `props/recourse-grammar`, cut from `main`.

**This is the last unit of this program.** PROPS closes when it lands,
and the rest of the slate has already moved to `work/flux/`.

## The eleven rows

All on `work/props/`, all about what a refusal SAYS:

1. `coincidence-recourse-says-lower-where-d4-says-tighten` — `geom-core`'s
   `COINCIDENCE_RECOURSE` third arm says "lower the tolerance" where D4
   ¶1 names it "tighten", conditional and valued.
2. `invalid-margin-display-calls-a-refused-enclosure-poisoned` (E) —
   `MarginDiag`'s `Invalid` rendering calls an interval's *certification
   refusal* a "poisoned enclosure". Those are different things.
3. `invalid-margin-recourse-cannot-tell-an-unimplemented-kind-from-bad-inputs`
   (M) — one recourse serves both poisoned inputs and a surface kind with
   no implicit form (`Nurbs`/`Approx`), and the coincidence levers do not
   reach the second.
4. `measure-assertion-offers-an-unvalued-tighten-and-drops-its-margin`
   (M) — `editor-core`: an undecided verdict offers "tighten the
   tolerance" with no value, and drops the escalation *and its margin*
   it was decided on.
5. `measure-refused-reduces-the-typed-refusal-to-its-name` —
   `RefusalReason::MeasureRefused` carries a typed refusal as its name,
   mixing the wiring's vocabulary with the clearance engine's.
6. `nappe-spanning-spells-its-kernel-defect-ending-by-hand` —
   `PropsError::NappeSpanning` hand-spells what
   `KERNEL_DEFECT_ENDING` exists for.
7. `not-iso-rectangle-names-off-surface-edges-and-inventory-gaps-alike` —
   one refusal for an edge off its own surface (a defect) and a valid
   face outside the inventory (not one), distinguishable only at the
   raising site.
8. `props-escalation-renders-the-coincidence-menu-unlabelled` —
   `PropsError::Escalated` forwards `Indeterminate`'s coincidence menu
   unlabelled where one face's contribution leaves nothing to declare.
   TOPO's PR 3398 left `validate`'s arm saying "There is no way through
   yet" until props carries its decision; this is that decision.
9. `props-refusal-prose-outgrows-the-viewer` — **Ev's own concision
   request**: the mass-properties and measure refusals run over 50 words.
10. `quadrature-budget-refusal-names-loosening-beside-a-geometry-lever`
    (E) — `QuadratureBudget` and its checks-window mirror name "loosen
    the tolerance" outside D4's last-resort shape, beside a lever that is
    geometric.
11. `fit-error-delegates-to-two-carriers-that-name-no-recourse` —
    `FitError`'s `Lsq` and `KnotAlgebra` arms delegate to carriers that
    name no repair.

## Use the machinery; do not rebuild it

ENCL and TOPO built the homes for this while PROPS was parked, and the
seam notes are on `work/props/log.md`. **Read them before writing a
word**: `geom_core::KERNEL_LIMIT_LAST_RESORT` and `KERNEL_LIMIT_RECOURSE`
(PR 3363), `geom_core::predicate::KERNEL_DEFECT_ENDING` and
`KERNEL_OR_FILE_DEFECT_ENDING` with their `concat!` macros (PR 3346),
`geom_brep::recourse` — the one table for sized decisions, with
`Reading`, `RefusedArm`, `SizedPass`, `SizedDecision`, `Classified`,
`Unsized`, `defect_ending` and `sized_recourse` (PRs 3382, 3398), and
`geom_core::SizedPass::Negative` (TOPO PR 3493). A twelfth spelling of a
recourse is the defect this unit closes, so minting one is the one thing
it must not do.

## Rulings

- **D4 is RATIFIED. This unit conforms to it; it does not reopen it.**
  If a row cannot be served without changing what D4 decides, stop and
  report — that is a fork and it is Ev's, not yours.
- **A recourse names a lever the caller actually has, and carries its
  value when it has one.** "Tighten the tolerance" with no number is the
  defect in row 4, not a template to copy. Where no lever exists, the
  last-resort ending is the home — through `KERNEL_LIMIT_RECOURSE`, not
  hand-spelled.
- **Row 7 is a TYPING question wearing a wording question's clothes.**
  An edge off its own surface is a kernel defect; a valid face outside
  the inventory is not. If one refusal cannot honestly say which, the fix
  is two arms, not better prose. Decide it and say why.
- **Row 9 is Ev's request and is measured, not estimated.** Count the
  words in the shipped refusals, report the before and after per site,
  and do not buy concision by dropping the value a recourse needs to
  carry. A shorter refusal that no longer tells the user what to change
  is worse than a long one.
- **Sweep for the SHAPE, not the symbol** (discipline §5): every arm in
  this program's ground that composes a recourse sentence. Put the hit
  list and its disposition in the PR body, one line per hit. The
  existing `recourse_roster` suites and
  `every_props_error_arm_names_a_recourse` are the red-first instrument —
  if they do not already cover an arm you touch, that gap is itself a
  finding.
- **File what you find outside the fence** (discipline §6). Most of this
  grammar is shared with ENCL, TOPO and VERDICT; a row on their ground
  goes on their slate in this PR, and the seam is announced in their log.

## Posture

- ε posture: none expected — no band, comparand or predicate name should
  move. If one does, that is a behaviour change and it is stated, not
  folded in.
- **Refusal text is asserted on**, so string assertions will move. Each
  moved assertion is re-baselined with its reason (discipline §3); a
  moved golden is never a cost.
- Verification is hosted CI, and **a green PR is not a green nightly** —
  the `recourse_roster` and concision suites may sit outside the per-PR
  filter. Say which rows you ran and where.
- **8-core 9 GB box, machine-wide build mutex**: read
  `memories/agent-lane-operations.md` §Build concurrency, wrap heavy
  cargo calls in `local-scripts/with-build-slot.sh`, pass no `-j`, never
  two batteries at once. Own `CARGO_TARGET_DIR` outside the worktree.
  **Do not use `ps`/`pgrep` to diagnose the build slot** — read your own
  `CARGO_TARGET_DIR` from `/proc/<pid>/environ`; two reviewers glimpsed
  each other's command lines that way and it had to be disclosed in an
  experiment record. Never end a turn with background work running and
  do not sleep on a CI wait.
- Review: SINGLE, FULL.
- **Landing: all eleven items get `pr:` and `status: review`. DO NOT
  MERGE.** No `CI-Config:` trailer, no empty commits. Use the commit
  trailers the harness asks for.

## Acceptance

All eleven rows served or stopped-and-reported with a reason; every
recourse naming a lever that exists and carrying its value where it has
one; no twelfth spelling minted, with each row's home named from the
machinery above; row 7 decided as a typing question with its argument;
row 9 measured with word counts before and after per site; the shape
sweep with its hit list and dispositions; everything found outside the
fence filed on the owning slate with the seam announced; `work.py lint`
clean; hosted CI green.
