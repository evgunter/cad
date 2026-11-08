---
id: the-pattern-gesture-evaluates-the-whole-document-to-read-its-pick
kind: issue
title: picked_heading evaluates the whole committed document on every pattern commit to read one pick's direction
status: open
opened: 2026-11-02
priority: P4
cost: E
---


Found by the PR 6394 dual review (R2 S5), filed by its fix pass.

`DocSession::picked_heading` (`crates/viewer/src/session.rs`) calls `pncad::document::evaluate` on the committed document every time a pattern commits on a picked edge or face, to read the pick's direction through `editor_core::eval::bound_pose`. The landed evaluation (`DocSession::landed_pair`) already holds every body the pick reads when it answers the committed document.

**Owed.** Read against the landed evaluation where it answers the committed document, and evaluate only where it does not.
