# POTDOC: the document-built pot's cavity certifies at Interval

Unit: `work/clear/document-pot-cavity-refuses-departure-at-its-mouth-rim-at-interval.md`
(P1/M). Program: CLEAR. Branch: `clear/potdoc`. Review: a single FULL
review with mutation. It blocks SHELL-4.

Read the row in full first.

## What is true when this lands

1. **The cause is found and stated, diagnosis first.** Build both pots at
   `Interval`: the document's (`vessel::document()` in editor-core's
   corpus) and sweep's `corpus_pot` (CAVITYDIAG). Shell each, and read the
   cavity's pair `(2v1, 6v1, Edge(12v1))` in both. Say where the readings
   part:
   - the rim's pcurve on the band, or its classification;
   - the band's chart, its seam or its sense;
   - the arc carrier (`ProgramArcData::Center` against `bulge_from_center`);
   - how the revolve splits the band;
   - or the presentation the departure arm takes.

   Give the values at both builds, then report that part in your
   final report.
2. **The fix is sound, and it is made at the producer.**
   - If the document build produces a carrier, a pcurve or a split that
     differs from sweep's for the same geometry, fix it at that producer
     (the arc, the revolve or the pcurve mint). Do not loosen the
     injectivity reader.
   - If the reader has a real gap that this build exposes, close it at a
     funnelled site with an audit row.
   - No new ε without a verdict that needs it: revert any concession
     alone and diff the verdicts.
   - If the honest answer at `Interval` is a refusal, stop and report.
3. **Rows.**
   - The document pot's shell certifies at `Interval`, sealed and
     opened. Pin it in editor-core, or at the producer's crate if the
     fix lives there.
   - The sweep pot still holds.
   - Every self-intersection and pipe row keeps its verdict class at
     both scalars.
   - f64 receipts are bit-identical; list any that move.
   - Mutation: reverting the fix, and making any decision it adds lax,
     each red a row.
4. **Nothing else moves.** If the fix is at the arc or the revolve
   producer, run the editor-core and sweep suites that build through it,
   and list any moved receipt with its cause.

## Ground

- `crates/editor-core` (corpus, document evaluation), the revolve and arc
  producers, and `crates/topo/src/injective.rs` and `clearance.rs`.
- `docs/predicate-dimension-audit.md` if a site is added.
- SHELL-4's branch `clear/shell4` (`3a6c20799f`) has the gate wired in.
  The failing row is
  `lib_g17_shell_node::both_documents_evaluate_green_at_the_interval_scalar`.
  You can reproduce the refusal without the gate by running the
  self-intersection check on the cavity clone directly. Do not merge
  `clear/shell4` into this branch.
- These editor-core rows are red on main and not ours, filed by GREEN:
  `m10_3_driver_interval` and `m10_9_pins_interval`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤100 lines, covering:
- the cause, with values at both builds;
- the fix and its soundness argument;
- the rows and receipts;
- the mutants;
- any moved receipts;
- the PR number and its CI result.
