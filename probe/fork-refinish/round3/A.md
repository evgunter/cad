RECONCILIATION ROUND 3, A. Tracing the readers in the tree settles both points on B's side, with one small difference left.

**1. Who reads each number** (traced in the tree)
- **The drive's decision totals (`ParamBoxVerdict::decisions`, the sum over leaves today).** Four readers:
  - The report line `render`: "N of {total} decisions were proven zero by the symbolic tier (share%)". It is a ratio, read as evidence for E12 ("the ratio of symbolic to numeric decisions is itself the E12 evidence").
  - Pinned totals: `m10_8_harness` (`(certified == 1, v.decisions())`), and `m10_9_pins_interval` (`open.decisions() >= shut.decisions()`; `off.decisions() != on.decisions()` for a dial).
  - The plain-memo differential, `m10_sym_drive_memo_interval`: it asserts `on.decisions() == off.decisions()` and reads `frozen`.
  - I found no consumer in the viewer, pncad-py or `tools/k-lint`. k-lint reads K samples, not drive receipts.
  - So the pins and the differential decide from the sum, and from its *invariance under a memo dial*. That precedent already ships: `DriveConfig::plain_memo` is "a dial whose effect on a document is a measurement rather than an assumption".
- **The `k_stats` funnel.** The drive's own `Sym<Interval>` replays put no `MarginSample` in it. Escalations do go on the verdict log, but that log rides the node's value, and `classify_replay` reads it per leaf. Samples reach the funnel only through `KProbe` (probe builds only). `KProbe` replays each *certified* leaf at `Probe` over that leaf's midpoint, and its doc says plainly that it is narrower than E6's sentence.
- **The population K was ratified on** (K-REPORT) is the corpus harness (`sweep/tests/k_report.rs`, `dump_k_samples`). It is not drive replays. So no ratified K statistic counts a decision repeated in 1,023 leaves at all. The memo touches the funnel only through `KProbe`'s midpoint replays, and there B's rule (a node's records travel with its value) keeps it exactly as it is today.

**2. Is "memo off" a mode?**
- Yes, and it should be the same dial as `plain_memo` (B's round-1 note: one dial, not two). It ships as a measurement dial, and the differential rows pin it.
- **Must not change with it:** the verdict, every leaf and its reason, the leaf receipts, the drive's `decisions()` totals, the verdict logs, and `KProbe`'s samples.
- **Expected to change:** time, and anything that counts work done.
- The differential compares the first list. So whatever carries the name `decisions()` must be invariant under the dial. The sum of leaf receipts is, and so is a count of distinct results defined by the tree. A count of what actually ran is not.

**3. Positions**
- **The drive's total: B's.** It stays the sum of the leaf receipts, under today's name. The pins, the report line and the differential all read it that way, it can be re-checked from the shipped leaves, and nothing re-baselines. I withdraw my round-2 change.
- **The one difference left:** I would not ship the new distinct-entries count. It has no reader in the tree. Its one use is to measure what the store saves, and that is the probe's trace (`BOXFINISH_TRACE`), not a receipt column (`[simple.checks]`). If one is shipped, B's condition is right: define it by the tree, so the dial does not change it.
- **`k_stats`: B's.** E6's K paragraph stands as written. What a node records travels with its value, so the funnel does not change with the memo. My round-2 K sentence is withdrawn. E6 text needed for K: none.
- **E6 text** (B's round-2 wording, which I sign exactly):
  - Split rule, replacing "Split rule named, deterministic (D9): max relative width, ties to lowest index":
    > Split rule named, deterministic (D9): of the variables the leaf's deciding node reads (the first undecided node in evaluation order) that have depth left, the one of greatest relative width, ties to lowest index. A leaf whose deciding node reads no variable of the box is refused, naming the node: no split changes it.
  - Leaf protocol, a new bullet:
    > A node whose inputs a leaf leaves unchanged is served from an earlier replay of the drive: result or refusal, and record, whole (DESIGN [determinism]). A leaf's receipt counts what its result rests on, served or built.

**4. The store.** I sign: "one store per drive, keyed by (the node's own id, the content key with the expression in it, the naming key). A measurement under the new split rule, on a document with two refined features, counting time in nodes whose entry repeats only at a cousin, may show the parent-only prior is enough." The two of us differ only in strength: firm here, "unsure" for B. Nothing else in the design depends on which home is chosen.

**For Ev, the one changed paragraph (replaces "Receipts")**
> **Receipts.** A leaf's receipt counts what its result rests on, served or built. The drive's total stays the sum of its leaves' receipts, as today. Both are unchanged by the memo, which becomes the same on/off dial the drive's form memo already is. The tests compare runs with that dial on and off, and today's pinned totals do not move. The K telemetry is unchanged too: what a node records travels with its value. (sure)
