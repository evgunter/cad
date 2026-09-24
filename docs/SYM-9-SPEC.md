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

## A1 — the 2026-09-22 amendment: DECIDE-3 landed, block DECIDE-B1 slot 1

Written by the orchestrator at dispatch; binding with the spec above,
and where the two differ this section wins.

**Block and tree.** SYM-9 is block **DECIDE-B1 slot 1** (the pre-draw
fields above stand — H / NUMERIC; the draw on `decide/b1-block`: byte
248 ⇒ fable at slot 2, so this slot's arm is **OPUS**); protocol v7 IN
(it changes what the door answers on a refusal); the full v6 dual. The
unit lives on DECIDE's slate (`work/decide/SYM-9.md`); the item it
answers is SYM's (`work/sym/coefficient-ring-width-is-not-monotone-in-reach`).
**DECIDE-3 (rule G, the two certified reads, A0's constant fold) is on
`props/sign-hull` (#3039 at `cd14d4fd9`), not on `main`**, and the
retry this unit builds is over the atoms rule G re-keys — so the branch
`sym/9-retry-ladder` is cut from `props/sign-hull`'s head, the PR
targets `props/sign-hull`, and no further `main` is merged into it (the
SYM-12/DECIDE-3 seam at `manifest.rs` is the `props/sign-hull` merge's
own measurement, `work/sym/the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic`,
and not this unit's). The six documents' numbers this unit measures
against are DECIDE-3's on that branch: plate 811/0/140/462, link
541/0/96/465, bracket 1104/7/144/766, pad 890/6/150/907, slab 490/255,
and R1's boss as its rows on that branch pin it.

**Phase 1 gains a table that already exists, and a shape.** The
kept-atom retry (b) has a MEASURED instance:
`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`
— on R2's link, rule G costs `carrier_on_surface_2` sixteen theorems
(`[98, 0, 0, 10]` → `[82, 0, 6, 20]`, pinned on both sides by
`decide_3_split_rows_interval`): TEN to rule A's companion rewrite
`|X|² = X²` opening the square of an `abs` node the document wrote (an
expansion that does not cancel where the closed atom did), SIX to
`sqrt(R²) = |R|` closed by the registrant's axiom. That row's remedy
record is where this unit starts: the provenance restriction measured
and REJECTED (recovers ten of sixteen, costs the document forty — after
rule G a `sqrt(R²)` and a document's `abs(R)` are one atom, so the
restriction narrows by traversal order); a size guard on the rewrite
recorded and ruled out as narrowness; the residual the registrant
closes still owed a render under both dial sets with its two atom keys
read off. So Phase 1's item 2 measures, beside "with rule A off", the
retry shapes **"with the companion rewrite off"** and **"with rule G's
magnitude door off"** on every refused decision of the six documents,
and the link's sixteen are the first row of its table. The row CLOSES
on a retry that recovers them without costing the document (the
predicate's both-sides pin then moves UP and is re-baselined and said),
or stays open with the measurement that says no retry does.

**The decision read's cost**
(`work/decide/decision-read-triples-the-plate-pin-suites-wall-time`,
P2) rides only as far as Phase 1's cost table: if the read dominates
the retry ladder's cost on any document, Phase 2 takes that row's
first cheap answer (the enclosability pre-pass, which changes what
nothing decides) and re-baselines the suites' wall times; otherwise the
row stays where it is.

**Ratified since the spec, not re-litigated:** rule G's canonical form
and the two certified reads (DECIDE-3, on `props/sign-hull`); rule F's
negative arm and the leaf's NEED (SYM-12, SYM-13, on `main`) — not in
this tree, and nothing here anticipates them.

**Ev's standing words for this session, carried:** never skip a change
that would make the code better because it requires re-baselining — a
pinned split that moves UP under a retry is re-baselined and said,
never held against the retry; the code that goes in is clean; every
causal sentence in the PR body, the rows and the headers names the
execution that shows it, and a retraction greps the CLAIM's vocabulary
across every file of the unit, not the sentence remembered.
