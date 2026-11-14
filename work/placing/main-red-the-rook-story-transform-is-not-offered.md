---
id: main-red-the-rook-story-transform-is-not-offered
kind: issue
title: Main red: the chess-rook story's Transform into the cut's space is not offered (offer answers None, no refusal)
status: open
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
that moves the cutter into the cut's space, about line 324) panics in
`common::transform` (`crates/viewer/tests/common/mod.rs`, the
`op.unwrap_or_else` after `tool.offer`): "the door takes the change:
None". `MateTool::offer` (`crates/viewer/src/matetool.rs`,
`offer_proposed`) answered `None` with no refusal from the door and no
`pending()` line, so either the draft was not `unsettled()` after the
values were typed, or the proposal wrote nothing
(`session::bundle_writes_nothing`). The row is in the slow set
(`.config/nextest.toml`), so the PR gate did not run it; the PLACING
editor batch (PRs 6916, 6919) is the recent change to that path.

**Owed.** Find which arm answers `None` and fix the editor or the row.
**Done when** the story row passes at origin/main.
