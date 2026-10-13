# DECIDE-10 — delete the label kind: a symbolic discharge answers "zero" and nothing about how (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/the-read-at-its-node-relabels-a-cancellation-above-it.md`
(P2). **Unit:** `work/decide/DECIDE-10.md`.
**Branch:** `decide/10-delete-the-kind`, cut from `main`.
**Implementer:** Opus. **Review tier:** dual, concurrent arm,
difficulty H (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item, whole. Its section "Question, and Ev's answer" is the
  ruling this unit builds;
- the closed draft PR #4095: its body, and its copy of the item
  (branch `decide/10-read-behind-the-parent`, sections "What Phase 1
  found" and "The fix pass"). That is the measured record of the
  shapes S1–S13 and C1–C7;
- `crates/geom-core/src/predicate.rs`: `Theorem`, `Zero`, `ZeroWord`,
  `MarginDiag` (`theorem`, `proven`, `jointly`, `tightens_below`);
- `crates/geom-core/src/sym.rs`, whole:
  - the header's "The decision read" and the rules table;
  - `Discharge`, `count_decision`, `SymCounts`, `rungs`, `discharge`,
    `discharge_retried`, `form_in`;
  - `zero_factors_gate`, `door_zero`, and `signed_root` (rule C);
- `crates/geom-core/src/sym/form.rs` (`Form::gated`, `Form::digest`),
  `sym/signed.rs`, `sym/deferred.rs` (`discharged`), `sym/report.rs`
  and `sym/revalue.rs`;
- `crates/geom-core/src/k_stats.rs` (`SampleOutcome`), and
  `tools/k-lint`'s vocabulary;
- `crates/topo/src/` `Discharge`, `Coincidence`, `Proof`;
- `crates/editor-core/src/coincide.rs` and `coincide/replay.rs`
  (`Rung::MarginIdentity`, `Rung::CarrierVerdict`), and `drive.rs`'s
  summary text.

## The ruling

Ev, on the `[ev]` PR (2026-10-12), took the designers' answer:
- **The label kind is deleted.** The kinds are `symbolic_zero`,
  `sign_gated` and `registered`: how a zero was proved.
- **A symbolic discharge answers one fact: "this margin is zero".**
- **Nothing may be able to branch on how.** That is certain by type,
  because no type a decision travels in holds the kind.

**Why deleting the kind loses nothing.** Each kind discharges E12's
claim: zero at every value where the construction takes the decisions
it took.
- **A theorem:** zero everywhere.
- **A read's zero:** a polynomial identity with a `min`/`max`/`Select`
  replaced by the arm the construction took.
- **A registered zero:** it rests on a constructor's axiom, which is
  checked at the witness.

**Rule C (`signed_root`) is the exception.** Its condition is a sign of
the box, not a branch the construction took.

## The change

1. **Decision types carry "proven", not "how".**
   - `Theorem`'s kinds go, and `Zero::Proven` carries nothing.
   - `MarginDiag::theorem()` becomes a "proven" bit.
   - `jointly` is proven if both are: no `Ord`, no `max`.
   - `ZeroWord` re-encodes.
   - `deferred::discharged` answers proven or not.
   - `topo::Discharge` becomes `{Numeric, Proven}`, and
     `Rung::MarginIdentity` and `Rung::CarrierVerdict` carry nothing.
   - `sym::revalue`'s recorder stores "proven".
2. **The read's `gated` flag on forms goes, with its bit in
   `Form::digest`.**
   - An atom is then keyed by its argument's polynomial, which fixes
     S8 and C6.
   - `zero_factors_gate` retires. It is DECIDE-9's zero-product rule,
     which only ever chose a label; it retires by Ev's ruling, and the
     PR says so.
   - The door's refusal of a gated zero goes (S12), and `door_zero`
     sees the read's zeros.
3. **Rule C retires.** Delete `signed_root` and its dial: a zero it
   reaches is not E12's claim. If any measured row or shipped
   configuration depends on it, stop and report.
4. **The receipt and the K vocabulary lose the split.**
   - `SymCounts` keeps `symbolic`, `numeric`, `frozen`, `retried`,
     and the two registry soundness columns, `registrations_refused`
     and `registrations_contradicted`.
   - The K tokens `sign_gated` and `registered` retire, and `k-lint`
     follows.
   - `drive.rs`'s summary text says one number for the tier.
5. **What a dial buys is a differential.** A row per measured family
   asserts, in decisions and verdicts, the evaluation with the dial off:
   - the read off (`SymRules::without_the_reads`), on the tilted
     derived-frame family: the end cap refuses at
     `newell_plane_residual` with the read off; the start cap does not;
   - the registry off (`registered: false`), on the documents whose
     `registered` column was non-zero.

   These replace the per-kind columns as the measurement of what the
   read and the registry buy.
6. **Docs.** Say only what the code now does:
   - the read's doc and rules-table row, and the header's "The decision
     read" (the read answers at its node, and a zero is a zero);
   - `signed.rs` "Where it runs";
   - `Form`'s docs;
   - `SymCounts`' docs;
   - any crate README design page that states the label kinds (DESIGN's
     companion table names them).

   The text that goes was written by agents (the designers checked with
   `git log -S`). DECIDE-9's rule is the one ratified clause that
   retires, and it retires by Ev's ruling.

**Rows:**
- **The shapes.** S1–S13 and C1–C7, from #4095's record, become rows
  asserting *answered* or *refused* with the read on, and the same with
  it off. S8 and S12 become answered. S13 stays refused: a freeze is
  keyed by its node. File it as its own P3 on DECIDE, naming its two
  candidates: a freeze keyed by its content, or a measured read-free
  retry mask.
- **No decision's value and no verdict moves** on the measured
  documents: the bracket replay at `certifies_at`, the pad's release
  leaf at `1e2·ε`, `m10_10_pins`, and the tilted family. The receipts
  re-baseline to totals; the sum of the old columns is the new
  `symbolic`, and each pin says so with before and after. Any document
  whose `numeric` count moves is a finding: report it, and do not
  absorb it into a re-baseline. Removing the gate bit from atom keys
  may merge atoms; C6 moves from numeric to answered by design.
- **The leaf cost:** the pad, release, best of 3, against `main`. It
  should not rise.
- Ev's standing words: "never skip out on a change that would make the
  code better because it would require rebaselining".

**Sweep.** Grep for every reader of the kind: `Theorem::`, `.theorem()`,
`sign_gated`, `registered` as a count, `SignGated`, `jointly`,
`weakest`, and `gated` on forms. Put the hit list and each hit's
disposition in the PR body. A crate outside the workspace (`tools/*`,
`demos/*`, `pncad-py`) is not in `--workspace`; grep it too.

**Stop rules.**
- Rule C is depended on (item 3): stop and report.
- A `numeric` count or a verdict moves on a measured document: report
  it, with the decision's identity. The orchestrator decides.

## Scope

- **Files:**
  - `geom-core` (`predicate.rs`, `sym.rs`, `sym/*`, `k_stats.rs`);
  - `topo`'s discharge types;
  - `editor-core` (`coincide*`, `drive.rs`);
  - `tools/k-lint`;
  - `pncad-py`, if it names a kind;
  - every test that pins a per-kind count;
  - the item, the unit, and the new S13 item.
- **Territory:** run `python3 scripts/work.py territory --base
  origin/main` and announce every seam.

## Review

**Dual review, concurrent arm, difficulty H**, recorded in
`work/decide/log.md` at spec time, per `docs/DUAL-REVIEW-PROTOCOL.md`.
It is a soundness argument (every kind is E12's claim) carried by a
type change across five crates. Claims to falsify:
1. **No reader can branch on how a zero was proved.** Find a path that
   still can.
2. **Each deleted kind was E12's claim.** Build a zero that the read,
   or the registry, reaches and that does not hold wherever the
   construction takes the decisions it took.
3. **Removing the gate bit from atom keys is sound within a leaf.** It
   merges atoms that stand for the same real at every point of the box.
4. **No value or verdict moves on the measured documents,** and every
   re-baseline is a sum.
5. **The differential rows measure what the read and the registry buy.**
6. Plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`, and #4095 is
  closed unmerged with a pointer to the new PR.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item.
