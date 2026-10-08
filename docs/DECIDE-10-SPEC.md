# DECIDE-10 — the read at its node relabels a cancellation above it: settle the parent first (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/the-read-at-its-node-relabels-a-cancellation-above-it.md`
(P2, filed by DECIDE-9's review). **Unit:** `work/decide/DECIDE-10.md`.
**Branch:** `decide/10-read-behind-the-parent`, cut from `main`.
**Implementer:** Opus. **Review tier:** single FULL review (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item whole;
- `crates/geom-core/src/sym.rs`:
  - the header section "The decision read";
  - `SymRules::decision_read` and its rules-table row, which since
    DECIDE-9 say "behind every value-free fold AT ITS NODE";
  - `combine`'s `read` arms at `Select`, `min` and `max`;
  - `form_in`;
  - `zero_factors_gate`;
- `crates/geom-core/src/sym/signed.rs`: `decision` and `order`;
- `crates/geom-core/tests/sym_root_rows.rs`: the DECIDE-9 rows, in
  particular `the_read_relabels_a_cancellation_above_its_node_filed_defect`;
- DECIDE-9's spec, which listed this unit's candidates:
  `git show 99a5b30ca:docs/DECIDE-9-SPEC.md`.

## The claim

The read answers a `min`/`max`/`Select` node at the node. With the read
shut, two distinct nodes with equal early forms mint as ONE atom, and
their difference cancels as a theorem. With the read on, each is read
first, the same zero arrives through the arms, and it counts
`sign_gated`:
- `max(x + Z, 3) − max(x, 3)`;
- `min(x + Z, 3) − min(x, 3)`;
- `max(x + Z, y) − max(x, y)`.

Here `x ∈ [1, 2]`, `y ∈ [3, 4]` and `Z = sqrt(x)² − x`.

No measured document carries the shape today (DECIDE-9's Phase 1
hook), so this unit restores the contract for the class, not a count.

**Ratified and not re-litigated:**
- the read itself (DECIDE-3), its enclosure and its `sign_gated` label;
- that it decides interior `Select`, `min` and `max` nodes;
- SYM-9's ladder;
- rule G;
- DECIDE-9's zero-product rule.

## Phase 1 — the candidates, measured

For each candidate the item names, report:
- what it restores on the three shapes (and on any further shape you
  find in the class);
- whether any decision's VALUE moves, or any `numeric` or `registered`
  count, on the bracket replay at `certifies_at`, the pad's release leaf
  at `1e2·ε` and `m10_10_pins` (dev);
- its leaf cost: pad, release, best of 3, against `main`.

The three candidates:
1. **Settle read-free first.** Try the decision form read-free, and read
   only where that does not settle.
2. **Read at the decision form, not at interior nodes.** This withdraws
   the read from interior nodes, which DECIDE-3 ratified.
   - Measure it only as a comparison.
   - It is not shippable in this unit: shipping it changes a ratified
     decision, and that is an `[ev]` question.
3. **Keep the node symbolic until its parent has folded.** Mint the
   atom, keep the arm beside it, and substitute only where the parent
   does not settle.

Recommend one, with its costs. Put a minimal row for every shape you
add.

**Stop rules.**
- If no shippable candidate (1 or 3) restores the shapes without moving
  a decision's value or a `numeric` count, stop after Phase 1 and
  report.
- If the only one that works is candidate 2, stop after Phase 1 and
  report. The orchestrator takes it to Ev.

## Phase 2 — the candidate Phase 1 justifies

- **The invariant:** every decision the read-free walk proves is a
  theorem with the read on, for the shapes and the class. `sign_gated`
  keeps only what no form settles. No `numeric` or `registered` count
  moves.
- **Rows:**
  - `the_read_relabels_a_cancellation_above_its_node_filed_defect`
    flips to `theorem`; rename it to say what it now pins;
  - one row per shape;
  - the read still `sign_gated` on a comparison no form settles.
- **The doc.** The read's doc and its rules-table row state the ordering
  the code now keeps; drop "at its node" if it no longer qualifies the
  claim.
- **Re-baselines:** every pin that moves, said. Ev's standing words:
  "never skip out on a change that would make the code better because
  it would require rebaselining".
- **Leaf cost,** best of 3, release, `ON + the ladder`, before and after.

## Scope

- **Files:**
  - `crates/geom-core/src/sym.rs`, `sym/signed.rs` and `sym/form.rs`;
  - tests under `crates/*/tests`;
  - the unit and item files.
- **Not in scope:**
  - the read's enclosure, its depth, and any dial's default;
  - the ladder;
  - `Form::mul`'s gate OR (`form-mul-carries-the-gate-of-a-factor-a-zero-annihilates`,
    its own P3).

## Review

**Single FULL review**, recorded in `work/decide/log.md` at spec time.
It is a contract-restoring change to the tier's labels, settled by
minimal rows and receipts, and reversible, as DECIDE-9 was. Claims to
falsify:
1. each shape a theorem with the read on, and the class closed (build
   one the implementer did not);
2. no decision's value and no `numeric` count moves;
3. `sign_gated` still fires where no form settles;
4. the cost;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item.
