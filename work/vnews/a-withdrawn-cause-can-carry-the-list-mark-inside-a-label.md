---
id: a-withdrawn-cause-can-carry-the-list-mark-inside-a-label
kind: issue
title: A withdrawn cause can carry the list mark inside a node's quoted label
status: open
opened: 2026-10-02
priority: P4
cost: E
---


Found by `viewer-refusals-speak-the-node`. `AdmissionFault` now speaks its nodes with their labels (`display.rs`, `SpokenNode` fields built from the document at the admission test). A label is author text: `Label::new` refuses line breaks and control characters, and nothing else, so a label may hold `; `, which is `frame::LIST_SEPARATOR`.

`Display for Withdrawal` joins a withdrawal's causes on `LIST_SEPARATOR`, and `frame_policy::a_withdrawn_cause_never_carries_the_list_mark` (`tests/frame_policy.rs`) holds that no cause writes the mark. Its population is `every_cause()`, forged with absent nodes (`node <tag>`), so it still passes; a cause naming an instance labelled `left; right` would write the mark inside one item.

`SpokenNode` quotes a label (`InstantiatePart "left; right" (3fa9c1d2a0b1)`), so a reader can tell, but the claim the test states is no longer true of the type. Decide whether the quotation is the answer (narrow the claim to "outside a quoted label", and plant a labelled cause in `every_cause`) or whether the join needs a mark a label cannot hold.
