---
id: the-source-and-copy-of-rows-re-point-without-the-offers-check
kind: issue
title: The source row and the copy-of row offer the re-point without the offer's refusal check, under a third verb each
status: closed
branch: placing/repoint-buttons-one-check
pr: 6948
opened: 2026-11-13
priority: P3
cost: E
closed: 2026-11-14
---


Found by both designers of the fork `placing-declined-offer` (`[ev]` PR 6915), off the question.

**What.** One edit, `SessionOp::RepointPlacements`, has three buttons:
- the re-point offer (`world::repoint_offer`, "Re-point bracket onto F"), whose rows ask `session::repoint_refusal` and are withheld when it refuses;
- the named source's row in the Properties pane (`pane/properties.rs`, `source_row_ui`, "read the selected body");
- the placement editor's "copy of" row (`pane/create.rs`, `copy_of_row`, "copy the selected body").

The last two are enabled for any selected node that has a body read, and they ask no refusal. So beside an offer that correctly withholds a button (the `U = B ∪ C1` cycle in `the_offer_and_its_button_re_point_the_same_placements`), the pane can show an enabled button whose op refuses loudly. That is one list computed two ways, and three verbs for one edit.

**Done when** all three read one check (`world::repoint_targets`' `redefinable`) and use one verb, and a row shows the source row's button disabled where the offer withholds its row.
