---
id: an-edited-profile-in-a-moved-space-is-previewed-where-it-stood
kind: issue
title: viewer: a profile in a free-moved space is previewed by its edit where it stood, not where it is drawn
status: open
opened: 2026-11-25
priority: P3
cost: E
---

Found building `vgeom/profile-display` (the rows `a-profile-has-no-display-toggle` and `a-profile-has-no-free-move`).

A committed outline in a free-moved space is drawn and picked under that space's display frame (`display::DisplayView::outline`, read by `pane::viewport::push_committed` and `pickindex::PickIndex::drawn_outlines`). While the edit door holds that profile, the committed lane leaves it out and the preview lane draws the edit's preview instead (`pane::viewport::push_previews`, from `sketch::ProfilePreview::plane`), and the pick measures the preview's outline (`display::PreviewedProfile::outline`, built in `PreviewedProfile::of`). Neither reads the display frame. So opening the edit of a moved profile makes its outline jump back to where its space stands unmoved, and it is drawn and picked there (the two agree, so nothing is picked where nothing is drawn) until the edit closes.

What it should do: the preview of a node in a moved space is drawn and picked under the same frame, through `DisplayView::outline` with the node's space. `PreviewedProfile::of` is handed the node only, so the space has to be read beside it (`Doc::node_space`), and the preview lane needs the display view it is not handed today.

