# DECIDE-6 — the decision read's cost: decline before enclosing (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/decision-read-triples-the-plate-pin-suites-wall-time.md`
(P2, cost D). **Unit:** `work/decide/DECIDE-6.md`. **Branch:**
`decide/6-read-cost`, cut from `props/sign-hull` at `1264640fa`
(DECIDE-5's merge). The PR targets `props/sign-hull`, and no `main` is
merged into it. **Implementer:** Opus. **Review tier:** single FULL
review (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item whole, including SYM-9's re-take (the read costs nothing
  measurable on the release leaf instrument, and everything on the dev
  pin suites);
- `crates/geom-core/src/sym/signed.rs`: `decision`, `order`,
  `enclose_indet`, `enclose_deep`, `ENCLOSE_DEPTH`, and `fold`'s
  enclosability pre-pass;
- `sym.rs`'s `combine` and `SymRules::decision_read`'s doc (the read is
  ordered behind every value-free fold, and its zeros are `sign_gated`);
- `sym/form.rs`'s `digest`;
- the pins the read moved at DECIDE-3 (`decide_3_split_rows_interval`,
  and the `m10_10_pins_interval` rows that name `sign_gated`).

## The claim

The decision read asks `signed::decision` at every `Select` node and
`signed::order` at every `min`/`max` node the early walk reaches. Each
call encloses both halves of the decision form to `ENCLOSE_DEPTH = 8`
before it can decline, and most calls DO decline, because the form
carries an indeterminate no bracket is known for. That is why
`m10_10_pins_interval` went from 143 s to 535 s in dev at DECIDE-3.

**The change:** answer the decline before paying for the enclosure,
without changing what the read decides. The item names three cheap
answers, in order:
1. an enclosability pre-pass, rule C's test widened by one level: a
   parameter, π, or an atom whose own arguments pass;
2. the enclosure memoised per form digest inside the session;
3. a depth-1 attempt first, widening only where it straddles.

**The invariant, and the acceptance:** every decision the read makes is
unchanged, as a decision. The class (`symbolic_zero`, `sign_gated`,
`registered`, `numeric`) is the same on every measured document, and
the pins do not move. A pre-pass that declines a read which would have
succeeded changes a decision. That is a defect, not a trade, and a row
must catch it.

## Phase 1 — measure before touching anything

1. **Where the cost is.** Instrument the read (under
   `sym-profile-testing`, not in shipped code): calls, declines, and the
   cause of each decline (an unbracketed indeterminate at depth 0 /
   deeper, a straddle, depth exhausted), and the time in `enclose_deep`.
   Run it on the plate's pin suite's documents and on the link.
   One table.
2. **What each answer would save**, by that table:
   - the declines the pre-pass could answer without enclosing;
   - the repeats a digest memo would hit;
   - the reads a depth-1 attempt would settle.

   If the pre-pass alone takes the suite back within 1.2× of its
   pre-DECIDE-3 time (143 s), the unit takes only the pre-pass.

## Phase 2 — the answers Phase 1 justifies, in the item's order

- **The pre-pass.** It lives beside `fold`'s, in one helper both use if
  the tests are the same test at different depths; say which. Its
  soundness argument is one sentence: an id-walk that finds an
  indeterminate with no bracket at any depth the enclosure would reach
  proves the enclosure would return unbounded, so the read would
  decline. Check that the argument holds at the depth the enclosure
  actually descends to, and pin it.
- **The memo, if taken.** It is keyed by the form's digest and scoped to
  the session. It is dropped where the session drops its memos, and it
  never crosses leaves or boxes: the enclosure depends on the box.
  State why a digest collision cannot return a wrong enclosure, or key
  it on the form itself.
- **Depth-1 first, if taken.** It must be monotone: a depth-1 success is
  a success at depth 8.
- **Rows:**
  - a read the pre-pass declines is one the full enclosure declines, at
    several shapes, including an atom whose argument is bracketed
    two levels down;
  - a read that succeeds only at depth ≥ 2 still succeeds;
  - the memo does not leak across boxes.
- **Measure again:**
  - the three suites' dev wall times (`m10_10_pins`, `m10_8_pins`,
    `m10_9_pins`) before and after, on this box, one heavy row at a time;
  - the release leaf instrument's times, which should be flat, and every
    receipt identical.

## Scope

- **Files:** `crates/geom-core/src/sym/signed.rs`, `sym.rs` (only if the
  memo lives on the session), and tests.
- **Not in scope:** no change to what the read decides, to its ordering,
  or to any rule's dial.

## Review

**Single FULL review**, recorded in `work/decide/log.md` at spec time.
The change is a cost change behind an invariant that the pins and a
receipt-equality row can check. Whether it holds takes executing the
pre-pass against the enclosure, not only reading it, so the review is
FULL rather than STYLE. It is not a design decision and it is
reversible, so it is not DUAL. Claims to falsify:
1. no decision changes (receipt equality on the measured documents, and
   the pre-pass against the enclosure at adversarial shapes);
2. the memo's scope and key;
3. the timings;
4. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit and the item, and deletes
  this spec (with a note under `docs/doc-ledger/`).
