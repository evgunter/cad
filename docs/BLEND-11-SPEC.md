# BLEND-11 — the overrun refusal reports the nearest fit, by a stated rule (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 11). **Item:**
`work/blend/overrun-attribution-picks-the-first-candidate.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — one arm of one loop (`sugar::arc_fillet_trims`), a stated rule
  for a pick, and rows that pin the rule; the measurement exists and is
  reproduced, not designed.
- **STRUCTURAL** — the rule is decided here; the lane gives it one spelling
  and a pin. No numeric question is open.

**Territory note.** `crates/profile/*` is S-BOOL's glob; the candidate
machinery is edited by announced seam (`work/bool/log.md`, at dispatch).
Nothing outside `crates/profile/src/sugar.rs`, `crates/profile/src/path.rs`
(only if the payload's rendering needs a word) and `crates/profile/tests/**`
moves.

## The claim

**At one corner, the anchor-fit refusal reports the FIRST corner-side
candidate whose trim overran, chosen by enumeration order, and the numbers
the author reads are that candidate's.** `sugar.rs::arc_fillet_trims`'s
loop keeps the first overrun (`} else if corner_side && overrun.is_none()`)
and reports it when no candidate survives; on FILLET-ATTR's grid A, 232 of
18,144 authorings had a SECOND corner-side candidate also overrun and
dropped, with different payload numbers (a kept setback of 0.259 m against
a discarded 0.609 m). The site's comment argues attribution is sound
because `corner_side` is a real test — true of the CORNER, silent on which
of several corner-side candidates at the same corner is reported.

**The rule, decided by the orchestrator (2026-09-13).** FILLET-ATTR's ruling
one level up (`crates/profile/README.md`, "The refusal envelope") names
every corner a pair tried and sorts by a presentation key nothing branches
on. One level down the candidates are circles rounding the SAME corner, and
the author's recourse ("reduce the radius or move the anchor") is metered
against the reported setback, so the honest pick is **the candidate the
author is nearest to fitting: the overrunning candidate with the smallest
deficit, `max(setback_in − extent_in, setback_out − extent_out)` over its two
legs** (the least radius reduction that would make it fit), ties by
enumeration order as the envelope's ties are. The pick is an `f64`
enclosure read of a quantity nothing decides on — no new predicate, no
branch on a margin, the recorded sample sequence unchanged (the loop
already classifies all four gates for every candidate). The payload
(`DoesNotFit` → `CornerReason::AnchorOutsideTrimmedExtent { side, carrier,
setback, available }`) does not change shape; its numbers become the
nearest fit's.

**Ratified and not re-litigated:** the envelope (README, FILLET-ATTR); the
four-classifications-per-candidate invariance and its escalation shape
(the comment above the loop); the corner-side test as the corner's
attribution.

## Phase 1 — measure before touching anything

Reproduce the item's measurement on grid A
(`crates/profile/tests/review_fillet_attr_r2_probes.rs::grid_a`): count the
corners where two or more corner-side candidates overrun; for each, the
first candidate's and the least-deficit candidate's `(setback, available,
deficit)`; how many corners would change their reported numbers under the
rule and by how much (min / median / max of the setback difference). One
table in the PR body BEFORE Phase 2 begins. If the count of corners where
the two picks differ is zero on grid A, construct one and say what it took.

## Phase 2 — the change

1. The overrun arm keeps the least-deficit candidate: compute the deficit
   per overrunning candidate from the four numbers it already has, keep the
   smaller, ties to the earlier. The comment above the arm states the rule
   and why (the recourse is metered against the reported setback), replacing
   the corner-attribution argument's silence with one sentence on the
   candidate level; the corner-level sentence stays.
2. **Rows**, in `crates/profile/tests/fillet_overrun_nearest_fit.rs`
   (aggregated; subject-named): a corner where two corner-side candidates
   overrun with different deficits, asserting the reported `setback` /
   `available` are the least-deficit candidate's; the recourse followed —
   reducing the radius by the reported deficit (plus one ulp of slack,
   stated) builds, and reducing it by the DISCARDED candidate's deficit
   would have been more than needed; a grid-A census row asserting the
   count of corners whose report changed matches Phase 1; the sample
   sequence invariance (the K samples per corner unchanged before and
   after, through the `Probe` scalar or the k-stats bracket).
3. **The mutant**, in the PR body: restore the first-pick and show exactly
   the nearest-fit rows red.

## Constraints, binding

- **Every fillet that builds today builds bit-identically** (the pick only
  moves a refusal's numbers): dump the profile suites' fillet outputs at
  both SHAs and diff.
- **Every refusal that fired at the merge base fires at the head with the
  same variant**; only the payload numbers of `AnchorOutsideTrimmedExtent`
  at multi-candidate corners may move, and Phase 1 says at how many.
- **No new predicate; no branch on a margin**; the sample sequence per corner
  class is unchanged.

## Acceptance

- The Phase 1 table; the rule at the site with its sentence; the rows; the
  mutant table; both differentials clean; hosted CI green.

## Out of scope

Reporting every candidate (an envelope one level down — a payload-shape
change the editor renders; file it if the measurement argues for it); the
corner-level envelope; the enclosing class.

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every fillet that built at the merge base is bit-identical at the
  head, and every refusal keeps its variant (re-run both differentials).
- **C2** The reported candidate IS the nearest fit: instrument the loop
  yourself over grid A and over your own corners (arc × arc with four
  centres; a corner where the two overruns are on different legs) and
  compare the report with the least deficit.
- **C3** The recourse is followable and tight: reduce the radius by the
  reported deficit and build; show a smaller reduction still refuses.
- **C4** The sample sequence per corner class is unchanged (count K samples
  before and after through the `Probe` scalar).
- **C5** The rule's sentence at the site is true of every path to the arm —
  including the escalation path (`?` aborts before the pick) — and the
  README's envelope prose does not now contradict it.
