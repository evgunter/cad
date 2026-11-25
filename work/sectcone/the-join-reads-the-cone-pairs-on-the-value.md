---
id: the-join-reads-the-cone-pairs-on-the-value
kind: unit
title: The join reads cone × oblique cylinder and cone × cone on the section value: the carried lane, the apex node, GermFrameUnsupported retired
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: cone-germ-pairs-against-a-curved-face-have-no-section-frame
---


U3 of `cone-germ-pairs-against-a-curved-face-have-no-section-frame`. It waits on U2 and on the cone's limb-2 reading at the search (`the-search-certificate-reads-limb-two-through-the-edge-reader`), because `PairSection::carry` marches the cone pairs.

The work:

- **The join:** `join.rs` `rides_carried` arms for `PairPose::{ConeCylinder, ConeCone}` (replacing PR 7357's compile-forced `JoinDesync` arm), and a desync arm in `pair_section_frame_at`.
- **The apex node:** the node pass reads the apex node, in `node.rs`.
- **Carry:** the carry's `NoCarrier::ConeUnenclosed` refusal retires once the march certifies.
- **Coverage:** `ruling.rs`'s `coverage` cone side gets its first caller and a row. PR 7357 built it with no caller.
- **Witnesses that flip:** `crates/sweep/tests/cone_operand_rows.rs` `the_bite_refuses_at_the_germ_frame` and `reach_cone_root_lane.rs`'s tilted-rod row.
- **Downstream:** it unblocks SPLITDOOR B3's follow-on.
