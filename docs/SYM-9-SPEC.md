# SYM-9 — what a refused decision may retry: the wider ring and the kept atom (spec)

**Program:** SYM (`work/sym/plan.md`, the ceiling lane). **Item:**
`work/sym/coefficient-ring-width-is-not-monotone-in-reach.md` (SYM-3's
finding; SYM-5 PR-2's review added a second mechanism — the scale
step's width cost). **Track:** kernel change — the standard v6 unit
(binding spec, drawn implementer arm, cross-model dual review, union
fix pass, record-at-merge; §Review). Block SYM-B2 slot 2. **Pre-draw
fields, logged before the draw:** difficulty **H**, task-class
**NUMERIC**.

- **H** — a change to the discharge LADDER (plain → early → door),
  which is the tier's central policy, argued from a non-monotonicity
  the record already shows.
- **NUMERIC** — it moves what the tier decides; the six documents'
  splits and ceilings are the acceptance.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole (both mechanisms: the frozen node that matches itself as one
indeterminate and is lost when opened; the scale step's bits);
`sym.rs`'s header (`# Freezing`, `# Cost`, the ladder in `discharge`
and its three rungs), `discharge` itself, `sym/rational.rs`
(`COEFF_BITS`, `Rat::from_parts`'s refusal, the readings that set the
bound — module docs), `sym/form.rs` (`FormSize`), `sym/profile.rs`
(freeze causes: `Terms` / `Degree` / the ring), SYM-3's log entry
(`work/sym/log.md`, "SYM-3 merged") and SYM-4's; the bulge fixtures
(`m10_bulge_interval.rs`, `m10_bulge_renders.txt`); the SYM-5 PR-2
width row (`r2_the_scale_step_loses_…`, adopted by its fix pass).

## The claim

The ring is bounded at `COEFF_BITS = 256`; a product past it FREEZES
the node into one opaque indeterminate. Two things the record shows:

1. **Widening the ring is not monotone in reach.** At 512 bits R1's
   boss gains (`carrier_matches_mapped_source` 6 → 0 numeric, its
   ceiling up) while, with an atom opened, decisions the door had
   closed — because a frozen node matches ITSELF as one indeterminate
   on both sides — fall numeric once the node is expanded into forms
   the ring cannot close. And the ring's cost is 4–9× per probe.
2. **A rule can cost bits**: rule E's scale step can push a product
   past the ring where the unscaled spelling fit (the width row).

So the ladder today makes one attempt per rung with one ring and one
rule set, and a decision the attempt refuses is numeric — even where a
second attempt, at a wider ring OR with the atom kept opaque, would
have closed it. **The proposal: a refused decision may retry, on the
early and door rungs only, (a) with the ring at a wider bound and (b)
with the rule that opened an atom off** — each retry a bounded cost
paid only on refusals (a few percent of decisions on every measured
document), each a sound discharge on its own terms (the ring's bound
is a cost, not a soundness condition; a rule set with fewer rules is a
subset of the same algebra), counted in the receipt so the retry that
closed it is visible.

**Ratified and not re-litigated:** E12; the three rungs and their
order (plain first — a plain theorem is never re-labelled); `frozen`'s
meaning; rule C's gating; `COEFF_BITS` as the FIRST attempt's bound
(the readings that set it stand; this unit adds attempts, it does not
move the first).

## Phase 1 — before touching anything (the measurement)

1. **Where the refusals are.** On the six measured documents (plate,
   annulus, bracket, pad, link, R1's boss) at the nominal and at each
   document's ceiling: the count of decisions refused at each rung and,
   per refusal, the freeze cause on its path (`Terms` / `Degree` /
   the ring) — `sym::profile` already has the causes; add the per-
   decision attribution under `sym-profile-testing` if it is missing.
   One table.
2. **What each retry would recover**, by a hand-driven second attempt
   (an env-driven probe, reverted): the refused decisions re-run at
   512 bits; re-run with rule E off; with rule C off; with rule A off.
   Per document: recovered by (a), by (b) per rule, by neither, and
   the cost of each retry per refusal (release). If (a) recovers
   nothing on any document, or the cost per retry is over the leaf
   line on every document, the unit stops after Phase 1 and reports.

## Phase 2 — the retries the measurement picks

1. **`SymBudget` gains the retry ladder** (`retry_bits: Option<u64>` —
   the wider ring's bound, `None` = no retry; `retry_without:
   SymRules`-shaped mask — which rules the kept-atom retry turns off),
   defaults chosen by Phase 1's numbers, `DEFAULT_SYM_*` in
   `drive.rs`'s dials by announced seam if a default lives there.
2. `discharge`: after the early rung refuses, the retries in the order
   Phase 1 ranks them; the same after the door rung; the plain rung is
   never retried. A retry builds its forms in a SEPARATE memo (the
   early memo's forms are the first attempt's and stay), keyed by
   `(id, attempt)`, dropped with the decision or kept for the leaf —
   measured, stated.
3. **The receipt** says which attempt closed it: `symbolic_zero` stays
   the count (a retry's theorem is a theorem); a new `retried` column
   counts decisions closed by a retry, and the profile's ledger carries
   the attempt. No new K token.
4. The header: `# Freezing` gains the ladder's second attempts with the
   argument above; `# Cost` the numbers.

## Scope

- Files: `crates/geom-core/src/sym.rs`, `sym/profile.rs`,
  `sym/rational.rs` (the bound as a parameter of the operations, not a
  constant, if that is what a wider retry needs — say so), tests under
  the program's globs; `crates/editor-core/src/drive.rs` only for a
  default dial, by announced seam.
- No change to the plain rung, to rule C's gating, to what any rule
  computes, or to `COEFF_BITS`'s value for the first attempt.

## Acceptance

- Phase 1's two tables in the PR body.
- If shipped: every pinned split moved UP or not at all on the six
  documents; every ceiling unmoved or up; the receipt's `retried`
  column pinned on the documents where a retry closes something; the
  cost on the leaf instrument with and without retries, disclosed
  against the line; a growth guard for the retry memo.
- A negative row: a decision the first attempt refuses AND every retry
  refuses stays numeric, with the receipt showing the attempts.

## Review

The full v6 dual. Claims to falsify: (1) soundness of a wider-ring
discharge (the ring's bound is a cost — show a discharge at 512 bits is
the same identity of reals) and of a fewer-rules discharge; (2) the
retries never re-label a plain theorem or a gated one; (3) the numbers
— refusals recovered per document, and none lost; (4) the memo
discipline (a retry's forms never leak into the first attempt's memo);
(5) the cost; plus `docs/prompts/reviewer-style-lane.md` in full. Union
fix pass on the implementer's lane; delta by R1; the row lands at merge.

## Landing

Status `review` on `work/sym/SYM-9.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge; the ring
row closes on the ladder or on Phase 1's "neither recovers" report.
