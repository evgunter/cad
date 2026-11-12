---
id: a-slider-placed-over-a-box-certifies-and-its-clearance-fold-runs-out-of-cell-pairs
kind: issue
title: the channel slider placed over a ±0.05 box certifies in one leaf, and the clearance fold over it refuses on the cell-pair budget
status: open
opened: 2026-11-10
priority: P2
---


Found by ONEHOME stage 2 (`onehome/rigid-motion`, the `moved` door). `crates/editor-core/tests/m10_5_r1_probes_interval.rs` `e2e_a_real_tolerance_gets_a_holds_over_zero_leaves` places the slider into the channel at `1 + place`, `place` uniform over ±0.05 m (`channel_and_slider`, `:946`).

- **On main** no leaf replays over that box: the transform's re-certification refuses. The drive certifies 0 leaves, and `clearance_over_at_dials` refuses `NothingCertified`.
- **With the move carrying its verdict** (`topo::LeafBody::moved`), the drive certifies the whole box in one leaf: `certified = 1, refused = 0`, accounting `certified: Ok(1.0)`.
- **The fold over that leaf** (`at_least(0.3)`, `max_leaves: 64`) then refuses `Engine { refusal: Budget(Pairs { max_cell_pairs: 65536 }) }`, sides 0 and 0.

So the refusal moves later: from "nothing was proved" to "the sweep ran out of cell pairs on the one certified leaf". The row passes through its non-empty arm, which only prints.

What is owed (CLEAR, the clearance sweep):
- Say whether a channel whose walls stand 0.45 to 0.55 m off the slider over the whole box should discharge `≥ 0.3` within the default budget.
- If it should, find which pairs the sweep fails to prune (the box-wide reach of the slider's faces is the suspect).
- Either way, pin the row's verdict, which today only prints.

Reproduce: `cargo nextest run -p editor-core --profile ci --ignore-default-filter --no-capture -E 'test(=m10_5_r1_probes_interval::e2e_a_real_tolerance_gets_a_holds_over_zero_leaves)'` on the stage-2 branch.
