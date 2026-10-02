# DECIDE-9 — the decision read answers theorems: keep it behind every form that settles (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/the-decision-read-answers-theorems-the-must-carry-stations-would-prove.md`
(P2), filed 2026-10-01 by LINALG's merge of `main` into
`props/sign-hull`. **Unit:** `work/decide/DECIDE-9.md`. **Branch:**
`decide/9-read-behind-theorems`, cut from `props/sign-hull` at
`494d477ef`. The PR targets `props/sign-hull`, and no `main` is merged
into it. **Implementer:** Opus. **Review tier:** single FULL review
(§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item whole;
- `crates/geom-core/src/sym.rs`'s header section "The decision read",
  the doc of `SymRules::decision_read` ("ordered behind every value-free
  fold"), its row in the rules table, and `combine`'s `read` arm at the
  `Select`, `min` and `max` nodes;
- `crates/geom-core/src/sym/signed.rs`'s `decision` and `order`, and
  the module doc's "three reads; one enclosure";
- `crates/editor-core/tests/m10_9_pins_interval.rs`'s `measured_studies`
  notes on the pad and the bracket, and
  `m10_9_no_registrant_lies_on_any_measured_document`.

## The claim

With the read on, these decisions count `sign_gated` although they are
theorems with the read shut:
- 32 of the pad's (893 against 925 `symbolic_zero`);
- 16 of the bracket's (1105 against 1121).

The arithmetic suggests the pad's 28 are `main`'s must-carry stations,
but no per-predicate split has shown it. The read's contract says it is
ordered behind every value-free fold, so a form that settles never
reaches it. These forms do settle, so the read is answering ahead of a
fold somewhere.

The likely mechanism is to be shown, not assumed. The read resolves a
`min`/`max`/`Select` node at the node itself, taking the arm its
certified comparison picks. The decision form above that node would
have cancelled identically with the node left symbolic (for example
`max(A, B) − max(A, B)`), but now cancels only through the read's
box-wise choice, so its zero is gated.

**Ratified and not re-litigated:**
- the read itself (DECIDE-3), its enclosure and its `sign_gated` label;
- SYM-9's ladder;
- rule G;
- what the read decides where no value-free form settles.

## Phase 1 — measure and attribute

1. **Which predicates, and which nodes.** Take the per-predicate split
   on the bracket at the nominal (`m10_10_splits_at_the_nominal_under_a_rule_set`,
   `CAD_M10_10_DOCS`), shipped against `without_the_reads`. Do the pad on
   the release leaf instrument only: its nominal split runs out of
   memory on this box. For each decision that moves from theorem to
   `sign_gated`:
   - render its early form with and without the read;
   - name the node the read answered at, and the form above it that
     would have cancelled.
2. **The mechanism, confirmed or replaced.** Is it the node-level read
   ahead of a cancellation above it? Or is it the statement being
   narrower than it reads, with the read correctly ordered at its node?
   Say which, with one minimal row per shape found.
3. **The answers, with what each costs.** For example:
   - try the decision form read-free first, and read only where that
     does not settle;
   - read at the decision form rather than at interior nodes;
   - keep the node symbolic until its parent has had its folds.

   For each: what it restores, whether any decision's VALUE moves (it
   must not), and its leaf cost.

**Stop rule.** If no answer restores the theorems without changing what
the read decides where no form settles (a `numeric` or a door answer
moving), stop after Phase 1 and report.

## Phase 2 — the answer Phase 1 justifies

- **The invariant.**
  - Every decision the read-free walk proves is a theorem with the read
    on. The pad reads 925 and the bracket 1121 `symbolic_zero`, or the
    split says why not.
  - `sign_gated` keeps only what no form settles.
  - No `numeric` or `registered` count moves except as the split
    attributes.
- **Re-baseline every pin that moves, and say each.** At least
  `m10_9_pins_interval`'s pad and bracket notes and `sym_9_retry_interval`.
  Ev's standing words: "never skip out on a change that would make the
  code better because it would require rebaselining".
- **Rows:**
  - one per shape Phase 1 found, each a theorem with the read on;
  - the read still `sign_gated` on a comparison no form settles.
- **The doc.** The read's doc and its rules-table row state the ordering
  the code now keeps, so they stop overclaiming.
- **Leaf cost,** best of 3, release, `ON + the ladder`, before and after.

## Scope

- **Files:**
  - `crates/geom-core/src/sym.rs` and `sym/signed.rs`;
  - tests under `crates/*/tests`;
  - the unit and item files.
- **Not in scope:** the read's enclosure, its depth, any dial's default,
  and the ladder.

## Review

**Single FULL review**, recorded in `work/decide/log.md` at spec time.
It is a contract-restoring change to the tier's labels, settled by
receipts and minimal rows, and reversible. Claims to falsify:
1. the attribution, against the split;
2. no decision's value and no `numeric` count moves;
3. `sign_gated` still fires where no form settles;
4. the re-baselines;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item.
