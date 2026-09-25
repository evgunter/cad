# DECIDE-7 — rule G's leaf cost: where inside the canonical root the time goes, and the part that can go (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/rule-g-is-the-link-and-pads-leaf-cost.md` (P2, cost D;
DECIDE-6's finding, with the review's separation). **Unit:**
`work/decide/DECIDE-7.md`. **Branch:** `decide/7-rule-g-cost`, cut from
`props/sign-hull` at `a7dd5c520` (DECIDE-6's merge). The PR targets
`props/sign-hull`, and no `main` is merged into it. **Implementer:**
Opus. **Review tier:** single FULL review (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item whole: the release and dev tables, and the separation from
  the exact quotient and A0's folds;
- `decision-read-triples-the-plate-pin-suites-wall-time`'s DECIDE-6
  sections, for the instrument and how it was used;
- `crates/geom-core/src/sym/root.rs`, whole: rule G's mint site, the
  content split, `sqrt(N/D)`'s side condition, `sqrt(R²) = |R|`, the
  magnitude door, and the exact quotient;
- `sym.rs`'s `combine`, `mint_atom` and `SymRules::canonical_root` /
  `abs_square` / `root_magnitude`, with their docs;
- `sym/profile.rs`, which already clocks each walk and the read.

## The claim

With rule G shut (`without_canonical_root`):
- the pad's release leaf goes 73.8 → 17.2 s, and its dev leaf
  280.6 → 122.4 s;
- the link's release leaf goes 8.5 → 3.8 s.

Neither the exact quotient nor A0's `min`/`max` folds carries the time.
What is not known is where inside rule G it goes. The candidates are:
- the mint site's own work: the content split, the side condition's
  sign reads, `poly_sqrt`'s perfect-square search;
- the magnitude door;
- the larger or different forms the canonical atoms leave downstream,
  which every later walk then pays for.

**Ratified and not re-litigated:** rule G as Ev ruled it on #2970
(shape 1: the canonical root at the mint site); what it decides on every
measured document; SYM-9's ladder and its shipped default; DECIDE-4's
exact quotient; DECIDE-5's span.

## Phase 1 — measure before touching anything

1. **A per-part clock inside rule G** (under `sym-profile-testing`,
   beside `ReadProfile`). It covers:
   - calls and time in `root::canonical` by branch: exact quotient,
     `den = 1`, the split, `sqrt(R²)`, and each decline;
   - `poly_sqrt`'s attempts, successes and time;
   - the side condition's reads;
   - the magnitude door.

   Also record the form sizes the walks see downstream, terms and
   degree per node, with G on against G off at the same node where the
   walks align. Run it on the pad's and the link's leaves (dev), and the
   pad on the release leaf instrument. One table.
2. **Attribute the 58 s (pad) and 5 s (link) differences:**
   - the mint site's own time;
   - the downstream walks' extra time (the rest of the difference);
   - within the mint site, which branch.
3. **The candidate answers, with what each would save.**
   - If the time is at the mint site: memoise `canonical` per argument
     digest inside the session, cheaper declines (a perfect-square
     pre-check before the full `poly_sqrt` search), or a cheaper side
     condition.
   - If the time is downstream: whether a different canonical spelling
     of the SAME atom would keep forms smaller. That changes no
     decision, but it changes the forms, so ledger digests move.

**Stop rules.**
- If every answer that recovers the time changes what rule G decides (a
  receipt moves), stop after Phase 1 and report. The orchestrator then
  takes the trade to Ev, since rule G's design is Ev's ruling.
- If Phase 1 finds the time is intrinsic to the canonical forms, with
  no cheaper spelling of the same decisions, the unit closes at Phase 1
  and the item records the cost as rule G's price.

## Phase 2 — the answers Phase 1 justifies

- **Every decision unchanged:**
  - receipts identical on the plate, annulus, bracket, pad, link and
    boss;
  - no split pin moves;
  - form digests may move (the walk ledger), and each one that does is
    re-baselined and said.
- **A memo, if taken:** keyed on the form, not a digest, or with a
  stated collision argument. Session-scoped, never across boxes.
- **Measure again:** the release leaf instrument, best of 3, on the pad,
  link and bracket; `m10_10_pins` in dev, before and after. Ev's
  standing words: "never skip out on a change that would make the code
  better because it would require rebaselining".

## Scope

- **Files:** `crates/geom-core/src/sym/root.rs`, `signed.rs` (the
  perfect-square search), `profile.rs`, `sym.rs` only where a memo lives
  on the session, and tests.
- **Not in scope:** what rule G decides, its ordering, any dial's
  default, and the ladder.

## Review

**Single FULL review**, recorded in `work/decide/log.md` at spec time.
It is a cost change behind a receipt-equality invariant, like DECIDE-6.
Claims to falsify:
1. no decision changes (receipt equality on the six documents, and
   probes at the canonical root's edge shapes);
2. the attribution in Phase 1's table;
3. the memo's key and scope, if one is taken;
4. the timings;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item or re-states it
  with what is left.
