---
id: main-red-the-rook-story-transform-is-not-offered
kind: issue
title: Main red: the chess-rook story's Transform into the cut's space is not offered (offer answers None, no refusal)
status: review
pr: 6947
branch: placing/rook-story-red
opened: 2026-11-14
priority: P1
cost: E
---


Found by `placing/p3p4-cleanup` running the viewer's whole app-feature
suite locally (2026-11-14); reproduced at origin/main `e3f7fa2f4b`
(the merge of PR 6919) with this branch's viewer sources reverted, so
it is main's, not the branch's.

`viewer::all story_authoring::a_chess_rook_is_authored_probed_branched_and_reopened`
(`crates/viewer/tests/story_authoring.rs`, the `transformed_into` call
that moves the cutter into the cut's space) panics in
`common::transform` (`crates/viewer/tests/common/mod.rs`, the
`op.unwrap_or_else` after `tool.offer`): "the door takes the change:
None". `MateTool::offer` (`crates/viewer/src/matetool.rs`,
`offer_proposed`) answered `None` with no refusal from the door and no
`pending()` line, so either the draft was not `unsettled()` after the
values were typed, or the proposal wrote nothing
(`session::bundle_writes_nothing`). The row is in the slow set
(`.config/nextest.toml`), so the PR gate did not run it.

**Owed.** Find which arm answers `None` and fix the editor or the row.
**Done when** the story row passes at origin/main.

## Cause

The culprit is PR 6856, merge `a9df07339b`: the row is green at its
first parent and red at the merge. Bisected on the branch's first-parent
line, the first bad commit is `afc6e6d919`. The two commits before it
do not build the viewer, so they are skipped. Neither 6916 nor 6919 is
involved, because the row is already red at `8240515cee^1`.

On main the mechanism is `MateTool::take_place`
(`crates/viewer/src/matetool.rs`): since 6856, a second Place of a
body whose one source is anonymous and already read asks for a name
(`world::place_asks`, `PlaceAsks::Name`). VARIABLES VR4
(`crates/editor-core/VARIABLES.md`, Ev on PR 6672) says two placements
share a source "only by its name, which the person gives when the body
is placed a second time". The cutter is placed once by `lifted`, so
its second Place asks for that name. `common::transform` took no
Place, so the tool held no draft. `set_target` and `type_value` then
did nothing, and `offer_proposed` answered `None` at its `unsettled()`
guard. The story went stale; the editor is right.

The same merge left five editor-core slow-set id pins red. Each
placement now mints a source, so node ids moved:
- `edit_placement_corpus_bits`;
- `m10_p_fence` at f64 and at Interval, plus the `probe`-gated row;
- `m4_pr3_names_ci::die_name_tables_are_golden`;
- `name_words_corpus`.

The id-masked fence holds. With ids masked, the die tables and the
said words are line for line the same.
