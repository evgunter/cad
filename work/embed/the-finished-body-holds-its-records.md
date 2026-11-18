---
id: the-finished-body-holds-its-records
kind: issue
title: The finished body is one type holding the body, its contact records and its verdict, with finish its only creator (Ev, PR 7068, decision 2)
status: open
opened: 2026-11-18
priority: P0
cost: H
refs: [the-at-rest-gate-certifies-embedding]
---

## Ruling

Ev on PR 7068 (2026-11-18), to its second decision ("the finished body
holds its records, and `finish` really runs tier 3′"): "yes, we should
definitely have a type for this bundle with finish as the only creator.
i'm surprised we don't already have this". The decision's text is the
finished-body paragraph of `docs/DESIGN.md` `[topology.tiers]` on
`embed/ev-finished-body`, and both designers' reports in that PR's
body. The PR merges when Ev has answered its third and fourth
decisions; this unit builds only the second and does not wait for it.

## What is wrong today

Nothing represents "a body with its records". `AtRestBody`
(`crates/topo/src/validate.rs`) is a body and a verdict, and
`topo::finish` (`crates/topo/src/finish.rs`) is handed no
`ContactRecords` (`crates/topo/src/boolean/records.rs`). The records
ride beside the body, each copy kept in step by hand:
- `BooleanBody` (`crates/topo/src/boolean/ops.rs`), whose doc says of
  its records that the door has not checked them;
- the evaluator's `Carry`;
- `Product`;
- the import's copy (`crates/step-import/src/lib.rs` calls
  `topo::record_at_rest` after the finish);
- the Python `Body`'s.

Who has the records, and when:
- **A Boolean** holds them before its finish (`Held` in
  `boolean/ops.rs`) and does not pass them on.
- **`shell`, split and blend** take a bare body; the evaluator
  attaches the records after the door returns.
- **A `⊔`** (assemble, product, import) has none when its aggregate
  finishes: its census (`record_at_rest`, `crates/topo/src/census.rs`)
  mints them afterwards over the finished body, and at an assemble node
  only on first read.

Three ATREST rows are this one gap:
`work/atrest/a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys.md`,
`work/atrest/a-boolean-reading-two-bodies-of-one-arrival-re-decides-their-contact.md`
and
`work/atrest/a-boolean-over-an-instance-drops-its-parts-census-findings.md`.

## The unit

1. **One finished type.** The scalar's finished type bundles the body,
   its contact records and its verdict. `finish` is its only
   constructor, its fields are private, and nothing else can pair a
   body with records and call the pair finished.
2. **`finish` runs tier 3′.** It is handed the records the door
   decided, and confirms each (tier 3′ certifies records in both
   directions) before anything relies on it. A door that takes finished
   operands has their records; it carries them through its lineage onto
   its result before the finish, not after.
3. **A `⊔`'s records are minted inside its finish**, since the
   finished type has one creator. Whether an assemble node's census can
   stay lazy (run on first read) under that is the first thing to
   settle: if it cannot be kept without a second creator, stop and ask.
4. **The loose copies become reads of the finished body:**
   `BooleanBody::contacts`, the evaluator's `Carry`, `Product`, the
   import's copy and the Python `Body`'s.

**Not in this unit:**
- an assembly's reported overlap riding the finished body (PR 7068's
  third decision, open);
- what a record between two cells of one solid does to the
  injectivity certificate (its fourth decision, and lane C of
  `the-at-rest-gate-certifies-embedding`);
- the census reading a deciding door's meetings between its result
  solids (waits on
  `work/reachhold/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`);
- the certificate itself: `embed/gate` (lane C) is unmerged and edits
  `validate.rs` and `finish.rs`. This unit is cut from main.

**Known to be coming, so shape for it without building it:** PR 7101
recommends that `finish` also be handed the finished operands its body
was built from, to carry the embedding facts of unchanged face pairs.
The operands are where a door's carried records come from too.

## Measure

Running tier 3′ inside every finish puts the coincidence census in
every finish, and nothing has measured that. Before and after, on one
box and build: the workspace's `ci` profile wall time and total test
time, and the census's own seconds and call count by finish site (a
Boolean's gate, `shell`, split, blend, the sweep doors, `topo::take`,
the product's aggregate, import). Every verdict that moves, either
way, with its test.

## Done when

- The finished type is the one home of a body's records, with `finish`
  its only constructor, and the five loose copies are gone or are reads
  of it.
- A record a door hands `finish` that tier 3′ does not confirm refuses
  at that door, pinned for each kind of deciding door, with the mutant
  "the gate takes the record on its word" red.
- A Boolean result reused as an operand keeps its records (the first
  ATREST row's pin), and the three ATREST rows are closed or re-scoped
  with what remains.
- The price is on this row.
