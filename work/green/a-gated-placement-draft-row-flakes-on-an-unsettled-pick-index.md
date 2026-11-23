---
id: a-gated-placement-draft-row-flakes-on-an-unsettled-pick-index
kind: issue
title: app::properties_pane_tests::an_offered_placement_draft_is_drawn_where_it_stands_until_its_run_lands fails intermittently: the settled app draws from no index
status: open
opened: 2026-11-23
priority: P3
cost: E
---


**The row:** `crates/viewer/src/app.rs`, `properties_pane_tests::an_offered_placement_draft_is_drawn_where_it_stands_until_its_run_lands`. It fails intermittently under load and passes alone. The panic is `Driven::on_screen`'s expect, "the settled app draws from an index" (app.rs ~8426): `pane::viewport::drawn_index(picks.index(), scene_key)` answered `None` at a point the row treats as settled.

**Evidence:**
- **Seen twice, on two unrelated heads.**
  - In #7356's re-review (2026-11-22, d496705c): failed once in a full viewer run under an unrelated mutant, then passed 3/3 at head and 1/1 under that mutant.
  - In the orchestrator's spot-check of #7562 (2026-11-23, 43d380c2): failed once in a 102-row filtered run with `--features app,test-support` at opt-level 1, then passed 6/6 run alone.
- Neither PR touched the pick index or the gated harness, so it is most likely a race between the gated run's landing and the pick index's rebuild.

**Owed:** find what "settled" waits on in `gated()` / `offered_behind_a_held_run`, and make the row wait for the pick index too, or say why it need not. Do not retry past it.

(OFFER orchestrator)
